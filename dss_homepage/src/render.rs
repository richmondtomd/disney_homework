use crate::api::api::download_image;
use crate::opengl_models::models::{Focus, Grid, Row, Tile, TileData};
use dss_models::home::ApiContent;
use dss_models::set_ref::RefContent;

use std::thread;

use sdl2::image as SDLImage;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureQuery;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::time::Duration;

use image as Img;

pub fn render(canvas: &mut WindowCanvas, color: Color, grid: &mut Grid) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(color);
    canvas.clear();

    (grid.bound_x, grid.bound_y) = canvas.output_size()?;

    let mut tile_handles: Vec<thread::JoinHandle<Result<(), String>>> = vec![];

    //draw grid
    let mut start_y = 40;
    let row_count = grid.rows.len() as i32;
    'row: for row_index in grid.hidden_rows..row_count {
        let row = &mut grid.rows[row_index as usize];

        match render_row(
            canvas,
            row,
            &mut grid.focus,
            start_y,
            grid.bound_y,
            grid.bound_x,
            &texture_creator,
            &mut tile_handles
        ) {
            Ok(_) => {}
            Err(_) => break 'row,
        };

        start_y = start_y + (row.tiles[0].tile.height() as i32) + 45
    }

    for tile_handle in tile_handles {
        tile_handle.join().unwrap();
    }

    canvas.present();

    Ok(())
}

fn render_row(
    canvas: &mut WindowCanvas,
    row: &mut Row,
    focus: &mut Focus,
    start_y: i32,
    bound_y: u32,
    bound_x: u32,
    texture_creator: &TextureCreator<WindowContext>,
    tile_handles: &mut Vec<thread::JoinHandle<Result<(), String>>>
) -> Result<(), String> {
    let mut start_x = 40;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("./assets/Avenir-Roman.ttf", 14)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    if start_y > bound_y as i32 {
        return Err(String::from("Out of bounds"));
    }

    let surface = font
        .render(row.title.as_ref())
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::from_center(
        Point::new(start_x + (width as i32) / 2, start_y + (height as i32) / 2),
        width,
        height,
    );
    canvas.copy(&texture, None, target)?;

    let mut tile_index = row.hidden_tiles;
    let tile_count = row.tiles.len() as i32;

    'tile: while tile_index < tile_count {
        let tile = &mut row.tiles[tile_index as usize];

        let screen_position;
        let screen_rect;

        if tile.focused {
            screen_position = tile.position
                + Point::new(
                    (tile.tile.width() as i32) / 2 + start_x,
                    (tile.tile.height() as i32) / 2 + start_y + 5 + ((height as i32) * 1),
                );
            screen_rect = Rect::from_center(
                screen_position,
                tile.tile.width() + 16,
                tile.tile.height() + 9,
            );
        } else {
            screen_position = tile.position
                + Point::new(
                    (tile.tile.width() as i32) / 2 + start_x,
                    (tile.tile.height() as i32) / 2 + start_y + 5 + ((height as i32) * 1),
                );
            screen_rect = Rect::from_center(screen_position, tile.tile.width(), tile.tile.height());
        }

        match render_tile(canvas, tile, focus, bound_x, screen_position, screen_rect, tile_handles) {
            Ok(_) => {
                start_x = start_x + (tile.tile.width() as i32) + 40;
                tile_index += 1;
            }
            Err(err) => match err.as_ref() {
                "Failed to download image" => {
                    row.tiles.remove(tile_index as usize);
                    continue 'tile;
                }
                "Out of bounds" => {
                    break 'tile;
                }
                "Path not yet set" => {
                    tile_index += 1;
                },
                _ => {
                    return Err(String::from("Unknown Error"));
                }
            },
        }
    }

    Ok(())
}

fn render_tile(
    canvas: &mut WindowCanvas,
    tile: &mut Tile,
    focus: &mut Focus,
    bound_x: u32,
    screen_position: Point,
    screen_rect: Rect,
    tile_handles: &mut Vec<thread::JoinHandle<Result<(), String>>>
) -> Result<(), String> {
    if screen_rect.x() > bound_x as i32 {
        return Err(String::from("Out of bounds"));
    }

    let image_path = format!("./assets/images/{}.jpeg", tile.tile_data.image_id);
    if tile.tile_data.image_path.is_none() {
        let image_url = tile.tile_data.image_url.clone();
        let image_path_clone = image_path.clone();
        let tile_handle = std::thread::spawn(move || {
            match download_image(&image_url, &image_path_clone) {
                Ok(_) => {
                    Ok(())
                },
                Err(_) => return Err(String::from("Failed to download image")),
            }
        });
        tile_handles.push(tile_handle);
        tile.tile_data.image_path = Some(image_path);
    }
    
    let texture_creator = canvas.texture_creator();
    let texture = match texture_creator.load_texture(format!("./assets/images/{}.jpeg", tile.tile_data.image_id)) {
        Ok(texture) => { texture },
        Err(_) => return Err(String::from("Path not yet set")),
    };

    canvas.copy(&texture, None, screen_rect)?;

    //Set focus
    if tile.focused {
        focus.position = screen_position;
        let screen_focus = Rect::from_center(
            focus.position,
            focus.tile.width() + 16,
            focus.tile.height() + 9,
        );
        canvas.draw_rect(screen_focus)?;
        canvas.set_draw_color(Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        });
        canvas.draw_rect(screen_rect)?;
    }

    Ok(())
}

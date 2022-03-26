use dss_models::home::ApiContent;
use dss_models::set_ref::RefContent;
use crate::opengl_models::models::{Grid, Tile, Row, TitleData, Focus, TileTexture};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::image as SDLImage;
use std::time::Duration;
use sdl2::render::TextureQuery;

use image as Img;

pub fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    grid: &mut Grid
) -> Result<(), String> {

    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(color);
    canvas.clear();

    (grid.bound_x, grid.bound_y) = canvas.output_size()?;

    //draw grid
    let mut start_y = 40;
    let row_count = (grid.rows.len() as i32);
    'row: for row_index in grid.hidden_rows..row_count {
        let row = &mut grid.rows[row_index as usize];
        let mut start_x = 40;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let mut font = ttf_context.load_font("./assets/Avenir-Roman.ttf", 14)?;
        font.set_style(sdl2::ttf::FontStyle::NORMAL);

        if start_y > grid.bound_y as i32 {
            break 'row
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

        let target = Rect::from_center(Point::new(start_x + (width as i32) / 2, start_y + (height as i32) / 2), width, height);
        canvas.copy(&texture, None, target)?;

        let mut tile_index = row.hidden_tiles;
        let tile_count = row.tiles.len() as i32;
        'tile: while tile_index < tile_count {
            let tile = &mut row.tiles[tile_index as usize];

            let mut screen_position;
            let mut screen_rect;

            if tile.focused {
                screen_position = tile.position + Point::new((tile.tile.width() as i32) / 2 + start_x, (tile.tile.height() as i32) / 2 + start_y + 5 + ((height as i32) * 1));
                screen_rect = Rect::from_center(screen_position, tile.tile.width()+16, tile.tile.height()+9);
            }
            else {
                screen_position = tile.position + Point::new((tile.tile.width() as i32) / 2 + start_x, (tile.tile.height() as i32) / 2 + start_y + 5 + ((height as i32) * 1));
                screen_rect = Rect::from_center(screen_position, tile.tile.width(), tile.tile.height());
            }

            if screen_rect.x() > grid.bound_x as i32 {
                break 'tile
            }

            let mut image_path = format!("./assets/images/{}.jpeg", tile.title_data.image_id);

            if tile.title_data.image_path.is_none() {
                match reqwest::blocking::get(format!("{}", tile.title_data.image_url)) {
                    Ok(img) => {
                        match img.bytes() {
                            Ok(img_bytes) => {
                                match image::load_from_memory(&img_bytes) {
                                    Ok(image) => {
                                        match image.save(&image_path) {
                                            Ok(_) => {},
                                            Err(err) => {
                                                println!("unsupported fmt");
                                                row.tiles.remove(tile_index as usize);
                                                continue 'tile
                                            }
                                        }
                                    },
                                    Err(_) => {
                                        println!("unsupported fmt");
                                        row.tiles.remove(tile_index as usize);
                                        continue 'tile
                                    }
                                }
                            },
                            Err(_) => {
                                println!("unsupported fmt");
                                continue 'tile
                            }
                        }
                    },
                    Err(_) => {
                        println!("unsupported fmt");
                        continue 'tile
                    },
                }
                tile.title_data.image_path = Some(image_path.clone());
            }
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator.load_texture(image_path)?;

            canvas.copy(&texture, None, screen_rect)?;

            //Set focus
            if tile.focused {
                grid.focus.position = screen_position;
                let screen_focus = Rect::from_center(grid.focus.position, grid.focus.tile.width()+16, grid.focus.tile.height()+9);
                canvas.draw_rect(screen_focus)?;
                //set focus
                canvas.set_draw_color(Color {r: 255, g: 255, b: 255, a: 255});
                canvas.draw_rect(screen_rect)?;
            }

            start_x = start_x + (tile.tile.width() as i32) + 40;
            tile_index += 1;
        }
        start_y = start_y + (row.tiles[0].tile.height() as i32) + 45
    }

    canvas.present();

    Ok(())
}

// fn render_row (
// ) {
// }

fn render_tile (
    canvas: &mut WindowCanvas,
    focus: &mut Focus,
    tile: &Tile,
    mut start_x: i32,
    start_y: i32,
    mut text_height: u32,
    screen_position: Point, 
    screen_rect: Rect
)  -> Result<(), String> {
    // if tile.focused {
    //     screen_position = tile.position + Point::new((tile.tile.width() as i32) / 2 + start_x, (tile.tile.height() as i32) / 2 + start_y + 5 + ((text_height as i32) * 1));
    //     screen_rect = Rect::from_center(screen_position, tile.tile.width()+16, tile.tile.height()+9);
    // }
    // else {
    //     screen_position = tile.position + Point::new((tile.tile.width() as i32) / 2 + start_x, (tile.tile.height() as i32) / 2 + start_y + 5 + ((text_height as i32) * 1));
    //     screen_rect = Rect::from_center(screen_position, tile.tile.width(), tile.tile.height());
    // }
    //src is none because entire image should be used

    //temporarily using stored image
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./assets/image.jpeg")?;
    canvas.copy(&texture, None, screen_rect)?;

    //Set focus
    if tile.focused {
        focus.position = screen_position;
        let screen_focus = Rect::from_center(focus.position, focus.tile.width()+16, focus.tile.height()+9);
        canvas.draw_rect(screen_focus)?;
        //set focus
        canvas.set_draw_color(Color {r: 255, g: 255, b: 255, a: 255});
        canvas.draw_rect(screen_rect)?;
    }

    start_x = start_x + (tile.tile.width() as i32) + 40;

    Ok(())
}    
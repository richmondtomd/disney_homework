pub mod api;
pub mod grid;
pub mod opengl_models;
pub mod render;

use dss_models::home::ApiContent;
use opengl_models::models::Grid;

use sdl2::event::Event;
use sdl2::image as SDLImage;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Get response from home api
    let request_url = "https://cd-static.bamgrid.com/dp-117731241344/home.json";
    let home_json: ApiContent = api::api::deserialize_api::<ApiContent>(String::from(request_url));

    // Build Home Grid initial position
    let mut home_grid = grid::populate_grid(home_json).expect("Grid failed to populate");

    // Run Application
    let _ = build_app(&mut home_grid);

    Ok(())
}

pub fn build_app(home_grid: &mut Grid) -> Result<(), String> {
    // Create window
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = SDLImage::init(InitFlag::PNG | InitFlag::JPG)?;
    let canvas_width = 1200;
    let canvas_height = 675;

    home_grid.bound_x = canvas_width;
    home_grid.bound_y = canvas_height;

    let window = video_subsystem
        .window("Disney+ Homescreen", canvas_width, canvas_height)
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut render = true;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if home_grid.focus_x > 0 {
                        home_grid.rows[home_grid.focus_y as usize].tiles
                            [home_grid.focus_x as usize]
                            .focused = false;
                        home_grid.focus_x -= 1;
                        home_grid.rows[home_grid.focus_y as usize].tiles
                            [home_grid.focus_x as usize]
                            .focused = true;

                        if home_grid.focus.position.x() - (home_grid.focus.tile.width() as i32) < 0
                        {
                            home_grid.rows[home_grid.focus_y as usize].hidden_tiles =
                                home_grid.rows[home_grid.focus_y as usize].hidden_tiles - 1;
                        } else {
                            home_grid.focus.position = home_grid.rows[home_grid.focus_y as usize]
                                .tiles[home_grid.focus_x as usize]
                                .position;
                            home_grid.screen_x -= 1;
                        }
                    }
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if home_grid.focus_x
                        < (home_grid.rows[home_grid.focus_y as usize].tiles.len() as u32) - 1
                    {
                        home_grid.rows[home_grid.focus_y as usize].tiles
                            [home_grid.focus_x as usize]
                            .focused = false;
                        home_grid.focus_x += 1;
                        home_grid.rows[home_grid.focus_y as usize].tiles
                            [home_grid.focus_x as usize]
                            .focused = true;

                        if home_grid.focus.position.x()
                            + (home_grid.focus.tile.width() as i32 * 3 / 2)
                            + 40
                            > (home_grid.bound_x as i32)
                        {
                            home_grid.rows[home_grid.focus_y as usize].hidden_tiles =
                                home_grid.rows[home_grid.focus_y as usize].hidden_tiles + 1;
                        } else {
                            home_grid.focus.position = home_grid.rows[home_grid.focus_y as usize]
                                .tiles[home_grid.focus_x as usize]
                                .position;
                            home_grid.screen_x += 1;
                        }
                    }
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if home_grid.focus_y > 0 {
                        home_grid.rows[home_grid.focus_y as usize].tiles
                            [home_grid.focus_x as usize]
                            .focused = false;
                        home_grid.focus_y -= 1;
                        home_grid.focus_x =
                            (home_grid.rows[home_grid.focus_y as usize].hidden_tiles
                                + home_grid.screen_x) as u32;
                        home_grid.rows[home_grid.focus_y as usize].tiles
                            [home_grid.focus_x as usize]
                            .focused = true;

                        if home_grid.focus.position.y() - (home_grid.focus.tile.height() as i32) * 2
                            < 0
                        {
                            home_grid.hidden_rows = home_grid.hidden_rows - 1;
                        } else if home_grid.hidden_rows > 0 {
                            home_grid.focus.position = home_grid.rows[home_grid.focus_y as usize]
                                .tiles[home_grid.focus_x as usize]
                                .position;
                        }
                    }
                    render = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if home_grid.focus_y < home_grid.rows.len() as u32 - 1 {
                        home_grid.rows[home_grid.focus_y as usize].tiles
                            [home_grid.focus_x as usize]
                            .focused = false;
                        home_grid.focus_y += 1;
                        home_grid.focus_x =
                            (home_grid.rows[home_grid.focus_y as usize].hidden_tiles
                                + home_grid.screen_x) as u32;
                        home_grid.rows[home_grid.focus_y as usize].tiles
                            [home_grid.focus_x as usize]
                            .focused = true;

                        if home_grid.focus.position.y()
                            + (home_grid.focus.tile.height() as i32 * 3 / 2)
                            + 45
                            > (home_grid.bound_y as i32)
                        {
                            home_grid.hidden_rows = home_grid.hidden_rows + 1;
                        } else {
                            home_grid.focus.position = home_grid.rows[home_grid.focus_y as usize]
                                .tiles[home_grid.focus_x as usize]
                                .position;
                        }
                    }
                    render = true;
                }
                _ => {}
            }
        }

        if render {
            render::render(&mut canvas, Color::RGB(3, 5, 20), home_grid)?
        };
        render = false;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

use crate::api;
use crate::opengl_models::models::{Focus, Grid, Row, Tile, TileData};



use std::thread;
use std::sync::{Mutex, Arc};

use sdl2::image as SDLImage;
use sdl2::image::InitFlag;
use sdl2::rect::{Point, Rect};

use dss_models::{home::ApiContent, set_ref::RefContent};

pub fn populate_grid(content: ApiContent) -> Result<Grid, String> {
    let standard_collection = content.data.standard_collection;

    // Create window
    let sdl_context = sdl2::init()?;
    let _video_subsystem = sdl_context.video()?;
    let _image_context = SDLImage::init(InitFlag::PNG | InitFlag::JPG)?;

    let position = Point::new(0, 0);
    let mut unfocused = true;

    //Create empty Grid with starting config
    let mut home_grid = Grid {
        rows: vec![],
        focus_x: 0,
        focus_y: 0,
        screen_x: 0,
        hidden_rows: 0,
        focus: Focus {
            position,
            tile: Rect::new(0, 0, 222, 125),
        },
        bound_x: 0,
        bound_y: 0,
    };

    let mut tile_handles: Vec<std::thread::JoinHandle<Result<(), String>>> = vec![];
    //Populate Grid
    for container in standard_collection.containers {
        // New Row
        let mut row: Row = Row {
            title: container.set.text.title.full.set.default.content.unwrap(),
            tiles: vec![],
            hidden_tiles: 0,
        };
        // Add Tiles to Row
        if container.set.items.is_some() {
            let items = container.set.items.unwrap();
            let tile_count = items.len();

            for item in items {
                // Set image metadata. Not downloaded until in screen.

                let image_url_clone = item.image.tile.image_component.series.default.url.clone();
                let image_id = item.image.tile.image_component.series.default.master_id.clone();
                let image_path = format!("./assets/images/{}.jpeg", image_id).clone();

                let tile_handle = std::thread::spawn(move || {
                    match api::api::download_image(&image_url_clone, &image_path) {
                        Ok(_) => {
                            Ok(())
                        },
                        Err(_) => return Err(String::from("Failed to download image")),
                    }
                });

                let tile = Tile {
                    position,
                    width: 222,
                    height: 125,
                    tile: Rect::new(0, 0, 222, 125),
                    focused: unfocused,
                    tile_data: TileData {
                        image_id: item.image.tile.image_component.series.default.master_id,
                        image_url: item.image.tile.image_component.series.default.url,
                        image_path: format!("./assets/images/{}.jpeg", image_id),
                    },
                };
                if unfocused {
                    unfocused = false
                };

                row.tiles.push(tile);

                tile_handles.push(tile_handle);
            }
        } else {
            if container.set.ref_id.is_some() {
                // Set image metadata. Not downloaded until in screen.
                let ref_id = container.set.ref_id.unwrap();
                let ref_url = format!(
                    "https://cd-static.bamgrid.com/dp-117731241344/sets/{}.json",
                    ref_id
                );
                let ref_api: RefContent =
                    api::api::deserialize_api::<RefContent>(String::from(ref_url));

                let items = ref_api.data.set.unwrap().items.unwrap();

                for item in items {

                    // Set image metadata. Not downloaded until in screen.
                    let image_url_clone = item.image.tile.image_component.series.default.url.clone();
                    let image_id = item.image.tile.image_component.series.default.master_id.clone();
                    let image_path = format!("./assets/images/{}.jpeg", image_id).clone();

                    let tile_handle = std::thread::spawn(move || {
                        match api::api::download_image(&image_url_clone, &image_path) {
                            Ok(_) => {
                                Ok(())
                            },
                            Err(_) => return Err(String::from("Failed to download image")),
                        }
                    });

                    let tile = Tile {
                        position,
                        width: 222,
                        height: 125,
                        tile: Rect::new(0, 0, 222, 125),
                        focused: unfocused,
                        tile_data: TileData {
                            image_id: item.image.tile.image_component.series.default.master_id,
                            image_url: item.image.tile.image_component.series.default.url,
                            image_path: format!("./assets/images/{}.jpeg", image_id),
                        },
                    };
                    if unfocused {
                        unfocused = false
                    };

                    row.tiles.push(tile);
                    tile_handles.push(tile_handle);
                }
            }
        }

        home_grid.rows.push(row);
    }

    for tile_handle in tile_handles {
        tile_handle.join().unwrap();
    }

    Ok(home_grid)
}

use dss_models::{home::ApiContent, set_ref::RefContent};
use crate::opengl_models::models::{Grid, Tile, Row, TitleData, Focus};
use crate::api;

use sdl2::rect::{Point, Rect};
use sdl2::image::InitFlag;
use sdl2::image as SDLImage;

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
        bound_y: 0
    };

    //Populate Grid
    for container in standard_collection.containers {
        // New Row
        let mut row: Row = Row {
            title: container.set.text.title.full.set.default.content.unwrap(),
            tiles: vec!(),
            hidden_tiles: 0
        };
        // Add Tiles to Row
        if container.set.items.is_some() {
            for item in container.set.items.unwrap() {

                // Set image metadata. Not downloaded until in screen.
                let image_url = item.image.tile.image_component.series.default.url;
                let image_id = item.image.tile.image_component.series.default.master_id;

                let mut tile = Tile {
                    position,
                    width: 222, 
                    height: 125,
                    tile: Rect::new(0, 0, 222, 125),
                    focused: unfocused,
                    title_data: TitleData {
                        image_id: image_id,
                        image_url: image_url,
                        image_path: None
                    }
                };
                if unfocused { unfocused = false };
    
                row.tiles.push(tile)
            }
        }
        else {
            if container.set.ref_id.is_some() {
                let ref_id = container.set.ref_id.unwrap();
                let ref_url = format!("https://cd-static.bamgrid.com/dp-117731241344/sets/{}.json", ref_id);
                let ref_api: RefContent = api::api::deserialize_api::<RefContent>(String::from(ref_url));

                for item in ref_api.data.set.unwrap().items.unwrap() {

                    // Set image metadata. Not downloaded until in screen.
                    let image_url = item.image.tile.image_component.series.default.url;
                    let image_id = item.image.tile.image_component.series.default.master_id;
    
                    let mut tile = Tile {
                        position,
                        width: 222, 
                        height: 125,
                        tile: Rect::new(0, 0, 222, 125),
                        focused: unfocused,
                        title_data: TitleData {
                            image_id: image_id,
                            image_url: image_url,
                            image_path: None
                        }
                    };
                    if unfocused { unfocused = false };
        
                    row.tiles.push(tile)
                }
            }
        }
        home_grid.rows.push(row);
    }

    Ok(home_grid)
}
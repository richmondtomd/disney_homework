pub mod models {
    use sdl2::rect::{Point, Rect};

    pub struct Grid {
        pub rows: Vec<Row>,
        pub focus: Focus,
        pub focus_x: u32,
        pub focus_y: u32,
        pub screen_x: i32,
        pub hidden_rows: i32,
        pub bound_x: u32,
        pub bound_y: u32,
    }

    #[derive(Clone)]
    pub struct Row {
        pub title: String,
        pub tiles: Vec<Tile>,
        pub hidden_tiles: i32,
    }

    #[derive(Clone)]
    pub struct Tile {
        pub position: Point,
        pub tile: Rect,
        pub width: u32,
        pub height: u32,
        pub focused: bool,
        pub tile_data: TileData,
        pub rendered: bool
    }

    #[derive(Clone)]
    pub struct TileData {
        pub image_id: String,
        pub image_url: String,
        pub image_path: Option<String>,
    }

    pub struct Focus {
        pub position: Point,
        pub tile: Rect,
    }
}

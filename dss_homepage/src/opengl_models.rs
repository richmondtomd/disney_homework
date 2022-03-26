pub mod models {
    use sdl2::render::{WindowCanvas, Texture, TextureCreator};
    use sdl2::rect::{Point, Rect};
    use sdl2::video::WindowContext;
    
    pub struct Grid<'a> {
        pub rows: Vec<Row<'a>>,
        pub focus: Focus,
        pub focus_x: u32,
        pub focus_y: u32,
        pub screen_x: i32,
        pub hidden_rows: i32,
        pub bound_x: u32,
        pub bound_y: u32
    }
    
    pub struct Row<'a> {
        pub title: String,
        pub tiles: Vec<Tile<'a>>,
        pub hidden_tiles: i32
    }
    
    pub struct Tile<'a> {
        pub position: Point,
        pub texture: Option<TileTexture<'a>>,
        pub tile: Rect,
        pub width: u32,
        pub height: u32,
        pub focused: bool,
        pub title_data: TitleData
    }

    pub struct TileTexture<'a> {
        pub texture_creator: TextureCreator<WindowContext>,
        pub texture: &'a Texture<'a>
    }
    
    pub struct TitleData {
        pub image_id: String,
        pub image_url: String,
        pub image_path: Option<String>
    }
    
    pub struct Focus {
        pub position: Point,
        pub tile: Rect
    }

}
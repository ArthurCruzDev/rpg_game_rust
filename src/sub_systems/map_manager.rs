const MAP_FILES_FOLDER: &str = "/maps";
const MAP_FILE_NAME: &str = "map.data";
const TILE_MAP_FILE_NAME: &str = "tilemap.data";

pub struct Tile {
    pub id: u32,
    pub image: Vec<u8>,
    pub isCollidable: bool,
    pub name: String,
}

pub struct MapManager {
    map: [[u32; 5120]; 5120],
    tile_map: Vec<Tile>,
}

impl Default for MapManager {
    fn default() -> Self {
        MapManager {
            map: [[0u32; 5120]; 5120],
            tile_map: Vec::new(),
        }
    }
}

impl MapManager {}

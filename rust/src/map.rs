use bincode::{
    Decode, Encode,
    config::{self},
};
use godot::{classes::file_access::ModeFlags, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MapManager {
    base: Base<Node>,

    resources: Vec<Resources>,
    tiles: Vec<Tiles>,
    entities: Vec<Entities>,
    objects: Vec<Objects>,
    #[var]
    filename: GString,
}

pub struct FileHeader {
    pub name: GString,
    pub version: u32,
    pub resources_file: GString,
    pub tiles_file: GString,
    pub entities_file: GString,
    pub objects_file: GString,
}

#[derive(Encode, Decode)]
pub struct Vector2Mem {
    pub x: f32,
    pub y: f32,
}

#[derive(Encode, Decode)]
pub struct ResourcesMem {
    pub pos: Vector2Mem,
    pub scene: String,
    pub quantity: u8,
}

#[derive(Encode, Decode)]
pub struct TilesMem {
    pub map_pos: Vector2Mem,
    pub atlas_pos: Vector2Mem,
    pub health: u8,
}

#[derive(Encode, Decode)]
pub struct EntitiesMem {
    pub pos: Vector2Mem,
    pub scene: String,
    pub health: u8,
    pub max_health: u8,
}

#[derive(Encode, Decode)]
pub struct ObjectsMem {
    pub pos: Vector2Mem,
    pub scene: String,
}

#[godot_api]
impl INode for MapManager {
    fn ready(&mut self) {
        self.save_header();
        let res = self.is_valid();
        match res {
            Ok(_) => godot_print!("Map file is valid"),
            Err(_) => godot_print!("Map file is invalid"),
        }
    }
}

impl MapManager {
    pub fn serialize_chunk<T: Encode>(&self, data: &T) -> Vec<u8> {
        let config = config::standard();
        bincode::encode_to_vec(data, config).expect("Failed to serialize chunk")
    }

    pub fn save_header(&self) {
        let mut key = PackedByteArray::new();
        for _ in 0..32 {
            key.push(u8::MAX);
        }
        let mut file = GFile::open_encrypted(
            &format!("res://saves/{}", self.filename).to_godot(),
            ModeFlags::WRITE,
            &key,
        )
        .expect("Failed to open map file");
        file.write_pascal_string(&self.filename)
            .expect("Failed to write map name");
        //let mut offset = file.position();
        file.write_u32(1).expect("Failed to write map version");
    }

    pub fn store<T: Encode>(&self, data: &T) -> Result<(), GString> {
        let data = self.serialize_chunk(data);
        Ok(())
    }

    pub fn is_valid(&self) -> Result<(), GString> {
        let mut key = PackedByteArray::new();
        for _ in 0..32 {
            key.push(u8::MAX);
        }
        let Ok(mut file) = GFile::open_encrypted(
            &format!("res://saves/{}", self.filename).to_godot(),
            ModeFlags::READ,
            &key,
        ) else {
            return Err("Failed to open map file".to_godot());
        };
        let Ok(save_name) = file.read_pascal_string() else {
            return Err("Failed to read map name".to_godot());
        };
        let Ok(save_version) = file.read_u32() else {
            return Err("Failed to read map version".to_godot());
        };
        godot_print!("Save name: {}", save_name);
        godot_print!("Save version: {}", save_version);
        match save_version {
            1 => Ok(()),
            _ => Err("Invalid save version".to_godot()),
        }
    }

    pub fn get_by_player_pos(
        &self,
        pos: Vector2,
    ) -> Result<(Vec<Resources>, Vec<Tiles>, Vec<Entities>, Vec<Objects>), GString> {
        let mut key = PackedByteArray::new();
        for _ in 0..32 {
            key.push(u8::MAX);
        }
        let mut file = GFile::open_encrypted(
            &format!("res://saves/{}", self.filename).to_godot(),
            ModeFlags::READ,
            &key,
        )
        .ok()?;
        let save_name = file.read_pascal_string().ok()?;
        let save_version = file.read_u32().ok()?;
        godot_print!("Save name: {}", save_name);
        godot_print!("Save version: {}", save_version);
        match save_version {
            1 => {
                let resources = file.read_u32().ok()? as u8;
                let tiles = file.read_u32().ok()? as u8;
                let entities = file.read_u32().ok()? as u8;
                let objects = file.read_u32().ok()? as u8;
                Ok((resources, tiles, entities, objects))
            }
            _ => None,
        }
    }
}

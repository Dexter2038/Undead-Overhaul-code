use std::io::BufRead;

use godot::{classes::file_access::ModeFlags, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MapManager {
    base: Base<Node>,

    #[var]
    filename: GString,
}

#[godot_api]
impl INode for MapManager {
    fn init(base: Base<Node>) -> Self {
        let filename = "map".to_godot();
        let mut key = PackedByteArray::new();
        for _ in 0..32 {
            key.push(u8::MAX);
        }
        let mut file = GFile::open_encrypted(
            &format!("res://saves/{filename}").to_godot(),
            ModeFlags::WRITE,
            &key,
        )
        .expect("Failed to open map file");
        file.write_pascal_string(&filename)
            .expect("Failed to write map name");
        let mut offset = file.position();
        file.write_u32(1).expect("Failed to write map version");
        let offset =
        file.write_u32()
        godot_print!("Created map file");
        MapManager { base, filename }
    }

    fn ready(&mut self) {
        let res = self.is_valid();
        match res {
            Some(_) => godot_print!("Map file is valid"),
            None => godot_print!("Map file is invalid"),
        }
    }
}

impl MapManager {
    pub fn is_valid(&self) -> Option<()> {
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
            1 => Some(()),
            _ => None,
        }
    }
}

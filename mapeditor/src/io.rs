use crate::world::WorldMap;
use std::fs::{self, File};
use std::io::{Write, Read};
use bincode;

pub fn save_map(world: &WorldMap, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = bincode::serialize(world)?;
    fs::create_dir_all("data")?;
    let mut file = File::create(path)?;
    file.write_all(&encoded)?;
    Ok(())
}

pub fn load_map(path: &str) -> Result<WorldMap, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(bincode::deserialize(&buffer).unwrap())
}

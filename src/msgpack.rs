
use rustc_serialize::{Encodable, Decodable};
use rmp_serialize::{encode, decode};
use rmp_serialize::{Encoder, Decoder};
use std::fs::{File, create_dir_all};
use std::path::*;
use std::io;

use super::traits::Storage;

/// Directory where msgpack files are saved.
pub struct MsgpackDir {
    path: PathBuf,
}

impl MsgpackDir {
    pub fn new(dirname: &str) -> Self {
        let path = Path::new(dirname);
        if !path.exists() {
            create_dir_all(path).expect("Failed to create directory");
        } else if !path.is_dir() {
            panic!("Non-directory file exists: {}", path.to_str().unwrap());
        }
        MsgpackDir { path: path.to_path_buf() }
    }
}

impl Storage for MsgpackDir {
    type Key = String;
    type SaveError = SaveError;
    type LoadError = LoadError;

    fn save_as<T: Encodable>(&self, obj: &T, name: &Self::Key) -> Result<(), Self::SaveError> {
        let filename = self.path.join(name);
        let mut buf = File::create(filename)?;
        let mut enc = Encoder::new(&mut buf);
        obj.encode(&mut enc)?;
        Ok(())
    }

    fn load<T: Decodable>(&self, name: &Self::Key) -> Result<T, Self::LoadError> {
        let filename = self.path.join(name);
        let mut buf = File::open(filename)?;
        let mut dec = Decoder::new(&mut buf);
        let obj = Decodable::decode(&mut dec)?;
        Ok(obj)
    }
}

#[derive(Debug, EnumError)]
pub enum SaveError {
    IO(io::Error),
    Msgpack(encode::Error),
}

#[derive(Debug, EnumError)]
pub enum LoadError {
    IO(io::Error),
    Msgpack(decode::Error),
}

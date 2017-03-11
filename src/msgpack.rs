
use rustc_serialize::{Encodable, Decodable};
use rmp_serialize::{Encoder, Decoder};
use std::fs::{File, create_dir_all};
use std::path::*;

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
    type SaveError = error::SaveError;
    type LoadError = error::LoadError;

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

mod error {
    use rmp_serialize::{encode, decode};
    use std::io;
    use std::error;
    use std::fmt;

    pub type SaveError = Error<encode::Error>;
    pub type LoadError = Error<decode::Error>;

    #[derive(Debug)]
    pub enum Error<T> {
        IO(io::Error),
        Msgpack(T),
    }

    impl<T> From<io::Error> for Error<T> {
        fn from(err: io::Error) -> Error<T> {
            Error::IO(err)
        }
    }

    impl From<decode::Error> for Error<decode::Error> {
        fn from(err: decode::Error) -> Error<decode::Error> {
            Error::Msgpack(err)
        }
    }

    impl From<encode::Error> for Error<encode::Error> {
        fn from(err: encode::Error) -> Error<encode::Error> {
            Error::Msgpack(err)
        }
    }

    impl<T: fmt::Display> fmt::Display for Error<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::IO(ref err) => err.fmt(f),
                Error::Msgpack(ref err) => err.fmt(f),
            }
        }
    }

    impl<T: error::Error> error::Error for Error<T> {
        fn description(&self) -> &str {
            match *self {
                Error::IO(ref err) => err.description(),
                Error::Msgpack(ref err) => err.description(),
            }
        }
    }

}

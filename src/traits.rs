
use rustc_serialize::{Encodable, Decodable};

pub trait Storage {
    type Key;
    type SaveError;
    type LoadError;
    fn save_as<T: Encodable>(&self, &T, Self::Key) -> Result<(), Self::SaveError>;
    fn load<T: Decodable>(&self, Self::Key) -> Result<T, Self::LoadError>;
}

pub trait AutoStorage: Storage {
    fn save<T: Encodable>(&self, &T) -> Result<Self::Key, Self::SaveError>;
}

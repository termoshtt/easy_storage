
use rustc_serialize::{Encodable, Decodable};

pub trait Storage {
    type Key;
    type SaveError;
    type LoadError;
    fn save_as<T: Encodable>(&self, &T, &Self::Key) -> Result<(), Self::SaveError>;
    fn load<T: Decodable>(&self, &Self::Key) -> Result<T, Self::LoadError>;
}

pub trait AutoStorage: Storage {
    fn save<T: Encodable>(&self, &T) -> Result<Self::Key, Self::SaveError>;
}

/// save series of data and the meta data of the series
pub trait SeriesStorage {
    type Index;
    type Data;
    type Info;
    fn save_series_as(&self, &[Self::Data], &Self::Info, &Self::Index);
    fn get_info(&Self::Index) -> Self::Info;
    fn load_series(&Self::Index) -> Vec<Self::Data>;
    // TODO add lazy loader
}

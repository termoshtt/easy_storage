
extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};

pub trait Storage {
    type Key;
    fn save<T: Decodable>(&self, &T) -> Self::Key;
    fn load<T: Encodable>(&self, Self::Key) -> T;
}

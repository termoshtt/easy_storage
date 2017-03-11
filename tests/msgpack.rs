
extern crate easy_storage;

use easy_storage::msgpack::*;
use easy_storage::traits::*;

#[test]
fn save_load() {
    let storage = MsgpackDir::new("test_data");
    let a = [1, 2, 3];
    storage.save_as(&a, &"a.msg".to_owned()).unwrap();
    let a_load: Vec<u32> = storage.load(&"a.msg".to_owned()).unwrap();
    println!("loaded a = {:?}", &a_load);
    assert!(a.len() == a_load.len());
    for (x, y) in a.iter().zip(a_load.iter()) {
        assert!(x == y);
    }
}

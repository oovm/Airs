use airis_device::{Device, VariableStore};

#[test]
fn test_name_join() {
    let store = VariableStore::new(Device::CPU);
    let root = store.root();
    &root / "a";
    &root / "b";
    println!("root: {:?}", store);
}
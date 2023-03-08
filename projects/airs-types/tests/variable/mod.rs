use airis_device::{Device, VariableStore};

#[test]
fn test_name_join() {
    let store = VariableStore::new(Device::CPU);
    let root = store.root();
    let a = &root / "a";
    let b = &root / "a";
    let c = &root / "a";
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("b: {:?}", c);
    println!("root: {:?}", root);
}

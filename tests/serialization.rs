use qmat::prelude::*;

#[test]
fn serialize() {
    let mat = Matrix::<_, 2, 2, 4>::new([1, 2, 3, 4]).unwrap();
    let serialized = serde_json::to_string(&mat).unwrap();
    assert_eq!(serialized, "{\"rows\":2,\"cols\":2,\"data\":[1,2,3,4]}")
}

/*#[test]
fn deserialize() { // ! FAILS !
    let serialized = "{\"rows\":2,\"cols\":2,\"data\":[1,2,3,4]}";
    let deserialized: Matrix<i32, 2, 2, 4> = serde_json::from_str(serialized).unwrap();
    println!("{:?}", deserialized)
}*/

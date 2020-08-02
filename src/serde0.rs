fn stringify() {
    let force = true;
    println!("{}", serde_json::json!({ stringify!(force): force }));
    println!("{}", serde_json::to_string(&3).unwrap());
}

#[test]
fn test() {
    stringify();
    assert_eq!(1, 2);
}

pub fn test() {
    let a = &4;
    let c = match a {
        &b if b > 3 => b,
        _ => 1,
    };
    let ref a1 = 3;
    let ref mut a2 = 3;
    // a2 = &mut 33;

    println!("{}", a2);
}

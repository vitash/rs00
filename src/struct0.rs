struct SA1;
struct SA2();

fn test1(a: &SA1, a2: &SA2) {}
fn test2() {
    test1(&SA1, &SA2())
}

// fn hm1(h1: &mut std::collections::HashMap<u8, String>) -> &String {
//     match h1.get(&22) {
//         Some(v) => v,
//         None => {
//             h1.insert(22, "dsd".into());
//             &h1[&22]
//         }
//     }
// }


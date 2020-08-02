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

struct User {
    id: u32,
    name: String,
}
impl User {
    fn new(id: u32, name: String) -> User {
        User { id, name }
    }
}
// impl std::cmp::Ord for User {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.name.cmp(other.name)   // 在 trait 里面，所有成员都可访问
//     }
// }

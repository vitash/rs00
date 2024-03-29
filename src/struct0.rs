use std::marker::PhantomData;

struct SA1;
struct SA2();
struct SA3 {}
enum EA1 {} // 不同于上面的两个结构体，这个甚至不可被构造出来

fn test1(a: &SA1, a2: &SA2) {}
fn test2() {
    test1(&SA1, &SA2());
    test3(SA2);
    // test3(SA1); 无法当做一个函数传递
    // test3(SA3); 无法当做一个函数传递
}
fn test3<T>(f: fn() -> T) {}

// fn hm1(h1: &mut std::collections::HashMap<u8, String>) -> &String {
//     // h1.entry(&22).or_insert("dsd")
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

// struct Mi<T>; // err
// parameter `T` is never used
// consider removing `T`, referring to it in a field, or using a marker such as `std::marker::PhantomData`

struct PhantomTuple<A, B>(A, PhantomData<B>);
fn phantom_data1() {
    let a = PhantomTuple(3_u8, PhantomData::<i32>);
}

fn pattern_dest() {
    struct Color(i32, i32, i32);
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let c1 = Color(1, 2, 3);
    // let c2 = Color(1, ..c1);
    // let c3 = Color(..c1);

    let origin = Point3d { x: 0, y: 0, z: 0 };

    let point = Point3d { z: 1, x: 2, ..origin };
}

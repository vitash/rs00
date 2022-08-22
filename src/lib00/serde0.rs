use serde::{Deserialize, Serialize};

fn stringify() {
    let force = true;
    println!("{}", serde_json::json!({ stringify!(force): force }));
    println!("{}", serde_json::to_string(&Ok::<bool, ()>(force)).unwrap());
    println!("{:?}", serde_json::json!({ "res": Ok::<bool, ()>(force) }));
}

type TResult1 = Result<Vec<u8>, Vec<char>>;
fn t_result1() {
    let a1 = TResult1::Ok(vec![3, 33]);
    let e1 = TResult1::Err(vec!['d', '3']);
    let json = &serde_json::to_string(&e1).unwrap();
    println!("{}", json); // 因为包含了 Err key 的信息，所以成功解析
    let res = serde_json::from_str::<TResult1>(json).unwrap();
    println!("{:?}", res);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ResultA<O, E> {
    Ok(O),
    Err(E),
}

type TResult2 = ResultA<Vec<u8>, Vec<u8>>;
fn t_result2() {
    let e1 = TResult2::Err(vec![3, 33]);
    let json = &serde_json::to_string(&e1).unwrap();
    println!("{}", json);
    let res = serde_json::from_str::<TResult2>(json).unwrap();
    println!("{:?}", res); // 结果是 Ok(...)
}

type TResult3 = ResultA<Vec<u8>, Vec<u16>>;
fn t_result3() {
    let e1 = TResult3::Err(vec![3, 333]);
    let json = &serde_json::to_string(&e1).unwrap();
    let res = serde_json::from_str::<TResult3>(json).unwrap();
    println!("{:?}", res); // 结果是 Err(...)
}

trait T1 {
    type JsonData;
    fn get_jsondata() -> Self::JsonData;
}

struct S1;
impl T1 for S1 {
    type JsonData = serde_json::Value;

    fn get_jsondata() -> Self::JsonData {
        let data = serde_json::json!({ "a": vec![1, 2] });
        data
    }
}

mod enum0 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)] // 对于结构体不加的话会是一个对象
    enum Pos {
        Left,
        Right,
        XY(u8, u8)
    }

    // const P1p: &'static mut Pos = &mut Pos::XY(3, 4);
    #[derive(Debug, Serialize, Deserialize)]
    struct A1 {
        #[serde(default)]
        pos: Option<Pos>, // 不能使用 `""`, `None` 对应 `null`
    }

    pub fn test() {
        let res = &serde_json::from_str::<A1>(r#"{"pos": null}"#).unwrap();
        println!("{:?}", res);
        let res = &mut serde_json::from_str::<A1>(r#"{"pos": [1, 2]}"#).unwrap();
        println!("{:?}", res);
        res.pos = Some(Pos::Left);
        println!("{:?}", res);
        // let res = &A1{pos: Some(Pos::XY(1, 2))};
        let json = &serde_json::to_string(res).unwrap();
        println!("{:?}", json);
    }
}

#[test]
fn test() {
    enum0::test();
    // assert_eq!(1, 2);
}

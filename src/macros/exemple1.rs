#[allow(unused_macros)]

macro_rules! count_exprs {
    () => (0);
    ($x:expr) => (1); // 为什么这个匹配是必须的？我想注释掉
    ($x:expr, $($rest:expr),*) => (count_exprs!($($rest),*) + 1);
}

macro_rules! impl_log_data {
    ($t:ty { $field0:ident $(, $fields:ident)* }) => {
        impl LogData for $t {
            fn log_data(&self) -> String {
                format!(
                    concat!("{{ ", stringify!($field0), ": {:?}" $(, concat!(", ", stringify!($fields), ": {:?}"))* , " }}"),
                    &self.$field0 $(, &self.$fields)*)
            }
        }
    };
}

trait LogData {
    fn log_data(&self) -> String;
}

// impl LogData for u8 {
//     fn log_data(&self) -> String {
//         format!("{{ id: {:?} }}", &self)
//     }
// }

struct User {
    id: usize,
    name: &'static str,
}
// trace_macros!(true);
impl_log_data!(User { id });

fn t1() {
    let n = count_exprs!(1, 2, 4);
    assert_eq!(3, n);
    println!("{}", User { id: 33, name: "sdsd" }.log_data());
    // let a = fwbonaqi!(3);
}

#[test]
fn test() {
    t1();
}

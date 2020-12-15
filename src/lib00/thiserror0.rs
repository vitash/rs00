use std::error::Error;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
// #[error("data store {0}")]
struct ErrA {
    pub code: u32,
    pub msg: String,
}

impl Display for ErrA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DDDDDispaly {:?}", self)
    }
}

#[derive(Debug, Error)]
enum ErrB {
    #[error("data store {0}")]
    A(ErrA),
    // #[error("data store {0}")]
    // A2(#[from] ErrA), // 可以让宏自动实现
    #[error("the data for key `{0}` is not available")]
    B(String),
}

impl From<ErrA> for ErrB {
    fn from(mut a: ErrA) -> Self {
        a.msg.push_str(" --from");
        match a.code {
            0 => panic!(""),
            _ => ErrB::A(a)
        }
    }
}


fn test_err1() -> Result<u8, ErrB> {
    let a = Err::<u8, _>(ErrA {
        code: 3,
        msg: "dsds".to_string(),
    });
    let a = a?;
    Ok(a)
}

#[test]
fn t1() {
    let a = &test_err1();
    if let Err(e) = a {
        println!("{}", e);
    }
    match a {
        Err(ErrB::A(ErrA { code, .. })) => assert_eq!(*code, 3),
        _ => assert!(false),
    }
}

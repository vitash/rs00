use std::ops::Add;

struct A(u8, u8);

impl Add<u8> for A {
    type Output = u8;
    fn add(self, rhs: u8) -> Self::Output {
        self.0 + rhs
    }
}

impl std::ops::Index<usize> for A {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("out of range"),
        }
    }
}

fn test_a(a: A) {
    let x = a[2];
    
}

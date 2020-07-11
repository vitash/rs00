use std::ops::Add;

struct A(u8, u8);

impl Add<u8> for A {
    type Output = u8;
    fn add(self, rhs: u8) -> Self::Output {
        self.0 + rhs
    }
}

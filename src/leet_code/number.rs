pub fn reverse(x: i32) -> i32 {
    let sig = x.signum();
    let mut x = x.abs() as i64;
    let mut r = 0_i64;
    while x != 0 {
        r = r * 10 + x % 10;
        x /= 10 
    }

    if r < -1 << 31 || r > 1 << 31 {
        0
    } else {
        r as i32 * sig
    }
}

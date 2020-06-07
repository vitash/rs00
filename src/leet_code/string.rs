// 6. Z 字形变换
pub fn convert_z(s: String, n: i32) -> String {
    let n = n as usize;
    if n == 1 || s.len() < 1 || s.len() < n { return s; }
    let mut res = String::new();
    let chs = s.chars().collect::<Vec<char>>();

    for row in 0..n {
        res.push(chs[row]);
        let mut i = row;
        let mut j = 0;
        loop {
            j += 1;
            i += if row == n - 1 {  
                row * 2 
            } else if row == 0 || j % 2 == 1 { 
                (n - row - 1) * 2
            } else {
                row * 2
            };
            if i >= s.len() { break; }
            res.push(chs[i]);
        }
    }
    return res;
}

#[test]
fn t_convert_z() {
    let res = convert_z("LEETCODEISHIRING".into(), 3);
    assert_eq!(res.as_str(), "LCIRETOESIIGEDHN");

    let res = convert_z("LEETCODEISHIRING".into(), 4);
    assert_eq!(res.as_str(), "LDREOEIIECIHNTSG");
}

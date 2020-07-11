// pub fn climb_stairs(n: i32) -> i32 {
//     if n <= 2 { return n; }
//     return climb_stairs(n - 1) + climb_stairs(n - 2);
// }

trait Solution {
    fn climb_stairs1(n: i32) -> i32 {
        if n <= 2 { return n; }
        return Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2);
    }
    fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        for _ in 1..n {
            let temp = b;
            b = a + b;
            a = temp;
            // (a, b) = (b, a + b);
        }
        return a + b;
    }
    
    fn max_profit_121(prices: Vec<i32>) -> i32 {
        prices.iter().take(prices.len() - 1).enumerate().fold(0, |max, (i , x)| {
            std::cmp::max(max, prices.iter().skip(i + 1).max().unwrap() - x)
        })
    }
    fn max_profit_121_2(prices: Vec<i32>) -> i32 {
        prices.iter().skip(1).fold((0, i32::MAX), |(max, min) , x| {
            (std::cmp::max(max,  x - min), std::cmp::min(min, *x))
        }).0
    }
}

fn test_tuple(mut p: (u8, u8)) {
    p = (p.1, p.0 + p.1);
    p.0.to_string();
}
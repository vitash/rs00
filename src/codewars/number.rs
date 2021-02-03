//

mod multiples_of_3_or_5 {

    fn solution(num: i32) -> i32 {
        (3..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
    }

    use std::ops::Range;
    #[test]
    fn test_range() {
        let r1: Range<i32> = 10..3;
        // 一次也不会执行
        (10..0).for_each(|x| println!("{}", x));
    }
}

mod digital_root {
    fn digital_root(mut n: i64) -> i64 {
        if n < 10 {
            return n;
        }

        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        digital_root(sum)
    }
}

mod remove_nb {
    // 1 到 m 的序列中找出 a b, 满足 a * b = 1 + 2 + 3 .. + m - a - b
    // sum - a - b = a * b
    // sum = a * b + a + b
    // 1) sum = (a + 1) * b + a
    //    (sum - a) / (a + 1) = b
    // 2) sum = a * (b + 1) + b
    // 3) sum = (a + 1) * (b + 1) - 1
    //    (sum + 1) / (a + 1) - 1 = b
    fn remove_nb(m: i32) -> Vec<(i32, i32)> {
        let mut sum = (m * m + m) / 2 + 1;
        let mut x = 2;
        let prime_factors = &mut vec![1];
        while x <= sum {
            if sum % x == 0 {
                sum /= x;
                prime_factors.push(x);
            } else {
                x += 1;
            }
        }

        let res = vec![];
        // 从质因数中找出两组合的数
        res
    }
    fn remove_nb3(m: i32) -> Vec<(i32, i32)> {
        let m = m as i128;
        let sum = (m * m + m) / 2 + 1;

        let mut res = vec![];
        let mut a = 2;
        while a <= m + 1 {
            if sum % a == 0 {
                let b = sum / a;
                if b <= m + 1 && a != b {
                    res.push(((a - 1) as i32, (b - 1) as i32));
                }
            }
            a += 1;
        }
        res
    }
    //    (sum - a) / (a + 1) = b
    fn remove_nb1(m: i32) -> Vec<(i32, i32)> {
        let m = m as i64;
        let sum = (m + 1) * m / 2;

        ((sum - m) / (m + 1)..m)
            .filter(|&a| (sum - a) % (a + 1) == 0)
            // .filter(|&a| (sum - a) % (a + 1) == 0 && (sum - a) / (a + 1) != a)
            .map(|a| (a as i32, ((sum - a) / (a + 1)) as i32))
            .collect()
    }

    #[test]
    fn basics_remove_nb() {
        assert_eq!(remove_nb1(26), vec![(15, 21), (21, 15)]);
        assert_eq!(remove_nb1(100), vec![]);
        assert_eq!(remove_nb1(101), vec![(55, 91), (91, 55)]);
        assert_eq!(remove_nb1(102), vec![(70, 73), (73, 70)]);
    }

    #[test]
    fn same_num() {
        (3..1000)
            .map(|n| (n, remove_nb1(n)))
            .filter(|(n, v)| v.iter().any(|(a, b)| a == b))
            .for_each(|(n, v)| println!("remove_nb({}) => {:?}", n, v));
    }
}

mod sum_of_factor {
    fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
        fn prime_factors(mut n: i64) -> Vec<i64> {
            n = n.abs();
            let mut res = vec![];
            let mut x = 2;
            while n >= x {
                if n % x == 0 {
                    n /= x;
                    res.push(x);
                } else {
                    x += 1;
                }
            }
            res
        }
        let mut res = l.iter().flat_map(|&x| prime_factors(x))
        .collect::<std::collections::HashSet<_>>().iter()
        .map(|&p| (p, l.iter().filter(|&x| x % p == 0).sum()))
        .collect::<Vec<_>>();

        res.sort_by_key(|&(p, s)| p);
        res
    }
}


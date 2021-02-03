mod num_as_roman {
    fn num_as_roman(num: i32) -> String {
        let mut num = num as usize;
        let mut s = String::with_capacity(16);
        let values = [(1000, '?', 'M'), (100, 'D', 'C'), (10, 'L', 'X'), (1, 'V', 'I')];
        for (i, (x, five, one)) in values.iter().enumerate() {
            match num / x {
                9 => s.extend([*one, values[i - 1].2].iter()),
                n @ 5..=8 => s.extend(std::iter::once(five).chain(std::iter::repeat(one).take(n - 5))),
                4 => s.extend([*one, *five].iter()),
                n => s.extend(std::iter::repeat(*one).take(n)),
            };
            num %= x;
        }

        s
    }
    #[test]
    fn returns_expected() {
        assert_eq!(num_as_roman_0(182), "CLXXXII");
        assert_eq!(num_as_roman_0(1990), "MCMXC");
        assert_eq!(num_as_roman_0(1875), "MDCCCLXXV");
    }
    fn num_as_roman_0(mut num: i32) -> String {
        let mut s = String::with_capacity(16);
        let values = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        for &(x, v) in values.iter() {
            while num >= x {
                s.push_str(v);
                num -= x;
            }
        }
        s
    }
}

mod matching_and_substituting {
    fn check_phone(phone: &str) -> bool {
        let iter = &mut phone.split('-');
        match iter.next() {
            Some("+1") => (),
            _ => return false,
        };
        let len_num = iter.filter(|&s| s.chars().all(|c| c.is_ascii_digit())).fold(0, |res, s| res * 10 + s.len());
        if len_num != 334 {
            return false;
        }
        
        true
    }
    fn change(s: &str, prog: &str, version: &str) -> String {
        // let err = "ERROR: VERSION or PHONE".to_string();
        // let verf = version.parse::<f64>();
        // if verf < 1.0 || verf >= 10.0 {
        //     return err;
        // }
        s.split('\n');
        todo!() // 弃置，懒得写了
    }

    fn dotest(s: &str, prog: &str, version: &str, exp: &str) -> () {
        println!("s:{:?}", s);
        println!("prog:{:?}", prog);
        println!("version:{:?}", version);
        let ans = change(s, prog, version);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        let s1 = "Program title: Primes\n\
        Author: Kern\n\
        Corporation: Gold\n\
        Phone: +1-503-555-0091\n\
        Date: Tues April 9, 2005\n\
        Version: 6.7\n\
        Level: Alpha";
        dotest(
            s1,
            "Ladder",
            "1.1",
            "Program: Ladder Author: g964 Phone: +1-503-555-0090 Date: 2019-01-01 Version: 1.1",
        );
        let s2 = "Program title: Balance\nAuthor: Dorries\nCorporation: Funny\nPhone: +1-503-555-0095\nDate: Tues July 19, 2014\nVersion: 6.7\nLevel: Release";
        dotest(
            s2,
            "Circular",
            "1.5",
            "Program: Circular Author: g964 Phone: +1-503-555-0090 Date: 2019-01-01 Version: 1.5",
        );
        let s13 = "Program title: Primes\nAuthor: Kern\nCorporation: Gold\nPhone: +1-503-555-0090\nDate: Tues April 9, 2005\nVersion: 67\nLevel: Alpha";
        dotest(s13, "Ladder", "1.1", "ERROR: VERSION or PHONE");
    }

    #[test]
    fn test_str() {
        let s1 = "\
        1,\
        2\
        ";
        assert_eq!(s1, "1,2");
    }
}

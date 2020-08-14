use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::process::Command;

#[test]
fn test() {
    spawn2();
}

fn spawn1() {
    let data = "\
        86967897737416471853297327050364959 \
        11861322575564723963297542624962850 \
        70856234701860851907960690014725639 \
        38397966707106094 172783238747669219 \
        52380795257888236525459303330302837 \
        58495327135744041048897885734297812 \
        69920216438980873548808413720956532 \
        16278424637452589860345374828574668";

    let res = data
        .split_whitespace()
        .map(|s: &str| thread::spawn(move || s.chars().map(|c: char| c.to_digit(10).unwrap()).sum::<u32>()))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|t: JoinHandle<_>| t.join().unwrap())
        .sum::<u32>();

    println!("Final sum result: {}   <------------", res);
}

fn mpsc1() {
    let (tx, rx) = mpsc::channel();
    let res = &(0..4)
        .map(|id| {
            let tx = tx.clone();
            thread::spawn(move || {
                (0..8)
                    .map(move |x| {
                        let tx = tx.clone();
                        thread::spawn(move || tx.send(id + x * 100).unwrap());
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>() // 如果不进行收集，结果就是串行
        .into_iter()
        .flat_map(|t| t.join().unwrap())
        .map(|_| rx.recv().unwrap())
        .collect::<Vec<_>>();
    println!("\n{:?}", res);
}

fn spawn2() {
    let res = (0..16)
        .map(|i| thread::spawn(move || (i * 2, print!("{},", i * 2)).0))
        .collect::<Vec<_>>() // 立即执行迭代器
        .into_iter()
        .map(|t| t.join().unwrap() * 3)
        .collect::<Vec<i32>>();
    println!("\n{:?}", res);
}

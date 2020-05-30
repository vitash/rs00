pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = (k as usize) % len;
    if k == 0 { return; }

    let l1 = len - (len % k);
    for i in 0..l1 {
        let temp = nums[i];
        nums[i] = nums[(len - k) + i % k];
        nums[(len - k) + i % k] = temp;
    }
    for i in l1..len {
        let temp = nums[i];
        nums[i] = nums[len - 1];
        nums[len - 1] = temp;
    }
}

#[test]
fn t_rotate() {
    let mut v1 = vec![1, 2, 3, 4, 5, 6];
    rotate(&mut v1, 4);
    assert_eq!(v1, vec![3, 4, 5, 6, 1, 2,]);

    let mut v1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    rotate(&mut v1, 2);
    assert_eq!(v1, vec![7, 8, 1, 2, 3, 4, 5, 6]);

    let mut v1 = vec![1, 2];
    rotate(&mut v1, 1);
    assert_eq!(v1, vec![2, 1,]);
}

#[test]
fn t_rotate2() {
    for i in -2..-1 {
        println!("{:?}", i);
    }
}

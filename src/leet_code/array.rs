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

pub fn merge_56(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() < 1 { return intervals;}
    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    let mut res = vec![];
    let mut curr = intervals[0].clone();
    for v in intervals.into_iter() {
        if v[0] > curr[1] {
            res.push(curr);
            curr = v;
            continue;
        }
        curr = vec![std::cmp::min(v[0], curr[0]), std::cmp::max(v[1], curr[1])];
    }
    res.push(curr);
    return res;
}

#[test]
fn t_merge_65() {
    let res = merge_56(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]);
    assert_eq!(res, vec![vec![1,6],vec![8,10],vec![15,18]]);

    let res = merge_56(vec![vec![1,4],vec![0,0]]);
    assert_eq!(res, vec![vec![0,0],vec![1,4]]);
}

pub fn merge_88(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // 如果全程使用类型约束，必须使用 usize，那么数组就下标就应该从 1 开始！
    // 
    if n < 1 { return; }
    let mut i1 = m - 1; 
    let mut i2  = n as usize - 1;
    loop {
        if i1 == -1 || nums2[i2] >= nums1[i1 as usize] {
            nums1[(i1 + 1) as usize + i2] = nums2[i2];
            if i2 == 0 { return; }
            i2 -= 1;
        } else {
            nums1[(i1 + 1) as usize + i2] = nums1[i1 as usize];
            i1 -= 1;
        }
    }
}

pub fn merge_88_usize(nums1: &mut Vec<i32>, mut m: usize, nums2: &mut Vec<i32>, mut n: usize) {
    if n < 1 { return; }
    loop {
        if m == 0 || nums2[n - 1] >= nums1[m - 1] {
            nums1[m + n - 1] = nums2[n];
            if n == 0 { return; }
            n -= 1;
        } else {
            nums1[m + n - 1] = nums1[m - 1];
            m -= 1;
        }
    }
}

pub fn t_88(ref mut a: Vec<i32>) {
    if a.len() > 100 { return; }
    a.push(3);
    t_88(vec![1, 2]);

    let b = vec![1, 3];
    t_88(b);
}

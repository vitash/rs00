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
    let mut res = vec![];
    // let mut a = intervals[0][0];
    // intervals.into_iter().fold_first(|v1, v2| {
    //     if v1[0] > v2[1] { 
    //         res.push(v1);
    //         v1 = v2;
    //     } else if v2[1] > v1[1] {
    //          v1[1] = v2[1]; 
    //     }
    //     v1
    // });
    // let mut start = intervals[0][0];
    // let mut end = intervals[0][1];
    // for v1 in intervals.into_iter() {
    //     if v1[0] > end { 
    //         res.push(vec![start, end]);
    //         start = v1[0];
    //         end = v1[1];
    //      }
    //     if v1[0] <= end && v1[1] > end  { end = v1[1]; }
    // }
    let mut curr = intervals[0].clone();
    for v in intervals.into_iter() {
        if v[0] > curr[1] { 
            res.push(curr);
            curr = v;
            continue;
        } 
        if v[0] < curr[0] {
            curr[0] = v[0]; 
        }
        if v[1] > curr[1] {
            curr[1] = v[1]; 
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
}
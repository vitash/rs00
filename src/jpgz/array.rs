fn transpose<T>(/*mut*/ arr: Vec<Vec<T>>) -> Vec<Vec<T>> {
    // let mut res = vec![];
    // for i in 0..arr.iter().map(|v| v.len()).max().unwrap_or_default() {
    //     let mut cols = vec![];
    //     cols.push(arr[i].get(3));
    // }
    // res
    // 真是糟心，要么用 remove 移除第一个元素，但这么频繁调整数组没必要
    todo!()
}

#[test]
fn test_transpose1() {
    assert_eq!(
        transpose(vec![vec![1, 2, 22], vec![3], vec![4, 5, 6]]),
        vec![vec![1, 3, 4], vec![2, 5], vec![22, 6]]
    );
}

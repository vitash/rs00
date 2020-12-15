pub fn range_extraction(a: &[i32]) -> String {
    let (mut i, len) = (0, a.len());
    let mut res = String::with_capacity(len);
    while i < len {
        let start = a[i];
        while i + 1 < len && a[i + 1] - a[i] == 1 { i += 1 }
        let end = a[i];
        i += 1;
        res.push_str(match end - start {
            0 => format!("{},", start),
            1 => format!("{},{},", start, end),
            _ => format!("{}-{},", start, end),
        }.as_str())
    }
    res.pop();
    return res;
}

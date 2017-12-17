use day10;

fn count_bits(x: u64) -> usize {
    let mut m: u64 = 1;
    let mut count: usize = 0;
    for _ in 0..64 {
        if x & m != 0 {
            count += 1;
        }
        m = m << 1
    }
    count
}

pub fn num_used_grids(input: &str) -> usize {
    let num_rows = 128;
    let mut count = 0;
    for i in 0..num_rows {
        let line = format!("{}-{}", input, i);
        let mut hash = day10::knot_hash(&line);
        let hash_len = hash.len();
        let r = hash.split_off(hash_len / 2);
        let left = u64::from_str_radix(&hash, 16).unwrap();
        let right = u64::from_str_radix(&r, 16).unwrap();
        let line_count = count_bits(left) + count_bits(right);
        count += line_count;
    }
    count
}

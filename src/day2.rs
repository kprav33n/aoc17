use std;

pub fn compute_checksum(input: &str) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let mut max = std::u64::MIN;
        let mut min = std::u64::MAX;
        for v in l.split_whitespace().map(|x| x.parse::<u64>().unwrap()) {
            max = std::cmp::max(max, v);
            min = std::cmp::min(min, v);
        }
        sum += max - min;
    }
    sum
}

pub fn compute_checksum2(input: &str) -> u64 {
    let mut sum = 0;
    'outer: for l in input.lines() {
        // fixme: This is quadratic.
        for x in l.split_whitespace().map(|x| x.parse::<u64>().unwrap()) {
            for y in l.split_whitespace().map(|x| x.parse::<u64>().unwrap()) {
                if x == y {
                    continue;
                }
                let min = std::cmp::min(x, y);
                let max = std::cmp::max(x, y);
                if max % min == 0 {
                    sum += max / min;
                    continue 'outer;
                }
            }
        }
    }
    sum
}

fn as_digit(x: u8) -> u8 {
    return x - '0' as u8;
}

pub fn solve_captcha(input: &str) -> u64 {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        0;
    }

    let sum = bytes.windows(2).fold(0, |acc, w| {
        if w[0] == w[1] {
            acc + as_digit(w[0]) as u64
        } else {
            acc
        }
    });

    let first = *bytes.first().unwrap();
    let last = *bytes.last().unwrap();
    if first == last {
        sum + as_digit(first) as u64
    } else {
        sum
    }
}

pub fn solve_captcha2(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut sum = 0;
    for i in 0..len / 2 {
        if bytes[i] == bytes[i + len / 2] {
            sum += (as_digit(bytes[i]) * 2) as u64
        }
    }
    sum
}

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

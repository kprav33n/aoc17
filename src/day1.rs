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

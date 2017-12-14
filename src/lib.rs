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

fn num_ports(n: u64) -> u64 {
    let sum = (n * (n + 1)) / 2;
    return 2 + sum * 8;
}

fn ports(n: u64) -> Vec<u64> {
    let count = n * 8;
    let start = num_ports(n - 1);
    (0..count).map(|i| start + i).collect()
}

fn port_location(p: u64) -> u64 {
    let t = ((p - 2) / 8) * 2;
    let d = 1 + 4 * t;
    let s = ((d as f64).sqrt() as u64 - 1) / 2;
    s
}

fn port_offsets(n: u64) -> Vec<u64> {
    (0..4).map(|i| (n - 1) * 8 + 1 + i * 2).collect()
}

fn adj_port(port: u64) -> u64 {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    let l = port_location(port) + 1;
    let perimeter = ports(l);
    let corners: HashSet<u64> = HashSet::from_iter(
        (1..5).map(|i| l * i * 2 - 1).map(|i| perimeter[i as usize])
    );
    if corners.contains(&port) {
        port - 1
    } else {
        let quadrant = (port - perimeter[0]) as usize / (perimeter.len() / 4);
        port - port_offsets(l)[quadrant]
    }
}

pub fn manhattan_distance(port: u64) -> u64 {
    let mut current = port;
    let mut steps = 0;
    while current != 1 {
        steps += 1;
        current = std::cmp::min(current - 1, adj_port(current));
    }
    return steps;
}

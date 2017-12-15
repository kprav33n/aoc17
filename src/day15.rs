fn gen(factor: u32, cdivisor: u32) -> Box<Fn(u32) -> u32> {
    let divisor = 2147483647;
    Box::new(move |start| {
        let mut x = start;
        loop {
            x = ((x as u64 * factor as u64) % divisor) as u32;
            if x % cdivisor == 0 {
                return x;
            }
        }
    })
}

pub fn num_match_duel(start_a: u32, start_b: u32, iterations: usize, slow: bool) -> usize {
    let gen_a = gen(16807, if slow {4} else {1});
    let gen_b = gen(48271, if slow {8} else {1});
    let mut last_a = start_a;
    let mut last_b = start_b;
    let mut matches = 0;
    let mask = 0x0000ffff;
    for _ in 0..iterations + 1 {
        last_a = gen_a(last_a);
        last_b = gen_b(last_b);
        if (last_a & mask) == (last_b & mask) {
            matches += 1;
        }
    }
    matches
}

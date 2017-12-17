pub fn spinlock_end_val(input: usize, num_iters: usize) -> usize {
    let mut buffer = vec![0];
    let mut cur_pos = 0;
    let mut val = 1;
    for _ in 0..num_iters {
        let i = (cur_pos + input) % buffer.len() + 1;
        buffer.insert(i, val);
        cur_pos = i;
        val += 1;
    }
    buffer[(cur_pos + 1) % buffer.len()]
}

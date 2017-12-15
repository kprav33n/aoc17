pub fn steps_until_exit(input: &str, strange_mode: bool) -> usize {
    let mut v: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    let mut pos: i64 = 0;
    let mut count = 0;
    loop {
        count += 1;
        let old_pos = pos;
        let offset = v[pos as usize];
        pos = pos + offset;
        if offset >= 3 && strange_mode {
            v[old_pos as usize] -= 1;
        } else {
            v[old_pos as usize] += 1;
        }
        if pos as usize >= v.len() {
            return count;
        }
    }
}

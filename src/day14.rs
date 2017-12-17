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

pub fn num_used_regions(input: &str) -> usize {
    let num_rows = 128;
    let mut v: Vec<Vec<usize>> = Vec::new();
    for i in 0..num_rows {
        let line = format!("{}-{}", input, i);
        let mut hash = day10::knot_hash(&line);
        let hash_len = hash.len();
        let r = hash.split_off(hash_len / 2);
        let left = u64::from_str_radix(&hash, 16).unwrap();
        let right = u64::from_str_radix(&r, 16).unwrap();

        let mut lv = Vec::new();

        let mut m: u64 = 0x8000000000000000;
        for _ in 0..64 {
            if left & m != 0 {
                lv.push(1);
            } else {
                lv.push(0);
            }
            m = m >> 1
        }

        m = 0x8000000000000000;
        for _ in 64..128 {
            if right & m != 0 {
                lv.push(1);
            } else {
                lv.push(0);
            }
            m = m >> 1
        }
        v.push(lv);
    }

    // Algorithm courtesy: https://en.wikipedia.org/wiki/Connected-component_labeling#One_component_at_a_time
    let mut label = 2;
    let mut q: Vec<(usize, usize)> = Vec::new();
    for i in 0..128 {
        for j in 0..128 {
            if v[i][j] == 1 {
                v[i][j] = label;
                q.push((i, j));

                while !q.is_empty() {
                    let (x, y) = q.pop().unwrap();
                    let l = v[x][y];
                    for &(a, b) in neighbors(x, y, 128).iter() {
                        if v[a][b] == 1 {
                            v[a][b] = l;
                            q.push((a, b));
                        }
                    }
                }
                label += 1;
            }
        }
    }

    label - 2
}

fn neighbors(i: usize, j: usize, dim: usize) -> Vec<(usize, usize)> {
    let si = i as i64;
    let sj = j as i64;
    let n = vec![(si, sj-1), (si-1, sj), (si+1, sj), (si, sj+1),];
    n.into_iter().
        filter(|&(x, y)| !(x < 0 || y < 0 || x as usize == dim || y as usize == dim)).
        map(|(x, y)| (x as usize, y as usize)).collect()
}

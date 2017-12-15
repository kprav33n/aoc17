use std;
use std::collections::HashSet;

pub fn num_redist(input: &str) -> usize {
    let mut v: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut visited: HashSet<Vec<usize>> = HashSet::new();
    let mut count = 0;
    loop {
        visited.insert(v.clone());
        let mut max = 0;
        let mut max_pos = std::usize::MAX;
        for (i, x) in v.iter().enumerate() {
            if *x > max {
                max = *x;
                max_pos = i;
            }
        }
        let mut taken = v[max_pos];
        v[max_pos] = 0;
        let mut i = (max_pos + 1) % v.len();
        while taken > 0 {
            v[i] += 1;
            taken -= 1;
            i = (i + 1) % v.len();
        }
        count += 1;
        if visited.contains(&v) {
            return count;
        }
    }
}

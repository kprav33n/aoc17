use std;
use std::collections::HashMap;

pub fn num_redist(input: &str) -> (usize, usize) {
    let mut v: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut visited: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut count = 0;
    loop {
        visited.insert(v.clone(), count);
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
        if visited.contains_key(&v) {
            return (count, count - *visited.get(&v).unwrap());
        }
    }
}

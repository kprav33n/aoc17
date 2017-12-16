pub fn knot_product(size: usize, lengths: Vec<usize>) -> usize {
    let mut v: Vec<usize> = (0..size).collect();
    let mut pos = 0;
    let mut skip_size = 0;
    for l in lengths.iter() {
        let mut t = vec![0; *l];
        for i in 0..*l {
            t[i] = v[(pos + i) % size];
        }
        let mut s = pos;
        for x in t.iter().rev() {
            v[s] = *x;
            s = (s + 1) % size;
        }
        pos = (pos + l + skip_size) % size;
        skip_size += 1;
    }
    v[0] * v[1]
}

pub fn knot_hash(lengths: &str) -> String {
    let size: usize = 256;
    let iterations = 64;
    let mut v: Vec<u8> = (0..size).map(|x| x as u8).collect();
    let mut ls = lengths.as_bytes().to_vec();
    ls.extend([17, 31, 73, 47, 23].iter().cloned());
    let mut pos = 0;
    let mut skip_size = 0;
    for _ in 0..iterations {
        for tl in ls.iter() {
            let l = *tl as usize;
            let mut t = vec![0; l];
            for i in 0..l {
                t[i] = v[(pos + i) % size];
            }
            let mut s = pos;
            for x in t.iter().rev() {
                v[s] = *x;
                s = (s + 1) % size;
            }
            pos = (pos + l + skip_size) % size;
            skip_size += 1;
        }
    }
    let dense_factor = 16;
    let dsize = size / dense_factor;
    let mut r = vec![0 as u8; dsize];

    for x in 0..dsize {
        r[x] = v.iter().skip(x * dense_factor).take(dense_factor).fold(0, |r, x| r ^ x)
    }

    r.iter().map(|x| format!("{:02x}", x)).collect::<String>()
}

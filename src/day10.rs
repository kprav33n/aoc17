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

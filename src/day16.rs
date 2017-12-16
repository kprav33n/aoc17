pub fn perm_promenade(line: &str, dance: &str) -> String {
    let moves: Vec<&str> = dance.split(",").collect();
    let mut v = line.as_bytes().to_vec();
    for m in moves {
        match m[..1].as_ref() {
            "s" => {
                let pos: usize = m[1..].parse().unwrap();
                let mut sv: Vec<u8> = vec![];
                {
                    let (a, b) = v.split_at(v.len() - pos);
                    sv.extend_from_slice(b);
                    sv.extend_from_slice(a);
                }
                v = sv;
            }
            "x" => {
                let t: Vec<&str> = m[1..].split('/').collect();
                let x: usize = t[0].parse().unwrap();
                let y: usize = t[1].parse().unwrap();
                let t = v[x];
                v[x] = v[y];
                v[y] = t;
            }
            "p" => {
                let t: Vec<&str> = m[1..].split('/').collect();
                let a = t[0].as_bytes()[0];
                let b = t[1].as_bytes()[0];
                let mut x = 0;
                let mut y = 0;
                for (i, e) in v.iter().enumerate() {
                    if *e == a {
                        x = i;
                    }
                    if *e == b {
                        y = i;
                    }
                }
                let t = v[x];
                v[x] = v[y];
                v[y] = t;
            }
            _ => assert!(false)
        }
    }
    String::from_utf8(v).unwrap()
}

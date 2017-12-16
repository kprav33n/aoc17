pub fn perm_promenade(line: &str, dance: &str, iterations: usize) -> String {
    let moves: Vec<&str> = dance.split(",").collect();
    let mut v = line.as_bytes().to_vec();
    let mut table = Vec::new();
    table.push(v.to_vec());
    for i in 0..iterations {
        for m in moves.iter() {
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
                    v.swap(x, y);
                }
                "p" => {
                    let t: Vec<&str> = m[1..].split('/').collect();
                    let a = t[0].as_bytes()[0];
                    let b = t[1].as_bytes()[0];
                    let x = v.iter().position(|&e| e == a).unwrap();
                    let y = v.iter().position(|&e| e == b).unwrap();
                    v.swap(x, y);
                }
                _ => assert!(false)
            }
        }
        if table[0] == v {
            return String::from_utf8(table[iterations % (i + 1)].to_vec()).unwrap();
        }
        table.push(v.to_vec());
    }
    String::from_utf8(v).unwrap()
}

use std::collections::HashSet;

pub fn find_bottom_program(input: &str) -> String {
    let mut ls: HashSet<&str> = HashSet::new();
    let mut rs: HashSet<&str> = HashSet::new();
    for l in input.trim().lines() {
        let v: Vec<&str> = l.split("->").collect();
        let lv: Vec<&str> = v[0].trim().split_whitespace().collect();
        ls.insert(lv[0].trim());
        if v.len() > 1 {
            let rv: Vec<&str> = v[1].trim().split(",").collect();
            for &x in rv.iter() {
                rs.insert(x.trim());
            }
        }
    }
    String::from(*ls.difference(&rs).next().unwrap())
}

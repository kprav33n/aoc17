use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

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

struct Node {
    label: String,
    parent: Option<Rc<RefCell<Box<Node>>>>,
    weight: usize,
    total_weight: usize,
    children: Vec<Rc<RefCell<Box<Node>>>>
}

fn update_weights(node: &mut Node) {
    if node.children.is_empty() {
    } else {
        let mut children_weight = 0;
        for c in node.children.iter_mut() {
            let mut ch = c.borrow_mut();
            update_weights(&mut ch);
            children_weight += ch.total_weight;
        }
        node.total_weight = node.weight + children_weight;
    }
}

fn locate_wrong_weight(node: &Node) -> (bool, String, usize) {
    let not_found = (false, String::from(""), 0);
    if node.children.is_empty() {
        match node.parent {
            Some(ref p) => {
                for s in p.borrow().children.iter() {
                    let si = s.borrow();
                    if si.total_weight < node.total_weight {
                        return (true, node.label.clone(), node.weight - (node.total_weight - si.total_weight));
                    }
                }
                return not_found;
            }
            None => {
                return not_found;
            }
        };
    } else {
        for c in node.children.iter() {
            let ch = c.borrow();
            let r = locate_wrong_weight(&ch);
            if r.0 {
                return r;
            }
        }
        match node.parent {
            Some(ref p) => {
                for s in p.borrow().children.iter() {
                    let si = s.borrow();
                    if si.total_weight < node.total_weight {
                        return (true, node.label.clone(), node.weight - (node.total_weight - si.total_weight));
                    }
                }
                return not_found;
            }
            None => {
                return not_found;
            }
        };
    }
}

pub fn adjusted_weight(input: &str) -> (bool, String, usize) {
    let mut ls: HashMap<String, usize> = HashMap::new();
    let mut rs: HashMap<String, Vec<&str>> = HashMap::new();
    for l in input.trim().lines() {
        let v: Vec<&str> = l.split("->").collect();
        let lv: Vec<&str> = v[0].trim().split_whitespace().collect();
        assert_eq!(lv.len(), 2);
        let label = lv[0].trim();
        let weight = lv[1][1..lv[1].len() - 1].trim().parse().unwrap();
        ls.insert(label.to_string(), weight);

        if v.len() > 1 {
            let rv: Vec<&str> = v[1].trim().split(", ").collect();
            rs.insert(label.to_string(), rv);
        }
    }
    let root_label = find_bottom_program(input);
    let root_weight = *ls.get(&root_label).unwrap();
    let root = Rc::new(RefCell::new(Box::new(
        Node{label: root_label, parent: None, weight: root_weight,
             total_weight: root_weight, children: Vec::new()})));
    {
        let mut parents: Vec<Rc<RefCell<Box<Node>>>> = Vec::new();
         parents.push(root.clone());
         while !parents.is_empty() {
             let parent = parents.pop().unwrap();
             let mut pmref = parent.borrow_mut();
             match rs.get(&pmref.label) {
                 Some(children) => {
                     for c in children {
                         let label = c.to_string();
                         let weight = *ls.get(&label).unwrap();
                         let p = Some(parent.clone());
                         let n = Node{label: String::from(label), parent: p, weight: weight,
                                      total_weight: weight, children: Vec::new()};
                         let rc = Rc::new(RefCell::new(Box::new(n)));
                         pmref.children.push(rc.clone());
                         let clone = Rc::clone(&rc);
                         parents.push(clone);
                     }
                 }
                 None => {
                 }
             }
         }
    }
    let cl = root.clone();
    update_weights(&mut cl.borrow_mut());
    (String::from(""), 0);
    let cl2 = root.clone();
    let r = locate_wrong_weight(&cl2.borrow());
    r
}

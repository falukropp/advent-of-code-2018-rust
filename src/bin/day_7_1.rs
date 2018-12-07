extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

type GenError = Box<std::error::Error>;

fn process_file(path: &str) -> Result<String, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();

    let mut encountered_in_child = HashSet::new();
    let mut encountered_in_parent = HashSet::new();

    let mut children = HashMap::new();
    let mut parents = HashMap::new();

    r.lines().map(|l| l.unwrap()).for_each(|line| {
        for cap in re.captures_iter(&line) {
            let parent = *&cap[1].chars().nth(0).unwrap();
            let child = *&cap[2].chars().nth(0).unwrap();

            encountered_in_parent.insert(parent);
            encountered_in_child.insert(child);

            // Add to multimaps (both childrens and parents).
            children.entry(parent).or_insert(Vec::new()).push(child);
            parents.entry(child).or_insert(Vec::new()).push(parent);
        }
    });

    let mut solved: HashSet<char> = encountered_in_parent
        .difference(&encountered_in_child)
        .cloned()
        .collect();

    // Binray heap sorted wrong way.... VecDequq has no sort.... WTF?
    let mut solvable: Vec<char> = solved.iter().cloned().collect();
    solvable.sort_by(|a, b| b.cmp(a));

    // println!("solvable {:?}", solvable);

    let mut solution = String::new();

    // Remove top of prioritylist, add to solution and add all it's children whose parent are all solved to priority list
    while let Some(c) = solvable.pop() {
        solution.push(c);
        solved.insert(c);

        if let Some(c_children) = children.get(&c) {
            // println!("children for {} {:?}", c, c_children);
            for child in c_children.iter() {
                if !solvable.contains(child) {
                    // println!("parents for {} {:?}", child, parents.get(&child));
                    if parents
                        .get(&child)
                        .map(|parents| parents.iter().all(|p| solved.contains(p)))
                        .unwrap_or(false)
                    {
                        // println!("parents all solved for {}", *child);
                        solvable.push(*child);
                    }
                }
            }
        }
        solvable.sort_by(|a, b| b.cmp(a)); // Reverse it.
                                           // println!("---------------------");
                                           // println!("solvable {:?}", solvable);
    }

    Ok(solution)
}

fn main() {
    // let sum = process_file("src/res/day_7_ex.txt").unwrap(); // CABDFE
    let sum = process_file("src/res/day_7.txt").unwrap(); // JKNSTHCBGRVDXWAYFOQLMPZIUE
    println!("Checksum {}", sum);
}

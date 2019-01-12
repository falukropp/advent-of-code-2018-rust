extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

type GenError = Box<std::error::Error>;

fn process_file(path: &str, workers: usize, cost: u32) -> Result<(String, u32), GenError> {
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

    let mut second = 0;
    let mut worker_done = vec![0; workers];
    let mut worker_work = vec![' '; workers];

    while !solvable.is_empty() || worker_work.iter().any(|c| *c != ' ') {
        loop {
            for idx in 0..workers {
                if worker_done[idx] <= second {
                    let current_work = worker_work[idx];
                    if current_work != ' ' {
                        solution.push(current_work);
                        solved.insert(current_work);

                        if let Some(c_children) = children.get(&current_work) {
                            // println!("children for {} {:?}", current_work, c_children);
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
                    }

                    if !solvable.is_empty() {
                        let next_work = solvable.pop().unwrap();
                        worker_work[idx] = next_work;
                        worker_done[idx] = second + (next_work as u32 - 'A' as u32 + 1 + cost);
                    } else {
                        worker_work[idx] = ' ';
                    }
                }
            }
            if solvable.is_empty() || worker_work.iter().all(|c| *c != ' ') {
                break;
            }
        }
        // println!(
        //     "second {}  {:?} {:?} {:?}",
        //     second, worker_work, worker_done, solvable
        // );
        second += 1;
    }

    Ok((solution, second - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_7_2() {
        assert_eq!(
            ("CABFDE".to_owned(), 15),
            process_file("src/res/day_7_ex.txt", 2, 0).unwrap()
        );
        assert_eq!(
            ("JNSVKDYTXHRCGBOWAFQLMPZIUE".to_owned(), 755),
            process_file("src/res/day_7.txt", 5, 60).unwrap()
        );
    }
}

fn main() {
    // let sum = process_file("src/res/day_7_ex.txt", 2, 0).unwrap(); // CABFDE, 15

    let sum = process_file("src/res/day_7.txt", 5, 60).unwrap(); // Result ("JNSVKDYTXHRCGBOWAFQLMPZIUE", 755)
    println!("Result {:?}", sum);
}

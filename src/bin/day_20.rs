use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

type GenError = Box<std::error::Error>;

// Ok. This is another broken puzzle. The problem statement makes it seem much more complex that it is.
// The makes the assumption the input plots a simple maze.

fn next_matching(instructions: &str) -> Option<(usize, char)> {
    let mut level = 0;

    // println!("Looking for {} in {}", match_c, run_slice);

    for (i, c) in instructions.char_indices() {
        match c {
            '(' => level += 1,
            '|' => {
                if level == 0 {
                    return Some((i, '|'));
                }
            }
            ')' => {
                if level == 0 {
                    return Some((i, ')'));
                }
                level -= 1
            }
            _ => {}
        }
    }
    None
}

fn find_longest(instructions: &str) -> u32 {
    find_longest_rec(&str, HashSet::new(), 0, 0, 0)
}

fn find_longest_rec(
    instructions: &str,
    already_visited: &mut HashSet<(i32, i32)>,
    start_length: u32,
    start_x: i32,
    start_y: i32,
) -> u32 {
    let mut longest_sub_detour = 0;
    let mut skip_to_idx = 0;
    let mut current_length = start_length;

    for (i, c) in instructions.char_indices() {
        if i < skip_to_idx {
            // println!("skipping");
            continue;
        }
        match c {
            // TODO: Update x,y, check already visited.
            'E' | 'N' | 'S' | 'W' => current_length += 1,
            '(' => {
                let mut branch_idx_start = i + 1;
                // println!("parenthesis {} ", &instructions[branch_idx_start..]);
                while let Some((delimiter_idx_in_branch, delimiter)) =
                    next_matching(&instructions[branch_idx_start..])
                {
                    let end_of_branch = branch_idx_start + delimiter_idx_in_branch;
                    // println!(
                    //     "found sub_branch {}",
                    //     &instructions[branch_idx_start..end_of_branch]
                    // );
                    longest_sub_detour = longest_sub_detour.max(find_longest_rec(
                        &instructions[branch_idx_start..end_of_branch],
                        already_visited,
                        current_length,
                        current_x,
                        current_y,
                    ));
                    branch_idx_start = end_of_branch + 1;
                    if delimiter == ')' {
                        break;
                    }
                }
                skip_to_idx = branch_idx_start;
            }
            '$' | '|' | ')' => {
                break;
            }
            '^' => {}
            _ => panic!("What the hell!? {}", c),
        }
    }
    current_length.max(longest_sub_detour)
}

fn process_file(path: &str) -> Result<u32, GenError> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(find_longest(&s))
}

fn main() {
    assert_eq!(3, find_longest("^WNE$"));
    assert_eq!(8, find_longest("^N(EWSNEWS)$"));
    assert_eq!(8, find_longest("^N(E|EWSNEWS)$"));
    assert_eq!(12, find_longest("^N(E|EWS(N|NEEEWWWW|EWS)$"));

    // println!("{:?}", next_matching("a(b)c)dd", ')'));

    // println!("longest_distance", process_file("src/res/day_20_ex_1.txt", 0).unwrap());

    assert_eq!(10, process_file("src/res/day_20_ex_1.txt").unwrap());
    assert_eq!(18, process_file("src/res/day_20_ex_2.txt").unwrap());
    assert_eq!(23, process_file("src/res/day_20_ex_3.txt").unwrap());
    assert_eq!(31, process_file("src/res/day_20_ex_4.txt").unwrap());

    println!(
        "longest_distance {}",
        process_file("src/res/day_20.txt").unwrap()
    );
}

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

fn find_longest(instructions: &str) -> (u32, u32) {
    let mut distant_rooms = 0;
    let longest = find_longest_rec(
        &instructions,
        &mut HashSet::new(),
        &mut distant_rooms,
        0,
        0,
        0,
    );
    (longest, distant_rooms)
}

fn find_longest_rec(
    instructions: &str,
    already_visited: &mut HashSet<(i32, i32)>,
    distant_rooms: &mut u32,
    start_length: u32,
    start_x: i32,
    start_y: i32,
) -> u32 {
    let mut longest_sub_detour = 0;
    let mut skip_to_idx = 0;
    let mut current_length = start_length;
    let mut current_x = start_x;
    let mut current_y = start_y;

    for (i, c) in instructions.char_indices() {
        if i < skip_to_idx {
            // println!("skipping");
            continue;
        }
        match c {
            d @ 'E' | d @ 'N' | d @ 'S' | d @ 'W' => {
                // println!("Currently at {} {}, going {}", current_x, current_y, d);
                let mut next_x = current_x;
                let mut next_y = current_y;
                match d {
                    'E' => next_x -= 1,
                    'N' => next_y -= 1,
                    'S' => next_y += 1,
                    'W' => next_x += 1,
                    _ => panic!("Huh!? {}", d),
                }
                if already_visited.contains(&(next_x, next_y)) {
                    // println!("Already been at {} {}!", next_x, next_y);
                    break;
                } else {
                    // println!("Has not been at {} {}!", next_x, next_y);
                    current_length += 1;
                    if current_length >= 1000 {
                        *distant_rooms += 1;
                    }
                    already_visited.insert((next_x, next_y));
                    current_x = next_x;
                    current_y = next_y;
                }
            }
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
                        distant_rooms,
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

fn process_file(path: &str) -> Result<(u32, u32), GenError> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(find_longest(&s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_20() {
        assert_eq!((3, 0), find_longest("^WNE$"));
        assert_eq!((8, 0), find_longest("^N(EENNEEN)$"));
        assert_eq!((8, 0), find_longest("^N(E|WWNNWWN)$"));

        // println!("{:?}", next_matching("a(b)c)dd", ')'));

        // println!("longest_distance", process_file("src/res/day_20_ex_1.txt", 0).unwrap());

        assert_eq!((10, 0), process_file("src/res/day_20_ex_1.txt").unwrap());
        assert_eq!((18, 0), process_file("src/res/day_20_ex_2.txt").unwrap());
        assert_eq!((23, 0), process_file("src/res/day_20_ex_3.txt").unwrap());
        assert_eq!((31, 0), process_file("src/res/day_20_ex_4.txt").unwrap());

        assert_eq!((3806, 8354), process_file("src/res/day_20.txt").unwrap());
    }
}

fn main() {
    println!(
        "longest_distance {:?}",
        process_file("src/res/day_20.txt").unwrap()
    ); // (3806, 8354)
}

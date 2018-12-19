use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

type GenError = Box<std::error::Error>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum BeingType {
    Elf,
    Goblin,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Being {
    y: usize,
    x: usize,
    hit_points: i32,
    being_type: BeingType,
}

fn process_file(path: &str) -> Result<i32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut row = 0;
    let mut cave: Vec<Vec<char>> = Vec::new();
    let mut beings = Vec::new();

    r.lines().map(|l| l.unwrap()).for_each(|line| {
        line.match_indices(|c| c == 'E' || c == 'G')
            .for_each(|(idx, s)| {
                let c = s.chars().next().unwrap();
                let being_type = if c == 'E' {
                    BeingType::Elf
                } else {
                    BeingType::Goblin
                };
                beings.push(Being {
                    y: row,
                    x: idx,
                    hit_points: 200,
                    being_type: being_type,
                });
            });
        row += 1;
        cave.push(line.chars().collect());
    });

    let cave_width = cave[0].len();
    let cave_height = cave.len();

    let mut round = 0;
    // println!("Initial");

    // for row in &cave {
    //     let row_as_string: String = row.iter().collect();
    //     println!("{}", row_as_string);
    // }
    // println!("{:?}", beings);

    loop {
        beings.sort();

        for idx in 0..beings.len() {
            let mut least_enemy_move_coords: Option<(usize, usize)> = None;

            {
                let ref being = beings[idx];
                // If killed... continue
                if being.hit_points <= 0 {
                    continue;
                }

                // let enemy_type = if being.being_type == BeingType::Goblin { BeingType::Elf } else {BeingType::Goblin};
                // let enemy_char = if being.being_type == BeingType::Goblin { 'E' } else { 'G' };

                // Bleh. Loads of redundancy. Restructure when working.
                let (elves, goblins): (Vec<Being>, Vec<Being>) =
                    beings.iter().partition(|b| b.being_type == BeingType::Elf);

                let (enemies, friends) = if being.being_type == BeingType::Goblin {
                    (&elves, &goblins)
                } else {
                    (&goblins, &elves)
                };

                if !enemies.iter().any(|e| e.hit_points > 0) {
                    for row in &cave {
                        let row_as_string: String = row.iter().collect();
                        println!("{}", row_as_string);
                    }
                    println!("{:?}", beings);

                    let friends_hp = friends.iter().fold(0, |s, b| s + b.hit_points);
                    println!("END! Round {}, friends hp = {} ", round, friends_hp);
                    return Ok(round * friends_hp);
                }

                // ---------------------------------------------------------------------------
                // Move?
                // ---------------------------------------------------------------------------

                // If next to any enemy, don't move.
                let least_enemy_attack_coords = find_enemy_within_range(&enemies, being.x, being.y);
                if least_enemy_attack_coords == None {
                    least_enemy_move_coords =
                        find_first_step_in_path_to_target(&enemies, being.x, being.y);
                }
            }

            // println!("idx {} move {:?}", idx, least_enemy_move_coords);

            least_enemy_move_coords.map(|c| {
                let ref mut being = beings[idx];
                cave[being.y][being.x] = '.';
                being.x = c.0;
                being.y = c.1;
                cave[c.1][c.0] = if being.being_type == BeingType::Elf {
                    'E'
                } else {
                    'G'
                };
            });

            // ---------------------------------------------------------------------------
            // Attack?
            // ---------------------------------------------------------------------------
            let being = beings[idx];
            // Bleh. Bleh. Bleh.
            let (elves, goblins): (Vec<Being>, Vec<Being>) =
                beings.iter().partition(|b| b.being_type == BeingType::Elf);

            let enemies = if being.being_type == BeingType::Goblin {
                &elves
            } else {
                &goblins
            };

            find_enemy_within_range(&enemies, being.x, being.y).map(|c| {
                let being = find_being_at_mut(&mut beings, c.0, c.1).unwrap();
                being.hit_points -= 3;
                if being.hit_points <= 0 {
                    cave[c.1][c.0] = '.';
                }
            });
        }

        // Remove all killed.
        beings.retain(|&b| b.hit_points > 0);

        round += 1;

        // println!("After round {}", round);

        // for row in &cave {
        //     let row_as_string: String = row.iter().collect();
        //     println!("{}", row_as_string);
        // }
        // println!("{:?}", beings);
    }
}

fn find_being_at(beings: &Vec<Being>, x: usize, y: usize) -> Option<&Being> {
    for being in beings {
        if being.x == x && being.y == y && being.hit_points > 0 {
            return Some(being);
        }
    }
    return None;
}

fn find_being_at_mut(beings: &mut Vec<Being>, x: usize, y: usize) -> Option<&mut Being> {
    for being in beings {
        if being.x == x && being.y == y && being.hit_points > 0 {
            return Some(being);
        }
    }
    return None;
}

fn find_enemy_within_range(enemies: &Vec<Being>, x: usize, y: usize) -> Option<(usize, usize)> {
    let mut least_enemy_attack_coords = None;
    let mut least_enemy_hp = std::i32::MAX;
    if let Some(enemy) = find_being_at(enemies, x, y - 1) {
        if enemy.hit_points < least_enemy_hp {
            least_enemy_hp = enemy.hit_points;
            least_enemy_attack_coords = Some((enemy.x, enemy.y));
        }
    }

    if let Some(enemy) = find_being_at(enemies, x - 1, y) {
        if enemy.hit_points < least_enemy_hp {
            least_enemy_hp = enemy.hit_points;
            least_enemy_attack_coords = Some((enemy.x, enemy.y));
        }
    }

    if let Some(enemy) = find_being_at(enemies, x + 1, y) {
        if enemy.hit_points < least_enemy_hp {
            least_enemy_hp = enemy.hit_points;
            least_enemy_attack_coords = Some((enemy.x, enemy.y));
        }
    }
    if let Some(enemy) = find_being_at(enemies, x, y + 1) {
        if enemy.hit_points < least_enemy_hp {
            least_enemy_attack_coords = Some((enemy.x, enemy.y));
        }
    }

    return least_enemy_attack_coords;
}

struct SearchNode {
    y : usize,
    x : usize,
    first_y : usize,
    first_x : usize,
    distance : u32,
};

fn find_first_step_in_path_to_target(cave: &Vec<Vec<char>>, enemies: &Vec<Being>, start_x : usize, start_y : usize) -> Option<(usize, usize)> {
    let mut search_nodes : VecDeque<SearchNode> = VecDeque::new();
    let mut already_visited : HashSet<(usize, usize))> = HashSet::new();
    let mut all_closest_targets : Vec<SearchNode> = Vec::new();

    search_nodes.push_back(SearchNode {
        x : start_x,
        y : start_y,
        prev_x : 0, // Mark this as the root-node
        prev_y : 0, // Mark this as the root-node
        distance : 0
    });
    already_visited.insert((start_x, start_y));

    let mut matched_distance = std::u32::MAX;

    while let Some(current_node) = search_node.pop_front() {
        if next_node.distance > matched_distance {
            break;
        }

        if find_enemy_within_range(enemies, current_node.x, current_node.y) {
            matched_distance = current_node.distance;
            all_closest_targets.insert(current_node);
        } else {
            // Add new nodes

            // Up
            if cave[current_node.y+1][current_node.x] == '.' && !already_visited.contains((current_node.x, current_node.y+1)) {
                search_nodes.push_back(SearchNode {
                    x : current_node.x,
                    y : current_node.y+1,
                    prev_x : if current_node.prev_x == 0 { current_node.x } else { current_node.prev_x },
                    prev_y : if current_node.prev_y == 0 { current_node.y+1 } else { current_node.prev_y }, // Mark this as the root-node
                    distance : current_node.distance + 1,
                });
            }

            // Left
            // Right
            // Down
        }

    }

    if all_closest_targets.is_empty() {
        return None;
    }

    all_closest_targets.sort();
    let selected target = all_closest_targets[0];
    // Return the starting node for the selected target
    Some(target.first_x, first_y)
}

fn generate_search_node() {
            if cave[current_node.y+1][current_node.x] == '.' && !already_visited.contains((current_node.x, current_node.y+1)) {
                search_nodes.push_back(SearchNode {
                    x : current_node.x,
                    y : current_node.y+1,
                    prev_x : if current_node.prev_x == 0 { current_node.x } else { current_node.prev_x },
                    prev_y : if current_node.prev_y == 0 { current_node.y+1 } else { current_node.prev_y }, // Mark this as the root-node
                    distance : current_node.distance + 1,
                });
            }

}

// fn create_distance_map(cave: &Vec<Vec<char>>, start_coords: &Vec<(usize, usize)>) -> Vec<Vec<u32>> {
//     let mut coords: VecDeque<(usize, usize)> = start_coords.iter().map(|v| v.clone()).collect();
//     let cave_width = cave[0].len();
//     let cave_height = cave.len();
//     let mut result = vec![vec![std::u32::MAX; cave_width]; cave_height];

//     while let Some(coord) = coords.pop_front() {
//         let prev_val = result[coord.1][coord.0];
//         let val = if prev_val == std::u32::MAX {
//             0
//         } else {
//             prev_val + 1
//         };

//         // Up
//         if coord.1 > 0
//             && cave[coord.1 - 1][coord.0] == '.'
//             && result[coord.1 - 1][coord.0] == std::u32::MAX
//         {
//             result[coord.1 - 1][coord.0] = val;
//             coords.push_back((coord.0, coord.1 - 1));
//         }
//         // Left
//         if coord.0 > 0
//             && cave[coord.1][coord.0 - 1] == '.'
//             && result[coord.1][coord.0 - 1] == std::u32::MAX
//         {
//             result[coord.1][coord.0 - 1] = val;
//             coords.push_back((coord.0 - 1, coord.1));
//         } // Right
//         if coord.0 < cave_width - 1
//             && cave[coord.1][coord.0 + 1] == '.'
//             && result[coord.1][coord.0 + 1] == std::u32::MAX
//         {
//             result[coord.1][coord.0 + 1] = val;
//             coords.push_back((coord.0 + 1, coord.1));
//         }
//         // Down
//         if coord.1 < cave_height - 1
//             && cave[coord.1 + 1][coord.0] == '.'
//             && result[coord.1 + 1][coord.0] == std::u32::MAX
//         {
//             result[coord.1 + 1][coord.0] = val;
//             coords.push_back((coord.0, coord.1 + 1));
//         }
//     }

//     return result;
// }

fn main() {
    // assert_eq!(27828, process_file("src/res/day_15_ex_1.txt").unwrap());
    // assert_eq!(27730, process_file("src/res/day_15_ex_2.txt").unwrap());
    // assert_eq!(36334, process_file("src/res/day_15_ex_3.txt").unwrap());
    // assert_eq!(39514, process_file("src/res/day_15_ex_4.txt").unwrap());
    // assert_eq!(27755, process_file("src/res/day_15_ex_5.txt").unwrap());
    // assert_eq!(28944, process_file("src/res/day_15_ex_6.txt").unwrap());
    // assert_eq!(18740, process_file("src/res/day_15_ex_7.txt").unwrap());

    assert_eq!(204531, process_file("src/res/day_15.txt").unwrap());
}

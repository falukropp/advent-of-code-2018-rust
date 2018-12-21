use std::collections::HashSet;
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

    // let cave_width = cave[0].len();
    // let cave_height = cave.len();

    let inital_elves = beings
        .iter()
        .filter(|&b| (*b).being_type == BeingType::Elf)
        .count();

    // println!("Initial");

    // for row in &cave {
    //     let row_as_string: String = row.iter().collect();
    //     println!("{}", row_as_string);
    // }
    // println!("{:?}", beings);

    for elf_attack_power in 4.. {
        let mut beings_in_battle = beings.clone();
        let mut cave_for_battle = cave.clone();
        let result = do_battle(
            &mut cave_for_battle,
            &mut beings_in_battle,
            elf_attack_power,
        )?;
        beings_in_battle.retain(|&b| b.hit_points > 0);

        if beings_in_battle.len() == inital_elves
            && beings_in_battle[0].being_type == BeingType::Elf
        {
            for row in cave_for_battle {
                let row_as_string: String = row.iter().collect();
                println!("{}", row_as_string);
            }
            println!("{:?}", beings_in_battle);
            println!("{:?}", elf_attack_power);

            return Ok(result);
        }
    }
    panic!("what?");
}

fn do_battle(
    cave: &mut Vec<Vec<char>>,
    beings: &mut Vec<Being>,
    elf_attack_power: i32,
) -> Result<i32, GenError> {
    let mut round = 0;
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
                    // for row in cave {
                    //     let row_as_string: String = row.iter().collect();
                    //     println!("{}", row_as_string);
                    // }
                    // println!("{:?}", beings);

                    let friends_hp = friends.iter().fold(0, |s, b| s + b.hit_points);
                    // println!("END! Round {}, friends hp = {} ", round, friends_hp);
                    return Ok(round * friends_hp);
                }

                // ---------------------------------------------------------------------------
                // Move?
                // ---------------------------------------------------------------------------

                // If next to any enemy, don't move.
                // println!("idx {} looking for move", idx);

                let least_enemy_attack_coords = find_enemy_within_range(&enemies, being.x, being.y);
                if least_enemy_attack_coords == None {
                    // println!("No enemies to attack for {} ", idx);
                    least_enemy_move_coords =
                        find_first_step_in_path_to_target(cave, &enemies, being.x, being.y);
                }
                // println!("idx {} done looking for move", idx);
            }

            // println!("idx {} move {:?}", idx, least_enemy_move_coords);

            if let Some(c) = least_enemy_move_coords {
                let ref mut being = beings[idx];
                cave[being.y][being.x] = '.';
                being.x = c.0;
                being.y = c.1;
                cave[c.1][c.0] = if being.being_type == BeingType::Elf {
                    'E'
                } else {
                    'G'
                };
            };

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

            // println!("idx {} looking for enemies", idx);
            if let Some(c) = find_enemy_within_range(&enemies, being.x, being.y) {
                let attack_power_for_being: i32 = if being.being_type == BeingType::Elf {
                    elf_attack_power
                } else {
                    3
                };
                let enemy = find_being_at_mut(beings, c.0, c.1).unwrap();
                enemy.hit_points -= attack_power_for_being;
                if enemy.hit_points <= 0 {
                    cave[c.1][c.0] = '.';
                }
            };
            // println!("idx {} done looking for enemies", idx);
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct SearchNode {
    y: usize,
    x: usize,
    first_y: usize,
    first_x: usize,
    distance: u32,
}

fn find_first_step_in_path_to_target(
    cave: &Vec<Vec<char>>,
    enemies: &Vec<Being>,
    start_x: usize,
    start_y: usize,
) -> Option<(usize, usize)> {
    let mut search_nodes: VecDeque<SearchNode> = VecDeque::new();
    let mut already_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut all_closest_targets: Vec<SearchNode> = Vec::new();

    // println!(
    //     "Entering First steps in path to target! {} {}",
    //     start_x, start_y
    // );

    search_nodes.push_back(SearchNode {
        x: start_x,
        y: start_y,
        first_x: 0, // Mark this as the root-node
        first_y: 0, // Mark this as the root-node
        distance: 0,
    });
    already_visited.insert((start_x, start_y));

    let mut matched_distance = std::u32::MAX;

    while let Some(current_node) = search_nodes.pop_front() {
        // println!("Looking at node {:?}", current_node);
        if current_node.distance > matched_distance {
            // println!("Bigger distance than closest! Quit!");
            break;
        }

        if find_enemy_within_range(enemies, current_node.x, current_node.y).is_some() {
            // println!("Found enemy from from current node");
            matched_distance = current_node.distance;
            all_closest_targets.push(current_node);
        } else if matched_distance == std::u32::MAX {
            // println!("Checking empty spaces nearby");
            // Add new nodes
            for d in [(0, -1), (-1, 0), (1, 0), (0, 1)].iter() {
                let next_x = (current_node.x as i32 + d.0) as usize;
                let next_y = (current_node.y as i32 + d.1) as usize;

                // println!("Checking {} {}", next_x, next_y);

                if cave[next_y][next_x] == '.' && !already_visited.contains(&(next_x, next_y)) {
                    search_nodes.push_back(SearchNode {
                        x: next_x,
                        y: next_y,
                        first_x: if current_node.first_x == 0 {
                            next_x
                        } else {
                            current_node.first_x
                        },
                        first_y: if current_node.first_y == 0 {
                            next_y
                        } else {
                            current_node.first_y
                        },
                        distance: current_node.distance + 1,
                    });
                    already_visited.insert((next_x, next_y));
                }
            }
        }
    }

    if all_closest_targets.is_empty() {
        return None;
    }

    all_closest_targets.sort();
    let target = all_closest_targets[0];
    // Return the starting node for the selected target

    // println!("Selecting move! {} {}", target.first_x, target.first_y);
    Some((target.first_x, target.first_y))
}

fn main() {
    // assert_eq!(27828, process_file("src/res/day_15_ex_1.txt").unwrap());
    assert_eq!(4988, process_file("src/res/day_15_ex_2.txt").unwrap()); // 15 power
    assert_eq!(31284, process_file("src/res/day_15_ex_4.txt").unwrap());

    assert_eq!(3478, process_file("src/res/day_15_ex_5.txt").unwrap());
    assert_eq!(6474, process_file("src/res/day_15_ex_6.txt").unwrap());
    assert_eq!(1140, process_file("src/res/day_15_ex_7.txt").unwrap());

    assert_eq!(49863, process_file("src/res/day_15.txt").unwrap());
}

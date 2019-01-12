use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct SearchNode {
    distance: u32,
    y: usize,
    x: usize,
    // 0 = neither
    // 1 = torch
    // 2 = climbing gear
    gear: usize,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &SearchNode) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.gear.cmp(&other.gear))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &SearchNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_route(depth: u32, target_x: usize, target_y: usize) -> u32 {
    let max_coord = target_x.max(target_y) + 50;

    let mut geological_index = vec![vec![0; max_coord + 1]; max_coord + 1];

    let modulo = 20183;

    for x in 0..=max_coord {
        geological_index[0][x] = (x as u32) * 16807;
    }

    for y in 1..=max_coord {
        geological_index[y][0] = (y as u32) * 48271;
        for x in 1..=max_coord {
            let erosion_level_left = (geological_index[y][x - 1] + depth) % modulo;
            let erosion_level_up = (geological_index[y - 1][x] + depth) % modulo;

            geological_index[y][x] = erosion_level_left * erosion_level_up;
        }
    }
    geological_index[target_x][target_y] = 0;

    let mut cave = vec![vec![' '; max_coord + 1]; max_coord + 1];

    for y in 0..=max_coord {
        for x in 0..=max_coord {
            cave[y][x] = match ((geological_index[y][x] + depth) % modulo) % 3 {
                0 => '.',
                1 => '=',
                2 => '|',
                _ => '?',
            }
        }
    }

    let mut min_distance = vec![vec![vec![std::u32::MAX; max_coord + 1]; max_coord + 1]; 3];
    // 0 = neither
    // 1 = torch
    // 2 = climbing gear

    let mut search_nodes = BinaryHeap::new();
    search_nodes.push(SearchNode {
        distance: 0,
        x: 0,
        y: 0,
        gear: 1, // Start with torch
    });

    while let Some(current_node) = search_nodes.pop() {
        if current_node.x == target_x && current_node.y == target_y && current_node.gear == 1 {
            return current_node.distance;
        }

        // println!("{:?}", current_node);

        if current_node.distance > min_distance[current_node.gear][current_node.y][current_node.x] {
            continue;
        }
        min_distance[current_node.gear][current_node.y][current_node.x] = current_node.distance;

        // Change gear
        let other_gear = match cave[current_node.y][current_node.x] {
            '.' => {
                if current_node.gear == 1 {
                    2
                } else {
                    1
                }
            }
            '=' => {
                if current_node.gear == 0 {
                    2
                } else {
                    0
                }
            }
            '|' => {
                if current_node.gear == 0 {
                    1
                } else {
                    0
                }
            }
            t => panic!("Stange cave tile found '{}'", t),
        };

        let mut next_cost = current_node.distance + 7;
        if next_cost < min_distance[other_gear][current_node.y][current_node.x] {
            min_distance[other_gear][current_node.y][current_node.x] = next_cost;
            search_nodes.push(SearchNode {
                distance: current_node.distance + 7,
                x: current_node.x,
                y: current_node.y,
                gear: other_gear,
            });
        }

        next_cost = current_node.distance + 1;
        // Go somewhere
        for d in [(0, -1), (-1, 0), (1, 0), (0, 1)].iter() {
            if d.0 < 0 && current_node.x == 0 {
                continue;
            }
            if d.1 < 0 && current_node.y == 0 {
                continue;
            }
            let next_x = (current_node.x as i32 + d.0) as usize;
            let next_y = (current_node.y as i32 + d.1) as usize;

            // println!("current_gear {} cave at {} {} is ",current_node.gear,  )

            // Right tool for the next tile?
            match cave[next_y][next_x] {
                '.' => {
                    if current_node.gear == 0 {
                        continue;
                    }
                }
                '=' => {
                    if current_node.gear == 1 {
                        continue;
                    }
                }
                '|' => {
                    if current_node.gear == 2 {
                        continue;
                    }
                }
                t => panic!("Stange cave tile found '{}'", t),
            }

            if next_cost < min_distance[current_node.gear][next_y][next_x] {
                min_distance[current_node.gear][next_y][next_x] = next_cost;
                search_nodes.push(SearchNode {
                    distance: next_cost,
                    x: next_x,
                    y: next_y,
                    gear: current_node.gear,
                });
            }
        }
    }

    12345678 // Should never happen.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_22_2() {
        assert_eq!(45, find_shortest_route(510, 10, 10));

        assert_eq!(1089, find_shortest_route(11991, 6, 797));
    }
}

fn main() {
    println!("shortest_route {}", find_shortest_route(510, 10, 10)); // 45

    println!("shortest_route {}", find_shortest_route(11991, 6, 797)); // 1089
}

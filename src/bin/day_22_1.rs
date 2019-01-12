#[macro_use]
extern crate log;

use log::Level::Debug;

fn dump_cave(cave: &Vec<Vec<u32>>) -> String {
    let mut result = String::new();
    for row in cave {
        for col in row {
            result.push(match col % 3 {
                0 => '.',
                1 => '=',
                2 => '|',
                _ => '?',
            });
        }
        result.push('\n');
    }
    result
}

fn dump_erosion(gelogical_index: &Vec<Vec<u32>>, depth: u32, modulo: u32) -> String {
    let mut result = String::new();
    for row in gelogical_index {
        for col in row {
            result.push(match ((col + depth) % modulo) % 3 {
                0 => '.',
                1 => '=',
                2 => '|',
                _ => '?',
            });
        }
        result.push('\n');
    }
    result
}

fn sum_cave(gelogical_index: &Vec<Vec<u32>>, depth: u32, modulo: u32) -> u32 {
    let mut sum = 0;
    for row in gelogical_index {
        for col in row {
            sum += ((col + depth) % modulo) % 3;
        }
    }
    sum
}

fn calculate_risk_level(depth: u32, max_x: usize, max_y: usize) -> u32 {
    let mut gelogical_index = vec![vec![0; max_x + 1]; max_y + 1];

    let modulo = 20183;

    for x in 0..=max_x {
        gelogical_index[0][x] = (x as u32) * 16807;
    }

    for y in 1..=max_y {
        gelogical_index[y][0] = (y as u32) * 48271;
        for x in 1..=max_x {
            let erosion_level_left = (gelogical_index[y][x - 1] + depth) % modulo;
            let erosion_level_up = (gelogical_index[y - 1][x] + depth) % modulo;

            gelogical_index[y][x] = erosion_level_left * erosion_level_up;
        }
    }
    gelogical_index[max_y][max_x] = 0;

    if log_enabled!(Debug) {
        debug!("\n{}", dump_cave(&gelogical_index));
        debug!("\n{}", dump_erosion(&gelogical_index, depth, modulo));
    }

    sum_cave(&gelogical_index, depth, modulo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_22_1() {
        assert_eq!(114, calculate_risk_level(510, 10, 10));

        assert_eq!(5622, calculate_risk_level(11991, 6, 797));
    }
}

fn main() {
    env_logger::init();
    println!("risk_level {}", calculate_risk_level(510, 10, 10)); // 114

    println!("risk_level {}", calculate_risk_level(11991, 6, 797)); // 5622
}

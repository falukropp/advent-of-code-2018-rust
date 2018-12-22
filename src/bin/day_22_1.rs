fn dump_cave(cave: &Vec<Vec<u32>>) {
    for row in cave {
        for col in row {
            print!(
                "{}",
                match (col % 3) {
                    0 => '.',
                    1 => '=',
                    2 => '|',
                    _ => '?',
                }
            );
        }
        println!("");
    }
}

fn dump_erosion(gelogical_index: &Vec<Vec<u32>>, depth: u32, modulo: u32) {
    for row in gelogical_index {
        for col in row {
            print!(
                "{}",
                match (((col + depth) % modulo) % 3) {
                    0 => '.',
                    1 => '=',
                    2 => '|',
                    _ => '?',
                }
            );
        }
        println!("");
    }
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
        gelogical_index[0][x] = ((x as u32) * 16807);
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

    /*     for row in &gelogical_index {
        for col in row {
            print!("{} ", col);
        }
        println!("");
    }

 */

    // dump_cave(&gelogical_index);
    // println!("");
    // dump_erosion(&gelogical_index, depth, modulo);

    sum_cave(&gelogical_index, depth, modulo)
}

fn main() {
    println!("risk_level {}", calculate_risk_level(510, 10, 10)); // 114

    println!("risk_level {}", calculate_risk_level(11991, 6, 797)); // 5622
}

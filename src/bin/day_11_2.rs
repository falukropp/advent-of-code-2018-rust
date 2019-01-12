type GenError = Box<std::error::Error>;

fn find_power_level(x: i32, y: i32, grid_serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;

    power_level += grid_serial;
    power_level %= 1000;

    power_level *= rack_id;
    power_level %= 1000;

    power_level /= 100;
    power_level -= 5;

    power_level
}

fn find_max_kernel(
    grid_serial: i32,
    width: i32,
    height: i32,
    max_square_size: i32,
) -> Result<(i32, i32, i32, i32), GenError> {
    let mut cumulative_power_level = vec![
        vec![0; (width + max_square_size + 1) as usize];
        (height + max_square_size + 1) as usize
    ];
    for y in 1..=height + max_square_size {
        let mut row_total = 0;
        for x in 1..=width + max_square_size {
            let cell_power_level = find_power_level(x, y, grid_serial);
            cumulative_power_level[y as usize][x as usize] =
                cell_power_level + cumulative_power_level[(y - 1) as usize][x as usize] + row_total;
            row_total += cell_power_level;
        }
    }

    // for y in 1..=height + max_square_size {
    //     for x in 1..=width + max_square_size{
    //         print!("{:8}", find_power_level(x, y, grid_serial));
    //     }
    //     println!("");
    // }
    // println!("-----------------------------------------------");

    // for row in &cumulative_power_level {
    //     for col in row {
    //         print!("{:8}", col);
    //     }
    //     println!("");
    // }

    let mut max_kernel_for_all_sizes = 0;
    let mut result = (0, 0, 0, 0);

    for s in 1..=max_square_size {
        let mut largest_pair = (0, 0);
        let mut max_kernel_sum = 0;

        for y in 1..=(height - s + 1) {
            for x in 1..=(width - s + 1) {
                let top = y - 1;
                let left = x - 1;
                let bottom = top + s;
                let right = left + s;
                let mut kernel_sum = cumulative_power_level[bottom as usize][right as usize];
                if top > 0 {
                    kernel_sum -= cumulative_power_level[top as usize][right as usize];
                    if left > 0 {
                        kernel_sum += cumulative_power_level[top as usize][left as usize];
                    }
                }
                if left > 0 {
                    kernel_sum -= cumulative_power_level[bottom as usize][left as usize];
                }

                if kernel_sum > max_kernel_sum {
                    max_kernel_sum = kernel_sum;
                    largest_pair = (x, y);
                }
            }
        }
        // println!("{} Max coords {:?} {}",s, largest_pair, max_kernel_sum);

        if max_kernel_sum > max_kernel_for_all_sizes {
            max_kernel_for_all_sizes = max_kernel_sum;
            result = (largest_pair.0, largest_pair.1, s, max_kernel_sum);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_11_2() {
        assert_eq!(
            (90, 269, 16, 113),
            find_max_kernel(18, 300, 300, 300).unwrap()
        );
        assert_eq!(
            (232, 251, 12, 119),
            find_max_kernel(42, 300, 300, 300).unwrap()
        );
        assert_eq!(
            (233, 187, 13, 91),
            find_max_kernel(7400, 300, 300, 300).unwrap()
        );
    }
}

fn main() {
    println!(
        "Max coords {:?}",
        find_max_kernel(18, 300, 300, 300).unwrap()
    ); // (90,269,16,113)
    println!(
        "Max coords {:?}",
        find_max_kernel(42, 300, 300, 300).unwrap()
    ); // (232,251,12,119)

    println!(
        "Max coords {:?}",
        find_max_kernel(7400, 300, 300, 300).unwrap()
    ); // (233,187,13,91)
}

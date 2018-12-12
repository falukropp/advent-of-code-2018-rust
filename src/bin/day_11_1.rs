extern crate regex;

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

fn find_max_kernel(grid_serial: i32, width: i32, height: i32) -> Result<(i32, i32), GenError> {
    let mut largest_pair = (0, 0);
    let mut max_kernel_sum = 0;

    for y in 1..=height {
        for x in 1..=width {
            let mut kernel_sum = 0;

            // BRUTAL!
            for ky in 0..3 {
                for kx in 0..3 {
                    kernel_sum += find_power_level(x + kx, y + ky, grid_serial);
                }
            }

            if kernel_sum > max_kernel_sum {
                max_kernel_sum = kernel_sum;
                largest_pair = (x, y);
            }
        }
    }

    Ok(largest_pair)
}

fn main() {
    println!("Max coords {:?}", find_max_kernel(18, 300, 300).unwrap()); // (33,45)
    println!("Max coords {:?}", find_max_kernel(42, 300, 300).unwrap()); // (21,61)

    println!("Max coords {:?}", find_max_kernel(7400, 300, 300).unwrap()); // (34,72)
}

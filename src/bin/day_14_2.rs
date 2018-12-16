extern crate regex;

use std::vec::Vec;

fn find_num_recipes(match_as_string: &str) -> u32 {
    let mut elf_1_pos = 0;
    let mut elf_2_pos = 1;

    let match_as_vec: Vec<usize> = match_as_string
        .chars()
        .map(|c| c as usize - '0' as usize)
        .collect();

    let mut recipes = Vec::new();
    recipes.push(3);
    recipes.push(7);

    loop {
        let new_recipe = recipes[elf_1_pos] + recipes[elf_2_pos];
        if new_recipe >= 10 {
            recipes.push(1);
            if recipes.ends_with(&match_as_vec) {
                return (recipes.len() - match_as_vec.len()) as u32;
            }
            recipes.push(new_recipe - 10);
        } else {
            recipes.push(new_recipe);
        }

        if recipes.ends_with(&match_as_vec) {
            return (recipes.len() - match_as_vec.len()) as u32;
        }

        elf_1_pos = (elf_1_pos + recipes[elf_1_pos] + 1) % recipes.len();
        elf_2_pos = (elf_2_pos + recipes[elf_2_pos] + 1) % recipes.len();
        // println!("{:?}", recipes);
    }
}

fn main() {
    // let collision_at = process_file("src/res/day_13_ex.txt").unwrap(); // (7,3)
    assert_eq!(5, find_num_recipes("01245"));
    assert_eq!(9, find_num_recipes("51589"));
    assert_eq!(18, find_num_recipes("92510"));
    assert_eq!(2018, find_num_recipes("59414"));

    println!("{}", find_num_recipes("509671"));
}

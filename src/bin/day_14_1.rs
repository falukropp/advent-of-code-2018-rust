extern crate regex;

use std::vec::Vec;

fn find_score(num_recipies: usize) -> String {
    let mut elf_1_pos = 0;
    let mut elf_2_pos = 1;

    let final_size = num_recipies + 10;

    let mut recipes = Vec::with_capacity(final_size);
    recipes.push(3);
    recipes.push(7);

    while recipes.len() < final_size {
        let new_recipe = recipes[elf_1_pos] + recipes[elf_2_pos];
        if new_recipe >= 10 {
            recipes.push(1);
            recipes.push(new_recipe - 10);
        } else {
            recipes.push(new_recipe);
        }

        elf_1_pos = (elf_1_pos + recipes[elf_1_pos] + 1) % recipes.len();
        elf_2_pos = (elf_2_pos + recipes[elf_2_pos] + 1) % recipes.len();
        // println!("{:?}", recipes);
    }
    recipes[num_recipies..num_recipies + 10]
        .iter()
        .map(|n| n.to_string())
        .collect()
}

fn main() {
    // let collision_at = process_file("src/res/day_13_ex.txt").unwrap(); // (7,3)
    assert_eq!("0124515891", find_score(5));
    assert_eq!("5158916779", find_score(9));
    assert_eq!("9251071085", find_score(18));
    assert_eq!("5941429882", find_score(2018));

    println!("{}", find_score(509671));
}

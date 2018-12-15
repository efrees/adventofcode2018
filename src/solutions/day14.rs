pub fn solve() {
    println!("Day 14");

    let input = 260321;
    let mut recipes = Vec::<u8>::with_capacity(input + 10);
    recipes.push(3);
    recipes.push(7);

    let mut elf_positions = (0, 1);
    build_recipes_to(&mut recipes, input + 10, &mut elf_positions);

    let mut next_ten = String::with_capacity(10);
    for i in 0..10 {
        next_ten.push((recipes[input + i] + 0x30) as char);
    }

    println!("Next ten: {}", next_ten);

    let input = [2, 6, 0, 3, 2, 1];
    for start_pos in 0.. {
        if start_pos > recipes.len() - 6 {
            let new_size = recipes.len() + 1000;
            build_recipes_to(&mut recipes, new_size, &mut elf_positions)
        }
        let mut matching = true;
        for i in 0..6 {
            if recipes[start_pos + i] != input[i] {
                matching = false;
                break;
            }
        }

        if matching {
            println!("Count before match: {}", start_pos);
            break;
        }
    }
}

fn build_recipes_to(
    recipes: &mut Vec<u8>,
    target_count: usize,
    elf_positions: &mut (usize, usize),
) {
    let mut first_elf = elf_positions.0;
    let mut second_elf = elf_positions.1;

    while recipes.len() < target_count {
        let first_recipe = recipes[first_elf];
        let second_recipe = recipes[second_elf];
        let total_score = first_recipe + second_recipe;

        if total_score >= 10 {
            recipes.push(total_score / 10);
        }
        recipes.push(total_score % 10);

        first_elf += 1 + first_recipe as usize;
        second_elf += 1 + second_recipe as usize;

        first_elf %= recipes.len();
        second_elf %= recipes.len();
    }

    elf_positions.0 = first_elf;
    elf_positions.1 = second_elf;
}

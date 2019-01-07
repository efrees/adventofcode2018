use std::collections::HashSet;

#[allow(unused)]
fn test_input() -> (u32, (usize, usize)) {
    (510, (10, 10))
}

#[allow(unused)]
fn input() -> (u32, (usize, usize)) {
    (4845, (6, 770))
}

pub fn solve() {
    let input = input();
    let depth = input.0;
    let target_coords = input.1;

    let y0_root = 16807;
    let x0_root = 48271;
    let erosion_mod = 20183;

    let mut risk_level = 0;

    let mut prev_row = Vec::new();
    for x_index in 0..=target_coords.0 {
        let geo_val = if x_index == 0 {
            0
        } else {
            (x_index * y0_root) as u32
        };
        let erosion_level = (geo_val + depth) % erosion_mod;
        prev_row.push(erosion_level);

        let cave_type = erosion_level % 3;
        risk_level += cave_type;
        //print!("{}", cave_type);
    }
    //println!();

    for y_index in 1..=target_coords.1 {
        let mut cur_row = Vec::with_capacity(prev_row.len());

        for x_index in 0..=target_coords.0 {
            let geo_val = if (x_index, y_index) == target_coords {
                0
            } else if x_index == 0 {
                (y_index * x0_root) as u32
            } else {
                cur_row[x_index - 1] * prev_row[x_index]
            };

            let erosion_level = (geo_val + depth) % erosion_mod;
            cur_row.push(erosion_level);

            let cave_type = erosion_level % 3;
            risk_level += cave_type;
            //print!("{}", cave_type);
        }
        //println!();
        prev_row = cur_row;
    }

    println!("Risk level: {}", risk_level); //= 5400
}

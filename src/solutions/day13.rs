use adventlib::grid::*;
use std::collections::HashSet;

pub fn solve() {
    println!("Day 13");

    let lines = adventlib::read_input_lines("day13input.txt");
    let grid = parse_grid(&lines);
    let mut carts = initiate_carts(&lines);
    let mut occupied_positions: HashSet<Point> = carts.iter().map(|c| c.position).collect();

    let mut collision_happened = false;
    while carts.len() > 1 {
        carts.sort_unstable_by_key(|c| (c.position.y, c.position.x));

        for cart_index in 0..carts.len() {
            if carts[cart_index].crashed {
                continue; // virtually removed already
            }

            occupied_positions.remove(&carts[cart_index].position);

            move_cart_on_grid(&mut carts[cart_index], &grid);

            let new_position = carts[cart_index].position;
            if occupied_positions.contains(&new_position) {
                if !collision_happened {
                    collision_happened = true;
                    println!("First collision at {},{}", new_position.x, new_position.y);
                }
                carts
                    .iter_mut()
                    .filter(|c| c.position == new_position)
                    .for_each(|c| c.crashed = true);
                occupied_positions.remove(&new_position);
            } else {
                occupied_positions.insert(new_position);
            }
        }

        carts.retain(|c| !c.crashed);
    }

    println!(
        "Last cart at {},{}",
        carts[0].position.x, carts[0].position.y
    );
}

fn parse_grid(lines: &Vec<String>) -> Vec<Vec<RailType>> {
    let mut grid_rows = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        // All ASCII - byte len is fine.
        let mut row = Vec::<RailType>::with_capacity(line.len());
        for byte in line.as_bytes() {
            let track_type = match byte {
                b'|' | b'^' | b'v' => RailType::Vertical,
                b'-' | b'<' | b'>' => RailType::Horizontal,
                b'/' => RailType::Corner('/'),
                b'\\' => RailType::Corner('\\'),
                b'+' => RailType::Intersection,
                _ => RailType::None,
            };
            row.push(track_type);
        }
        grid_rows.push(row);
    }
    return grid_rows;
}

fn initiate_carts(lines: &Vec<String>) -> Vec<Cart> {
    let mut carts = Vec::new();
    let mut y = 0;

    for line in lines {
        let mut x = 0;
        for byte in line.as_bytes() {
            match byte {
                b'^' => carts.push(Cart {
                    intersection_count: 0,
                    direction: Direction::Up,
                    position: Point::new(x, y),
                    crashed: false,
                }),
                b'v' => carts.push(Cart {
                    intersection_count: 0,
                    direction: Direction::Down,
                    position: Point::new(x, y),
                    crashed: false,
                }),
                b'<' => carts.push(Cart {
                    intersection_count: 0,
                    direction: Direction::Left,
                    position: Point::new(x, y),
                    crashed: false,
                }),
                b'>' => carts.push(Cart {
                    intersection_count: 0,
                    direction: Direction::Right,
                    position: Point::new(x, y),
                    crashed: false,
                }),
                _ => (),
            }
            x += 1;
        }
        y += 1;
    }
    return carts;
}

fn move_cart_on_grid(cart: &mut Cart, grid: &Vec<Vec<RailType>>) {
    let move_vector = cart.direction.as_vector();
    let new_location = Point::new(
        cart.position.x + move_vector.x,
        cart.position.y + move_vector.y,
    );

    // only non-negative coordinates, so cast is fine
    match grid[new_location.y as usize][new_location.x as usize] {
        RailType::Vertical => (),
        RailType::Horizontal => (),
        RailType::Corner(c) => cart.direction = cart.direction.turn_corner(c),
        RailType::Intersection => {
            match cart.intersection_count % 3 {
                0 => cart.direction = cart.direction.turn_left(),
                2 => cart.direction = cart.direction.turn_right(),
                _ => (),
            };
            cart.intersection_count += 1;
        }
        _ => println!("Off track at {},{}", new_location.x, new_location.y),
    }
    cart.position = new_location;
}

#[derive(Debug)]
struct Cart {
    position: Point,
    direction: Direction,
    intersection_count: u32,
    crashed: bool,
}

enum RailType {
    None,
    Vertical,
    Horizontal,
    Intersection,
    Corner(char),
}

use adventlib::grid::*;
use std::collections::HashSet;

pub fn solve() {
    println!("Day 13");

    let lines = adventlib::read_input_lines("day13input.txt");
    let grid = parse_grid(&lines);
    let mut carts = initiate_carts(&lines);
    let mut occupied_positions: HashSet<Point> = carts.iter().map(|c| c.position).collect();

    let mut collision_location = Point::new(-1, -1);
    let mut collision_happened = false;
    while !collision_happened {
        carts.sort_unstable_by_key(|c| c.position);

        //move each cart by rules, checking for collision
        for mut cart in carts.iter_mut() {
            occupied_positions.remove(&cart.position);

            move_cart_on_grid(&mut cart, &grid);

            //capture collision location when it happens
            if occupied_positions.contains(&cart.position) {
                collision_happened = true;
                collision_location = cart.position;
                break;
            }

            occupied_positions.insert(cart.position);
        }
    }

    println!(
        "First collision at {},{}",
        collision_location.x, collision_location.y
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
                    intersectionCount: 0,
                    direction: Direction::Up,
                    position: Point::new(x, y),
                }),
                b'v' => carts.push(Cart {
                    intersectionCount: 0,
                    direction: Direction::Down,
                    position: Point::new(x, y),
                }),
                b'<' => carts.push(Cart {
                    intersectionCount: 0,
                    direction: Direction::Left,
                    position: Point::new(x, y),
                }),
                b'>' => carts.push(Cart {
                    intersectionCount: 0,
                    direction: Direction::Right,
                    position: Point::new(x, y),
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
            match cart.intersectionCount % 3 {
                0 => cart.direction = cart.direction.turn_left(),
                2 => cart.direction = cart.direction.turn_right(),
                _ => (),
            };
            cart.intersectionCount += 1;
        }
        _ => println!("Off track at {},{}", new_location.x, new_location.y),
    }
    cart.position = new_location;
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_vector(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    fn is_horizontal(&self) -> bool {
        *self == Direction::Left || *self == Direction::Right
    }

    fn is_vertical(&self) -> bool {
        *self == Direction::Up || *self == Direction::Down
    }

    fn turn_corner(&self, c_type: char) -> Direction {
        if self.is_vertical() {
            return match c_type {
                '/' => self.turn_right(),
                _ => self.turn_left(),
            };
        }
        return match c_type {
            '/' => self.turn_left(),
            _ => self.turn_right(),
        };
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug)]
struct Cart {
    position: Point,
    direction: Direction,
    intersectionCount: u32,
}

enum RailType {
    None,
    Vertical,
    Horizontal,
    Intersection,
    Corner(char),
}

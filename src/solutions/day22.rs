use std::cmp::Ordering;
use std::collections::*;

type Coords = (usize, usize);

#[allow(unused)]
fn test_input() -> (u32, Coords) {
    (510, (10, 10))
}

#[allow(unused)]
fn input() -> (u32, Coords) {
    (4845, (6, 770))
}

const Y0_ROOT: usize = 16807;
const X0_ROOT: usize = 48271;
const EROSION_MOD: u32 = 20183;

pub fn solve() {
    println!("Day 22");
    let input_params = input();
    let target_coords = input_params.1;

    let mut risk_level = 0;
    let mut erosion_levels = HashMap::<Coords, u32>::new();

    for y_index in 0..=target_coords.1 {
        for x_index in 0..=target_coords.0 {
            let cave_type = get_cave_type(x_index, y_index, &input_params, &mut erosion_levels);
            risk_level += cave_type as u32;
        }
    }

    println!("Risk level: {}", risk_level);

    let mut already_searched_cost = HashMap::new();
    let mut search_front = BinaryHeap::new();
    search_front.push(SearchState {
        cost: 0,
        estimated_rem: (target_coords.0 + target_coords.1) as u32,
        position: (0, 0),
        tool: 1,
    });

    let tools = vec![0u8, 1, 2]; // neither, torch, climbing
    let mut state = search_front.pop().expect("Just added one");
    while state.position != target_coords || state.tool != 1 {
        // println!(
        //     "Debug searching at {:?}, {}, {}",
        //     state.position, state.cost, state.tool
        // );

        if !already_searched_cost.contains_key(&(state.position, state.tool))
            || already_searched_cost[&(state.position, state.tool)] > state.cost
        {
            already_searched_cost
                .entry((state.position, state.tool))
                .and_modify(|c| *c = state.cost)
                .or_insert(state.cost);

            let mut neighbors = vec![
                (state.position.0 + 1, state.position.1),
                (state.position.0, state.position.1 + 1),
            ];
            if state.position.0 > 0 {
                neighbors.push((state.position.0 - 1, state.position.1))
            }
            if state.position.1 > 0 {
                neighbors.push((state.position.0, state.position.1 - 1));
            }

            let cur_type = get_cave_type(
                state.position.0,
                state.position.1,
                &input_params,
                &mut erosion_levels,
            );
            for neighbor in neighbors {
                let nb_type =
                    get_cave_type(neighbor.0, neighbor.1, &input_params, &mut erosion_levels);

                for tool in tools.iter().filter(|&&t| cur_type != t && nb_type != t) {
                    let cost_add = if *tool != state.tool { 8 } else { 1 };
                    let next_state = SearchState {
                        cost: state.cost + cost_add,
                        estimated_rem: estimate_cost(neighbor, *tool, target_coords),
                        position: neighbor,
                        tool: *tool,
                    };
                    search_front.push(next_state);
                }
            }
        }

        state = search_front.pop().expect("Must be a path to target");
    }

    println!("Search minutes: {}", state.cost);
}

fn get_cave_type(
    x: usize,
    y: usize,
    input_params: &(u32, Coords),
    erosion_levels: &mut HashMap<Coords, u32>,
) -> u8 {
    let erosion_level = get_erosion_level(x, y, input_params, erosion_levels);
    return (erosion_level % 3) as u8;
}

fn get_erosion_level(
    x: usize,
    y: usize,
    input_params: &(u32, Coords),
    erosion_levels: &mut HashMap<Coords, u32>,
) -> u32 {
    let depth = input_params.0;
    let target_coords = input_params.1;
    if erosion_levels.contains_key(&(x, y)) {
        return erosion_levels[&(x, y)];
    }

    let geo_val = if (x, y) == (0, 0) || (x, y) == target_coords {
        0
    } else if y == 0 {
        (x * Y0_ROOT) as u32
    } else if x == 0 {
        (y * X0_ROOT) as u32
    } else {
        get_erosion_level(x - 1, y, input_params, erosion_levels)
            * get_erosion_level(x, y - 1, input_params, erosion_levels)
    };

    let erosion_level = (geo_val + depth) % EROSION_MOD;
    erosion_levels.insert((x, y), erosion_level);

    return erosion_level;
}

fn estimate_cost(position: Coords, cur_tool: u8, target_pos: Coords) -> u32 {
    (target_pos.0 as i32 - position.0 as i32).abs() as u32
        + (target_pos.1 as i32 - position.1 as i32).abs() as u32
        + if cur_tool != 1 { 7 } else { 0 } as u32
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchState {
    cost: u32,
    estimated_rem: u32,
    position: Coords,
    tool: u8,
}

impl Ord for SearchState {
    fn cmp(&self, other: &SearchState) -> Ordering {
        // Flipping ordering to produce min-heap
        (other.cost + other.estimated_rem).cmp(&(self.cost + self.estimated_rem))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &SearchState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

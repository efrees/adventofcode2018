use adventlib::grid::*;
use std::collections::*;

pub fn solve() {
    println!("Day 15");

    let lines = adventlib::read_input_lines("day15input.txt");

    let grid = parse_grid(&lines);
    let mut units = parse_units(&lines);

    let mut battle_ongoing = true;
    let mut last_completed_round = 0;
    while battle_ongoing {
        // print_battlefield(&grid, &units);

        let mut turn_order: Vec<_> = units.keys().cloned().collect();
        turn_order.sort_by_key(|u| (u.y, u.x));
        for unit_pos in turn_order.iter() {
            if !units.contains_key(unit_pos) {
                continue; // unit was already killed
            }

            if !has_targets(unit_pos, &units) {
                battle_ongoing = false;
                break;
            }
            take_turn(unit_pos, &grid, &mut units);
        }

        if battle_ongoing {
            last_completed_round += 1;
        }
    }

    println!("Full round count: {}", last_completed_round);
    let remaining_hit_points: u32 = units.values().map(|u| u.hit_points as u32).sum();
    println!("Outcome: {}", remaining_hit_points * last_completed_round);
}

fn parse_grid(lines: &Vec<String>) -> Vec<Vec<CellType>> {
    let mut grid_rows = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        let row = line
            .as_bytes()
            .iter()
            .map(|&b| match b {
                b'#' => CellType::Wall,
                _ => CellType::Open,
            })
            .collect();

        grid_rows.push(row);
    }
    return grid_rows;
}

fn parse_units(lines: &Vec<String>) -> HashMap<Point, BattleUnit> {
    let mut units = HashMap::new();
    let mut y = 0;
    for line in lines.iter() {
        for x in 0..line.len() {
            let loc = Point::new(x as i64, y as i64);
            match line.as_bytes()[x] {
                b'E' => units.insert(loc, BattleUnit::new_elf(x as u32, y)),
                b'G' => units.insert(loc, BattleUnit::new_goblin(x as u32, y)),
                _ => None,
            };
        }
        y += 1;
    }
    return units;
}

fn has_targets(unit_pos: &Point, units: &HashMap<Point, BattleUnit>) -> bool {
    let own_team = &units[unit_pos].team;
    units.values().any(|u| u.team != *own_team)
}

fn has_adjacent_targets(unit_pos: &Point, units: &HashMap<Point, BattleUnit>) -> bool {
    let own_team = &units[unit_pos].team;
    let adjacent_positions = units[unit_pos].position.neighbors4();
    adjacent_positions
        .iter()
        .any(|p| units.contains_key(p) && units[p].team != *own_team)
}

fn take_turn(
    unit_pos: &Point,
    grid: &Vec<Vec<CellType>>,
    units: &mut HashMap<Point, BattleUnit>,
) -> bool {
    let mut landscape_changed = false;
    let mut current_pos = *unit_pos;
    if !has_adjacent_targets(&current_pos, units) {
        let new_pos = make_best_move(&current_pos, grid, units);
        if new_pos != current_pos {
            current_pos = new_pos;
            landscape_changed = true;
        }
    }

    if has_adjacent_targets(&current_pos, units) {
        landscape_changed = attack(&current_pos, units);
    }

    return landscape_changed;
}

fn make_best_move(
    unit_pos: &Point,
    grid: &Vec<Vec<CellType>>,
    units: &mut HashMap<Point, BattleUnit>,
) -> Point {
    let targets: HashSet<_> = units
        .values()
        .filter(|u| u.team != units[unit_pos].team)
        .flat_map(|u| u.position.neighbors4())
        .filter(|p| grid[p.y as usize][p.x as usize] == CellType::Open)
        .filter(|p| !units.contains_key(p))
        .collect();

    // Units don't move if they can't trace a path to some target.
    if targets.len() == 0 {
        return *unit_pos;
    }

    // find shortest paths, with preference to target first in reading order,
    // with preference to first move by reading order
    let mut first_moves = units[unit_pos].position.neighbors4();
    first_moves.sort_by_key(|p| (p.y, p.x));

    let mut shortest_dist: u32 = !0;
    let mut chosen_target_yx = (!0u32, !0u32);
    let mut first_move = Point::new(-1, -1);
    for mv in first_moves.iter() {
        let target_and_dist = bfs_best_target(mv, grid, units, &targets);
        let target_yx = (target_and_dist.0.y as u32, target_and_dist.0.x as u32);
        if target_and_dist.1 < shortest_dist
            || (target_and_dist.1 == shortest_dist && target_yx < chosen_target_yx)
        {
            shortest_dist = target_and_dist.1;
            chosen_target_yx = target_yx;
            first_move = *mv;
        }
    }

    // move
    if shortest_dist < !0 {
        let mut mover = units.remove(unit_pos).expect("Unit starts here");
        mover.position = first_move;
        units.insert(mover.position, mover);
        return first_move;
    } else {
        return *unit_pos;
    }
}

fn bfs_best_target(
    start_pos: &Point,
    grid: &Vec<Vec<CellType>>,
    units: &HashMap<Point, BattleUnit>,
    targets: &HashSet<Point>,
) -> (Point, u32) {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_pos.clone(), 0));

    let mut smallest_dist: u32 = !0;
    let mut closest_target = targets.iter().nth(0).expect("Any target").clone();

    while to_visit.len() > 0 {
        let next = to_visit.pop_front().expect("Pulling from queue");

        if next.1 > smallest_dist {
            break;
        }

        if targets.contains(&next.0)
            && (next.1 < smallest_dist
                || (next.0.y, next.0.x) < (closest_target.y, closest_target.x))
        {
            smallest_dist = next.1;
            closest_target = next.0;
        }

        if grid_get(&next.0, grid) == CellType::Open
            && !units.contains_key(&next.0)
            && !visited.contains(&next.0)
        {
            visited.insert(next.0);

            let mut neighbors = next.0.neighbors4();
            neighbors.sort_by_key(|p| (p.y, p.x));
            for n in neighbors.iter() {
                to_visit.push_back((*n, next.1 + 1));
            }
        }
    }

    return (closest_target, smallest_dist);
}

fn grid_get(point: &Point, grid: &Vec<Vec<CellType>>) -> CellType {
    if point.x < 0
        || point.y < 0
        || point.y as usize >= grid.len()
        || point.x as usize >= grid[point.y as usize].len()
    {
        return CellType::Wall;
    }
    return grid[point.y as usize][point.x as usize];
}

fn attack(unit_pos: &Point, units: &mut HashMap<Point, BattleUnit>) -> bool {
    let mut kill_made = false;

    let mut adjacent_positions = units[unit_pos].position.neighbors4();
    adjacent_positions.sort_unstable_by_key(|p| (p.y, p.x));

    let mut least_hit_points = !0u8;
    let mut attack_pos = &adjacent_positions[3];
    for pos in adjacent_positions.iter() {
        if units.contains_key(pos)
            && units[pos].team != units[unit_pos].team
            && units[pos].hit_points < least_hit_points
        {
            least_hit_points = units[pos].hit_points;
            attack_pos = pos;
        }
    }

    if units[attack_pos].hit_points <= 3 {
        units.remove(attack_pos);
        kill_made = true;
    } else {
        units.entry(*attack_pos).and_modify(|u| u.hit_points -= 3);
    }

    return kill_made;
}

#[allow(unused)]
fn print_battlefield(grid: &Vec<Vec<CellType>>, units: &HashMap<Point, BattleUnit>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let loc = Point::new(x as i64, y as i64);
            if units.contains_key(&loc) {
                match units[&loc].team {
                    BattleTeam::Goblin => print!("G"),
                    BattleTeam::Elf => print!("E"),
                }
            } else {
                match grid[y as usize][x as usize] {
                    CellType::Open => print!(" "),
                    CellType::Wall => print!("#"),
                }
            }
        }
        println!();
    }
}

#[derive(PartialEq, Copy, Clone)]
enum CellType {
    Open,
    Wall,
}

#[derive(PartialEq)]
enum BattleTeam {
    Goblin,
    Elf,
}

struct BattleUnit {
    team: BattleTeam,
    hit_points: u8,
    position: Point,
}

impl BattleUnit {
    fn new(team: BattleTeam, x: u32, y: u32) -> BattleUnit {
        BattleUnit {
            team: team,
            hit_points: 200,
            position: Point::new(x as i64, y as i64),
        }
    }

    fn new_goblin(x: u32, y: u32) -> BattleUnit {
        BattleUnit::new(BattleTeam::Goblin, x, y)
    }

    fn new_elf(x: u32, y: u32) -> BattleUnit {
        BattleUnit::new(BattleTeam::Elf, x, y)
    }
}

use adventlib::grid::Point;
use std::collections::HashMap;

pub fn solve() {
    println!("Day 20");

    let lines = adventlib::read_input_lines("day20input.txt");
    let mut node_store = Vec::new();
    node_store.push(MapNode::empty(None));

    let head_idx = parse_map(&lines[0].as_bytes(), 0, &mut node_store);
    let mut distances = HashMap::<Point, u32>::new();
    let mut memos = HashMap::<(Point, usize), u32>::new();

    let start_point = Point::new(0, 0);
    distances.insert(start_point, 0);
    follow_sequential_steps(
        start_point,
        0,
        head_idx,
        &node_store,
        &mut distances,
        &mut memos,
    );

    let most_doors_required = distances.values().max().expect("Must be a max");
    println!("Farthest room: {}", most_doors_required);

    let rooms_1000_away = distances.values().filter(|&&v| v >= 1000).count();
    println!("Rooms at least 1000 away: {}", rooms_1000_away)
}

fn parse_map<'a, 'b>(raw_map: &'a [u8], end: usize, node_store: &'b mut Vec<MapNode<'a>>) -> usize {
    let mut cursor = raw_map.len() - 1;
    parse_sequence(&mut cursor, raw_map, end, node_store)
}

fn parse_sequence<'a, 'b>(
    cur: &mut usize,
    raw_map: &'a [u8],
    mut head: usize,
    node_store: &'b mut Vec<MapNode<'a>>,
) -> usize {
    let mut new_head = MapNode::empty(Some(head));

    *cur -= 1; // we'll at least have a $ or ) for padding
    let mut end_index = *cur;
    loop {
        let mut cur_byte = raw_map[*cur];

        // Collect simple directions into a run
        while cur_byte == b'N' || cur_byte == b'E' || cur_byte == b'S' || cur_byte == b'W' {
            *cur -= 1;
            cur_byte = raw_map[*cur];
        }

        // store run as new step in sequence (can be empty)
        let run = &raw_map[*cur + 1..=end_index];
        if raw_map[end_index] == b')' && run.len() > 0 {
            println!("Run ends at {}", end_index);
        }
        new_head.run = run;
        head = node_store.len();
        node_store.push(new_head);

        new_head = MapNode::empty(Some(head));

        // branches feed into our current sequence
        if cur_byte == b')' {
            let branches = parse_branches(cur, raw_map, head, node_store);
            new_head.branches = branches;
            node_store.push(new_head);
            head = node_store.len() - 1;
            new_head = MapNode::empty(Some(head));
        }

        if cur_byte == b'(' || cur_byte == b'|' || cur_byte == b'^' {
            // These characters belong to a parent call
            break;
        }

        end_index = *cur;
    }
    return head;
}

fn parse_branches<'a, 'b>(
    cur: &mut usize,
    raw_map: &'a [u8],
    next: usize,
    node_store: &'b mut Vec<MapNode<'a>>,
) -> Vec<usize> {
    // this method should start with cur at ')'
    let mut branches = Vec::new();
    let mut cur_byte = raw_map[*cur];
    while cur_byte != b'(' {
        let new_node = parse_sequence(cur, raw_map, next, node_store);
        branches.push(new_node);

        cur_byte = raw_map[*cur];
    }

    // This method leaves cur pointing just left of the '('
    *cur -= 1;
    return branches;
}

fn follow_sequential_steps(
    start_point: Point,
    start_dist: u32,
    head: usize,
    node_store: &Vec<MapNode>,
    distances: &mut HashMap<Point, u32>,
    memos: &mut HashMap<(Point, usize), u32>,
) {
    if memos.contains_key(&(start_point, head)) && memos[&(start_point, head)] <= start_dist {
        return;
    }
    memos.insert((start_point, head), start_dist);

    let mut point = start_point;
    let mut dist = start_dist;
    let mut next = Some(head);
    while next != None && !node_store[next.unwrap()].has_branches() {
        let cur_node = &node_store[next.unwrap()];
        for b in cur_node.run {
            point = match b {
                b'N' => Point::new(point.x, point.y - 1),
                b'E' => Point::new(point.x + 1, point.y),
                b'S' => Point::new(point.x, point.y + 1),
                b'W' => Point::new(point.x - 1, point.y),
                _ => {
                    println!("Unexpected byte in run. {}", b);
                    point
                }
            };
            dist += 1;
            distances
                .entry(point)
                .and_modify(|d| {
                    if *d > dist {
                        *d = dist;
                    } else {
                        dist = *d;
                    }
                })
                .or_insert(dist);
        }

        next = cur_node.next;
    }

    if next != None {
        follow_branches(point, dist, next.unwrap(), node_store, distances, memos);
    }
}

fn follow_branches(
    start_point: Point,
    start_dist: u32,
    head: usize,
    node_store: &Vec<MapNode>,
    distances: &mut HashMap<Point, u32>,
    memos: &mut HashMap<(Point, usize), u32>,
) {
    for branch in node_store[head].branches.iter() {
        follow_sequential_steps(
            start_point,
            start_dist,
            *branch,
            node_store,
            distances,
            memos,
        );
    }
}

struct MapNode<'a> {
    next: Option<usize>,
    run: &'a [u8],
    branches: Vec<usize>,
}

impl<'a> MapNode<'a> {
    fn empty(next: Option<usize>) -> MapNode<'a> {
        MapNode {
            next: next,
            run: &[0; 0],
            branches: Vec::new(),
        }
    }

    fn has_branches(&self) -> bool {
        self.branches.len() > 0
    }
}

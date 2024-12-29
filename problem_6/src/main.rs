use std::collections::HashSet;
use std::env;
use std::fs;
use std::vec;

#[derive(Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
    is_blocked: bool,
    was_visited: bool,
}

impl Position {
    fn move_forward(&mut self) {
        self.y -= 1
    }
    fn move_back(&mut self) {
        self.y += 1
    }
    fn move_left(&mut self) {
        self.x -= 1
    }
    fn move_right(&mut self) {
        self.x += 1
    }
}

#[derive(Clone, Debug)]

struct Map {
    positions: Vec<Position>,
    upper_bound: usize,
    lower_bound: usize,
    left_bound: usize,
    right_bound: usize,
}

impl Map {
    fn at(&self, x: usize, y: usize) -> &Position {
        self.positions
            .iter()
            .find(|&item| item.x == x && item.y == y)
            .expect("not found")
    }
}

struct Guard<'a> {
    current_position: Position,
    heading: String,
    map: &'a Map,
    visited_locations: HashSet<(usize, usize)>,
}

impl<'a> Guard<'a> {
    fn next(&mut self) {
        if self.heading == "north" {
            if self
                .map
                .at(self.current_position.x, self.current_position.y - 1)
                .is_blocked
            {
                self.heading = String::from("west");
            } else {
                self.current_position.move_forward()
            }
        } else if self.heading == "west" {
            if self
                .map
                .at(self.current_position.x + 1, self.current_position.y)
                .is_blocked
            {
                self.heading = String::from("south");
            } else {
                self.current_position.move_right()
            }
        } else if self.heading == "south" {
            if self
                .map
                .at(self.current_position.x, self.current_position.y + 1)
                .is_blocked
            {
                self.heading = String::from("east");
            } else {
                self.current_position.move_back()
            }
        } else {
            if self
                .map
                .at(self.current_position.x - 1, self.current_position.y)
                .is_blocked
            {
                self.heading = String::from("north");
            } else {
                self.current_position.move_left();
            }
        }
    }

    fn is_on_map_limit(&self) -> bool {
        (self.heading == "north" && self.current_position.y == self.map.upper_bound)
            || (self.heading == "south" && self.current_position.y == self.map.lower_bound)
            || (self.heading == "east" && self.current_position.x == self.map.left_bound)
            || (self.heading == "west" && self.current_position.x == self.map.right_bound)
    }
}

fn parse_input(file_path: &String) -> Map {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut positions = vec![];
    let mut max_row = 0;
    let mut max_col = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, value) in line.chars().enumerate() {
            let x = j;
            let y = i;
            let is_blocked = value == '#';
            let was_visited = matches!(value, '>' | '<' | 'v' | '^');
            let position = Position {
                x,
                y,
                is_blocked,
                was_visited,
            };
            positions.push(position);
            max_col = j;
            max_row = i;
        }
    }
    Map {
        positions,
        upper_bound: 0,
        lower_bound: max_row,
        left_bound: 0,
        right_bound: max_col,
    }
}

fn count_loop_obstructions(map: &mut Map, guard: &mut Guard) -> usize {
    let mut valid_positions = 0;

    // Iterate over all positions to find open spots
    for pos in &mut map.positions {
        // Skip if the position is already blocked or is the starting position
        if pos.is_blocked || pos.was_visited {
            continue;
        }

        // Temporarily place an obstruction
        pos.is_blocked = true;

        // Reset guard's state for simulation
        let mut test_guard = Guard {
            current_position: pos.clone(), // Clone to avoid mutable reference issues
            heading: guard.heading.clone(),
            map: &mut map.clone(),
            visited_locations: HashSet::new(),
        };

        let mut visited_states = HashSet::new();
        let mut loop_detected = false;

        loop {
            let state = (
                test_guard.current_position.x,
                test_guard.current_position.y,
                test_guard.heading.clone(),
            );
            if !visited_states.insert(state) {
                loop_detected = true;
                break;
            }
            test_guard.next();
            if test_guard.is_on_map_limit() {
                break;
            }
        }

        if loop_detected {
            valid_positions += 1;
        }

        // Remove the temporary obstruction
        pos.is_blocked = false;
    }

    valid_positions
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path: &String = &args[1];

    let map = parse_input(file_path);
    let mut guard = Guard {
        current_position: map
            .positions
            .iter()
            .find(|pos| pos.was_visited)
            .cloned()
            .expect("No visited position found"),
        heading: String::from("north"),
        map: &map,
        visited_locations: HashSet::new(),
    };

    while !guard.is_on_map_limit() {
        guard
            .visited_locations
            .insert((guard.current_position.x, guard.current_position.y));
        guard.next();
    }

    println!("part 1:{}", guard.visited_locations.len() + 1);
}

use itertools::Itertools;
use priority_queue::DoublePriorityQueue;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let initial_state_representation = read_to_string("input.txt").unwrap();
    let initial_state = initial_state(&initial_state_representation);

    let mut final_cost = 0;
    let mut visited: HashSet<State> = HashSet::new();
    let mut priority_queue: DoublePriorityQueue<State, u16> = DoublePriorityQueue::new();
    priority_queue.push(initial_state, 0);
    while let Some((current_state, cost)) = priority_queue.pop_min() {
        if visited.contains(&current_state) {
            continue;
        }

        if end_state(&current_state) {
            final_cost = cost;
            break;
        }

        for (next_state, transition_cost) in possible_states(&current_state) {
            if !visited.contains(&next_state) {
                priority_queue.push_decrease(next_state, cost + transition_cost);
            }
        }
        visited.insert(current_state.clone());
    }

    println!("{}", final_cost);
}

#[derive(Hash, Clone)]
struct State {
    hallway: Vec<char>,
    rooms: Vec<Vec<char>>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.hallway == other.hallway && self.rooms == other.rooms
    }
}

impl Eq for State {}

fn initial_state(representation: &str) -> State {
    let room = |x| {
        representation
            .split_terminator('\n')
            .skip(2)
            .take(4)
            .map(|line| line.chars().nth(x).unwrap())
            .collect::<Vec<char>>()
    };
    let rooms: Vec<Vec<char>> = (3..10).step_by(2).map(|column| room(column)).collect();

    State {
        hallway: vec!['.'; 11],
        rooms,
    }
}

fn end_state(state: &State) -> bool {
    state.hallway.iter().all(|x| *x == '.')
        && state.rooms[0] == ['A'; 4]
        && state.rooms[1] == ['B'; 4]
        && state.rooms[2] == ['C'; 4]
        && state.rooms[3] == ['D'; 4]
}

fn possible_states(state: &State) -> Vec<(State, u16)> {
    let hallway_tiles = state.hallway.iter().enumerate();
    let amphipods = hallway_tiles.filter(|(_, value)| is_amphipod(value));
    let hallway_to_room_states = amphipods
        .map(|(tile, amphipod)| (tile, amphipod))
        .filter(|(tile, amphipod)| amphipod_can_go_to_room(state, *tile, amphipod))
        .map(|(tile, amphipod)| move_to_room(state, amphipod, tile))
        .collect();

    let rooms = state.rooms.iter().enumerate();
    let movable_rooms_amphipods = rooms
        .filter(|(_, room)| !empty_room(room))
        .filter(|(i, room)| !room_contains_expected_amphipods(room, &room_to_amphipod(i)))
        .map(|(i, room)| (i, first_occupied_position_in_room(room)));
    let valid_hallway_positions = [0, 1, 3, 5, 7, 9, 10].iter();
    let room_to_hallway_states: Vec<(State, u16)> = movable_rooms_amphipods
        .cartesian_product(valid_hallway_positions)
        .filter(|&((end, _), &start)| can_go_to_hallway(state, start, end))
        .map(|((room_index, depth), &start)| move_to_hallway(state, start, room_index, depth))
        .collect();

    [hallway_to_room_states, room_to_hallway_states].concat()
}

fn is_amphipod(tile: &char) -> bool {
    *tile != '.'
}

fn amphipod_can_go_to_room(state: &State, position: usize, amphipod: &char) -> bool {
    let room_index = amphipod_to_room(amphipod);
    let room_pos = 2 + 2 * room_index;
    let room = &state.rooms[room_index];

    room[0] == '.'
        && (room[3] == *amphipod || room[3] == '.')
        && ((room_pos < position && free_hallway(&state.hallway, room_pos, position))
            || (room_pos > position && free_hallway(&state.hallway, position + 1, room_pos + 1)))
}

fn free_hallway(hallway: &[char], start: usize, end: usize) -> bool {
    hallway[start..end].iter().all(|c| *c == '.')
}

fn move_to_room(state: &State, amphipod: &char, start: usize) -> (State, u16) {
    let room_index = amphipod_to_room(amphipod);
    let room = &state.rooms[room_index];
    let end = 2 + 2 * room_index;

    let first_empty_position = (0usize..4).filter(|&i| !is_amphipod(&room[i])).last();
    let depth = first_empty_position.unwrap();
    let mut next_state = state.clone();
    next_state.rooms[room_index][depth] = *amphipod;
    next_state.hallway[start] = '.';

    (
        next_state,
        amphipod_to_cost(amphipod) * manhattan_distance(start, end, depth),
    )
}

fn manhattan_distance(x1: usize, x2: usize, depth: usize) -> u16 {
    ((x2 as i8 - x1 as i8).abs() + depth as i8 + 1) as u16
}

fn empty_room(room: &[char]) -> bool {
    room[0] == room[1] && room[1] == room[2] && room[2] == room[3] && room[3] == '.'
}

fn first_occupied_position_in_room(room: &[char]) -> usize {
    let amphipods = room.iter().enumerate().filter(|(_, c)| is_amphipod(c));

    amphipods.map(|(i, _)| i).nth(0).unwrap()
}

fn room_contains_expected_amphipods(room: &[char], expected_amphipod: &char) -> bool {
    let expected_or_empty = |x: usize| (room[x] == *expected_amphipod) || (room[x] == '.');

    expected_or_empty(0) && expected_or_empty(1) && expected_or_empty(2) && expected_or_empty(3)
}

fn can_go_to_hallway(state: &State, start: usize, end: usize) -> bool {
    let room_pos = 2 + 2 * end;
    (start < room_pos && free_hallway(&state.hallway, start, room_pos + 1))
        || (start > room_pos && free_hallway(&state.hallway, room_pos, start + 1))
}

fn move_to_hallway(state: &State, start: usize, room_index: usize, depth: usize) -> (State, u16) {
    let mut possible_state = state.clone();
    possible_state.hallway[start] = state.rooms[room_index][depth];
    possible_state.rooms[room_index][depth] = '.';

    let end = 2 + (2 * room_index);
    (
        possible_state,
        amphipod_to_cost(&state.rooms[room_index][depth]) * manhattan_distance(start, end, depth),
    )
}

fn amphipod_to_room(amphipod: &char) -> usize {
    (*amphipod as u8 - 'A' as u8) as usize
}

fn amphipod_to_cost(amphipod: &char) -> u16 {
    (10u16).pow(*amphipod as u32 - 'A' as u32) as u16
}

fn room_to_amphipod(room: &usize) -> char {
    ('A' as u8 + *room as u8) as char
}

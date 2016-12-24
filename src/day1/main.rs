use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct State {
    coordinates: Point,
    direction: Direction
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Turn {
    Left,
    Right
}

#[derive(PartialEq, Eq, Debug)]
pub struct Move {
    turn: Turn,
    blocks: i32
}

pub fn right(direction:&Direction) -> Direction {
    match *direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North
    }
}

pub fn left(direction:&Direction) -> Direction {
    match *direction {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North
    }
}

pub fn make_turn(direction:&Direction, turn:&Turn) -> Direction {
    match *turn {
        Turn::Left => left(direction),
        Turn::Right => right(direction)
    }
}

pub fn move_forward(coordinates:&Point, direction:&Direction, distance:&i32) -> Point {
    let new_x = coordinates.x + match *direction {
        Direction::East => *distance,
        Direction::West => *distance * -1,
        _ => 0
    };

    let new_y = coordinates.y + match *direction {
        Direction::North => *distance,
        Direction::South => *distance * -1,
        _ => 0
    };

    return Point { x: new_x, y: new_y };
}

pub fn make_move(state:&State, mov:&Move) -> State {
    let new_direction = make_turn(&state.direction, &mov.turn);
    let new_coordintes = move_forward(&state.coordinates, &new_direction, &mov.blocks);

    return State {
        coordinates: new_coordintes,
        direction: new_direction
    };
}

pub fn make_moves(initial_state:&State, movs:&Vec<Move>) -> State {
    return movs.iter().fold(*initial_state, |state, mov| { make_move(&state, &mov) });
}

pub fn manhattan_distance(p1:&Point, p2:&Point) -> i32 {
    return (p2.x - p1.x).abs() + (p2.y - p1.y).abs();
}

pub fn parse_move(s:&String) -> Result<Move, String> {
    let mut chars = s.chars();

    let turn = match chars.next() {
        Some('L') => Turn::Left,
        Some('R') => Turn::Right,
        _ => return Err("Couldn't parse turn from: ".to_string() + s)
    };

    let blocks = match chars.collect::<String>().parse::<i32>() {
        Ok(val) => val,
        Err(s) => return Err(s.to_string())
    };


    return Ok(Move { turn: turn, blocks: blocks });
}

pub fn parse_moves(s:&String) -> Result<Vec<Move>, String> {
    let tokens = s.trim().split(", ");
    let mut moves = vec![];
    for token in tokens {
        match parse_move(&token.to_string()) {
            Ok(mov) => moves.push(mov),
            Err(e) => return Err(e)
        }
    }
    return Ok(moves);
}

pub fn calculate_distance(movs:&Vec<Move>) -> i32 {
    let start = State {
        coordinates: Point { x: 0, y: 0 },
        direction: Direction::North
    };

    let end = make_moves(&start, movs);

    return manhattan_distance(&start.coordinates, &end.coordinates);
}

pub fn calculate_distance2(movs:&Vec<Move>) -> Option<i32> {
    let mut state = State {
        coordinates: Point { x: 0, y: 0 },
        direction: Direction::North
    };

    let mut visited_coordinates = HashSet::new();
    let start = state.coordinates;
    visited_coordinates.insert(start);

    for mov in movs {
        let new_direction = make_turn(&state.direction, &mov.turn);
        for _ in 0..mov.blocks {
            let new_coordintes = move_forward(&state.coordinates, &new_direction, &1);

            if visited_coordinates.contains(&new_coordintes) {
                let distance = manhattan_distance(&start, &new_coordintes);
                return Some(distance);
            }
            visited_coordinates.insert(new_coordintes);

            state = State { direction: new_direction, coordinates: new_coordintes };
        }
    }

    return None;
}

fn main() {
    let file = env::args().nth(1).expect("Specify an input file");
    let mut contents = String::new();
    let mut f = File::open(file).expect("Unable to open file");
    f.read_to_string(&mut contents).expect("Unable to read contents");
    let moves = parse_moves(&contents).expect("Unable to parse moves");

    let distance = calculate_distance2(&moves).expect("No point visited twice");
    println!("Distance: {}", distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn making_a_move() {
        let original_state = State {
            coordinates: Point { x: 0, y: 0 },
            direction: Direction::North
        };

        let new_state = make_move(&original_state, &Move {
            turn: Turn::Right,
            blocks: 10
        });

        assert_eq!(&new_state, &State {
            coordinates: Point { x: 10, y: 0 },
            direction: Direction::East
        });
    }

    #[test]
    fn making_multiple_moves() {
        let original_state = State {
            coordinates: Point { x: 0, y: 0 },
            direction: Direction::North
        };

        let new_state = make_moves(&original_state, &vec![
                                  Move {
                                      turn: Turn::Right,
                                      blocks: 10
                                  },
                                  Move {
                                      turn: Turn::Right,
                                      blocks: 10
                                  }
        ]);

        assert_eq!(&new_state, &State {
            coordinates: Point { x: 10, y: -10 },
            direction: Direction::South
        });
    }

    #[test]
    fn parsing_moves() {
        assert_eq!(
            parse_moves(&"R2, L3".to_string()).unwrap(),
            vec![
                Move { turn: Turn::Right, blocks: 2 },
                Move { turn: Turn::Left, blocks: 3}
            ]);
    }

    #[test]
    fn test_examples() {
        assert_eq!(
            calculate_distance(&parse_moves(&"R2, L3".to_string()).unwrap()),
            5);

        assert_eq!(
            calculate_distance(&parse_moves(&"R2, R2, R2".to_string()).unwrap()),
            2);

        assert_eq!(
            calculate_distance(&parse_moves(&"R5, L5, R5, R3".to_string()).unwrap()),
            12);
    }

    #[test]
    fn test_part2_examples2() {
        assert_eq!(
            calculate_distance2(&parse_moves(&"R8, R4, R4, R8".to_string()).unwrap()).unwrap(),
            4);
    }
}

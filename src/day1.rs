#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct State {
    coordinates: Point,
    direction: Direction
}

pub enum Turn {
    Left,
    Right
}

pub struct Move {
    turn: Turn,
    blocks: i32
}

pub fn right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North
    }
}

pub fn left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::North
    }
}

pub fn make_turn(direction:Direction, turn: Turn) -> Direction {
    match turn {
        Turn::Left => left(direction),
        Turn::Right => right(direction)
    }
}

pub fn move_forward(coordinates:Point, direction:Direction, distance:i32) -> Point {
    let new_x = coordinates.x + match direction {
        Direction::East => distance,
        Direction::West => -distance,
        _ => 0
    };

    let new_y = coordinates.y + match direction {
        Direction::North => distance,
        Direction::South => -distance,
        _ => 0
    };

    return Point { x: new_x, y: new_y };
}

pub fn make_move(state: State, mov: Move) -> State {
    let new_direction = make_turn(state.direction, mov.turn);
    let new_coordintes = move_forward(state.coordinates, new_direction, mov.blocks);

    return State {
        coordinates: new_coordintes,
        direction: new_direction
    };
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

        let new_state = make_move(original_state, Move {
            turn: Turn::Right,
            blocks: 10
        });

        assert_eq!(new_state, State {
            coordinates: Point { x: 10, y: 0 },
            direction: Direction::East
        });
    }
}

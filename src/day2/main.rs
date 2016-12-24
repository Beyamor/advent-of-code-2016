use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::cmp;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    U,
    D,
    L,
    R
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err(())
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32
}

pub fn clamp<T>(v:T, min:T, max:T) -> T where T: Ord {
    return cmp::min(cmp::max(v, min), max);
}

static KEYS:&'static[[i32; 3]; 3] = &[
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

impl Position {
    fn make_move(&self, direction:&Direction) -> Position {
        Position {
            x: clamp(self.x + match *direction {
                Direction::L => -1,
                Direction::R => 1,
                _ => 0
            }, 0, 2),

            y: clamp(self.y + match *direction {
                Direction::U => -1,
                Direction::D => 1,
                _ => 0
            }, 0, 2)
        }
    }

    fn make_moves(&self, dirs:&Vec<Direction>) -> Position {
        return dirs.iter().fold(*self, |pos, dir| { pos.make_move(dir) });
    }

    fn get_key(&self) -> i32 {
        KEYS[self.y as usize][self.x as usize]
    }

    fn from_key(key:i32) -> Position {
        for y in 0..3 {
            for x in 0..3 {
                if KEYS[y as usize][x as usize] == key {
                    return Position { x: x, y: y};
                }
            }
        }
            panic!("Unhandled key");
    }
}

pub fn parse_moves(s:&String) -> Vec<Vec<Direction>> {
    s.lines()
        .map(|s| {
            s.trim()
                .chars()
                .map(|c:char| c.to_string().parse::<Direction>().unwrap())
                .collect()
        })
        .collect()
}

pub fn get_keys(start:&Position, moves:&Vec<Vec<Direction>>) -> Vec<i32> {
    let mut position = *start;
    let mut keys = vec![];
    for set_of_moves in moves {
        position = position.make_moves(&set_of_moves);
        keys.push(position.get_key())
    }
    return keys;
}

fn main() {
    let file = env::args().nth(1).expect("Specify an input file");
    let mut contents = String::new();
    let mut f = File::open(file).expect("Unable to open file");
    f.read_to_string(&mut contents).expect("Unable to read contents");
    let moves = parse_moves(&contents);

    let keys = get_keys(&Position::from_key(5), &moves);
    print!("Keys: ");
    for key in keys {
        print!("{}", key);
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_directions() {
        assert_eq!(
            Ok(Direction::U),
            "U".to_string().parse::<Direction>());
        assert_eq!(
            Ok(Direction::D),
            "D".to_string().parse::<Direction>());
        assert_eq!(
            Ok(Direction::L),
            "L".to_string().parse::<Direction>());
        assert_eq!(
            Ok(Direction::R),
            "R".to_string().parse::<Direction>());
    }

    #[test]
    fn test_getting_keys() {
        let get_key = |x:i32, y:i32| Position{ x: x, y: y}.get_key();

        assert_eq!(1, get_key(0, 0));
        assert_eq!(2, get_key(1, 0));
        assert_eq!(3, get_key(2, 0));
        assert_eq!(4, get_key(0, 1));
        assert_eq!(5, get_key(1, 1));
        assert_eq!(6, get_key(2, 1));
        assert_eq!(7, get_key(0, 2));
        assert_eq!(8, get_key(1, 2));
        assert_eq!(9, get_key(2, 2));
    }

    #[test]
    fn test_moving_position() {
        let pos = Position { x: 1, y: 1 };
        let moved_pos = pos.make_move(&Direction::U);
        assert_eq!(
            moved_pos.get_key(),
            2);
    }

    #[test]
    fn test_clamping() {
        assert_eq!(1, clamp(1, 0, 2));
        assert_eq!(0, clamp(-1, 0, 2));
        assert_eq!(2, clamp(3, 0, 2));
    }

    #[test]
    fn test_examples() {
        assert_eq!(
            1,
            Position::from_key(5)
            .make_moves(&vec![Direction::U, Direction::L, Direction::L])
            .get_key());
        assert_eq!(
            9,
            Position::from_key(1)
            .make_moves(&vec![Direction::R, Direction::R, Direction::D, Direction::D, Direction::D])
            .get_key());
        assert_eq!(
            8,
            Position::from_key(9)
            .make_moves(&vec![Direction::L, Direction::U, Direction::R, Direction::D, Direction::L])
            .get_key());
        assert_eq!(
            5,
            Position::from_key(8)
            .make_moves(&vec![Direction::U, Direction::U, Direction::U, Direction::U, Direction::D])
            .get_key());
    }

    #[test]
    fn test_parsing_moves() {
        assert_eq!(
            parse_moves(&"ULL  \nRRDDD\nLURDL\nUUUUD".to_string()),
            vec![vec![Direction::U, Direction::L, Direction::L],
                 vec![Direction::R, Direction::R, Direction::D, Direction::D, Direction::D],
                 vec![Direction::L, Direction::U, Direction::R, Direction::D, Direction::L],
                 vec![Direction::U, Direction::U, Direction::U, Direction::U, Direction::D]]);
    }
}

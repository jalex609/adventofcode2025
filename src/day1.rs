#[derive(PartialEq, Debug)]
enum TurnDirection {
    Left,
    Right,
}

struct WrapperLock {
    current_number: i16,
    max_number: i16,
}

trait TurnLock {
    fn turn_lock(&mut self, turn_direction: TurnDirection, amt_turn: i16) -> i16;
}

impl WrapperLock {
    fn check_zeros_from_turning(&mut self, input_lines: std::str::Lines) -> (i16, i16) {
        return input_lines.fold((0, 0), |mut acc, line| {
            let first_char = line.chars().nth(0).unwrap(); // assume we have this
            let turn_dir = if first_char == 'L' {
                TurnDirection::Left
            } else {
                TurnDirection::Right
            };
            let amt_trn = &line[1..].parse::<i16>().unwrap();
            let turn_zero = self.turn_lock(turn_dir, *amt_trn);
            acc.1 += turn_zero;
            if self.current_number == 0 {
                acc.0 += 1;
            }
            return acc;
        });
    }
}

impl TurnLock for WrapperLock {
    fn turn_lock(&mut self, turn_direction: TurnDirection, amt_turn: i16) -> i16 {
        let delta = match turn_direction {
            TurnDirection::Left => -amt_turn,
            TurnDirection::Right => amt_turn,
        };

        let old = self.current_number;
        let raw = old + delta;
        let max = self.max_number;

        let crossings = if turn_direction == TurnDirection::Left {
            // We wrap around to make left turns not an issue
            (old - 1).div_euclid(self.max_number) - (raw - 1).div_euclid(self.max_number)
        } else {
            raw.div_euclid(max) - old.div_euclid(max)
        };

        self.current_number = raw.rem_euclid(max);
        crossings
    }
}

fn main() {
    let contents = include_str!("../inputs/day1.txt").lines();
    let mut lock = WrapperLock {
        current_number: 50,
        max_number: 100,
    };
    let num = lock.check_zeros_from_turning(contents);
    println!("Part I num: {}", num.0);
    println!("Part II num: {}", num.1);
}

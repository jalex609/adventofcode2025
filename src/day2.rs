struct IDRange {
    min_number: u64,
    max_number: u64,
}

fn parse_input(input_str: &str) -> Vec<IDRange> {
    return input_str
        .split(",")
        .map(|range_str| {
            // yay some cool stuff to make it pattern matchable as a slice
            match range_str.split('-').collect::<Vec<_>>()[..] {
                [a, b] => IDRange {
                    min_number: a.parse::<u64>().unwrap(),
                    max_number: b.parse::<u64>().unwrap(),
                },
                _ => panic!("This should be impossible"),
            }
        })
        .collect();
}

fn count_invalid_ids(range: &IDRange, part_i: bool) -> usize {
    let num_digits_max = range.max_number.checked_ilog10().unwrap_or_default() + 1;
    let mut final_num = 0;
    for i in (range.min_number)..=(range.max_number) {
        let num_str = i.to_string();
        if part_i {
            let halfway = (num_digits_max / 2) as usize;
            if &num_str[..halfway] == &num_str[halfway..] {
                final_num += i;
            }
        } else {
            let i_num_digits = (i.checked_ilog10().unwrap_or_default() + 1) as usize;
            for j in 2..=i_num_digits {
                if (i_num_digits as usize) % j == 0 {
                    // split into j equal parts
                    let parts: Vec<&str> = (0..j)
                        .map(|index| {
                            &num_str[index * (num_str.len() / j)..(index + 1) * (num_str.len() / j)]
                        })
                        .collect();
                    if parts.iter().all(|part| *part == parts[0]) {
                        final_num += i;
                        // no repeats. Once we found the largest max, we dont want to double count on even matches.
                        break; 
                    }
                }
            }
        }
    }

    return final_num as usize;
}

fn main() {
    let contents = include_str!("../inputs/day2.txt");
    let inputs = parse_input(contents);
    let final_num_part_i = inputs
        .iter()
        .fold(0, |acc, range| acc + count_invalid_ids(range, true));
    let final_num_part_ii = inputs
        .iter()
        .fold(0, |acc, range| acc + count_invalid_ids(range, false));
    println!("Final num part I : {}", final_num_part_i);
    println!("Final num part II : {}", final_num_part_ii);
}

#[test]
fn adds_right() {
    let count = count_invalid_ids(&IDRange { min_number: 85, max_number: 113 }, false);
    assert_eq!(count, 298);
}
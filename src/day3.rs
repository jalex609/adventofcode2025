
fn day3<I>(input_lines: I, is_part_i: bool) -> u64
where
    I: Iterator<Item = &'static str>,
{
    return input_lines.fold(0, |mut acc, line| {
        let num_array: Vec<u32> = line
            .chars()
            .map(|x| x.to_digit(10).expect("Is a digit"))
            .collect();
        if is_part_i {
            for i in (1..=9).rev() {
                // leftmost position of highest digit possible
                let first_position = num_array.iter().position(|x| *x == i);
                if first_position != None {
                    if first_position.unwrap() != num_array.len() - 1 {
                        for j in (1..=9).rev() {
                            let sliced_arr = &num_array[first_position.unwrap() + 1..];
                            let both_digits = sliced_arr.iter().position(|x| *x == j);
                            if both_digits != None {
                                acc += (i as u64 * 10) + j as u64;
                                return acc;
                            }
                        }
                    }
                }
            }
        } else {
            let n = find_n_digit_combination(&num_array, 12, vec![]);
            println!("{}", n);
            return acc + n;
        }
        return acc;
    });
}

fn find_n_digit_combination(
    num_array: &[u32],
    digits_needed: usize,
    digits_list: Vec<u32>,
) -> u64 {
    let mut max = 0;
    while digits_needed > 0 {
        for i in (1..=9).rev() {
            // find all possible digit position for the highest digit
            let current_posns: Vec<usize> = num_array
                .iter()
                .enumerate() // Get the index
                .filter(|(index, &num)| num == i && index + digits_needed <= num_array.len()) // Check if digit exists
                .map(|(index, _)| index) // Return the index + 1 for the slice
                .collect(); // Collect into a Vector

            let mut new_digits_list = digits_list.clone();
            if current_posns.len() > 0 {
                new_digits_list.push(i);
            } else {
                continue;
            }
            for posn in current_posns {
                let result = find_n_digit_combination(
                    &num_array[posn + 1..],
                    digits_needed - 1,
                    new_digits_list.clone(),
                );
                max = max.max(result);
            } 
            return max;
        }
    }
        return digits_list
        .iter()
        .map(|d| char::from_digit(*d, 10).expect("invalid digit"))
        .collect::<String>()
        .parse::<u64>()
        .expect("parse failed");

}

fn main() {
    let contents = include_str!("../inputs/day3.txt").lines();
    let contents2 = include_str!("../inputs/day3.txt").lines();
    let part_i = day3(contents, true);
    let part_ii = day3(contents2, false);
    println!("Part I answer: {}", part_i);
    println!("Part II answer: {}", part_ii);
}

#[test]
fn day3_example() {
    let contents = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n".lines();
    assert_eq!(day3(contents.clone(), true), 357);
    assert_eq!(day3(contents, false), 3121910778619);
}

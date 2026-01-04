use std::iter::repeat_with;

use crate::grid::{Grid, Point};

mod grid;

fn eq_char(c: char) -> impl Fn(&Grid<char>, &Point) -> bool {
    return move |g, p| {
        return if let Some(actual) = g.at(p) {
            *actual == c
        } else {
            false
        };
    };
}

fn neigh_less_than_n(g: &Grid<char>, n : usize) -> impl Fn(&[Option<Point>]) -> bool + '_ {
    return move | x: &[Option<Point>] | {
        let mut total = 0;
        for point in x {
            if let Some(p) = point {
                if let Some(t) = g.at(p) {
                    if *t == '@' {
                        total += 1;
                    }
                }
            }
        }
        return total < n;
    };
}

fn main() {
    let input = include_str!("../inputs/day4.txt");
    let mut g: Grid<char> = input.parse().unwrap();
    let part_i = part_i(&g);
    println!("Count: {}", part_i);
    let part_ii = part_ii(&mut g);
    println!("Rolls removed: {}", part_ii);
}

fn part_i(g: &Grid<char>) -> usize {
    return g.search(eq_char('@'), neigh_less_than_n(g,4)).count();
}

fn remove_rolls(g: &mut Grid<char>) -> usize {
    let points : Vec<Point> = g.search(eq_char('@'), neigh_less_than_n(g,4)).collect();
    let mut total = 0;
    for p in points {
        g.write_point(&p, '.');
        total += 1;
    }
    return total;
}

fn part_ii(g: &mut Grid<char>) -> usize {
    return repeat_with(|| remove_rolls(g))
        .take_while(|&n| n > 0)
        .sum();
}
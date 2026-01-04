use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point(isize, isize);

pub const DOWN_RIGHT: Direction = Direction(1, 1);
pub const UP_LEFT: Direction = Direction(-1, -1);
pub const DOWN_LEFT: Direction = Direction(-1, 1);
pub const UP_RIGHT: Direction = Direction(1, -1);
pub const UP: Direction = Direction(0, -1);
pub const DOWN: Direction = Direction(0, 1);
pub const RIGHT: Direction = Direction(1, 0);
pub const LEFT: Direction = Direction(-1, 0);

#[derive(Eq, PartialEq)]
/// Construction intentionally left private, we only expose a set directions (each 8th)
pub struct Direction(isize, isize);

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl Point {
    pub fn add(&self, d: &Direction) -> Point {
        return Point(self.0 + d.0, self.1 + d.1);
    }
}

pub struct GridSearch<'a, T, P, N> {
    g: &'a Grid<T>,
    pred: P,
    neigh_pred: N,
    x: usize,
    y: usize,
}

impl<'a, T, P, N> Iterator for GridSearch<'a, T, P, N>
where
    T: Eq,
    P: Fn(&'a Grid<T>, &Point) -> bool,
    N: Fn(&[Option<Point>]) -> bool,
{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut last_x = self.x;
        for sy in self.y..self.g.data.len() {
            for sx in last_x..self.g.data[sy].len() {
                if (self.pred)(&self.g, &Point(sx as isize, sy as isize)) {
                    let neighbors = self.g.neighbors(&Point(sx as isize, sy as isize));
                    if (self.neigh_pred)(&neighbors) {
                        if sx + 1 == self.g.data[sy].len() {
                            self.x = 0;
                            self.y = sy + 1;
                        } else {
                            self.x = sx + 1;
                            self.y = sy;
                        }
                        return Some(Point(sx as isize, sy as isize));
                    }
                }
            }
            last_x = 0;
        }

        return None;
    }
}

impl<T> Grid<T>
where
    T: Eq,
{
    pub fn search<'b, P, N>(&'b self, pred: P, neigh_pred: N) -> impl Iterator<Item = Point> + 'b
    where
        P: Fn(&'b Grid<T>, &'_ Point) -> bool + 'b,
        N: Fn(&[Option<Point>]) -> bool + 'b,
    {
        return GridSearch {
            g: self,
            x: 0,
            y: 0,
            pred,
            neigh_pred
        };
    }

    pub fn write_point(&mut self, p: &Point, value: T) {
        if self.in_bounds(&p) {
            self.data[p.1 as usize][p.0 as usize] = value;
        }
    }

    pub fn neighbors(&self, p: &Point) -> Vec<Option<Point>> {
        let directions = [
            &UP,
            &UP_RIGHT,
            &RIGHT,
            &DOWN_RIGHT,
            &DOWN,
            &DOWN_LEFT,
            &LEFT,
            &UP_LEFT,
        ];

        return directions
            .iter()
            .map(|dir| {
                let neighbor_point = p.add(dir);
                if self.in_bounds(&neighbor_point) {
                    Some(neighbor_point)
                } else {
                    None
                }
            })
            .collect();
    }
}

impl<T> Grid<T> {
    pub fn at<'a>(&'a self, p: &Point) -> Option<&'a T> {
        if p.1 < 0 || p.1 as usize >= self.data.len() {
            return None;
        }

        if p.0 < 0 || p.0 as usize >= self.data[p.1 as usize].len() {
            return None;
        }

        return Some(&self.data[p.1 as usize][p.0 as usize]);
    }

    pub fn in_bounds(&self, p: &Point) -> bool {
        return self.at(p).is_some();
    }
}

impl<T> FromStr for Grid<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        let mut scratch = [0u8; 4];
        for line in s.lines() {
            let mut cols = Vec::new();
            for c in line.chars() {
                cols.push(c.encode_utf8(&mut scratch).parse()?);
            }
            rows.push(cols);
        }
        return Ok(Grid { data: rows });
    }
}

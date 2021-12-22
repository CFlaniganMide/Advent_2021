use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub struct Point2d {
    x: u64,
    y: u64,
}

pub struct Line2d {
    a: Point2d,
    b: Point2d,
}

pub struct Plane2d {
    width: usize,
    height: usize,
    data: Vec<i64>,
}

impl Point2d {
    pub fn new() -> Point2d {
        return Point2d {
            x: 0,
            y: 0,
        }
    }

    pub fn from_vals(x: u64, y: u64) -> Point2d {
        return Point2d {
            x,
            y,
        }
    }

    pub fn get_x(&self) -> u64 {
        return self.x;
    }

    pub fn get_y(&self) -> u64 {
        return self.y;
    }

}

impl Line2d {

    pub fn new() -> Line2d {
        return Line2d {
            a: Point2d::new(),
            b: Point2d::new(),
        }
    }

    pub fn from_points(a: Point2d, b: Point2d) -> Line2d {
        return Line2d {
            a,
            b,
        }
    }

    pub fn min_x(&self) -> u64 {
        if self.a.x < self.b.x {
            return self.a.x;
        } else {
            return self.b.x;
        }
    }

    pub fn max_x(&self) -> u64 {
        if self.a.x > self.b.x {
            return self.a.x;
        } else {
            return self.b.x;
        }
    }

    pub fn min_y(&self) -> u64 {
        if self.a.y < self.b.y {
            return self.a.y;
        } else {
            return self.b.y;
        }
    }

    pub fn max_y(&self) -> u64 {
        if self.a.y > self.b.y {
            return self.a.y;
        } else {
            return self.b.y;
        }
    }

    pub fn horizontal(&self) -> bool {
        return self.a.y == self.b.y;
    }

    pub fn vertical(&self) -> bool {
        return self.a.x == self.b.x;
    }

}

impl Plane2d {
    pub fn new(width: u64, height: u64) -> Plane2d {
        let mut out = Plane2d {
            width: width as usize,
            height: height as usize,
            data: Vec::<i64>::with_capacity((width*height) as usize),
        };

        for _ in 0..(width*height) {
            out.data.push(0);
        }

        return out;
    }

    pub fn mark(&mut self, line: Line2d) {

        if line.vertical() {
            let x = line.min_x() as usize;
            for y in (line.min_y() as usize)..((line.max_y() + 1) as usize) {
                self[(x, y)] += 1;
            }
        } else if line.horizontal() {
            let y = line.min_y() as usize;
            for x in (line.min_x() as usize)..((line.max_x() + 1) as usize) {
                self[(x, y)] += 1;
            }
        } else {
            let angle = ((line.a.y as i64) - (line.b.y as i64))/((line.a.x as i64) - (line.b.x as i64));
            if angle == 1 {
                let start = (line.min_x(), line.min_y());
                for i in 0..(line.max_x() - line.min_x() + 1) {
                    self[(start.0 + i, start.1 + i)] += 1;
                }
            } else {
                let start = (line.max_x(), line.min_y());
                for i in 0..(line.max_x() - line.min_x() + 1) {
                    self[(start.0 - i, start.1 + i)] += 1;
                }
            }

        }

    }

}

impl FromStr for Point2d {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.split(",").collect();
        return Result::Ok(
            Point2d {
                x: points[0].parse::<u64>().unwrap(),
                y: points[1].parse::<u64>().unwrap(),
            },
        );
    }

}

impl Display for Point2d {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        return formatter.write_str( format!("({}, {})", self.x, self.y).as_str());
    }

}

impl FromStr for Line2d {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let point_strings: Vec<&str> = s.split(" -> ").collect();
        return Result::Ok(
            Self::from_points(
                point_strings[0].parse::<Point2d>().unwrap(),
                point_strings[1].parse::<Point2d>().unwrap(),
            ),
        );

    }
}

impl Display for Line2d {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        return formatter.write_str( format!("<{} -> {}>", self.a, self.b).as_str());
    }

}

impl Display for Plane2d {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        let mut lines = Vec::<String>::new();
        for y in 0..self.height {
            let mut row = Vec::<String>::new();
            for x in 0..self.width {
                row.push(self[(x, y)].to_string())
            }
            lines.push(row.join(", "));
        }
        return formatter.write_str( lines.join("\n").as_str());
    }
}

impl Index<(usize, usize)> for Plane2d {
    type Output = i64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.data[self.width*index.0 + index.1];
    }
}

impl Index<(u64, u64)> for Plane2d {
    type Output = i64;

    fn index(&self, index: (u64, u64)) -> &Self::Output {
        return &self.data[self.width*(index.0 as usize) + (index.1 as usize)];
    }
}

impl IndexMut<(u64, u64)> for Plane2d {
    fn index_mut(&mut self, index: (u64, u64)) -> &mut i64 {
        return &mut self.data[self.width*(index.0 as usize) + (index.1 as usize)];
    }
}

impl IndexMut<(usize, usize)> for Plane2d {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut i64 {
        return &mut self.data[self.width*index.0 + index.1];
    }
}

impl IntoIterator for Plane2d {
    type Item = i64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.data.into_iter();
    }
}

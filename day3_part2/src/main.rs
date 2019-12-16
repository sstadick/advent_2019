use snafu::Snafu;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self};
use std::str::FromStr;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not parse direction: {}", input_dir))]
    ParseDirection { input_dir: char },
    #[snafu(display("Could not parse distance: {}", input_dist))]
    ParseDist { input_dist: String },
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

// The number of grid steps it took to get to this point, first time only
type Step = usize;

// Hold the points that make up the path
struct Path {
    ordered: Vec<Point>,
    lookup: HashMap<Point, Step>,
}

type Distance = u32;
enum Direction {
    Up(Distance),
    Down(Distance),
    Right(Distance),
    Left(Distance),
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(direction: &str) -> Result<Self, Self::Err> {
        let mut direction = direction.chars();
        let dir = direction.next().unwrap();
        let dir_rest = direction.collect::<String>();
        let dist = match dir_rest.parse::<Distance>() {
            Ok(dist) => dist,
            Err(_) => {
                return Err(Error::ParseDist {
                    input_dist: dir_rest,
                });
            }
        };
        match dir {
            'L' => Ok(Direction::Left(dist)),
            'R' => Ok(Direction::Right(dist)),
            'U' => Ok(Direction::Up(dist)),
            'D' => Ok(Direction::Down(dist)),
            _ => Err(Error::ParseDirection { input_dir: dir }),
        }
    }
}

fn parse_path(raw_path: &str) -> Path {
    use Direction::*;
    let instructions: Vec<Direction> = raw_path
        .split(',')
        .map(|raw| Direction::from_str(raw).unwrap())
        .collect();
    let mut step = 0;
    let mut current = Point { x: 0, y: 0 };
    let mut path = Path {
        ordered: vec![],
        lookup: HashMap::new(),
    };
    for dir in instructions.iter() {
        let points: Vec<Point> = match dir {
            Up(dist) => (0..*dist)
                .map(|_| {
                    current.y += 1;
                    Point { ..current }
                })
                .collect(),
            Down(dist) => (0..*dist)
                .map(|_| {
                    current.y -= 1;
                    Point { ..current }
                })
                .collect(),
            Left(dist) => (0..*dist)
                .map(|_| {
                    current.x -= 1;
                    Point { ..current }
                })
                .collect(),
            Right(dist) => (0..*dist)
                .map(|_| {
                    current.x += 1;
                    Point { ..current }
                })
                .collect(),
        };
        path.ordered.extend(points.clone());
        for p in points {
            step += 1;
            path.lookup.entry(p).or_insert(step);
        }
    }
    path
}

fn find_intersections<'a>(path1: &'a Path, path2: &'a Path) -> Vec<&'a Point> {
    path1
        .ordered
        .iter()
        .filter(|point| path2.lookup.contains_key(point))
        .collect()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock().lines();
    let wire1 = handle.next().unwrap()?;
    let wire2 = handle.next().unwrap()?;
    let wire1_path = parse_path(&wire1);
    let wire2_path = parse_path(&wire2);
    let intersections = find_intersections(&wire1_path, &wire2_path);
    let closest = intersections
        .iter()
        .min_by(|a, b| {
            (wire1_path.lookup.get(a).unwrap() + wire2_path.lookup.get(a).unwrap())
                .cmp(&(wire1_path.lookup.get(b).unwrap() + wire2_path.lookup.get(b).unwrap()))
        })
        .unwrap();
    println!(
        "Closest steps: {:#?}",
        wire1_path.lookup.get(closest).unwrap() + wire2_path.lookup.get(closest).unwrap()
    );
    Ok(())
}

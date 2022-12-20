use std::fmt;
use std::collections::{HashSet,HashMap};

#[derive(Clone, Copy)]
pub struct Point {
    pub c : u8,
    pub x : usize,
    pub y : usize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.c {
            0..=25 => (('a' as u8) + self.c) as char,
            26..=51 => (('A' as u8) - 26 + self.c) as char,
            _ => unimplemented!()
        };
        write!(f, "{}", c)
    }
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Point> {
    let mut c = 0;
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split(|c| !char::is_numeric(c));
            let x = tokens.next().unwrap().parse().unwrap();
            tokens.next();
            let y = tokens.next().unwrap().parse().unwrap();
            let p = Point {c, x, y};
            c += 1;
            p
        })
        .collect()
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Id(pub usize);
#[derive(Clone)]
struct Distance(pub usize);

#[derive(Clone, Copy)]
enum Cell {
    Open,
    Candidate(Point),
    Tied,
    Confirmed(Point),
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Open => write!(f, " "),
            Cell::Tied => write!(f, "."),
            Cell::Candidate(_) =>  write!(f, "?"),
            Cell::Confirmed(p) =>  write!(f, "{:?}", p),
        }
    }
}

fn print<T>(grid: &mut [&mut [T]], width: usize, height: usize)
    where T : fmt::Debug
{
    for _ in 0..width {
        print!("-");
    }
    println!();

    for y in 0..height {
        print!("|");
        for x in 0..width {
            print!("{:?}", grid[x][y]);
        }
        print!("|");
        println!();
    }
    for _ in 0..width {
        print!("-");
    }
    println!();
    println!();
}

#[aoc(day6, part1)]
pub fn solve_part1(coords: &Vec<Point>) -> usize {
    let width  : usize = coords.iter().max_by(|a,b| a.x.cmp(&b.x)).unwrap().x + 1;
    let height : usize = coords.iter().max_by(|a,b| a.y.cmp(&b.y)).unwrap().y + 1;

    let mut grid_raw = vec![ Cell::Open; width * height];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(height).collect();
    // Final 2d array
    let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    for point in coords {
        grid[point.x][point.y] = Cell::Confirmed(*point);
    }

    let mut infinite = HashSet::new();

    let max_distance = (width + height) as isize;
    for distance in 1..max_distance {
        for p in coords {
            for xx in -1*distance..=distance {
                let x = p.x as isize + xx;
                if x < 0 || x >= width as isize {
                    continue;
                }

                let updown = distance - isize::abs(xx);
                let zero_case_yys : &[isize] = &[updown];
                let normal_yys : &[isize] = &[updown, -1 * updown];
                let yys = if updown == 0 { zero_case_yys } else { normal_yys };
                for yy in yys {
                    let y = p.y as isize + yy;
                    if y < 0 || y >= height as isize {
                        continue;
                    }

                    // println!("xx={} yy={}", xx, yy);

                    let cell = &mut grid[x as usize][y as usize];
                    match cell.clone() {
                        Cell::Open => {*cell = Cell::Candidate(*p); },
                        Cell::Candidate(_) => {*cell = Cell::Tied; },
                        Cell::Tied | Cell::Confirmed(_) => { },
                    }
                }
            }
        }

        for x in 0..width {
            for y in 0..height {
                let cell = &mut grid[x][y];
                match cell.clone() {
                    Cell::Candidate(p) => {
                        *cell = Cell::Confirmed(p); 

                        if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                            infinite.insert(p.c);
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    let mut counts : HashMap<u8, usize> = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            match grid[x][y] {
                Cell::Confirmed(p) => {
                    if !infinite.contains(&p.c) {
                        let entry = counts.entry(p.c).or_insert(0);
                        *entry += 1;
                    }
                },
                _ => {},
            }
        }
    }



    print(grid, width, height);
    let (_largest_point, largest_area) = counts.iter().max_by(|x,y| x.1.cmp(y.1)).unwrap();
    *largest_area
}

#[aoc(day6, part2)]
pub fn solve_part2(coords: &Vec<Point>) -> usize {
    let width  : usize = coords.iter().max_by(|a,b| a.x.cmp(&b.x)).unwrap().x + 1;
    let height : usize = coords.iter().max_by(|a,b| a.y.cmp(&b.y)).unwrap().y + 1;

    let mut grid_raw = vec![0 as usize; width * height];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(height).collect();
    // Final 2d array
    let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    for p in coords {
        for x in 0..width {
            for y in 0..height {
                let dist = isize::abs(p.x as isize - x as isize) + isize::abs(p.y as isize - y as isize);
                grid[x][y] += dist as usize;
            }
        }
    }

    let mut largest_area = 0;
    for x in 0..width {
        for y in 0..height {
            grid[x][y] = if grid[x][y] < 10000 { largest_area += 1; 1 } else { 0 };
        }
    }

    print(grid, width as usize, height as usize);
    largest_area
}

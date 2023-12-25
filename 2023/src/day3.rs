use std::{fmt::Display, collections::BTreeMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Number(u32),
    Empty,
    Symbol(char),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            c if char::is_digit(c, 10) => Cell::Number(c.to_digit(10).unwrap()),
            c => Cell::Symbol(c),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Number(n) => write!(f, "{}", n),
            Cell::Empty => write!(f, "."),
            Cell::Symbol(c) => write!(f, "{}", c),
        }
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> (Vec<Vec<Cell>>,Vec<(u32,usize,usize,usize)>) {
    let mut rows: Vec<Vec<Cell>> = Vec::new();
    for line in input.trim().lines() {
        rows.push(line.trim().chars().map(|c| c.into()).collect());
    }

    // dbg!(&rows);

    let mut numbers = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        // println!("row {:?}", row);
        let mut number = None;
        for (x, cell) in row.iter().enumerate() {
            if let Cell::Number(n) = cell {
                if let Some((num, _, digits)) = &mut number {
                    *num = 10 * *num + n;
                    *digits += 1;
                } else {
                    number = Some((*n, x, 1usize));
                }
            } else {
                if let Some((n, start, digits)) = number {
                    numbers.push((n, start, digits, y));
                    // println!(" {:?}", number);
                    number = None;
                }
            }
        }

        if let Some((n, start, digits)) = number {
            numbers.push((n, start, digits, y));
        }
    }

    (rows, numbers)
}

#[aoc(day3, part1)]
fn part1(input: &(Vec<Vec<Cell>>,Vec<(u32,usize,usize,usize)>)) -> u32 {
    
    let (rows, numbers) = input;
    let mut sum = 0;
    let mut cleaned = rows.clone();

    'number: for (n, x, digits, y) in numbers {
        let idigits = *digits as isize; 
        for dy in -1..=1 {
            for dx in -1..=idigits {
                if dy == 0 && dx >= 0 && dx < idigits {
                    continue;
                }
                let (xx,yy) = (*x as isize + dx, *y as isize + dy);
                let (xx, yy): (Option<usize>, Option<usize>) = (xx.try_into().ok(), yy.try_into().ok());
                if let (Some(xx),Some(yy)) = (xx,yy) {
                    println!("Checking ({},{}) for {}@({},{},{})", xx, yy, n, x, digits, y);
                    if let Some(Cell::Symbol(_)) = rows.get(yy).and_then(|r| r.get(xx)) {
                        sum += n;
                        
                            for dx in 0..*digits {
                                if let Some(row) = cleaned.get_mut(*y) {
                                    if let Some(cell) = row.get_mut(*x + dx) {
                                        *cell = Cell::Empty;
                                    }
                                }
                            }

                        continue 'number;
                    }
                }
            }
        }
    }

    for line in &cleaned {
        for cell in line {
            print!("{}", cell);
        }
        println!();
    }
   
    sum
}


#[aoc(day3, part2)]
fn part2(input: &(Vec<Vec<Cell>>,Vec<(u32,usize,usize,usize)>)) -> u32 {
    let (rows, numbers) = input;

    let mut gears: BTreeMap<(usize, usize), Vec<u32>> = BTreeMap::new();
    for (y,row) in rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Cell::Symbol(c) = cell {
                if *c == '*' {
                    gears.insert((x,y), Vec::new());
                }
            }
        }
    }

    for (n, x, digits, y) in numbers {
        let idigits = *digits as isize; 
        for dy in -1..=1 {
            for dx in -1..=idigits {
                if dy == 0 && dx >= 0 && dx < idigits {
                    continue;
                }
                let (xx,yy) = (*x as isize + dx, *y as isize + dy);
                let (xx, yy): (Option<usize>, Option<usize>) = (xx.try_into().ok(), yy.try_into().ok());
                if let (Some(xx),Some(yy)) = (xx,yy) {
                    println!("Checking ({},{}) for {}@({},{},{})", xx, yy, n, x, digits, y);
                    if let Some(Cell::Symbol(c)) = rows.get(yy).and_then(|r| r.get(xx)) {
                        if *c == '*' {
                            gears.get_mut(&(xx,yy)).unwrap().push(*n);
                        }
                    }
                }
            }
        }
    }

    let mut sum = 0;

    for (_,numbers) in &gears {
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }
   
    sum
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = parse_input(r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#.trim());
        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn part1_missing() {
        let input = parse_input(r#"
        ...277
        ..*...
        "#.trim());
        assert_eq!(part1(&input), 277);
    }

    #[test]
    fn part2_example() {
        let input = parse_input(r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#.trim());
        assert_eq!(part2(&input), 467835);
    }
}
use std::fmt::Display;

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
fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    let mut rows = Vec::new();
    for line in input.trim().lines() {
        rows.push(line.trim().chars().map(|c| c.into()).collect());
    }

    // dbg!(&rows);

    rows
}

#[aoc(day3, part1)]
fn part1(rows: &Vec<Vec<Cell>>) -> u32 {
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
    let mut sum = 0;
    let mut cleaned = rows.clone();

    'number: for (n, x, digits, y) in numbers {
        let idigits = digits as isize; 
        for dy in -1..=1 {
            for dx in -1..=idigits {
                if dy == 0 && dx >= 0 && dx < idigits {
                    continue;
                }
                let (xx,yy) = (x as isize + dx, y as isize + dy);
                let (xx, yy): (Option<usize>, Option<usize>) = (xx.try_into().ok(), yy.try_into().ok());
                if let (Some(xx),Some(yy)) = (xx,yy) {
                    println!("Checking ({},{}) for {}@({},{},{})", xx, yy, n, x, digits, y);
                    if let Some(Cell::Symbol(_)) = rows.get(yy).and_then(|r| r.get(xx)) {
                        sum += n;
                        
                            for dx in 0..digits {
                                if let Some(row) = cleaned.get_mut(y) {
                                    if let Some(cell) = row.get_mut(x + dx) {
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

// 531267 is too low



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

    // #[test]
    // fn part2_example() {
    //     let input = parse_input(r#"
    //     Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    //     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    //     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    //     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    //     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    //     "#.trim());
    //     assert_eq!(part2(&input), 2286);
    // }
}
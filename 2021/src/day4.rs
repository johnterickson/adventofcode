use std::collections::BTreeSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (Vec<usize>, Vec<[[usize;5]; 5]>) {
    let mut lines = input.lines();
    let draws = lines.next().unwrap().trim().split(',')
        .map(|n| usize::from_str_radix(n, 10).unwrap())
        .collect();

    assert_eq!("", lines.next().unwrap().trim());
    let lines: Vec<_> = lines.collect();
    (
        draws,
        lines.chunks(6).map(|b| {
            let mut board = [[0;5]; 5];
            for (y, line) in b.iter().take(5).enumerate() {
                for (x,n) in line.trim().split(' ').filter_map(|n| if n.trim().len() > 0 { Some(n.trim()) } else { None } ).enumerate() {
                    board[y][x] = usize::from_str_radix(n, 10).unwrap();
                }
            }

            board
        }).collect()
    )
}

#[aoc(day4, part1)]
fn part1(inputs: &(Vec<usize>, Vec<[[usize;5]; 5]>)) -> usize {
    let (draws, boards) = inputs;

    let mut boards: Vec<_> = boards.iter().map(|b| {
        (b, [0u8; 5])
    }).collect();

    let win_masks = [
        [0b11111, 0b00000, 0b00000, 0b00000, 0b00000,],
        [0b00000, 0b11111, 0b00000, 0b00000, 0b00000,],
        [0b00000, 0b00000, 0b11111, 0b00000, 0b00000,],
        [0b00000, 0b00000, 0b00000, 0b11111, 0b00000,],
        [0b00000, 0b00000, 0b00000, 0b11111, 0b11111,],
        [0b10000, 0b10000, 0b10000, 0b10000, 0b10000,],
        [0b01000, 0b01000, 0b01000, 0b01000, 0b01000,],
        [0b00100, 0b00100, 0b00100, 0b00100, 0b00100,],
        [0b00010, 0b00010, 0b00010, 0b00010, 0b00010,],
        [0b00001, 0b00001, 0b00001, 0b00001, 0b00001,],
    ];

    let is_winner = |hits: &[u8; 5]| {
        for win_mask in win_masks {
            if hits.iter().zip(win_mask).all(|(h, m)| h & m == m) {
                // dbg!(hits);
                // dbg!(win_mask);
                return true;
            }
        }
        false
    };

    for d in draws {
        // dbg!(&d);
        for (board,hits) in boards.iter_mut() {
            for (y,row) in board.iter().enumerate() {
                for (x,n) in row.iter().enumerate() {
                    if n == d {
                        hits[y] |= 1 << x;
                        if is_winner(hits) {
                            let mut sum_unmarked = 0;
                            // dbg!(&board);
                            for yy in 0..5 {
                                for xx in 0..5 {
                                    if (hits[yy] >> xx) & 0x1 == 0 {
                                        sum_unmarked += board[yy][xx];
                                    }
                                }
                            }
                            return d * sum_unmarked;

                        }
                    }
                }
            }
        }
    }

    unreachable!();
}

#[aoc(day4, part2)]
fn part2(inputs: &(Vec<usize>, Vec<[[usize;5]; 5]>)) -> usize {
    let (draws, boards) = inputs;

    let mut boards: Vec<_> = boards.iter().map(|b| {
        (b, [0u8; 5])
    }).collect();

    let win_masks = [
        [0b11111, 0b00000, 0b00000, 0b00000, 0b00000,],
        [0b00000, 0b11111, 0b00000, 0b00000, 0b00000,],
        [0b00000, 0b00000, 0b11111, 0b00000, 0b00000,],
        [0b00000, 0b00000, 0b00000, 0b11111, 0b00000,],
        [0b00000, 0b00000, 0b00000, 0b00000, 0b11111,],
        [0b10000, 0b10000, 0b10000, 0b10000, 0b10000,],
        [0b01000, 0b01000, 0b01000, 0b01000, 0b01000,],
        [0b00100, 0b00100, 0b00100, 0b00100, 0b00100,],
        [0b00010, 0b00010, 0b00010, 0b00010, 0b00010,],
        [0b00001, 0b00001, 0b00001, 0b00001, 0b00001,],
    ];

    let is_winner = |hits: &[u8; 5]| {
        for win_mask in win_masks {
            if hits.iter().zip(win_mask).all(|(h, m)| h & m == m) {
                // dbg!(hits);
                // dbg!(win_mask);
                return true;
            }
        }
        false
    };

    let mut winners = BTreeSet::new();
    let len = boards.len();
    // dbg!(len);

    for d in draws {
        // dbg!(&d);
        for (bi, (board,hits)) in boards.iter_mut().enumerate() {
            for (y,row) in board.iter().enumerate() {
                for (x,n) in row.iter().enumerate() {
                    if n == d {
                        hits[y] |= 1 << x;
                        if is_winner(hits) {
                            if winners.insert(bi) {
                                // dbg!(&bi, winners.len());
                                if winners.len() == len {
                                    let mut sum_unmarked = 0;
                                    for yy in 0..5 {
                                        for xx in 0..5 {
                                            if (hits[yy] >> xx) & 0x1 == 0 {
                                                sum_unmarked += board[yy][xx];
                                            }
                                        }
                                    }
                                    // dbg!(d, sum_unmarked);
                                    return d * sum_unmarked;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input =
    r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"#;
        let input = parse_input(input);
        assert_eq!(part1(&input), 4512);
    }

    #[test]
    fn part2_example() {
        let input =
    r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"#;
        let input = parse_input(input);
        assert_eq!(part2(&input), 1924);
    }
}
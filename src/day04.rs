use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn count_xmas_occurences(
    graph: &[Vec<char>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    word: &[char],
) -> i32 {
    count_vertical(graph, rows, cols, r, c, word)
        + count_horizontal(graph, rows, cols, r, c, word)
        + count_diagonal(graph, rows, cols, r, c, word)
}

fn count_diagonal(
    graph: &[Vec<char>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    word: &[char],
) -> i32 {
    let mut nw = Vec::with_capacity(word.len());
    let mut count = 0;

    for i in 0..word.len() {
        let r1 = r + i;
        let c1 = c + i;
        if r1 >= rows || c1 >= cols {
            break;
        }
        nw.push(graph[r1][c1]);
    }

    if !nw.is_empty() && nw.len() == word.len() && nw == word {
        count += 1
    }

    nw.clear();
    for i in 0..word.len() as i32 {
        let r1 = (r as i32) - i;
        let c1 = (c as i32) - i;
        if r1 < 0 || c1 < 0 {
            break;
        }
        nw.push(graph[r1 as usize][c1 as usize]);
    }

    if !nw.is_empty() && nw.len() == word.len() && nw == word {
        count += 1
    }

    nw.clear();
    for i in 0..word.len() as i32 {
        let r1 = r + i as usize;
        let c1 = (c as i32) - i;
        if r1 >= rows || c1 < 0 {
            break;
        }
        nw.push(graph[r1][c1 as usize]);
    }

    if !nw.is_empty() && nw.len() == word.len() && nw == word {
        count += 1
    }

    nw.clear();
    for i in 0..word.len() as i32 {
        let r1 = (r as i32) - i;
        let c1 = c + i as usize;
        if r1 < 0 || c1 >= cols {
            break;
        }
        nw.push(graph[r1 as usize][c1]);
    }

    if !nw.is_empty() && nw.len() == word.len() && nw == word {
        count += 1
    }

    count
}

fn count_horizontal(
    graph: &[Vec<char>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    word: &[char],
) -> i32 {
    let mut nw = Vec::with_capacity(word.len());
    let mut count = 0;

    for i in 0..word.len() {
        let nc = c + i;
        if nc >= cols {
            break;
        }
        nw.push(graph[r][nc]);
    }

    if !nw.is_empty() && nw.len() == word.len() && nw == word {
        count += 1
    }

    nw.clear();
    for i in 0..word.len() as i32 {
        let nc = (c as i32) - i;
        if nc < 0 {
            break;
        }
        nw.push(graph[r][nc as usize]);
    }

    if !nw.is_empty() && nw.len() == word.len() && nw == word {
        count += 1
    }

    count
}

fn count_vertical(
    graph: &[Vec<char>],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    word: &[char],
) -> i32 {
    let mut nw = Vec::with_capacity(word.len());
    let mut count = 0;

    for i in 0..word.len() {
        let nr = r + i;
        if nr >= rows {
            break;
        }
        nw.push(graph[nr][c]);
    }

    if !nw.is_empty() && nw.len() == word.len() && nw == word {
        count += 1
    }

    nw.clear();
    for i in 0..word.len() as i32 {
        let nr = (r as i32) - i;
        if nr < 0 {
            break;
        }
        nw.push(graph[nr as usize][c]);
    }

    if !nw.is_empty() && nw.len() == word.len() && nw == word {
        count += 1
    }

    count
}

pub fn run() {
    let f = File::open("testdata/day04").unwrap();
    let mut reader = BufReader::new(f);

    let mut lines: Vec<Vec<char>> = vec![];
    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        lines.push(line.trim().chars().collect());
    }

    let mut count = 0;
    let rows = lines.len();
    assert!(rows > 0);
    let cols = lines[0].len();
    let target = vec!['X', 'M', 'A', 'S'];

    for r in 0..rows {
        for c in 0..cols {
            if lines[r][c] == 'X' {
                count += count_xmas_occurences(&lines, rows, cols, r, c, &target);
            }
        }
    }

    println!("Result (part 1): {}", count);

    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if lines[r][c] == 'A' {
                count += count_mas_occurence(&lines, rows, cols, r, c);
            }
        }
    }

    println!("Result (part 2): {}", count);
}

fn count_mas_occurence(lines: &[Vec<char>], rows: usize, cols: usize, r: usize, c: usize) -> i32 {
    if r == 0 || c == 0 || r == rows - 1 || c == cols - 1 {
        return 0;
    }
    let left_diagonal = (lines[r - 1][c - 1], lines[r + 1][c + 1]);
    let right_diagonal = (lines[r - 1][c + 1], lines[r + 1][c - 1]);
    if (left_diagonal == ('M', 'S') || left_diagonal == ('S', 'M'))
        && (right_diagonal == ('M', 'S') || right_diagonal == ('S', 'M'))
    {
        return 1;
    }
    0
}

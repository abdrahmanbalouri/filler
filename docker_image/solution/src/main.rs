use std::io::{self, BufRead};

fn valid(
    board: &Vec<Vec<u8>>,
    piece: &Vec<Vec<u8>>,
    x: i32,
    y: i32,
    me: u8,
    me_old: u8,
    en: u8,
    en_old: u8,
) -> bool {

    let mut overlap = 0;

    for py in 0..piece.len() {
        for px in 0..piece[0].len() {

            if piece[py][px] == b'.' {
                continue;
            }

            let bx = x + px as i32;
            let by = y + py as i32;

            if bx < 0
                || by < 0
                || by >= board.len() as i32
                || bx >= board[0].len() as i32
            {
                return false;
            }

            let c = board[by as usize][bx as usize];

            if c == en || c == en_old {
                return false;
            }

            if c == me || c == me_old {
                overlap += 1;
            }
        }
    }

    overlap == 1
}

fn distance(board: &Vec<Vec<u8>>, x: i32, y: i32, en: u8, en_old: u8) -> i32 {

    let mut best = i32::MAX;

    for ey in 0..board.len() {
        for ex in 0..board[0].len() {

            let c = board[ey][ex];

            if c == en || c == en_old {

                let d =
                    (x - ex as i32).abs() +
                    (y - ey as i32).abs();

                if d < best {
                    best = d;
                }

            }
        }
    }

    best
}

fn main() {

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let first = lines.next().unwrap();

    let (me, me_old, en, en_old) = if first.contains("p1") {
        (b'@', b'a', b'$', b's')
    } else {
        (b'$', b's', b'@', b'a')
    };

    loop {

        let mut w = 0;
        let mut h = 0;

        while let Some(line) = lines.next() {

            if line.contains("Anfield") {

                let p: Vec<&str> = line.split_whitespace().collect();

                w = p[1].parse().unwrap();
                h = p[2].trim_end_matches(':').parse().unwrap();

                break;
            }
        }

        if w == 0 {
            return;
        }

        let mut board = Vec::new();

        while board.len() < h {

            let line = lines.next().unwrap();

            if line.trim_start().chars().all(|c| c.is_digit(10)) {
                continue;
            }

            let row = line.as_bytes();

            board.push(row[row.len() - w..].to_vec());
        }

        let mut pw = 0;
        let mut ph = 0;

        while let Some(line) = lines.next() {

            if line.contains("Piece") {

                let p: Vec<&str> = line.split_whitespace().collect();

                pw = p[1].parse().unwrap();
                ph = p[2].trim_end_matches(':').parse().unwrap();

                break;
            }
        }

        let mut piece = Vec::new();

        for _ in 0..ph {
            piece.push(lines.next().unwrap().as_bytes()[..pw].to_vec());
        }

        let mut best_move = (0,0);
        let mut best_score = i32::MAX;
        let mut found = false;

        for y in 0..h {
            for x in 0..w {

                if valid(&board,&piece,x as i32,y as i32,me,me_old,en,en_old) {

                    let sc = distance(&board,x as i32,y as i32,en,en_old);

                    if sc < best_score {

                        best_score = sc;
                        best_move = (x,y);
                        found = true;
                    }

                }
            }
        }

        if found {
            println!("{} {}",best_move.0,best_move.1);
        } else {
            println!("0 0");
        }
    }
}


pub fn valid(
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

pub fn distance(board: &Vec<Vec<u8>>, x: i32, y : i32, en: u8, en_old: u8) -> i32 {

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

use std::{thread, time};
use crossterm::terminal;
use game_of_life::{ScreenRes, get_neighbours, print_2d_arr, is_ok, is_populated};

const CHAR: char = '*';

fn main() {
    let (width, height) = terminal::size().unwrap();
    let res = ScreenRes {
        width: width as usize,
        height: height as usize
    };

    let mut screen: Vec<Vec<char>> = vec![vec![' '; res.width]; res.height];

    screen[6][10] = CHAR;
    screen[6][11] = CHAR;
    screen[6][12] = CHAR;
    screen[5][12] = CHAR;
    screen[4][11] = CHAR;

    print_2d_arr(&screen);

    loop {
        let mut copied_screen:Vec<Vec<char>> = screen[..].to_vec();

        for i in 0..res.height {
            for j in 0..res.width {
                let neighbours = get_neighbours(&screen, i, j);
                if screen[i][j] == CHAR {
                    if !is_ok(&neighbours, &CHAR) {
                        copied_screen[i][j] = ' ';
                    }
                } else {
                    if is_populated(&neighbours, &CHAR) {
                        copied_screen[i][j] = CHAR;
                    }
                }
            }
        }

        print_2d_arr(&copied_screen);

        if screen == copied_screen {
            break;
        }
        screen = copied_screen[..].to_vec();

        let mut is_end = true;
        for i in &screen {
            for j in i {
                if *j == CHAR {
                    is_end = false;
                    break;
                }
            }
        }

        if is_end { break; }
        thread::sleep(time::Duration::from_millis(100));
    }
}
use std::collections::HashMap;

pub fn get_neighbours<T>(arr: &Vec<Vec<T>>, x: usize, y: usize) -> HashMap<(usize, usize), &T> {
    let mut neighbours = HashMap::new();
    if x > 0 {
        neighbours.insert((x - 1, y), &arr[x - 1][y]);
        if y > 0 {
            neighbours.insert((x - 1, y - 1), &arr[x - 1][y - 1]);
        }

        if y < arr[x - 1].len() - 1 {
            neighbours.insert((x - 1, y + 1), &arr[x - 1][y + 1]);
        }
    }

    if x < arr.len() - 1 {
        neighbours.insert((x + 1, y), &arr[x + 1][y]);
        if y > 0 {
            neighbours.insert((x + 1, y - 1), &arr[x + 1][y - 1]);
        }
        if y < arr[x + 1].len() - 1 {
            neighbours.insert((x + 1, y + 1), &arr[x + 1][y + 1]);
        }
    }

    if y > 0 {
        neighbours.insert((x, y - 1), &arr[x][y - 1]);
    }

    if y < arr[x].len() - 1 {
        neighbours.insert((x, y + 1), &arr[x][y + 1]);
    }

    neighbours
}

pub fn is_ok<T: std::cmp::PartialEq>(neighbours: &HashMap<(usize, usize), T>, elem: T) -> bool {
    let mut count: u8 = 0;
    for item in neighbours.iter() {
        if *item.1 == elem {
            count += 1;
        }
    }

    count > 1 && count < 4
}

pub fn is_populated<T: std::cmp::PartialEq>(neighbours: &HashMap<(usize, usize), T>, elem: T) -> bool {
    let mut count: u8 = 0;

    for item in neighbours.iter() {
        if *item.1 == elem {
            count += 1;
        }
    }

    count == 3
}

pub fn print_2d_arr<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
    let mut text = String::new();

    for i in arr {
        for j in i {
            text.push_str(format!("{}", j).as_str());
        }
    }

    print!("{}", text);
}

pub struct ScreenRes {
    pub width: usize,
    pub height: usize
}
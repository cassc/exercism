use std::char;

fn get_char(minefield: &[&str], x: i32, y: i32) -> Option<char> {
    if x < 0 || y < 0 {
        None
    } else {
        minefield.get(y as usize)?.chars().nth(x as usize)
    }
}

fn mines_at(minefield: &[&str], x: i32, y: i32) -> char {
    if get_char(minefield, x, y) == Some('*') {
        '*'
    } else {
        let mine = |a, b| match get_char(minefield, a, b) {
            Some('*') => 1,
            _ => 0,
        };
        let n: i32 = (-1..=1)
            .map(|i| {
                (-1..=1)
                    .map(|j| {
                        if i == j && i == 0 {
                            0
                        } else {
                            let c1 = x + i;
                            let c2 = y + j;
                            mine(c1, c2)
                        }
                    })
                    .sum::<i32>()
            })
            .sum();
        if n > 0 {
            char::from_digit(n as u32, 10).unwrap()
        } else {
            ' '
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let width = minefield.get(0).unwrap_or(&"").len();
    let height = minefield.len();
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| mines_at(minefield, x as i32, y as i32))
                .collect::<String>()})
        .collect()
}

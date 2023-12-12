use super::reader;

#[allow(dead_code)]
pub fn one() {
    println!("Day three, part one");
    let _content = reader::read_file("./input/day_three.txt");

    let lines: Vec<String> = _content.split("\n").map(|s| s.to_string()).collect();
    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        let indexes = get_index_number(&line);
        if indexes.len() == 0 {
            continue;
        }
        for index in indexes {
            println!("Index: {:?}", index);
            if is_island(lines.clone(), i as i32, index[0], index[1]) {
                total += get_number_from_index(&line, index);
            }
        }
    }

    println!("Total is: {:?}", total);
}

fn get_index_number(input: &str) -> Vec<[i32; 2]> {
    let mut indexes = Vec::new();
    let mut coord: [i32; 2] = [0, 0];
    let mut last = false;

    for (i, c) in input.char_indices() {
        if c.is_numeric() && !last {
            coord[0] = i as i32;
            last = true;
        }
        if !c.is_numeric() && last {
            coord[1] = (i-1) as i32;
            indexes.push(coord);
            last = false;
        }
    }

    indexes
}

fn is_island(lake: Vec<String>, y:i32, mut x1: i32, mut x2: i32) -> bool {
    if x1 > 0 {
        x1 -= 1;
        if lake[y as usize].chars().nth(x1 as usize).unwrap() != '.' {
            return true;
        }
    }
    if x2 < lake[y as usize].len() as i32 {
        x2 += 1;
        if lake[y as usize].chars().nth(x2 as usize).unwrap() != '.' {
            return true;
        }
    }
    if y > 0 {
        let i = y - 1;
        let mut x = x1;
        while x <= x2 {
            if lake[i as usize].chars().nth(x as usize).unwrap() != '.' {
                return true;
            }
            x += 1;
        }
    }

    if y < (lake.len() - 1) as i32 {
        let i = y + 1;
        let mut x = x1;
        println!("i: {:?} x1: {:?}, x2: {:?}", i, x1, x2);
        while x <= x2 {
            if lake[i as usize].chars().nth(x as usize).unwrap() != '.' {
                return true;
            }
            x += 1;
        }
    }

    false
}

fn get_number_from_index(input: &str, index: [i32; 2]) -> i32 {
    let mut number = String::new();
    for (i, c) in input.char_indices() {
        if i as i32 >= index[0] && i as i32 <= index[1] {
            number.push(c);
        }
    }

    number.parse::<i32>().unwrap()
}
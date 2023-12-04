use std::fs;

pub fn one() {
    // pub .map(|s| s.to_string()).collect()
    println!("Hello from day two part one");
    let _content = read_file("./input/day_two.txt");
    let blue = "blue"; 
    let red = "red"; 
    let green = "green";

    let mut game_count = 0;

    let max_blue = 14;
    let max_red = 12;
    let max_green = 13;

    for line in _content {
        let segments = split_string(&line, ':');
        let game_id = extract_end(&segments[0], ' ');
        
        let sets = split_string(&segments[1], ';');
        println!("Sets are: {:?}", sets);
        // let mut blue_count = 0;
        // let mut red_count = 0;
        // let mut green_count = 0;
        let mut possible = true;
        for set in sets {
            let colors = split_string(&set, ',');
            for color in colors {
                if !possible {
                    break;
                }
                let number = extract_between(&color, ' ', ' ');
                let n = number.map(|s| s.parse::<i32>().ok()).flatten();
                if color.contains(blue) {
                    // blue_count += n.unwrap();
                    if n.unwrap() > max_blue {
                        possible = false;
                        break;
                    }
                } else if color.contains(red) {
                    // red_count += n.unwrap();
                    if n.unwrap() > max_red {
                        possible = false;
                        break;
                    }
                } else if color.contains(green) {
                    // green_count += n.unwrap();
                    if n.unwrap() > max_green {
                        possible = false;
                        break;
                    }
                }
            } 
        }
        if !possible {
            continue;
        }
        
        let n = game_id.map(|s| s.parse::<i32>().ok()).flatten();
        println!("Game id is: {}", n.unwrap());
        game_count += n.unwrap();    
        println!("Game count is: {}", game_count)
        // println!("Blue count is: {}", blue_count);
        // println!("Red count is: {}", red_count);
        // println!("Green count is: {}", green_count);
        // if blue_count <= max_blue && red_count <= max_red && green_count <= max_green {
        //     let n = game_id.map(|s| s.parse::<i32>().ok()).flatten();
        //     game_count += n.unwrap();
        // }

    }

    println!("Game count is: {}", game_count);
}


fn read_file(file_path: &str) -> Vec<String> {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    split_string(&contents, '\n')
}

fn split_string(input: &str, split_char: char) -> Vec<String> {
    input.split(split_char).map(|s| s.to_string()).collect()
}

fn extract_between(input: &str, start_char: char, end_char: char) -> Option<&str> {
    let start = input.find(start_char)? + 1; // Find the index of start_char and move past it
    let end = input[start..].find(end_char)? + start; // Find the index of end_char after start_char
    Some(&input[start..end]) // Extract the substring
}

fn extract_end(input: &str, start_char: char) -> Option<&str> {
    let start = input.find(start_char)? + 1; // Find the index of start_char and move past it
    Some(&input[start..]) // Extract the substring
}

fn _extract_start(input: &str, end_char: char) -> Option<&str> {
    let end = input.find(end_char)?; // Find the index of end_char
    Some(&input[..end]) // Extract the substring
}
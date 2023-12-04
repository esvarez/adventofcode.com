use std::fs;

#[allow(dead_code)]
pub fn day1_2() {
    let file_path = "./day1_1.txt";
    println!("In file {}", file_path);

    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    

    // let contents = "dqfournine5four2jmlqcgv
    // 7ggzdnjxndfive
    // twofive4threenine
    // dttwonezbgmcseven5seven
    // 5vsrcnine
    // 95zzbjck";

    let lines = contents.split("\n");
    for line in lines {
        println!("Line is {}", line);
        let mut first_digit = None;
        let mut last_digit = None;
    
        let mut first_digit_index = None;
        let mut last_digit_index = None;

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if first_digit_index.is_none() {
                    first_digit_index = Some(i);
                    first_digit = Some(c);
                }
                last_digit_index = Some(i);
                last_digit = Some(c);
            }
        }

        println!("First digit is {:?} at index {:?}", first_digit, first_digit_index);
        println!("Last digit is {:?} at index {:?}", last_digit, last_digit_index);


        for (i, number) in numbers.iter().enumerate() {
            if let Some(index) = line.find(number) {
                println!("{} found at index {}", number, index);
                if first_digit_index.is_none() || index < first_digit_index.unwrap(){
                    first_digit_index = Some(index);
                    first_digit = std::char::from_u32((i + 49) as u32);
                    println!("First digit is {}", first_digit.unwrap());
                }
            }

            if let Some(index) = line.rfind(number) {
                println!("{} found at index {}", number, index);
                if last_digit_index.is_none() || index > last_digit_index.unwrap() {
                    last_digit_index = Some(index);
                    last_digit = std::char::from_u32((i + 49) as u32);
                    println!("Last digit is {}", last_digit.unwrap());
                }
            }
        }



        let s = match (first_digit, last_digit) {
            (Some(first), Some(last)) => format!("{}{}", first, last),
            _ => String::new(), // Return empty string if either first or last digit is not found
        };

        match s.parse::<i32>() {
            Ok(n) => {
                println!("{} is a number!", s);
                sum += n;
            },
            Err(_) => println!("{} is not a number!", s),
        }
        println!("NEXT");
    }

    println!("Sum is {}", sum);
}

#[allow(dead_code)]
pub fn day1_1() {

    let file_path = "./day1_1.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    let lines = contents.split("\n");
    for line in lines {
        let first_digit = line.chars().find(|&c| c.is_digit(10));
        let last_digit = line.chars().rev().find(|&c| c.is_digit(10));

        let s = match (first_digit, last_digit) {
            (Some(first), Some(last)) => format!("{}{}", first, last),
            _ => String::new(), // Return empty string if either first or last digit is not found
        };

        match s.parse::<i32>() {
            Ok(n) => {
                println!("{} is a number!", s);
                sum += n;
                println!("Sum is {}", sum);
            },
            Err(_) => println!("{} is not a number!", s),
        }
    }

    println!("Sum is {}", sum);
}
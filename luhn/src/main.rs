fn validation_a_number(_num_str: String) -> String {
    let num_str = _num_str.replace(" ", "");
    if num_str.len() <= 1 {
        return "not a valid string".to_string();
    }

    let mut sum = 0;
    let check_even_index = num_str.len() % 2 == 0;

    for (i, ch) in num_str.chars().enumerate() {
        if !ch.is_numeric() {
            return "not a valid string".to_string();
        };
        let mut new_num;
        if (i % 2 == 0) == check_even_index {
            new_num = ch.to_digit(10).unwrap() * 2;
            if new_num > 9 {
                new_num -= 9;
            }
            sum = sum+new_num;
        } else {
            sum = sum + ch.to_digit(10).unwrap() as u32;
        }
    }

    if sum % 10 == 0 {
        format!("{sum} is evenly divisible by 10, so this number is valid")
    } else {
        format!("{sum} is not evenly divisible by 10, so this number is not valid")
    }
    
}

fn main() {
    let valid_card = "4539 3195 0343 6467".to_string();
    // let invalid_card = "8173 1232 7352 0569".to_string();
    let result = validation_a_number(valid_card);
    // let result = validation_a_number(invalid_card);
    println!("{result}");
}

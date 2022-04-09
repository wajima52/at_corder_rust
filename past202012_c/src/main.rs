use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result = base_conversion(input.trim().parse::<u32>().unwrap(), 36);
    match result {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}


/*
進法に用いる文字列をvec型で返す。
base_number: X進法のXの数値
 */
fn get_strings_to_positional_notation(base_number: &u8) -> Result<Vec<char>, &'static str> {
    if base_number < &2 {
        Err("Too Small Number")
    } else if base_number <= &10 {
        Ok((0..*(base_number)).map(|num| (num + b'0') as char).collect::<Vec<_>>())
    } else if base_number <= &36{
        let mut numbers: Vec<char> = (0..10).map(|num| (num + b'0') as char).collect::<Vec<_>>();
        let mut alphabets: Vec<char> = (0..(base_number - 10)).map(|num| (num + b'A') as char).collect::<Vec<_>>();
        numbers.append(&mut alphabets);
        Ok(numbers)
    } else {
        Err("Too Large Number")
    }
}

fn base_conversion(number: u32, base_number: u8) -> Result<String, &'static str> {
    let mut number = number;
    let notation = get_strings_to_positional_notation(&base_number);
    if notation.is_err() {
        return Err("Invalid base_number");
    }
    let positional_notation = notation.unwrap();
    let mut converted_string: String = String::new();
    loop {
        if number < base_number.into() {
            converted_string = positional_notation[(number) as usize].to_string() + &converted_string;
            return Ok(converted_string);
        } else {
            converted_string = positional_notation[(number % u32::from(base_number)) as usize].to_string() + &converted_string;
            number = number / u32::from(base_number);
        }
    }
}

#[test]
fn get_strings_to_positional_notation_test() {
    // 0,1進法はエラーを返す
    assert_eq!(get_strings_to_positional_notation(&0), Err("Too Small Number"));
    assert_eq!(get_strings_to_positional_notation(&1), Err("Too Small Number"));
    // 2進法 ~ 36進法の文字列が正しいかをテスト
    assert_eq!(get_strings_to_positional_notation(&2), Ok(vec!['0', '1']));
    assert_eq!(get_strings_to_positional_notation(&10), Ok(vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']));
    assert_eq!(get_strings_to_positional_notation(&16), Ok(vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F']));
    assert_eq!(get_strings_to_positional_notation(&36), Ok(vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z']));
    // 37進法はエラ＝を返す
    assert_eq!(get_strings_to_positional_notation(&37), Err("Too Large Number"));
}

#[test]
fn base_conversion_test() {
    assert_eq!(base_conversion(10, 16), Ok(String::from("A")));
    assert_eq!(base_conversion(17, 16), Ok(String::from("11")));
    assert_eq!(base_conversion(123, 36), Ok(String::from("3F")));
    assert_eq!(base_conversion(2304, 36), Ok(String::from("1S0")));
}





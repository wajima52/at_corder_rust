use std::io;

#[derive(PartialEq)]
#[derive(Debug)]
enum Mark {
    circle, cross
}



fn determine_mark(mark: &str) -> Result<Mark, &'static str> {
    if mark == String::from("○") {
        Ok(Mark::circle)
    } else if mark == "×"{
        Ok(Mark::cross)
    } else {
        Err("Invalid Input")
    }
}

#[test]
fn determine_mark_test() {
    assert_eq!(determine_mark("○"), Ok(Mark::circle));
    assert_eq!(determine_mark("×"), Ok(Mark::cross));
    assert_eq!(determine_mark("a"), Err("Invalid Input"));
    assert_eq!(determine_mark(""), Err("Invalid Input"));
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mark = determine_mark(input.trim());
    if mark.is_ok() {
        println!("Valid Mark")
    } else {
        println!("Invalid Input")
    }
}


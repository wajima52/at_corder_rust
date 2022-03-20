use std::io;

// enum Mark {
//     circle, cross
// }



fn determine_mark(mark: &str) -> String {
    if mark == String::from("○") {
        String::from("○")
    } else if mark == "×"{
        String::from("×")
    } else {
        String::new()
    }
}

#[test]
fn determine_mark_test() {
    assert_eq!(determine_mark("○"), "○");
    assert_eq!(determine_mark("×"), "×");
    assert_eq!(determine_mark("a"), "");
    assert_eq!(determine_mark(""), "");
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mark = determine_mark(input.trim());
    if mark == "" {
        println!("Invalid Input")
    } else {
        println!("{}", mark)
    }
}


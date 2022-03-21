use std::io;

#[derive(PartialEq)]
#[derive(Debug)]
enum Mark {
    Circle,
    Cross,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct MarkCount {
    mark: Mark,
    count: i32,
}

fn determine_mark(mark: &str) -> Result<Mark, &'static str> {
    if mark == String::from("○") {
        Ok(Mark::Circle)
    } else if mark == "×" {
        Ok(Mark::Cross)
    } else {
        Err("Invalid Input")
    }
}

fn calculate_mark_count(mark_count: MarkCount, mark: Mark) -> MarkCount {
    if mark_count.mark == mark {
        MarkCount { mark: mark_count.mark, count: mark_count.count + 1 }
    } else {
        MarkCount { mark: mark, count: 1 }
    }
}

fn count_continuous_mark(input: String) -> Result<MarkCount, &'static str> {
    let mut mark_count: MarkCount = MarkCount { mark: Mark::Circle, count: 0 };
    for char in input.as_str().chars() {
        let mark = determine_mark(&char.to_string());
        match mark {
            Ok(mark) => mark_count = calculate_mark_count(mark_count, mark),
            Err(str) => {
                return Err(str);
            }
        }
    }
    Ok(mark_count)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    for i in input.as_str().chars() {
        println!("{}", i.to_string())
    }
    let mark = determine_mark(input.trim());
    if mark.is_ok() {
        println!("Valid Mark")
    } else {
        println!("Invalid Input")
    }
}

#[test]
fn determine_mark_test() {
    assert_eq!(determine_mark("○"), Ok(Mark::Circle));
    assert_eq!(determine_mark("×"), Ok(Mark::Cross));
    assert_eq!(determine_mark("a"), Err("Invalid Input"));
    assert_eq!(determine_mark(""), Err("Invalid Input"));
}

#[test]
fn calculate_mark_count_test() {
    assert_eq!(
        calculate_mark_count(
            MarkCount { mark: Mark::Circle, count: 2 },
            Mark::Circle
        ),
        MarkCount { mark: Mark::Circle, count: 3 }
    );
    assert_eq!(
        calculate_mark_count(
            MarkCount { mark: Mark::Cross, count: 2 },
            Mark::Cross
        ),
        MarkCount { mark: Mark::Cross, count: 3 }
    );
    assert_eq!(
        calculate_mark_count(
            MarkCount { mark: Mark::Cross, count: 2 },
            Mark::Circle
        ),
        MarkCount { mark: Mark::Circle, count: 1 }
    );
}

#[test]
fn count_continuous_mark_test() {
    assert_eq!(count_continuous_mark(String::from("○○○○")), Ok(MarkCount { mark: Mark::Circle, count: 4 }));
    assert_eq!(count_continuous_mark(String::from("××××")), Ok(MarkCount { mark: Mark::Cross, count: 4 }));
    assert_eq!(count_continuous_mark(String::from("×○××")), Ok(MarkCount { mark: Mark::Cross, count: 2 }));
    assert_eq!(count_continuous_mark(String::from("×○a○")), Err("Invalid Input"));
}
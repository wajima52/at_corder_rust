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
    if mark == String::from("o") {
        Ok(Mark::Circle)
    } else if mark == "x" {
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

fn get_continuous_mark(input: String) -> Result<Option<Mark>, &'static str> {
    if input.len() >= 6 {
        return Err("Too Many Marks")
    }
    let mut mark_count: MarkCount = MarkCount { mark: Mark::Circle, count: 0 };
    for char in input.as_str().chars() {
        let mark = determine_mark(&char.to_string());
        match mark {
            Ok(mark) => {
                mark_count = calculate_mark_count(mark_count, mark);
                if mark_count.count >= 3 {
                    return Ok(Some(mark_count.mark))
                }
            },
            Err(str) => {
                return Err(str);
            }
        }
    }
    Ok(None)
}

fn get_game_result(input: String) -> Result<&'static str, &'static str> {
    match get_continuous_mark(input.trim().to_string()) {
        Ok(Some(Mark::Circle)) => Ok("o"),
        Ok(Some(Mark::Cross)) => Ok("x"),
        Ok(None) => Ok("draw"),
        Err(str) => Err(str)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result = get_game_result(input);
    match result {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}

#[test]
fn determine_mark_test() {
    assert_eq!(determine_mark("o"), Ok(Mark::Circle));
    assert_eq!(determine_mark("x"), Ok(Mark::Cross));
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
    assert_eq!(get_continuous_mark(String::from("oooo")), Ok(Some(Mark::Circle)));
    assert_eq!(get_continuous_mark(String::from("xxxx")), Ok(Some(Mark::Cross)));
    assert_eq!(get_continuous_mark(String::from("xoxx")), Ok(None));
    assert_eq!(get_continuous_mark(String::from("xoao")), Err("Invalid Input"));
}

#[test]
fn get_game_result_test() {
    assert_eq!(get_game_result(String::from("xooox")), Ok("o"));
    assert_eq!(get_game_result(String::from("xxxxx")), Ok("x"));
    assert_eq!(get_game_result(String::from("xoxxo")), Ok("draw"));
    assert_eq!(get_game_result(String::from("xoxxox")), Err("Too Many Marks"));
    assert_eq!(get_game_result(String::from("xoxxy")), Err("Invalid Input"));
}
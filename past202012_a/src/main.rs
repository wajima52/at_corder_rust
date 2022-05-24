use std::io;

const MARK_CONTINUOUS_COUNT: i32 = 3;
const INPUT_MARK_COUNT: usize = 5;

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

/*
連続するマークの数を数える関数
mark_count: 既存のマークのカウント
mark: 新規のマーク
 */
fn calculate_mark_count(mark_count: MarkCount, mark: Mark) -> MarkCount {
    if mark_count.mark == mark {
        MarkCount { mark: mark_count.mark, count: mark_count.count + 1 }
    } else {
        MarkCount { mark: mark, count: 1 }
    }
}

/*
連続するマークの数を数える関数
mark_count: 既存のマークのカウント
mark: 新規のマーク
 */
fn get_continuous_mark(input: String) -> Result<Option<Mark>, &'static str> {
    if input.len() > INPUT_MARK_COUNT  {
        return Err("Too Many Marks")
    }
    let mut mark_count: MarkCount = MarkCount { mark: Mark::Circle, count: 0 };
    for char in input.as_str().chars() {
        let mark = determine_mark(&char.to_string());
        match mark {
            Ok(mark) => {
                mark_count = calculate_mark_count(mark_count, mark);
                if mark_count.count >= MARK_CONTINUOUS_COUNT {
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

/*
ゲームの勝ち負けを取得する関数
input: 入力値
 */
fn get_game_result(input: String) -> Result<&'static str, &'static str> {
    match get_continuous_mark(input.trim().to_string()) {
        Ok(Some(Mark::Circle)) => Ok("o"),
        Ok(Some(Mark::Cross)) => Ok("x"),
        Ok(None) => Ok("draw"),
        Err(str) => Err(str)
    }
}

#[test]
fn determine_mark_test() {
    // oはCircleと判別
    assert_eq!(determine_mark("o"), Ok(Mark::Circle));
    // xはCrossと判別
    assert_eq!(determine_mark("x"), Ok(Mark::Cross));
    // o,x以外はInvalidと判別
    assert_eq!(determine_mark("a"), Err("Invalid Input"));
    assert_eq!(determine_mark(""), Err("Invalid Input"));
}

#[test] //マークを数える関数のテスト
fn calculate_mark_count_test() {
    // circleが２回の状態でcircleが出た時にcircleが３回になること
    assert_eq!(
        calculate_mark_count(
            MarkCount { mark: Mark::Circle, count: 2 },
            Mark::Circle
        ),
        MarkCount { mark: Mark::Circle, count: 3 }
    );
    // crossが２回の状態でcrossが出た時にcrossが3回になること
    assert_eq!(
        calculate_mark_count(
            MarkCount { mark: Mark::Cross, count: 2 },
            Mark::Cross
        ),
        MarkCount { mark: Mark::Cross, count: 3 }
    );
    // crossが２回の状態でcircleが出た時にcircleが1回になること
    assert_eq!(
        calculate_mark_count(
            MarkCount { mark: Mark::Cross, count: 2 },
            Mark::Circle
        ),
        MarkCount { mark: Mark::Circle, count: 1 }
    );
}

#[test] // 連続でMARK_CONTINUOUS_COUNTの回数の連続したマークを返す。なければNoneを返すテスト
fn count_continuous_mark_test() {
    assert_eq!(get_continuous_mark(String::from("oooo")), Ok(Some(Mark::Circle)));
    assert_eq!(get_continuous_mark(String::from("xxxx")), Ok(Some(Mark::Cross)));
    assert_eq!(get_continuous_mark(String::from("xoxx")), Ok(None));
    assert_eq!(get_continuous_mark(String::from("xoao")), Err("Invalid Input"));
}

#[test] // 5つのマークの各inputに対してゲームの結果を返す関数をテスト。
fn get_game_result_test() {
    assert_eq!(get_game_result(String::from("xooox")), Ok("o"));
    assert_eq!(get_game_result(String::from("xxxxx")), Ok("x"));
    assert_eq!(get_game_result(String::from("xoxxo")), Ok("draw"));
    assert_eq!(get_game_result(String::from("xoxxox")), Err("Too Many Marks"));
    assert_eq!(get_game_result(String::from("xoxxy")), Err("Invalid Input"));
}
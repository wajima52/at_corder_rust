use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let initial_input = input.clone();
    for char in initial_input.chars() {
        input = add_character_after_remove_same(input, &char.to_string());
    }
    print!("{}", input);
}


fn add_character_after_remove_same<'a> (string: String, added_char: &str) -> String {
    let string = string.replace(added_char, "");
    [string, added_char.to_string()].concat()
}

#[test]
fn add_character_after_remove_same_test() {
    assert_eq!(add_character_after_remove_same(String::from("abc"), "a"), "bca");
    assert_eq!(add_character_after_remove_same(String::from("abbbc"), "b"), "acb");
}

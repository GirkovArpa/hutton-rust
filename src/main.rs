fn main() {
    let input = String::from("foo");
    let output = swap_chars(&input, 0, 2);
    println!("{}", output);

    let input = String::from("foo");
    let output = rotate_string(&input);
    println!("{}", output);
}

fn rotate_string(input: &str) -> String {
    let first = input.chars().nth(0).unwrap();
    let rest = remove_first(&input);
    rest + &first.to_string()
}

fn remove_first(input: &str) -> String {
    let mut output = input.chars();
    output.next();
    String::from(output.as_str())
}

fn swap_chars(input: &str, i: usize, j: usize) -> String {
    let a = input.chars().nth(i).unwrap();
    let b = input.chars().nth(j).unwrap();
    input
        .chars()
        .enumerate()
        .map(|(idx, c)| match idx {
            idx if idx == i => b,
            idx if idx == j => a,
            _ => c,
        })
        .collect()
}

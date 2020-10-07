use std::collections::HashSet; // for removing dupe chars from string

fn main() {
    let input = String::from("foo");
    let output = swap_chars(&input, 0, 2);
    println!("{}", output);

    let input = String::from("foo");
    let output = rotate_string(&input);
    println!("{}", output);

    let input = String::from("foo");
    let abc = String::from("abcdefghijklmnopqrstuvwxyz");
    let output = create_key(&input, &abc);
    println!("{}", output);

    let input = String::from("helloworld");
    let password = String::from("foo");
    let key = String::from("bar");
    let output = encrypt(&input, &password, &key);
    println!("{}", output);
}

fn encrypt(input: &str, password: &str, key: &str) -> String {
    let abc = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut alphabet_key = create_key(&key, &abc);
    let mut output = String::new();
    for (i, c) in input.iter().enumerate() {

    }
    abc
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

fn create_key(input: &str, abc: &str) -> String {
    let mut a = input.chars().collect::<Vec<char>>();
    let mut b = abc.chars().collect::<Vec<char>>();
    a.append(&mut b);

    let mut uniques = HashSet::new();
    a.retain(|e| uniques.insert(e.clone()));

    a.into_iter().collect()
}

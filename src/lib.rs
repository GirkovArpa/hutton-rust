use std::collections::HashSet; // for removing dupe chars from string

pub fn encrypt(input: &str, password: &str, key: &str, decrypt: bool) -> String {
    let abc = String::from("abcdefghijklmnopqrstuvwxyz");
    let mut alphabet_key = create_key(&key, &abc);
    let mut output = String::new();

    let mut password = String::from(password);

    for (_, input_char) in input.char_indices() {
        let password_char = password.chars().nth(0).unwrap();
        let password_number = abc.find(password_char).unwrap();
        let alphabet_key_char = alphabet_key.chars().nth(0).unwrap();
        let alphabet_key_number = abc.find(alphabet_key_char).unwrap();
        let mut shift: i64 = (password_number + alphabet_key_number + 2) as i64;

        if decrypt {
            shift = -shift;
        }

        let input_char_number = alphabet_key.find(input_char).unwrap();
        let output_char_number = (shift + (input_char_number as i64)).rem_euclid(26);
        let output_char = alphabet_key
            .chars()
            .nth(output_char_number as usize)
            .unwrap();
        let input_char_number_b = alphabet_key.find(input_char).unwrap();

        password = rotate_string(&password);
        output += &output_char.to_string();
        alphabet_key = swap_chars(
            &alphabet_key,
            input_char_number_b,
            output_char_number as usize,
        );
    }
    output
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

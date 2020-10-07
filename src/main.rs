fn main() {
    let input: &str = "foo";
    let output = swap_chars(input, 0, 2);
    println!("{}", output);
}

fn swap_chars(input: &str, i: usize, j: usize) -> String {
    let a = input.chars().nth(i).unwrap();
    let b = input.chars().nth(j).unwrap();
    input.chars()
        .enumerate()
        .map(|(idx, c)| 
            match idx {
                idx if idx == i => b,
                idx if idx == j => a,
                _ => c
            }
        )
        .collect()
}

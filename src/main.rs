fn main() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    let result = swap_chars(pangram, 0, 1);
    println!("{}", result);
}

fn swap_chars(word: &str, i: usize, j: usize) -> String {
    let a = word.chars().nth(i).unwrap();
    let b = word.chars().nth(j).unwrap();
    word.chars()
        .enumerate()
        .map(|(idx, c)| {
            if idx == i {
                b
            } else if idx == j {
                a
            } else {
                c
            }
        })
        .collect()
}

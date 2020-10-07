# The Hutton Cipher

The Hutton Cipher of [Eric Bond Hutton](https://old.reddit.com/EricBondHutton), implemented in Rust (as a crate).

## Usage Example

```rust
extern crate hutton_rust;

fn main() {
  // the following 3 values must all consist of lowercase letters in the range [a-z]
  // else, panicc!
  let input = String::from("helloworld");
  let password = String::from("foo");
  let key = String::from("bar");
  // the last boolean argument is whether to decrypt instead of encrypt
  let output = encrypt(&input, &password, &key, false);
  println!("{}", output); // => pwckfenttc
}
```
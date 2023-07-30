use derive_builder::Builder;

#[derive(Builder)]
struct Command {
    executable: String,
}

fn main() {
    println!("hello world");
}

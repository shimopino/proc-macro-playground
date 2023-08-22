use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field<T> {
    value: T,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

// pub struct Fielded<T> {
//     value: T,
//     bitmask: u8,
// }

// impl<T: std::fmt::Debug> std::fmt::Debug for Fielded<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Fielded")
//             .field("value", &self.value)
//             .field("bitmask", &self.bitmask)
//             .finish()
//     }
// }

fn main() {
    let f = Field {
        value: "F",
        bitmask: 0b00011100,
    };

    let debug = format!("{:?}", f);
    let expected = r#"Field { value: "F", bitmask: 0b00011100 }"#;

    assert_eq!(debug, expected);
}

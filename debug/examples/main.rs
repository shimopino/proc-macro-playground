use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

// impl std::fmt::Debug for Field {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Field")
//             .field("name", &self.name)
//             .field("bitmask", &format_args!("{:08b}", &self.bitmask))
//             .finish()
//     }
// }

fn main() {
    let f = Field {
        name: "F",
        bitmask: 0b00011100,
    };

    println!("{:?}", f);
}

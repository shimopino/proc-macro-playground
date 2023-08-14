use derive_builder::Builder;

// #[derive(Builder)]
// pub struct Command {
//     executable: String,
//     #[builder(each = "arg")]
//     args: Vec<String>,
//     #[builder(each = "env")]
//     env: Vec<String>,
//     current_dir: Option<String>,
// }

// fn main() {
//     let command = Command::builder()
//         .executable("cargo".to_owned())
//         .arg("build".to_owned())
//         .arg("--release".to_owned())
//         // .env("development".to_string())
//         .build()
//         .unwrap();

//     assert_eq!(command.executable, "cargo");
//     assert_eq!(command.args, vec!["build", "--release"]);
//     let expected: Vec<String> = Vec::new();
//     assert_eq!(command.env, expected);
//     assert_eq!(command.current_dir, None);
// }

type Option = ();
type Some = ();
type None = ();
type Result = ();
type Box = ();

#[derive(Builder)]
pub struct Command {
    executable: String,
}

fn main() {}

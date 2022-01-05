mod print; // print file
mod strings;
mod types;
mod vars; // variables file

fn main() {
    println!("Hello, world!"); // from here
    print::run(); // calling run from print
    vars::run(); // calling var files
    types::run();
    strings::run();
}

mod print; // print file
mod types;
mod vars; // variables file // type file

fn main() {
    println!("Hello, world!"); // from here
    print::run(); // calling run from print
    vars::run(); // calling var files
    types::run();
}

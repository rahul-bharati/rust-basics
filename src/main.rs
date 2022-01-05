mod print; // print file
mod vars; // variables file

fn main() {
    println!("Hello, world!"); // from here
    print::run(); // calling run from print
    vars::run(); // calling var files
}

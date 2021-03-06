mod arrays;
mod conditional;
mod loops;
mod print; // print file
mod strings;
mod tuples;
mod types;
mod vars; // variables file
mod vectors;

fn main() {
    println!("Hello, world!"); // from here
    print::run(); // calling run from print
    vars::run(); // calling var files
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditional::run();
    loops::run();
}

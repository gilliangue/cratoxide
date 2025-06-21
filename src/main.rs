// import cli crate
use cratoxide::cli::parser::parse_args; 

fn main() {
    let args = parse_args();
    println!("Parsed arguments: {:?}", args);

    // Here you can use the parsed arguments

}

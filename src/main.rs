use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("No JSON entered to parse.")
    }

    let json_string: &String = &args[1].split_whitespace().collect();
    dbg!(json_string);
    ()
}

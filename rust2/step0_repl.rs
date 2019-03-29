use std::io;

fn main() {
    loop {
        let done = rep();
        if done {
            return;
        }
    }
}

fn read() -> (String, usize) {
    let mut input = String::new();
    let bytes = io::stdin().read_line(&mut input).expect("Something went wrong while trying to read input");
    (input, bytes)
}

fn eval(input: &str) -> String {
    String::from(input)
}

fn print(input: &str) {
    println!("{}", input);
}

fn rep() -> bool {
    println!("user> ");
    let (input, bytes) = read();
    if bytes == 0 {
        return true;
    }
    let result = eval(&input);
    print(&result);
    false
}
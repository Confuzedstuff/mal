use std::io;

enum StopReason {
    EOF
}

enum REPLState {
    Running,
    Stopping(StopReason),
}

fn main() {
    loop {
        match rep() {
            REPLState::Stopping(_) => return,
            _ => (),
        }
    }
}

fn read() -> (String, usize) {
    let mut input = String::new();
    let n_bytes = io::stdin().read_line(&mut input).expect("Something went wrong while trying to read input");
    (input, n_bytes)
}

fn eval(input: &str) -> String {
    String::from(input)
}

fn print(input: &str) {
    println!("{}", input);
}

fn rep() -> REPLState {
    println!("user> ");
    let (input, n_bytes) = read();
    if n_bytes == 0 {
        return REPLState::Stopping(StopReason::EOF);
    }
    let result = eval(&input);
    print(&result);
    REPLState::Running
}

use std::env;

#[derive(Debug)]
enum State {
    S1,
    S2,
    S3,
    S4,
    S5,
    Err
}

impl State {

    fn next(self, sym: char) -> State {
        match self {
            State::S1 => match sym {
                'a' => State::S2,
                'c' => State::S3,
                _ => State::Err
            }, 
            State::S2 => match sym {
                'b' => State::S4,
                _ => State::Err
            },
            State::S3 => match sym {
                'a' => State::S4,
                'b' => State::S5,
                _ => State::Err
            },
            State::S4 => match sym {
                _ => State::Err
            },
            State::S5 => match sym {
                'b' => State::S5,
                _ => State::Err
            },
            State::Err => State::Err
        }
    }

    fn resolved(self) -> bool {
        match self {
            State::S1 => true, 
            State::S3 => true,
            State::S4 => true,
            State::S5 => true,
            _ => false
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of args!");
    }
    let mut state = State::S1;
    for sym in args[1].chars() { 
        state = state.next(sym);
    }
    if state.resolved() {
        println!("Bingo!");
    } else {
        println!("No!");
    }
}

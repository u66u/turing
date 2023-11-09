use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::fmt;
use std::error::Error;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum State {
    A,
    B,
    C,
    Halt,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Symbol {
    Zero,
    One,
    Blank,
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

struct Rule {
    write: Symbol,
    move_: Move,
    next_state: State,
}

type Rules = HashMap<(State, Symbol), Rule>;


struct TuringMachine {
    state: State,
    tape: VecDeque<Symbol>,
    position: usize,
    rules: Rules,
}

impl TuringMachine {
    fn new(rules: Rules, initial_state: State, initial_tape: Vec<Symbol>) -> Self {
        let mut tape = VecDeque::new();
        for symbol in initial_tape {
            tape.push_back(symbol);
        }
        Self {
            state: initial_state,
            tape,
            position: 0,
            rules,
        }
    }

    fn step(&mut self) {
        let symbol = self.tape[self.position];
        match self.rules.get(&(self.state, symbol)) {
            Some(rule) => {
                self.tape[self.position] = rule.write;
                self.state = rule.next_state;
                match rule.move_ {
                    Move::Left => {
                        if self.position == 0 {
                            self.tape.push_front(Symbol::Blank);
                        } else {
                            self.position -= 1;
                        }
                    }
                    Move::Right => {
                        self.position += 1;
                        if self.position == self.tape.len() {
                            self.tape.push_back(Symbol::Blank);
                        }
                    }
                }
            },
            None => {
                // Default behavior when rule does not exist
                self.state = State::Halt;
            }
        }
        println!("Current state: {:?}, Current symbol: {:?}", self.state, self.tape[self.position]); // print the current state and symbol
    }
    
    

    fn run(&mut self) {
        while self.state != State::Halt {
            self.step();
        }
        println!("The Turing machine has halted."); // print when the Turing machine has halted
    }
    
}

#[derive(Debug)]
struct ParseEnumError;

impl fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse the enum")
    }
}

impl Error for ParseEnumError {}

impl FromStr for State {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(State::A),
            "B" => Ok(State::B),
            "C" => Ok(State::C),
            "Halt" => Ok(State::Halt),
            _ => Err(ParseEnumError),
        }
    }
}

impl FromStr for Symbol {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Symbol::Zero),
            "1" => Ok(Symbol::One),
            "Blank" => Ok(Symbol::Blank),
            _ => Err(ParseEnumError),
        }
    }
}

impl FromStr for Move {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Move::Left),
            "R" => Ok(Move::Right),
            _ => Err(ParseEnumError),
        }
    }
}



fn main() {
    let mut rules = Rules::new();

    // Read from user input or a file
    let input = io::stdin();
    let file = File::open("rules.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 5 {
            continue;
        }

        let current_state = State::from_str(parts[0].trim()).unwrap();
        let current_symbol = Symbol::from_str(parts[1].trim()).unwrap();
        let write_symbol = Symbol::from_str(parts[2].trim()).unwrap();
        let move_direction = Move::from_str(parts[3].trim()).unwrap();
        let next_state = State::from_str(parts[4].trim()).unwrap();

        rules.insert(
            (current_state, current_symbol),
            Rule {
                write: write_symbol,
                move_: move_direction,
                next_state: next_state,
            },
        );
    }

    // Print the rule table
    for ((state, symbol), rule) in &rules {
        println!("State: {:?}, Symbol: {:?} => Write: {:?}, Move: {:?}, Next State: {:?}", state, symbol, rule.write, rule.move_, rule.next_state);
    }

    // Initialize the Turing machine with an initial state and tape
    let initial_tape = vec![Symbol::Zero, Symbol::One, Symbol::Zero, Symbol::Zero];
    let mut machine = TuringMachine::new(rules, State::A, initial_tape);

    // Run the Turing machine
    machine.run();

    
}

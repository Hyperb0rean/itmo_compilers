use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use std::env;

// Определяем типы для символов грамматики
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Symbol {
    NonTerminal(char),
    Terminal(char),
    Epsilon,
    EndMarker,
}

// Описание правил грамматики
#[derive(Debug)]
struct Grammar {
    rules: HashMap<Symbol, Vec<Vec<Symbol>>>,
    first: HashMap<Symbol, HashSet<Symbol>>,
    follow: HashMap<Symbol, HashSet<Symbol>>,
    table: HashMap<(Symbol, Symbol), Vec<Symbol>>,
}

impl Grammar {
    fn new() -> Grammar {
        let mut grammar = Grammar {
            rules: HashMap::new(),
            first: HashMap::new(),
            follow: HashMap::new(),
            table: HashMap::new(),
        };


        /*
        S → ACAbb
        A → bA'
        D → CbBD | ε
        B → ε | b
        C → bbE | c
        E→ aAA | bbbbb
        */
        grammar.rules.insert(
            Symbol::NonTerminal('S'),
            vec![vec![Symbol::NonTerminal('A'), Symbol::NonTerminal('C'), Symbol::NonTerminal('A'), Symbol::NonTerminal('B'), Symbol::NonTerminal('B')]],
        );
        grammar.rules.insert(
            Symbol::NonTerminal('A'),
            vec![vec![Symbol::Terminal('b'), Symbol::NonTerminal('D')]],
        );
        grammar.rules.insert(
            Symbol::NonTerminal('D'),
            vec![vec![Symbol::NonTerminal('C'), Symbol::Terminal('b'), Symbol::NonTerminal('B'), Symbol::NonTerminal('D')], vec![Symbol::Epsilon]],
        );
        grammar.rules.insert(
            Symbol::NonTerminal('B'),
            vec![vec![Symbol::Terminal('b')], vec![Symbol::Epsilon]],
        );
        grammar.rules.insert(
            Symbol::NonTerminal('C'),
            vec![vec![Symbol::Terminal('b'), Symbol::Terminal('b'), Symbol::NonTerminal('E')], vec![Symbol::Terminal('c')]],
        );
        grammar.rules.insert(
            Symbol::NonTerminal('E'),
            vec![vec![Symbol::Terminal('a'), Symbol::NonTerminal('A'), Symbol::NonTerminal('A')], 
            vec![Symbol::Terminal('b'),Symbol::Terminal('b'),Symbol::Terminal('b'),Symbol::Terminal('b'),Symbol::Terminal('b')]],
        );

        grammar.compute_first();
        grammar.compute_follow();
        grammar.create_parsing_table();

        grammar
    }

    fn compute_first(&mut self) {
        for (non_terminal, _) in &self.rules {
            self.first.insert(non_terminal.clone(), HashSet::new());
        }


        self.first.get_mut(&Symbol::NonTerminal('A')).unwrap().insert(Symbol::Terminal('b'));
        self.first.get_mut(&Symbol::NonTerminal('A')).unwrap().insert(Symbol::Epsilon);

  }

    fn compute_follow(&mut self) {
        for (non_terminal, _) in &self.rules {
            self.follow.insert(non_terminal.clone(), HashSet::new());
        }

        self.follow.get_mut(&Symbol::NonTerminal('S')).unwrap().insert(Symbol::EndMarker);

    }

    fn create_parsing_table(&mut self) {

        self.table.insert(
            (Symbol::NonTerminal('S'), Symbol::Terminal('b')),
            vec![Symbol::NonTerminal('A'), Symbol::NonTerminal('C'), Symbol::NonTerminal('A'), Symbol::NonTerminal('B'), Symbol::NonTerminal('B')],
        );

    }

    fn parse(&self, input: &String) -> bool {
        let mut stack: Vec<Symbol> = vec![Symbol::EndMarker, Symbol::NonTerminal('S')]; 
        let mut input_chars: Vec<char> = input.chars().collect(); 
        input_chars.push('$'); 

        let mut idx = 0;

        while let Some(top) = stack.pop() {
            if top == Symbol::EndMarker && input_chars[idx] == '$' {
                return true;
            }

            match top {
                Symbol::Terminal(c) => {
                    if c == input_chars[idx] {
                        idx += 1;
                    } else {
                        return false;
                    }
                }
                Symbol::NonTerminal(_) => {
                    let current_input = Symbol::Terminal(input_chars[idx]);
                    if let Some(rule) = self.table.get(&(top.clone(), current_input)) {
                        for symbol in rule.iter().rev() {
                            stack.push(symbol.clone());
                        }
                    } else {
                        return false;
                    }
                }
                Symbol::Epsilon => {
                    continue;
                }
                _ => {}
            }
        }
        false
    }
}

fn main() {
    let grammar = Grammar::new();
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of args!");
    }

    if grammar.parse(&args[1]) {
        println!("Строка '{}' принята", args[1]);
    } else {
        println!("Строка '{}' отклонена", args[1]);
    }
}

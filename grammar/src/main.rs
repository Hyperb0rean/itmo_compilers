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
        S -> ACAbb
        A -> bD
        D -> CbBD | ε
        B -> ε | b
        C -> bbE | c
        E -> aAA | bbbbb
        */

        grammar.rules.insert(
            Symbol::NonTerminal('S'),
            vec![vec![Symbol::NonTerminal('A'), Symbol::NonTerminal('C'), Symbol::NonTerminal('A'), Symbol::Terminal('b'), Symbol::Terminal('b')]],
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


        self.first.get_mut(&Symbol::NonTerminal('S')).unwrap().insert(Symbol::Terminal('b'));

        self.first.get_mut(&Symbol::NonTerminal('A')).unwrap().insert(Symbol::Terminal('b'));
        self.first.get_mut(&Symbol::NonTerminal('A')).unwrap().insert(Symbol::Terminal('c'));

        self.first.get_mut(&Symbol::NonTerminal('D')).unwrap().insert(Symbol::Terminal('b'));
        self.first.get_mut(&Symbol::NonTerminal('D')).unwrap().insert(Symbol::Terminal('c'));
        self.first.get_mut(&Symbol::NonTerminal('D')).unwrap().insert(Symbol::Epsilon);

        self.first.get_mut(&Symbol::NonTerminal('B')).unwrap().insert(Symbol::Terminal('b'));
        self.first.get_mut(&Symbol::NonTerminal('B')).unwrap().insert(Symbol::Epsilon);

        self.first.get_mut(&Symbol::NonTerminal('C')).unwrap().insert(Symbol::Terminal('b'));
        self.first.get_mut(&Symbol::NonTerminal('C')).unwrap().insert(Symbol::Terminal('c'));

        self.first.get_mut(&Symbol::NonTerminal('E')).unwrap().insert(Symbol::Terminal('a'));
        self.first.get_mut(&Symbol::NonTerminal('E')).unwrap().insert(Symbol::Terminal('b'));
    

  }

    fn compute_follow(&mut self) {
        for (non_terminal, _) in &self.rules {
            self.follow.insert(non_terminal.clone(), HashSet::new());
        }

        self.follow.get_mut(&Symbol::NonTerminal('S')).unwrap().insert(Symbol::EndMarker);

        self.follow.get_mut(&Symbol::NonTerminal('A')).unwrap().insert(Symbol::Terminal('b'));
        self.follow.get_mut(&Symbol::NonTerminal('A')).unwrap().insert(Symbol::Terminal('c'));

        self.follow.get_mut(&Symbol::NonTerminal('D')).unwrap().insert(Symbol::Terminal('b'));
        self.follow.get_mut(&Symbol::NonTerminal('D')).unwrap().insert(Symbol::Terminal('c'));

        self.follow.get_mut(&Symbol::NonTerminal('B')).unwrap().insert(Symbol::Terminal('b'));
        self.follow.get_mut(&Symbol::NonTerminal('B')).unwrap().insert(Symbol::Terminal('c'));
        self.follow.get_mut(&Symbol::NonTerminal('B')).unwrap().insert(Symbol::Epsilon);

        self.follow.get_mut(&Symbol::NonTerminal('C')).unwrap().insert(Symbol::Terminal('b'));

        self.follow.get_mut(&Symbol::NonTerminal('E')).unwrap().insert(Symbol::Terminal('b'));
    }

    fn create_parsing_table(&mut self) {

        for (non_terminal, productions) in &self.rules {
            for production in productions {
                let first_set = self.get_first_of_production(production);

                for terminal in &first_set {
                    if *terminal != Symbol::Epsilon {
                        self.table.insert((non_terminal.clone(), terminal.clone()), production.clone());
                    }
                }

                if first_set.contains(&Symbol::Epsilon) {
                    if let Some(follow_set) = self.follow.get(non_terminal) {
                        for terminal in follow_set {
                            self.table.insert((non_terminal.clone(), terminal.clone()), production.clone());
                        }
                    }
                }
            }
        }
    }

    fn get_first_of_production(&self, production: &Vec<Symbol>) -> HashSet<Symbol> {
        let mut result = HashSet::new();

        for symbol in production {
            match symbol {
                Symbol::Terminal(_) => {
                    result.insert(symbol.clone());
                    break;
                }
                Symbol::NonTerminal(_) => {
                    if let Some(first_set) = self.first.get(symbol) {
                        result = &result | first_set;

                        if !first_set.contains(&Symbol::Epsilon) {
                            break;
                        }
                    }
                }
                Symbol::Epsilon => {
                    result.insert(Symbol::Epsilon);
                    break;
                }
                _ => {}
            }
        }

        result
    }


    fn parse(&self, input: &String) -> bool {
        let mut stack: Vec<Symbol> = vec![Symbol::EndMarker, Symbol::NonTerminal('S')]; 
        let mut input_chars: Vec<char> = input.chars().collect(); 
        input_chars.push('$'); 

        let mut idx = 0;

        while let Some(top) = stack.pop() {
            println!("{:?} ", top);
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
        println!("String '{}' accepted", args[1]);
    } else {
        println!("String '{}' rejected", args[1]);
    }
}

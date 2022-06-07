#![allow(non_snake_case)]
use std::ops::Deref;
use std::rc::Rc;
use std::collections::HashSet;
use std::str;
use std::io;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct RegEx;

pub enum Regex {
    Empty(),
    Eps(),
    Letter(String),
    Or(Rc<Regex>, Rc<Regex>),
    Concat(Rc<Regex>, Rc<Regex>),
    Star(Rc<Regex>)
}

use crate::Regex::{Empty, Eps, Letter, Or, Concat, Star};

// Finds whether a regular expression is nullable
fn findLambda (regexp: &Rc<Regex>) -> Rc<Regex> {
    match regexp.deref() {
        Empty() => Rc::new(Empty()),
        Eps()   =>  Rc::new(Eps()),
        Letter(_) => Rc::new(Empty()),
        Or(r1, r2) => {
            let s1 = findLambda(r1);
            let s2 = findLambda(r2);
            if matches!(*s1.deref(), Eps()) || matches!(*s2.deref(), Eps()) {
                Rc::new(Eps())
            } else {
                Rc::new(Empty())
            }
        },
        Concat(r1, r2) => {
            let s1 = findLambda(r1);
            let s2 = findLambda(r2);
            if matches!(*s1.deref(), Eps()) && matches!(*s2.deref(), Eps()) {
                Rc::new(Eps())
            } else {
                Rc::new(Empty())
            }
        },
        Star(_)    => Rc::new(Eps())
    }
}

// Finds P set : all the starting letters possible in the language of the regular expression
fn constructP (regexp: &Rc<Regex>) -> HashSet<String> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructP(r1);
            let s2 = constructP(r2);
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<String> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Hello11 : {}", *x);
            }
            // println!("End11");
            hset
        },
        Concat(r1, r2) => {
            let s = findLambda(r1);
            match s.deref() {
                Empty() => {
                    // println!("Go");
                    constructP(r1)
                    
                }
                Eps() => {
                    let s1 = constructP(r1);
                    let s2 = constructP(r2);
                    let union_s: HashSet<_> = s1.union(&s2).collect();
                    let mut hset: HashSet<String> = HashSet::new();
                    for x in union_s {
                        hset.insert(x.clone());
                        // println!("Hello : {}", *x);
                    }
                    // println!("End");
                    hset
                }
                _   => {
                    let hset: HashSet<String> = HashSet::new();
                    hset
                }
            }
        },
        Star(r1) => constructP(r1),
        Letter(x) => {
            let mut hset: HashSet<String> = HashSet::new();
            hset.insert(x.clone());
            hset
        },
        _ => {
            let hset: HashSet<String> = HashSet::new();
            hset
        }

    }
}

// Finds D set : all the terminating letters possible in the language of the regular expression
fn constructD (regexp: &Rc<Regex>) -> HashSet<String> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructD(r1);
            let s2 = constructD(r2);
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<String> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Hello11 : {}", *x);
            }
            // println!("End11");
            hset
        },
        Concat(r1, r2) => {
            let s = findLambda(r2);
            match s.deref() {
                Empty() => {
                    // println!("Go");
                    constructD(r2)
                    
                }
                Eps() => {
                    let s1 = constructD(r1);
                    let s2 = constructD(r2);
                    let union_s: HashSet<_> = s1.union(&s2).collect();
                    let mut hset: HashSet<String> = HashSet::new();
                    for x in union_s {
                        hset.insert(x.clone());
                        // println!("Hello : {}", *x);
                    }
                    // println!("End");
                    hset
                }
                _   => {
                    let hset: HashSet<String> = HashSet::new();
                    hset
                }
            }
        }
        Star(r1) => constructD(r1),
        Letter(x) => {
            let mut hset: HashSet<String> = HashSet::new();
            hset.insert(x.clone());
            hset
        },
        _ => {
            let hset: HashSet<String> = HashSet::new();
            hset
        }

    }
}
// Finds F set : all the letter-pairs possible in the language of the regular expression
fn constructF (regexp: &Rc<Regex>) -> HashSet<(String, String)> {
    match regexp.deref() {
        Or(r1, r2)  => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End or s1");
            let s2 = constructF(r2);
            // for x in &s2 {
            //     println!("in s2 : {:?}", *x);
            // }
            // println!("End or s2");
            let union_s: HashSet<_> = s1.union(&s2).collect();
            let mut hset: HashSet<(String, String)> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Finial union or: {:?}", *x);
            }
            // println!("End10");
            hset
        },
        Concat(r1, r2) => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End concat s1");
            let s2 = constructF(r2);
            // for x in &s2 {
            //     println!("in s2 : {:?}", *x);
            // }
            // println!("End concat s2");
            let mut hset0: HashSet<(String, String)> = HashSet::new();
            let union_helper: HashSet<_> = s1.union(&s2).collect();
            for x in union_helper {
                hset0.insert(x.clone());
                // println!("in union helper concat : {:?}", *x);
            }
            // println!("End union helper concat");
            let hs1 = constructD(r1);
            let hs2 = constructP(r2);
            let mut hset1: HashSet<(String, String)> = HashSet::new();
            for x in hs1 {
                for y in &hs2 {
                    hset1.insert((x.clone(), y.clone()));
                }
            }
            // for x in &hset1 {
            //     println!("in hset1 : {:?}", *x);
            // }
            // println!("End concat hset1");
            let union_s: HashSet<_> = hset0.union(&hset1).collect();
            let mut hset: HashSet<(String, String)> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Hello12 : {:?}", *x);
            }
            // for x in &hset {
            //     // println!("in hset : {:?}", *x);
            // }
            // println!("Final concat hset");
            hset
        }
        Star(r1) => {
            let s1 = constructF(r1);
            // for x in &s1 {
            //     println!("in s1 : {:?}", *x);
            // }
            // println!("End star s1");
            let hs1 = constructD(r1);
            let hs2 = constructP(r1);
            let mut hset0: HashSet<(String, String)> = HashSet::new();
            for x in hs1 {
                for y in &hs2 {
                    hset0.insert((x.clone(), y.clone()));
                }
            }
            let union_s: HashSet<_> = s1.union(&hset0).collect();
            let mut hset: HashSet<(String, String)> = HashSet::new();
            for x in union_s {
                hset.insert(x.clone());
                // println!("Hello12 : {:?}", *x);
            }
            hset
        },
        _ => {
            let hset: HashSet<(String, String)> = HashSet::new();
            hset
        }
    }
}

// Checks whether a state is a final state
fn checkstate (curr: usize, final_states: &HashSet<String>) -> bool {
    let mut res = false;
    for x in final_states {
        // println!("Final State: {}", x);
        if  x[1..].parse::<usize>().unwrap() == curr {
            res = true;
            break;
        }
    }
    res
}

// Checks string against a regular expression by executing the Glushkov-NFA
fn checkstr(s: &str, nfa: &Vec<Vec<u8>>, final_states: &HashSet<String>, curr: usize, idx: usize, states: u8, state_letter: &Vec<char>) -> bool {
    let mut res = false;
    let mut next_state = 0;
    
    while next_state != states.into() {
        // println!("char = {} Curr state = {} Checking state {} idx = {}, state letter: ", s.chars().nth(idx).unwrap(), curr,  next_state, idx);
        if checkstate(curr, final_states) && idx == s.len() {
            res = true;
            break;
        }
        if !checkstate(curr, final_states) && idx == s.len() {
            res = false;
            break;
        }
        if nfa[curr][next_state] == 1 && (s.chars().nth(idx).unwrap() == state_letter[next_state-1]) {
            println!("Char encountered: {}, Going to State: {}", s.chars().nth(idx).unwrap(), next_state);
            res = res || checkstr(s, &nfa, &final_states, next_state, idx+1, states, &state_letter);
        }
        if res {
            break;
        }
        next_state += 1;
    }
    res
}

// Generates the augmented regular expression e' from given regular expression e
fn augment(regexp: &Rc<Regex>, cnt: &mut u8) -> Rc<Regex> {
    match regexp.deref() {
        Letter(a) => {
            *cnt = *cnt + 1;
            // println!("cnt {}", a);
            // println!("cnt {}", ((*cnt - 1)*10 + a));
            let x1 = *cnt - 1;
            let val = x1.to_string();
            let ret = Rc::new(Letter(format!("{}{}", a, val)));
            ret
        },
        Or(r1, r2) => Rc::new(Or(augment(r1, cnt), augment(r2, cnt))),
        Concat(r1, r2) => Rc::new(Concat(augment(r1, cnt), augment(r2, cnt))),
        Star(r1) => Rc::new(Star(augment(r1, cnt))),
        Empty() => Rc::new(Empty()),
        Eps() => Rc::new(Eps())
    }
}

// Counts number of states in the generated NFA
fn findstates(regexp: &Rc<Regex>, cnt: &mut u8) -> u8 {
    match regexp.deref() {
        Letter(_) => {
            *cnt = *cnt + 1;
            *cnt
        },
        Or(r1, r2) => {
            let mut a = findstates(r1, cnt);
            let b = findstates(r2, &mut a);
            b
        },
        Concat(r1, r2) => {
            let mut a = findstates(r1, cnt);
            let b = findstates(r2, &mut a);
            b
        },
        Star(r1) => findstates(r1, cnt),
        _ => *cnt
    }
}

// Generates a Vector of char of the letter labelled states
fn addstates(regexp: &Rc<Regex>, state_letter: &mut Vec<char>) {
    match regexp.deref() {
        Letter(a) => {
            let t0 = a.clone();
            state_letter.push(t0.chars().nth(0).unwrap());
        },
        Or(r1, r2) => {
            addstates(r1, state_letter);
            addstates(r2, state_letter)
        },
        Concat(r1, r2) => {
            addstates(r1, state_letter);
            addstates(r2, state_letter)
        },
        Star(r1) => addstates(r1, state_letter),
        _ => print!("")
    }
}

// Parses a given pair to AST
fn parse_to_AST(token: &pest::iterators::Pair<Rule>) -> Rc<Regex> {
    let mut tmp0 = token.clone().into_inner();
    // println!("Yes : {:#?}", token);
    match token.as_rule() {
        Rule::Regex   => {
            // println!("in Exp");
            parse_to_AST(&tmp0.next().unwrap())
        },
        Rule::Or=> {
            // println!("Concat - ");
            let r1 = parse_to_AST(&tmp0.next().unwrap());
            let r2 = parse_to_AST(&tmp0.next().unwrap());
            Rc::new(Or(r1, r2))
        },
        Rule::T0    => {
            // println!("in T0 ");
            parse_to_AST(&tmp0.next().unwrap())
        },
        Rule::Concat=> {
            // println!("Concat - ");
            let r1 = parse_to_AST(&tmp0.next().unwrap());
            let r2 = parse_to_AST(&tmp0.next().unwrap());
            Rc::new(Concat(r1, r2))
        },
        Rule::T1    => {
            // println!("in T1 ");
            parse_to_AST(&tmp0.next().unwrap())
        },
        Rule::Star  => {
            // println!("Star - ");
            Rc::new(Star(parse_to_AST(&tmp0.next().unwrap())))
        },
        Rule::T2    => {
            // println!("in T2 ");
            parse_to_AST(&tmp0.next().unwrap())
        },
        Rule::Paren    => {
            // println!("in Paren ");
            parse_to_AST(&tmp0.next().unwrap())
        },
        Rule::Letter=> Rc::new(Letter(token.as_str().to_string())),
        _ => {
            println!("Empty Generated");
            Rc::new(Empty())
        }
    }
}


fn main() {

    println!("Enter a RegEx:");
    let mut tmp1 = String::new();
    io::stdin().read_line(&mut tmp1).expect("failed to readline");
    let regex_input = tmp1.trim();
    // let regex_input = "a(bc | cd | ef)*";

    println!("Enter a string:");
    let mut tmp2 = String::new();
    io::stdin().read_line(&mut tmp2).expect("failed to readline");
    let s: &str= tmp2.trim();
    // let s: &str = "abccdcd";
    
    // Generate pair for the regex
    let mut pairs = RegEx::parse(Rule::Regex, regex_input).unwrap_or_else(|e| panic!("{}", e));
    
    // Parse the pair to an AST
    let x = parse_to_AST(&pairs.next().unwrap());

    // println!("Hello: {:?}", x);
    
    let mut cnt = 1;
    let a = augment(&x, &mut cnt); 
    
    // Generate P, D, F sets
    let P_set = constructP(&a);
    let D_set = constructD(&a);
    let F_set = constructF(&a);

    // Finding Number of states with initial value 1 being for Start State
    let mut init_states: u8 = 1;
    let no_of_states = findstates(&a, &mut init_states);

    let width:usize = no_of_states.into();
    let height:usize = no_of_states.into();
    
    // Getting the letter labels for states
    let mut state_letter: Vec<char> = Vec::new();
    addstates(&a, &mut state_letter);

    
    // Generating the NFA in the form of Adjacency Matrix
    let mut array = vec![vec![0; width]; height];
    for x in &P_set {
        array[0][x[1..].parse::<usize>().unwrap()] = 1 as u8;
    }
    for x in &F_set {
        array[x.0[1..].parse::<usize>().unwrap()][x.1[1..].parse::<usize>().unwrap()] = 1 as u8;
    }
    
    println!("Pset = {:?}", P_set);
    println!("Dset = {:?}", D_set);
    println!("Fset = {:?}", F_set);
    println!("Number of States = {}", no_of_states);
    println!("Letter labels for state = {:?}", state_letter);
    println!("NFA Adjacency Matrix : {:?}", array);


    if s.len() == 0 && matches!(findLambda(&a).deref(), Eps()) {
        println!("Accepted");
    }
    let res = checkstr(s, &array, &D_set, 0, 0, no_of_states, &state_letter);
    if res {
        println!("Accepted");
    } else {
        println!("Rejected");
    }
}

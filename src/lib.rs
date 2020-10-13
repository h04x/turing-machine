mod macros;
mod test;

use indexmap::IndexMap;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub enum TapeMotion {
    Left,
    Right,
    None,
}

pub type Symbols = IndexMap<i32, char>;
#[derive(Clone, Debug, PartialEq)]
pub struct Tape {
    pub cursor_pos: i32,
    pub symbols: Symbols,
}

impl Tape {
    fn get_symbol(&self) -> Option<char> {
        match self.symbols.get(&self.cursor_pos) {
            Some(c) => Some(*c),
            None => None,
        }
    }
    fn mod_symbol(&mut self, new: char) {
        self.symbols.insert(self.cursor_pos, new);
    }
    fn motion(&mut self, motion: &TapeMotion) {
        match motion {
            TapeMotion::Right => self.cursor_pos += 1,
            TapeMotion::Left => self.cursor_pos -= 1,
            TapeMotion::None => (),
        }
    }
}

impl TryFrom<&str> for Tape {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.chars().peekable();
        let mut symbols = Symbols::new();
        let mut cursor_test = 0;
        let mut cursor = None;
        while let Some(ch) = iter.next() {
            match ch {
                '\\' => match iter.peek() {
                    Some(pch) => {
                        symbols.insert(cursor_test, *pch);
                        cursor_test += 1;
                        iter.next();
                    }
                    None => return Err(()),
                },
                '>' => match cursor {
                    None => cursor = Some(cursor_test),
                    Some(_) => return Err(()),
                },
                ch => {
                    symbols.insert(cursor_test, ch);
                    cursor_test += 1;
                }
            }
        }
        if let Some(cursor) = cursor {
            return Ok(Tape {
                cursor_pos: cursor,
                symbols,
            });
        }
        Err(())
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub next_state: String,
    pub next_symbol: char,
    pub tape_motion: TapeMotion,
}

pub type Rules = IndexMap<char, Rule>;
pub type States = IndexMap<String, Rules>;
pub struct TuringMachine {
    states: States,
}

impl<'a> TuringMachine {
    pub fn new(states: States) -> Result<Self, String> {
        if states.is_empty() {
            Err(String::from("TM must contain at least one state"))
        } else {
            Ok(TuringMachine { states })
        }
    }
    pub fn run(&self, lent: Tape) -> Run {
        Run {
            states: &self.states,
            cur_state: self.states.keys().next().unwrap(),
            lent,
        }
    }
}

pub struct Run<'a> {
    states: &'a States,
    cur_state: &'a String,
    lent: Tape,
}

#[derive(Debug)]
pub struct Step {
    cur_state: String,
    cur_symbol: char,
    rule: Rule,
    lent: Tape,
}

impl<'a> Iterator for Run<'a> {
    type Item = Step;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.lent.get_symbol();
        let chf = ch.unwrap_or(' ');
        match self.states.get(self.cur_state) {
            Some(rules) => match rules.get(&chf) {
                Some(rule) => {
                    let ret_state = self.cur_state;
                    self.cur_state = &rule.next_state;
                    if !(rule.next_symbol == ' ' && ch.is_none()) {
                        self.lent.mod_symbol(rule.next_symbol);
                    }
                    self.lent.motion(&rule.tape_motion);
                    return Some(Step {
                        cur_state: ret_state.to_string(),
                        cur_symbol: chf,
                        rule: rule.clone(),
                        lent: self.lent.clone(),
                    });
                }
                None => None,
            },
            None => None,
        }
    }
}

fn trim_rem(line: &str) -> &str {
    let mut iter = line.char_indices();
    while let Some((idx, ch)) = iter.next() {
        match ch {
            ';' => return &line[..idx],
            '\\' => {
                iter.next();
            }
            _ => (),
        };
    }
    line
}

fn trim(line: &str) -> Option<&str> {
    let mut line = line.trim();
    line = trim_rem(line);
    line = line.trim();
    if !line.is_empty() {
        return Some(line);
    }
    None
}

fn split_arrow(rule: &str) -> Result<(&str, &str), ()> {
    let mut iter = rule.char_indices().peekable();
    let mut sep = None;
    while let Some((idx, ch)) = iter.next() {
        match ch {
            '\\' => {
                iter.next();
            }
            '>' => return Err(()),
            '-' => match iter.peek() {
                Some((_, '>')) if sep.is_none() => {
                    sep = Some(idx);
                    iter.next();
                }
                _ => return Err(()),
            },
            _ => (),
        }
    }
    if let Some(sep) = sep {
        Ok((&rule[..sep], &rule[sep + 2..]))
    } else {
        Err(())
    }
}

fn parse_left(left: &str) -> Result<(String, char), ()> {
    let mut iter = left.chars().peekable();
    let mut cur_state = String::new();
    let mut cur_symbol = String::new();
    while let Some(ch) = iter.next() {
        match ch {
            '\\' => match iter.next() {
                Some(pch) => match iter.peek() {
                    Some(_) => cur_state.push(pch),
                    None => cur_symbol.push(pch),
                },
                None => return Err(()),
            },
            _ => match iter.peek() {
                Some(_) => cur_state.push(ch),
                None => cur_symbol.push(ch),
            },
        }
    }
    if !cur_state.is_empty() && !cur_symbol.is_empty() {
        return Ok((cur_state, cur_symbol.chars().next().unwrap()));
    }
    Err(())
}

fn parse_right(right: &str) -> Result<(String, char, TapeMotion), ()> {
    if let Ok((left, tape_motion)) = parse_left(right) {
        let tape_motion = match tape_motion {
            'R' => Some(TapeMotion::Right),
            'L' => Some(TapeMotion::Left),
            'N' => Some(TapeMotion::None),
            _ => None,
        };
        let mut fin_state = String::new();
        let mut fin_sym = String::new();
        let mut iter = left.chars().peekable();
        while let Some(ch) = iter.next() {
            match iter.peek() {
                Some(_) => fin_state.push(ch),
                None => fin_sym.push(ch),
            }
        }
        if !fin_state.is_empty() && !fin_sym.is_empty() && tape_motion.is_some() {
            return Ok((
                fin_state,
                fin_sym.chars().next().unwrap(),
                tape_motion.unwrap(),
            ));
        }
    }
    Err(())
}

fn parse_rule_line(rule: &str, states: &mut States) -> bool {
    if let Ok((left, right)) = split_arrow(rule) {
        if let Ok((cur_state, cur_sym)) = parse_left(left) {
            if let Ok((fin_state, fin_sym, tape_mot)) = parse_right(right) {
                let rule = Rule {
                    next_state: fin_state,
                    next_symbol: fin_sym,
                    tape_motion: tape_mot,
                };
                if let Some(rules) = states.get_mut(cur_state.as_str()) {
                    rules.insert(cur_sym, rule);
                } else {
                    let mut rules = Rules::new();
                    rules.insert(cur_sym, rule);
                    states.insert(cur_state, rules);
                }
                return true;
            }
        }
    }
    false
}

impl TryFrom<&str> for TuringMachine {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut states = States::new();
        for line in value.lines() {
            if let Some(line) = trim(line) {
                if !parse_rule_line(line, &mut states) {
                    return Err(format!("Error parse line '{}'", line));
                }
            }
        }
        TuringMachine::new(states)
    }
}

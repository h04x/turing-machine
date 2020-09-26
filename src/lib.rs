mod macros;
mod test;

use indexmap::IndexMap;
use std::convert::TryFrom;

pub enum TapeMotion {
    Left,
    Right,
    None,
}

pub type Symbols = IndexMap<i32, char>;
#[derive(Clone, Debug)]
pub struct Lent {
    pub cursor_pos: i32,
    pub symbols: Symbols,
}

impl Lent {
    fn get_symbol(&self) -> Option<&char> {
        match self.symbols.get(&self.cursor_pos) {
            Some(c) => Some(c),
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

pub struct Rule<'a> {
    pub next_state: &'a str,
    pub next_symbol: char,
    pub tape_motion: TapeMotion,
}

pub type Rules<'a> = IndexMap<char, Rule<'a>>;
pub struct State<'a> {
    pub identifier: String,
    pub rules: Option<Rules<'a>>,
}

pub type States<'a> = IndexMap<&'a str, Rules<'a>>;
pub struct TuringMachine<'a> {
    states: States<'a>,
}

impl<'a> TuringMachine<'a> {
    pub fn new(states: States<'a>) -> Result<Self, &'static str> {
        if states.is_empty() {
            Err("TM must contain at least one state")
        } else {
            Ok(TuringMachine { states })
        }
    }
    pub fn run(&self, lent: Lent) -> Run {
        Run {
            states: &self.states,
            cur_state: self.states.keys().next().unwrap(),
            lent: lent,
        }
    }
}

pub struct Run<'a> {
    states: &'a States<'a>,
    cur_state: &'a str,
    lent: Lent,
}

impl<'a> Iterator for Run<'a> {
    type Item = Lent;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.lent.get_symbol();
        match self.states.get(self.cur_state) {
            Some(rules) => match rules.get(ch.unwrap_or(&' ')) {
                Some(rule) => {
                    self.cur_state = &rule.next_state;
                    if !(rule.next_symbol == ' ' && ch == None) {
                        self.lent.mod_symbol(rule.next_symbol);
                    }
                    let lent = self.lent.clone();
                    self.lent.motion(&rule.tape_motion);
                    return Some(lent);
                }
                None => return None,
            },
            None => return None,
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
            },
            _ => ()
        };
    }
    line
}

fn trim(line: &str) -> Option<&str> {
    let mut line = line.trim();
    line = trim_rem(line);
    line = line.trim();
    if !line.is_empty() {
        Some(line)
    } else {
        None
    }
}

fn parse_rule_line(rule: &str, states: &mut States) -> Result<(), &'static str> {
    let split = rule.split("->").collect::<Vec<_>>();
    if split.len() != 2 {
        return Err("error while parsing");
    }
    let left = split.first().unwrap().replace("\\", "");

    Ok(())
}

impl<'a> TryFrom<&str> for TuringMachine<'a> {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut states = States::new();
        for line in value.lines() {
            if let Some(line) = trim(line) {
                if let Err(e) = parse_rule_line(line, &mut states) {
                    return Err(e);
                }
            }
        }
        TuringMachine::new(states)
    }
}

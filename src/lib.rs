mod test;

use indexmap::IndexMap;
use std::convert::TryFrom;

/*pub struct Rule {
    cur_state: String,
    cur_symbol: char,
    final_state: String,
    final_symbol: char,
    tape_motion: TapeMotion
}*/

/*impl TryFrom<&str> for Rule {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Err("qwe")
    }
}*/

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
    //cur_state: *const State,
    states: States<'a>,
}

impl<'a> TuringMachine<'a> {
    pub fn new(states: States<'a>) -> Result<Self, &'static str> {
        if states.is_empty() {
            Err("States must not be empty")
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

fn trim_rem(line: &str, rem: char, escape: char) -> &str {
    if Some(rem) == line.chars().next() {
        return &line[0..0];
    }
    let mut iter = line.char_indices().peekable();
    while let Some((_, ch)) = iter.next() {
        match iter.peek() {
            Some((idx, ch_nxt)) if *ch_nxt == rem && ch != escape => {
                return &line[..*idx];
            }
            _ => (),
        }
    }
    line
}

fn trim(line: &str, rem: char, escape: char) -> Option<&str> {
    let mut line = line.trim();
    line = trim_rem(line, ';', '\\');
    line = line.trim();
    if !line.is_empty() {
        Some(line)
    } else {
        None
    }
}

impl<'a> TryFrom<&str> for TuringMachine<'a> {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        /*let mut machine = TuringMachine::new();
        for line in value.lines() {
            if let Some(line) = trim(line, ';', '\\') {
                match Rule::try_from(line) {
                    Ok(rule) => machine.rules.push(rule),
                    Err(e) => return Err(e)
                }
            }
        }*/
        Err("qwe")
        //Ok()
    }
}

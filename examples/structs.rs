extern crate turing_machine;

use std::convert::TryFrom;
use turing_machine::{Lent, Rule, Rules, State, States, Symbols, TapeMotion, TuringMachine};

fn main() {

    let mut rules = Rules::new();
    rules.insert(
        '1',
        Rule {
            next_state: "q0",
            next_symbol: '0',
            tape_motion: TapeMotion::Right,
        },
    );
    rules.insert(
        ' ',
        Rule {
            next_state: "q1",
            next_symbol: ' ',
            tape_motion: TapeMotion::Left,
        },
    );
    let mut states = States::new();
    states.insert("q0", rules);

    let mut rules = Rules::new();
    rules.insert(
        '0',
        Rule {
            next_state: "q1",
            next_symbol: '0',
            tape_motion: TapeMotion::Left,
        },
    );
    rules.insert(
        ' ',
        Rule {
            next_state: "q2",
            next_symbol: ' ',
            tape_motion: TapeMotion::None,
        },
    );
    states.insert("q1", rules);

    let mut symbols = Symbols::new();
    symbols.insert(-1, '1');
    symbols.insert(-0, '1');
    symbols.insert(1, '1');

    let lent = Lent {
        cursor_pos: -1,
        symbols: symbols,
    };

    let tm = TuringMachine::new(states).unwrap();

    for iter in tm.run(lent) {
        println!("{:?}", iter);
    }
}

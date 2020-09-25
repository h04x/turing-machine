extern crate turing_machine;

use std::convert::TryFrom;
use turing_machine::{Lent, Rule, Rules, State, States, Symbols, TapeMotion, TuringMachine};

fn main() {
    let mut states = States::new();
    let mut rules_q0 = Rules::new();
    rules_q0.insert(
        '1',
        Rule {
            next_state: "q0".to_string(),
            next_symbol: '0',
            tape_motion: TapeMotion::Right,
        },
    );
    rules_q0.insert(
        ' ',
        Rule {
            next_state: "q1".to_string(),
            next_symbol: ' ',
            tape_motion: TapeMotion::Left,
        },
    );

    let mut rules_q1 = Rules::new();
    rules_q1.insert(
        '0',
        Rule {
            next_state: "q1".to_string(),
            next_symbol: '0',
            tape_motion: TapeMotion::Left,
        },
    );
    rules_q1.insert(
        ' ',
        Rule {
            next_state: "q2".to_string(),
            next_symbol: ' ',
            tape_motion: TapeMotion::None,
        },
    );
    states.insert("q0".to_string(), rules_q0);
    states.insert("q1".to_string(), rules_q1);

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

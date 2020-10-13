use turing_machine::TapeMotion::*;
use turing_machine::*;

fn main() {
    let mut rules = Rules::new();
    rules.insert(
        '1',
        Rule {
            next_state: "q0".to_string(),
            next_symbol: '0',
            tape_motion: TapeMotion::Right,
        },
    );
    rules.insert(
        '0',
        Rule {
            next_state: "q0".to_string(),
            next_symbol: '0',
            tape_motion: TapeMotion::Right,
        },
    );
    rules.insert(
        ' ',
        Rule {
            next_state: "q1".to_string(),
            next_symbol: ' ',
            tape_motion: TapeMotion::Left,
        },
    );
    let mut states = States::new();
    states.insert("q0".to_string(), rules);

    let mut rules = Rules::new();
    rules.insert(
        '0',
        Rule {
            next_state: "q1".to_string(),
            next_symbol: '0',
            tape_motion: TapeMotion::Left,
        },
    );
    rules.insert(
        ' ',
        Rule {
            next_state: "q2".to_string(),
            next_symbol: ' ',
            tape_motion: TapeMotion::None,
        },
    );
    states.insert("q1".to_string(), rules);

    let mut symbols = Symbols::new();
    symbols.insert(-1, '1');
    symbols.insert(-0, '0');
    symbols.insert(1, '1');

    let lent = Tape {
        cursor_pos: -1,
        symbols: symbols,
    };

    let tm = TuringMachine::new(states).unwrap();

    for step in tm.run(lent) {
        println!("{:?}", step);
    }
}

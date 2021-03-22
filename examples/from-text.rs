use std::str::FromStr;
use turing_machine::*;

fn main() {
    let tm = TuringMachine::from_str(
        r#"
        ; this program replace all 1 to 0
        ; and then return cursor to start position

        q01->q00R ;initial line = initial state (q0)
        q00->q00R
        q0 ->q1 L
        q10->q10L
        q1 ->q2 N ;empty state (q2) = exit
    "#,
    )
    .expect("machine build failed");

    let tape = Tape::from_str(">101").expect("tape build failed");

    for step in tm.run(tape) {
        println!("{:?}", step);
    }
}

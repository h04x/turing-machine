use std::convert::TryFrom;
use turing_machine::*;

fn main() {
    let program = r#"
        ; this program replace all 1 to 0
        ; and then return cursor to start position

        q01->q00R ;initial line = initial state (q0)
        q00->q00R
        q0 ->q1 L
        q10->q10L
        q1 ->q2 N ;empty state (q2) = exit
    "#;

    let tape = ">101";

    match TuringMachine::try_from(program) {
        Ok(tm) => match Tape::try_from(tape) {
            Ok(tape) => {
                for step in tm.run(tape) {
                    println!("{:?}", step);
                }
            }
            Err(_) => println!("Lent build failed"),
        },
        Err(e) => println!("Turing machine build failed:\n\t{}", e),
    };
}

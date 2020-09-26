use std::convert::TryFrom;
use turing_machine::TuringMachine;

fn main() {
    let program = r#"
        ; this program replace all 1 to 0
        ; and return cursor to start position
        q01->q00R
        q00->q00R
        q0 ->q1 R
        q10->q10L
        q1 ->q2 N
    "#;

    let lent = ">101 ";

    match TuringMachine::try_from(program) {
        Ok(_) => (),
        Err(e) => println!("Turing machine build failed:\n\t{}", e),
    };
}

use std::convert::TryFrom;
use turing_machine::*;

fn main() {
    let program = r#"
        ; this program replace all 1 to 0
        ; and return cursor to start position
        q01->q00R
        q00->q00R
        q0 ->q1 L
        q10->q10L
        q1 ->q2 N
    "#;

    let lent = ">101 ";

    let lent = lent![
        -1,
        symbols![
            -1 => '1',
            0 => '0',
            1 => '1'
        ]
    ];

    match TuringMachine::try_from(program) {
        Ok(tm) => {
            for step in tm.run(lent) {
                println!("{:?}", step);
            }
        },
        Err(e) => println!("Turing machine build failed:\n\t{}", e),
    };
}

extern crate turing_machine;

use std::convert::TryFrom;
use turing_machine::TuringMachine;

fn main() {
    let program = r#"
        ;
        ;welcome
        qwe;pelcome
        asd\;zxc;xcvbcbv
        q0 ->q11R  ; comment line
          q1*->q3*N
    "#;

    let lent = "> 1234*";

    match TuringMachine::try_from(program) {
        Ok(_) => (),
        Err(e) => println!("Turing machine build failed:\n\t{}", e),
    };
    //println!("{}", tm);

    //tm.run(lent);

    // tm.run(lent);
}

# Turing machine
Deterministic Turing machine on rust. Matching based on HashMap's
## Usage
There are several ways to use the machine. Let's consider two of them.  
**First way, using text rules to build the machine, e.g.**
```rust,no_run
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
```
Consider rule 
```
q01->q10R
```

Here ```q0``` is a matched state, followed ```1``` is a matched character. 
Next, state ```q0``` will be changed to ```q1```, and matched character ```1``` changed to ```0```.
The ending ```R``` means move tape right (allowed variants ```R/L/N```).

Little bit more about rules
```
Ident19->Ident29R         # the state can be a string e.g. Ident1
Ident1 ->Ident29R         # Here 'space' is a matched symbol
Id\-en\>t19->Ident\\2\#9R # escape control symbols 
```

Ok, let's look at the tape definition  ```">101"```. Here tape contain three characters 
and cursor ```>``` points to the first character.

**Next way, using macros**
```rust,no_run
use turing_machine::TapeMotion::*;
use turing_machine::*;

fn main() {
    let states = states![
        "q0" => rules![
            '1' => rule!["q0", '0', Right],
            '0' => rule!["q0", '0', Right],
            ' ' => rule!["q1", ' ', Left]
        ],
        "q1" => rules![
            '0' => rule!["q1", '0', Left],
            ' ' => rule!["q2", ' ', None]
        ]
    ];

    let tape = tape![
        -1, // cursor start position
        symbols![
            -1 => '1',
            0 => '0',
            1 => '1'
        ]
    ];

    let tm = TuringMachine::new(states).unwrap();

    for step in tm.run(tape) {
        println!("{:?}", step);
    }
}
```

See examples ```cargo run --example```

//extern crate turing_machine;

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

    let lent = lent![
        -1,
        symbols![
            -1 => '1',
            0 => '0',
            1 => '1'
        ]
    ];

    let tm = TuringMachine::new(states).unwrap();

    for step in tm.run(lent) {
        println!("{:?}", step);
    }
}

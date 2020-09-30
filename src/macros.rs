#[macro_export]
macro_rules! rule {
    ($next_state:expr, $next_symbol:expr, $tape_motion:ident) => {
        Rule {
            next_state: String::from($next_state),
            next_symbol: $next_symbol,
            tape_motion: $tape_motion,
        }
    };
}

#[macro_export]
macro_rules! rules {
    ($($key:expr => $value:expr),+) => {
        {
            let mut rules = Rules::new();
            $( rules.insert($key, $value); )+
            rules
        }
    };
}

#[macro_export]
macro_rules! states {
    ($($key:expr => $value:expr),+) => {
        {
            let mut states = States::new();
            $( states.insert(String::from($key), $value); )+
            states
        }
    };
}

#[macro_export]
macro_rules! lent {
    ($cursor_pos:expr, $symbols:expr) => {
        Lent {
            cursor_pos: $cursor_pos,
            symbols: $symbols,
        }
    };
}

#[macro_export]
macro_rules! symbols {
    ($($key:expr => $value:expr),+) => {
        {
            let mut symbols = Symbols::new();
            $( symbols.insert($key, $value); )+
            symbols
        }
    };
}

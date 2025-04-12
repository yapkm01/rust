#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year < 1819,
            UsState::Alaska => year < 1959,
            // --snip--
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // same as above but using if let but with a count on others
    let mut count = 0;
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured as to be {max}")
    } else {
        count += 1;
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let state_desc_quarter = describe_state_quarter(coin);
    println!("{state_desc_quarter:?}");
}

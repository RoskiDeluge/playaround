#[derive(Debug)]

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("I quit!");
        }
        Message::ChangeColor(red, green, blue) => {
            println!("Red {}, Green {}, Blue {}", red, green, blue);
        }
        Message::Move { x, y: new_name } => {
            println!("X is {}, Y as new_name is {}", x, new_name);
        }
        Message::Write(text) => {
            println!("{}", text);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::panic;

    #[test]
    fn tests_large_enum() {
        let my_quit = Message::Quit;
        let my_color = Message::ChangeColor(10, 20, 255);
        let my_move = Message::Move { x: 10, y: 200 };
        let my_write = Message::Write("My awesome string".to_string());
        process_message(my_write);
    }

    #[test]
    fn tests_match_literals() {
        let number = 11;

        let res = match number {
            1 => "It was the first",
            2 | 3 | 5 | 7 | 11 => "This is a prime",
            _ => "It was something else",
        };

        println!("{}", res);
    }

    #[test]
    fn tests_match_options() {
        let some_num: Option<i32> = Some(10);
        // let prob_none: Option<i32> = None;

        let my_int: i32 = if let Some(i) = some_num {
            i
        } else {
            100
            // panic!("There was a problem");
        };

        println!("My int: {}", my_int);

        // let res = match some_num {
        //     Some(i) => i,
        //     None => {
        //         panic!("There was a problem");
        //     }
        // };
        // println!("{:?}", res);
        // println!("{}", res);
    }

    #[test]
    fn tests_match_result() {
        let some_res: Result<i32, &str> = Ok(50);
        let some_err: Result<i32, &str> = Err("There was a problem");

        let res = match some_res {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        println!("{}", res);

        let my_int: i32 = if let Ok(r) = some_res {
            r
        } else {
            100
            // panic!("There was a problem");
        };
        println!("{}", my_int);
    }

    #[test]
    fn tests_match_guard() {
        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("They match!"),
            (x, y) if x + y == 0 => println!("They neutralize!"),
            (_, y) if y == 2 => println!("Y is indeed +2"),
            _ => println!("We are not bothered"),
        };
    }

    #[test]
    fn tests_match_struct() {
        struct Location {
            x: i32,
            y: i32,
        }

        let location = Location { x: 0, y: 20 };

        match location {
            Location { x, y: 0 } => println!("Y is on the axis"),
            Location { x: 0, y } => println!("X is on the axis"),
            Location { x, y } => println!("X and Y are not the axis"),
        };
    }
}

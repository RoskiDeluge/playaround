#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

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
}

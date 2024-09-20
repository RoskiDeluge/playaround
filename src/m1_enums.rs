use core::num;

#[derive(Debug)]
enum CarColor {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]

enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]

enum GivenOption<T> {
    None,
    Some(T),
}

fn create_car_color_red() -> CarColor {
    let my_car_color: CarColor = CarColor::Red;
    my_car_color
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Number is greater than 5".to_string())
    }
}

fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Number is greater than 5".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_color: CarColor = create_car_color_red();
        dbg!(car_color);

        let under_five_res = check_under_five_built_in(7);
        dbg!(under_five_res);

        let remainder: Option<f32> = remainder_zero_built_in(12.2);
        dbg!(remainder);
    }
}

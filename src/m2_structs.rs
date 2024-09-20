#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let mut user_1: User = User {
            username: String::from("Roberto"),
            email: String::from("roberto@email.com"),
            sign_in_count: 1,
            active: true,
        };

        change_username(&mut user_1, "roberto2@email.com");

        dbg!(user_1);

        let mut user_2: User = User {
            username: String::from("kevin"),
            email: String::from("kevin@email.com"),
            sign_in_count: 0,
            active: false,
        };

        user_2.increment_sign_in_count();

        user_2.change_email("kevin2@email.com");

        dbg!(user_2);
    }
}

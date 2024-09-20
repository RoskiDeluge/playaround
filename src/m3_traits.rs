trait Attacker {
    fn chose_fighting_style(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn chose_fighting_style(&self) -> String {
        match self {
            Character::Warrior => "wing chug".to_string(),
            Character::Archer => "bow and arrow".to_string(),
            Character::Wizard => "magic".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_traits() {
        let my_character: Character = Character::Wizard;
        let fighting_style: String = my_character.chose_fighting_style();
        dbg!(fighting_style);
    }
}

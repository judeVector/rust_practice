pub fn test_option_type() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);
    return opt1;
}

pub fn test_option_string() -> Option<String> {
    let mut opt1: Option<String> = None;
    opt1 = Some("Trevor Dev".to_string());
    return opt1;
}

pub fn test_option_chartype() -> Option<CharacterType> {
    let mut chartype: Option<CharacterType> = None;
    chartype = Some(CharacterType::Mage);
    return chartype;
}

pub enum CharacterType {
    Archer,
    Warrior,
    Mage,
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match &self {
            CharacterType::Archer => "Archer",
            CharacterType::Warrior => "Warrior",
            CharacterType::Mage => "Mage",
        }
        .to_string()
    }
}
// println!(
//     "Character at index 8: {:?}",
//     match name.chars().nth(1) {
//         Some(c) => c.to_string(),
//         None => "There is no character".to_string(),
//     }
// );

// println!(
//     "Occupation is {}",
//     match get_occupation(&name) {
//         Some(o) => o,
//         None => "Not found!",
//     }
// )

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Dominic" => Some("Software Developer"),
        "Micheal" => Some("Dentist"),
        _ => None,
    }
}

fn main() {
    println!("Hello, world!");
}

enum Pulse {
    Short,
    Long,
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

type Letter = Vec<Pulse>;

type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}",pulse)
        }
        print!(" ")
    }
    println!()
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut message = Vec::new();
        for letter in self.to_lowercase().chars() {
            match letter {
                'a' => {message.push(vec![Pulse::Long, Pulse::Short]);},
                'b' => {message.push(vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short]);},
                'c' => {message.push(vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short]);},
                _ => {}
            }
        }
        message
    }
}


#[test]
fn abc_to_morse() {
    let abc = String::from("abc");
    print_morse_code(&abc.to_morse_code())
}
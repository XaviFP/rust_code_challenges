fn main() {
    println!("Hello, world!");
}

fn info<T: AsRef<str>>(text: &T) {
    println!("{}", text.as_ref())
}


#[test]
fn string_input(){
    let text = String::from("text_trait_training");
    info(&text)
}

#[test]
fn tr_input(){
    let text = "text_trait_training";
    info(&text)
}

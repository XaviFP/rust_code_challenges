use std::env;


fn main() {
    //let numbers: Vec<f32> = Vec::new();
    let result: Option<f32> = None;
    
    let args = env::args();
    if args.len() == 1 {
        dbg!(result);
        return
    }

    let mut numbers = args.skip(1).fold(Vec::new(),|mut numbers, arg| {
        let number = arg.parse::<f32>().expect("args should be f32");
        numbers.push(number);
        numbers
    });

    numbers.sort();

}

use std::env;

fn main() {
    let result: Option<f32> = None;
    
    let args = env::args();
    if args.len() == 1 {
        dbg!(result);
        return
    }

    let numbers = args.skip(1).fold(Vec::new(),|mut numbers, arg| {
        let number = arg.parse::<f32>().expect("args should be f32");
        numbers.push(number);
        numbers
    });

    dbg!(median(numbers).unwrap());
}

fn median(mut numbers: Vec<f32>) -> Option<f32> {
    let len = numbers.len();
    if len == 0 {
        return None
    }

    numbers.sort_by(|a, b|{a.partial_cmp(b).unwrap()});
    let med: f32;
    let middle = len /2;
    if len % 2 != 0 {
        med = numbers[middle];
    } else {
        med = (numbers[middle - 1] + numbers[middle]) / 2.0;
    }

    Some(med)
}

#[test]
fn empty_list() {
    let input: Vec<f32> = vec![];
    let expected: Option<f32>  = None;
    let actual = median(input);

    assert_eq!(actual, expected)
}

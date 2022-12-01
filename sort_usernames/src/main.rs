fn main() {
    println!("Hello, world!");
}

fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by_cached_key(|username| {
        username.as_ref().to_lowercase()
    })
}

#[test]
fn test_sort() {
    let mut input = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let expected = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut input);
    assert_eq!(expected, input)
}
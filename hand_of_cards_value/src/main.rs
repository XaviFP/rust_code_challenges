fn main() {
    println!("Hello, world!");
}

enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {

    fn new() -> Self{
        Self { cards: vec![] }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }

    fn value(&self) -> usize {
        use crate::Card::*;

        let mut acc: usize = 0;
        let mut num_aces = 0;
        for card in self.cards.iter() {
            acc += match card {
                Ace => {num_aces += 1; 0},
                Two => {2},
                Three => {3},
                Four => {4},
                Five => {5},
                Six => {6},
                Seven => {7},
                Eight => {8},
                Nine => {9},
                Jack | Queen | King => {10},
            }
        }
        for _ in 0..num_aces {
            let ace_value = if acc <= 10 {11} else {1};
            acc += ace_value;
        }

        acc
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}



#[test]
fn seventeen() {
    use crate::Card::*;

    let mut test_hand = Hand::new();

    test_hand.add_card(Six);
    assert_eq!(test_hand.value(), 6);

    test_hand.add_card(Four);
    assert_eq!(test_hand.value(), 10);

    test_hand.add_card(Three);
    assert_eq!(test_hand.value(), 13);

    test_hand.add_card(Two);
    assert_eq!(test_hand.value(), 15);

    test_hand.add_card(Two);
    assert_eq!(test_hand.value(), 17);
    assert_eq!(test_hand.is_loosing_hand(), false);
}

#[test]
fn empty_hand() {
     assert_eq!(Hand::new().value(), 0)
}

#[test]
fn normal_ace() {
    use crate::Card::*;

    let mut test_hand = Hand::new();

    test_hand.add_card(Six);
    assert_eq!(test_hand.value(), 6);

    test_hand.add_card(Four);
    assert_eq!(test_hand.value(), 10);

    test_hand.add_card(Ace);
    assert_eq!(test_hand.value(), 21);
    assert_eq!(test_hand.is_loosing_hand(), false);
}

#[test]
fn overflowing_ace() {
    use crate::Card::*;

    let mut test_hand = Hand::new();

    test_hand.add_card(Six);
    assert_eq!(test_hand.value(), 6);

    test_hand.add_card(Four);
    assert_eq!(test_hand.value(), 10);

    test_hand.add_card(Ace);
    assert_eq!(test_hand.value(), 21);
    assert_eq!(test_hand.is_loosing_hand(), false);

    test_hand.add_card(Ace);
    assert_eq!(test_hand.value(), 22);
    assert_eq!(test_hand.is_loosing_hand(), true);
}
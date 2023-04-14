#![allow(dead_code)]

#[derive(Debug, PartialEq)]
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
    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card)
    }

    fn value(&self) -> usize {
        let mut value = 0;
        for card in self.cards.iter() {
            let card_value = match card {
                Card::Ace => {
                    if value > 21 {
                        1
                    } else {
                        11
                    }
                }
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                _ => 10,
            };
            value += card_value
        }
        value
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 22
    }
}

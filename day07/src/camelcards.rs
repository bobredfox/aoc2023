pub mod camel_cards {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
    pub enum Cards {
        Numeric(u32),
        T,
        J,
        Q,
        K,
        A,
    }

    impl Cards {
        pub fn new(card: char) -> Cards {
            match card {
                '2'..='9' => Cards::Numeric(card.to_digit(10).unwrap()),
                'T' => Cards::T,
                'J' => Cards::J,
                'Q' => Cards::Q,
                'K' => Cards::K,
                'A' => Cards::A,
                _ => Cards::Numeric(0),
            }
        }

        pub fn cards_from_string(cards: &str) -> Vec<Cards> {
            return cards.chars().map(|c| Cards::new(c)).collect();
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Types {
        High_card,  // 5 Karten
        One_pair,   // 2 Karten, 3 Unterschiedlich
        Two_pair,   // 2 Karten, 2 Karten, 1 Unterschiedlich
        Three_kind, // 3 Karten, 2 Unterschiedlich
        Full_house, // 3 Karten, 2 Karten
        Four_kind,  // 4 Karten, 1 Unterschiedlich
        Five_kind,  // 5 Karten
    }

    impl Types {
        pub fn new(cards: &Vec<Cards>) -> Option<Types> {
            let mut cache: HashMap<Cards, usize> = HashMap::new();

            for element in cards.iter() {
                if !cache.contains_key(element) {
                    cache.insert(element.clone(), 1);
                } else {
                    let (_, value) = cache.get_key_value(element).unwrap();
                    let _ = cache.insert(element.clone(), value + 1);
                }
            }

            match cache.len() {
                5 => Some(Types::High_card),
                4 => Some(Types::One_pair),
                3 => {
                    if cache.values().max().unwrap() == &3 {
                        Some(Types::Three_kind)
                    } else {
                        Some(Types::Two_pair)
                    }
                }
                2 => {
                    if cache.values().max().unwrap() == &4 {
                        Some(Types::Four_kind)
                    } else {
                        Some(Types::Full_house)
                    }
                } // Full house oder Four Four_kind
                1 => Some(Types::Five_kind),
                _ => None, // Five kind
            }
        }
    }

    #[derive(Debug)]
    pub struct Hand {
        cards: Vec<Cards>,
        bid: usize,
        hand_type: Option<Types>,
        rank: Option<usize>,
    }

    impl Hand {
        pub fn new(cards: &str, bid_value: usize) -> Hand {
            let card_vec = Cards::cards_from_string(cards);
            return Hand {
                cards: card_vec.clone(),
                bid: bid_value,
                hand_type: Types::new(&card_vec),
                rank: None,
            };
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            
        }
        // add code here
    }
    }

}

#[cfg(test)]
mod tests {
    use super::camel_cards::{Cards, Types};

    #[test]
    fn ordering_numeric() {
        let numeric_one = Cards::Numeric(2);
        let numeric_two = Cards::Numeric(3);

        assert_eq!(numeric_one < numeric_two, true);
    }

    #[test]
    fn ordering_numeric_text() {
        let numeric = Cards::Numeric(2);
        let text = Cards::T;

        assert_eq!(text > numeric, true);
    }

    #[test]
    fn ordering_text() {
        let text_1 = Cards::T;
        let text_2 = Cards::A;

        assert_eq!(text_2 > text_1, true);
    }

    #[test]
    fn ordering_type() {
        let first_type = Types::High_card;
        let second_type = Types::Five_kind;

        assert_eq!(second_type > first_type, true);
    }

    #[test]
    fn card_from_numeric_char() {
        let card = Cards::new('2');

        assert_eq!(card, Cards::Numeric(2));
    }

    #[test]
    fn card_from_char() {
        let card = Cards::new('T');

        assert_eq!(card, Cards::T);
    }
}

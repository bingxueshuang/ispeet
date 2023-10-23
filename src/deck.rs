use core::fmt;

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return write!(f, "{:?}", self);
        }
        use Suit::*;
        let symbol = match self {
            Clubs => "♣",
            Diamonds => "♦",
            Hearts => "♥",
            Spades => "♠",
        };
        write!(f, "{symbol}")
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let debug_name = format!("{self:?}");
        if f.alternate() {
            return write!(f, "{debug_name}");
        }
        let number = *self as i8;
        let string = if number <= 10 {
            format!("{number}")
        } else {
            // obtain first character of face cards
            debug_name[0..1].to_owned()
        };
        write!(f, "{string}")
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn rank_display_as_words() {
        use Rank::*;
        let map = HashMap::from([
            (Two, "Two"),
            (Three, "Three"),
            (Four, "Four"),
            (Five, "Five"),
            (Six, "Six"),
            (Seven, "Seven"),
            (Eight, "Eight"),
            (Nine, "Nine"),
            (Ten, "Ten"),
            (Jack, "Jack"),
            (Queen, "Queen"),
            (King, "King"),
            (Ace, "Ace"),
        ]);
        for (rank, words) in map.iter() {
            let rank_words = format!("{:#}", rank);
            assert_eq!(words, &rank_words);
        }
    }

    #[test]
    fn rank_display() {
        use Rank::*;
        let map = HashMap::from([
            (Two, "2"),
            (Three, "3"),
            (Four, "4"),
            (Five, "5"),
            (Six, "6"),
            (Seven, "7"),
            (Eight, "8"),
            (Nine, "9"),
            (Ten, "10"),
            (Jack, "J"),
            (Queen, "Q"),
            (King, "K"),
            (Ace, "A"),
        ]);
        for (rank, display) in map.iter() {
            let rank_display = format!("{}", rank);
            assert_eq!(display, &rank_display);
        }
    }

    #[test]
    fn suit_display_as_words() {
        use Suit::*;
        let map = HashMap::from([
            (Clubs, "Clubs"),
            (Diamonds, "Diamonds"),
            (Hearts, "Hearts"),
            (Spades, "Spades"),
        ]);
        for (suit, words) in map.iter() {
            let suit_words = format!("{:#}", suit);
            assert_eq!(words, &suit_words);
        }
    }

    #[test]
    fn suit_display() {
        use Suit::*;
        let map = HashMap::from([(Clubs, "♣"), (Diamonds, "♦"), (Hearts, "♥"), (Spades, "♠")]);
        for (suit, display) in map.iter() {
            let suit_display = format!("{}", suit);
            assert_eq!(display, &suit_display);
        }
    }
}

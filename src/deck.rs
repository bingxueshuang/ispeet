use core::fmt;
use std::{cmp::Ordering, str};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

use Suit::*;

impl Suit {
    const ALL: [Suit; 4] = [Clubs, Diamonds, Hearts, Spades];
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return write!(f, "{:?}", self);
        }
        let symbol = match self {
            Clubs => "♣",
            Diamonds => "♦",
            Hearts => "♥",
            Spades => "♠",
        };
        write!(f, "{symbol}")
    }
}

impl str::FromStr for Suit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suit = match s.to_lowercase().as_ref() {
            "♣" | "c" | "club" | "clubs" => Clubs,
            "♦" | "d" | "diamond" | "diamonds" => Diamonds,
            "♥" | "h" | "heart" | "hearts" => Hearts,
            "♠" | "s" | "spade" | "spades" => Spades,
            _ => Err(Error::ParseSuit(s.to_owned()))?,
        };
        Ok(suit)
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

use Rank::*;

impl Rank {
    const ALL: [Rank; 13] = [
        Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
    ];
    fn face_value(&self) -> bool {
        let value = *self as u8;
        value > 10
    }
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

impl str::FromStr for Rank {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rank = match s.to_lowercase().as_ref() {
            "1" | "one" | "a" | "ace" => Ace,
            "2" | "two" | "d" | "deuce" => Two,
            "3" | "three" | "trey" => Three,
            "4" | "four" => Four,
            "5" | "five" => Five,
            "6" | "six" => Six,
            "7" | "seven" => Seven,
            "8" | "eight" => Eight,
            "9" | "nine" => Nine,
            "10" | "ten" | "t" => Ten,
            "11" | "jack" | "j" => Jack,
            "12" | "queen" | "q" => Queen,
            "13" | "king" | "k" => King,
            _ => Err(Error::ParseRank(s.to_owned()))?,
        };
        Ok(rank)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
    fn rank(&self) -> Rank {
        self.rank
    }
    fn suit(&self) -> Suit {
        self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#} of {:#}", self.rank, self.suit)
        } else {
            write!(f, "{}{}", self.suit, self.rank)
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = if self.suit == other.suit {
            self.rank.cmp(&other.rank)
        } else {
            None?
        };
        Some(ord)
    }
}

impl From<(Rank, Suit)> for Card {
    fn from((rank, suit): (Rank, Suit)) -> Self {
        Card { rank, suit }
    }
}

impl From<(Suit, Rank)> for Card {
    fn from((suit, rank): (Suit, Rank)) -> Self {
        Card { rank, suit }
    }
}

#[derive(Clone, Debug, Error)]
enum Error {
    #[error("cannot parse {0:?} into Suit")]
    ParseSuit(String),
    #[error("cannot parse {0:?} into Rank")]
    ParseRank(String),
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[test]
    fn rank_display_as_words() {
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
    fn rank_face_value() {
        let court_cards = HashSet::from([Jack, Queen, King, Ace]);
        for rank in Rank::ALL {
            let face_value = court_cards.contains(&rank);
            assert_eq!(rank.face_value(), face_value);
        }
    }
}

//! The standard deck comprises of 13 ranks in each of the four suits:
//! clubs (♣), diamonds (♦), hearts (♥) and spades (♠).
//! Each suit includes three court cards (face cards), King, Queen and Jack.
//! Each suit also includes ten numeral cards or pip cards, from one (Ace) to ten.

use core::fmt;
use std::{cmp::Ordering, str};

use rand::distributions::{Distribution, Standard, Uniform};
use thiserror::Error;

/// The cards in the pack are grouped in suits. The English pattern of French-suited
/// cards consists of four suits: clubs (♣), diamonds (♦), hearts (♥) and spades (♠).
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

use Suit::*;

impl Suit {
    /// All four suits in a deck for ease of iteration.
    ///
    /// ```
    /// use ispeet::deck::Suit;
    /// for suit in Suit::ALL {
    ///     println!("{:?}", suit)
    /// }
    /// ```
    pub const ALL: [Suit; 4] = [Clubs, Diamonds, Hearts, Spades];
}

/// Pretty printing of suits. Normal formatting gives suit symbols.
/// Alternate formatting prints them in words.
///
/// ```
/// use ispeet::deck::Suit;
/// let clubs = Suit::Clubs;
/// let symbol = format!("{}", clubs);
/// let words = format!("{:#}", clubs);
/// assert_eq!(symbol, "♣");
/// assert_eq!(words, "Clubs");
/// ```
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

/// Parse suit from a string that is a case-insensitive match of:
/// - suit symbol (unicode)
/// - first letter of suit name
/// - suit name in words (singular or plural)
///
/// ```
/// use ispeet::deck::{Suit};
/// let suit: Suit = "c".parse().unwrap();
/// assert_eq!(suit, Suit::Clubs);
/// ```
impl str::FromStr for Suit {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suit = match s.to_lowercase().as_ref() {
            "♣" | "♧" | "c" | "club" | "clubs" => Clubs,
            "♦" | "♢" | "d" | "diamond" | "diamonds" => Diamonds,
            "♥" | "♡" | "h" | "heart" | "hearts" => Hearts,
            "♠" | "♤" | "s" | "spade" | "spades" => Spades,
            _ => Err(Error::ParseSuit(s.to_owned()))?,
        };
        Ok(suit)
    }
}

/// Support random sampling on Suit.
///
/// ```
/// use ispeet::deck::Suit;
/// use rand;
/// let suit: Suit = rand::random();
/// assert!(Suit::ALL.into_iter().any(|v| v == suit));
/// println!("{:?}", suit);
/// ```
impl Distribution<Suit> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        let suits = Suit::ALL;
        let index = Uniform::new(0, suits.len()).sample(rng);
        suits[index]
    }
}

/// The cards within each suit are distinguished by their ranks.
/// Each suit includes three court cards (face cards), King, Queen and Jack;
/// and ten numeral cards or pip cards: from one (Ace) to ten. The card with
/// single pip is called an 'Ace'.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rank {
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
    /// Collect all the ranks in one place for ease of iteration.
    ///
    /// ```
    /// use ispeet::deck::Rank;
    /// for rank in Rank::ALL {
    ///     println!("{:?}", rank);
    /// }
    /// ```
    pub const ALL: [Rank; 13] = [
        Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
    ];

    /// Reports whether a card of the given rank is a face card (or court card)
    /// or not. Only Jack, Queen and King are court cards.
    ///
    /// ```
    /// use ispeet::deck::Rank;
    /// let rank = Rank::Seven;
    /// assert!(!rank.face_card());
    /// ```
    pub fn face_card(&self) -> bool {
        let value = *self as u8;
        10 < value && value <= 13
    }
}

/// Pretting printing options for Rank. Normat formatting gives number or
/// single letter for the corresponding rank. Alternate formatting obtains
/// the rank in words.
///
/// ```
/// use ispeet::deck::{Rank};
/// let rank = Rank::Seven;
/// let number = format!("{}", rank);
/// let words = format!("{:#}", rank);
/// assert_eq!(number, "7");
/// assert_eq!(words, "Seven");
/// ```
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

/// Parsing rank from string if it is case case-insensitive match of one of these:
/// - number representing the rank
/// - one letter representing the rank
///   * A: Ace
///   * D: Two (deuce)
///   * T: Ten
///   * J: Jack
///   * Q: Queen
///   * K: King
/// - name of the rank in words
///
/// ```
/// use ispeet::deck::{Rank};
/// let rank = "J".parse::<Rank>().unwrap();
/// assert_eq!(rank, Rank::Jack);
/// ```
impl str::FromStr for Rank {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rank = match s.to_lowercase().as_ref() {
            "1" | "14" | "one" | "a" | "ace" => Ace,
            "2" | "two" | "d" | "deuce" => Two,
            "3" | "three" | "trey" => Three,
            "4" | "four" => Four,
            "5" | "five" => Five,
            "6" | "six" => Six,
            "7" | "seven" => Seven,
            "8" | "eight" => Eight,
            "9" | "nine" => Nine,
            "10" | "ten" | "t" => Ten,
            "11" | "jack" | "knave" | "j" => Jack,
            "12" | "queen" | "q" => Queen,
            "13" | "king" | "k" => King,
            _ => Err(Error::ParseRank(s.to_owned()))?,
        };
        Ok(rank)
    }
}

/// Supports random sampling for [Rank].
///
/// ```
/// use ispeet::deck::{Rank};
/// use rand;
/// let rank:Rank = rand::random();
/// assert!(Rank::ALL.into_iter().any(|v| v == rank));
/// println!("{:?}", rank);
/// ```
impl Distribution<Rank> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Rank {
        let ranks = Rank::ALL;
        let index = Uniform::new(0, ranks.len()).sample(rng);
        ranks[index]
    }
}

/// Playing cards are grouped into suits and are distinguished by its rank.
/// A card of each rank occurs once in each of the suits.
/// The standard deck consists of 52 cards. In addition, commercial decks
/// include one to six jokers, which are not implemented in this module.
///
/// ```
/// use ispeet::deck::{Card, Rank, Suit};
/// let rank = Rank::Seven;
/// let suit = Suit::Hearts;
/// let card = Card::from((rank, suit));
/// assert_eq!(card.rank(), rank);
/// assert_eq!(card.suit(), suit);
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Getter for rank of the card.
    pub fn rank(&self) -> Rank {
        self.rank
    }

    /// Getter for suit of the card.
    pub fn suit(&self) -> Suit {
        self.suit
    }
}

/// Pretty printing [Card]. Normal formatting prints symbol of suit and short
/// form of rank and alternate formatting distinguishes the card in words.
///
/// ```
/// use ispeet::deck::{Card, Suit, Rank};
/// let card = Card::from((Suit::Hearts, Rank::Seven));
/// assert_eq!(format!("{}", card), "♥7");
/// assert_eq!(format!("{:#}",card), "Seven of Hearts");
/// ```
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#} of {:#}", self.rank, self.suit)
        } else {
            write!(f, "{}{}", self.suit, self.rank)
        }
    }
}

/// Cards of the same suit are comparable.
///
/// ```
/// use ispeet::deck::{Card, Suit, Rank};
/// let small = Card::from((Suit::Hearts, Rank::Five));
/// let card = Card::from((Suit::Hearts, Rank::Seven));
/// let incomparable = Card::from((Suit::Spades, Rank::Five));
/// assert!(small < card);
/// assert!(card > small);
/// assert!(!(card < small));
/// assert!(!(small > card));
/// assert!(!(incomparable < card));
/// assert!(!(incomparable > card));
/// assert!(!(card > incomparable));
/// assert!(!(card < incomparable));
/// ```
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

/// New card construction.
impl From<(Rank, Suit)> for Card {
    fn from((rank, suit): (Rank, Suit)) -> Self {
        Card { rank, suit }
    }
}

/// Ease of card construction.
impl From<(Suit, Rank)> for Card {
    fn from((suit, rank): (Suit, Rank)) -> Self {
        Card { rank, suit }
    }
}

/// Random sampling for [Card].
///
/// ```
/// use ispeet::deck::{Rank, Suit, Card};
/// use rand;
/// let card: Card = rand::random();
/// let rank:Rank = rand::random();
/// assert!(Rank::ALL.into_iter().any(|v| v == card.rank()));
/// assert!(Suit::ALL.into_iter().any(|v| v == card.suit()));
/// println!("{:?}", card);
/// ```
impl Distribution<Card> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Card {
        let rank: Rank = rng.gen();
        let suit: Suit = rng.gen();
        Card { rank, suit }
    }
}

/// Programming errors in deck module.
///
/// ```
/// use ispeet::deck::{Suit, Error, Rank};
/// let suit_error = Error::ParseSuit("f".to_owned());
/// let error = "f".parse::<Suit>().unwrap_err();
/// assert_eq!(error, suit_error);
/// let rank_error = Error::ParseRank("f".to_owned());
/// let error = "f".parse::<Rank>().unwrap_err();
/// assert_eq!(error, rank_error);
/// ```
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
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
    fn rank_face_card() {
        let court_cards = HashSet::from([Jack, Queen, King]);
        for rank in Rank::ALL {
            let face_card = court_cards.contains(&rank);
            assert_eq!(rank.face_card(), face_card);
        }
    }

    #[test]
    fn ace_is_not_face() {
        assert!(!Ace.face_card());
    }
}

// In Rust a Crate == Package
// mod games;

use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)] // This automatically adds all the debug functions to the struct
struct Deck {
    cards: Vec<String>
}
// Vec = Vector a vector is kind of an array it can grow and shrink
// in rust arrays has fixed length

// Implementations is the fancy topic to add functions to an struct similar to
// the pointers in Golang
impl Deck {
    // this is an associated function
    fn new() -> Self {
        // List of 'suits' = 'hearts', 'spades'
        // List of 'values' = 'ace', 'two', 'three'

        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // return Deck { cards };
        Deck { cards } // This is an implicit return, as long as it does not has a ;
    }

    // this is a method
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }
}
// Association function is tied to struct definition
// Method operates in an instance of the struct

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Heres your deck: {:#?}", deck);
}

// Variables are inmutable by default (can't change). here we call
// bindings instead of variables.
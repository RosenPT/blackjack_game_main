#![allow(unused)]
use std::io;
use rand::prelude::SliceRandom;
#[derive(Debug, Copy, Clone, PartialEq)]
enum Suit{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Rank{
    Ace,
    Two,
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
}
#[derive(Debug, PartialEq)]
struct Card{
    suit: Suit,
    rank: Rank
}
impl Card {
    fn value(&self) -> u32{
        match self.rank {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::King | Rank::Queen => 10
        }
    }
}
struct Deck{
    cards: Vec<Card>
}
impl Deck {
    fn new() -> Deck{
        let mut cards = Vec::new();
        for i in 0..3 {
            for &suit in &[Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades] {
                for &rank in &[
                    Rank::Ace, Rank::Eight, Rank::Five, Rank::Four, Rank::Jack, Rank::King, 
                    Rank::Nine, Rank::Queen, Rank::Seven, Rank::Six, Rank::Ten, Rank::Three, Rank::Two
                ] 
                {
                    cards.push(Card { rank, suit })
                }
            }
        }
        Deck { cards }
    }
    fn shuffle(&mut self){
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self) -> Option<Card>{
        self.cards.pop()
    }
}
struct Hand{
    cards: Vec<Card>
}
impl Hand{
    fn new() -> Hand{
        Hand {cards: Vec::new()}
    }
    fn add_card(&mut self, card: Card){
        self.cards.push(card);
    }
    fn value(&self) -> u32{
        let mut value = 0;
        let mut num_aces = 0;
        for card in &self.cards {
            value += card.value();
            if card.rank == Rank::Ace {
                num_aces += 1;
            }
        }
        while value > 21 && num_aces > 0 {
            value -= 10;
            num_aces -= 1;
        }
        value
    }
}

fn main() {
    println!("Welcome to Blackjack");
    println!("--------------------");
    let mut credits: u32 = 500;
    let mut count = 0;
    let mut deck = Deck::new();
    while credits > 0 {
        if count == 11{
            deck = Deck::new();
        }
        deck.shuffle();
        println!("wallet: {}", credits);
        println!("--------------------");
        println!("place your bet:");
        let mut bet = get_bet(credits);
        println!("--------------------");
        let mut user_hand = Hand::new();
        let mut dealer_hand = Hand::new();
        
        user_hand.add_card(deck.deal().unwrap());
        dealer_hand.add_card(deck.deal().unwrap());
        user_hand.add_card(deck.deal().unwrap());
        dealer_hand.add_card(deck.deal().unwrap());

        println!("Dealer's Hand: [?], {:?}", dealer_hand.cards[1]);
        println!("Your Hand: {:?}", user_hand.cards);
        println!("Your Hand Value: {}", user_hand.value());

        while user_hand.value() < 21 {
            println!("Hit, Stand or Double (h/s/d)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        if choice == "h" {
            user_hand.add_card(deck.deal().unwrap());
            println!("Your Hand: {:?}", user_hand.cards);
            println!("Your Hand Value: {}", user_hand.value());
            println!("--------------------");
        } 
        else if choice == "s" {
            break;
        }
        else if choice == "d" {
            bet = bet + bet;
            user_hand.add_card(deck.deal().unwrap());
            println!("Your Hand: {:?}", user_hand.cards);
            println!("Your Hand Value: {}", user_hand.value());
            println!("--------------------");
            break;
        }
        }

        if user_hand.value() > 21 {
            println!("You bust! Dealer wins.");
            credits = credits - bet;
            break;
        }

        println!("Dealer's Hand: {:?}", dealer_hand.cards);
        println!("Dealer's Hand Value: {}", dealer_hand.value());
        while dealer_hand.value() < 17 {
        dealer_hand.add_card(deck.deal().unwrap());
        println!("Dealer hits: {:?}", dealer_hand.cards);
        println!("Dealer's Hand Value: {}", dealer_hand.value());
        }

        if dealer_hand.value() > 21 || user_hand.value() > dealer_hand.value() {
            println!("You win!");
            credits += bet;
        } else if user_hand.value() < dealer_hand.value() {
            println!("Dealer wins!");
            credits -= bet;
        } else {
            println!("Push!");
        }
    }

    println!("your credits: {}", credits);
}

fn get_bet(credits: u32) -> u32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to place bet");
    let your_bet: u32 = buffer.trim().parse().unwrap();
    if your_bet > credits {
        println!("Don't have enough credits");
        get_bet(credits)
    }
    else if your_bet <= 0 {
        println!("try again");
        get_bet(credits)
    }
    else {
        println!("your bet is: {}", your_bet);
        return your_bet;
    }
}

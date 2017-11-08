extern crate itertools;
use itertools::Itertools;

fn main() {
    println!("RIP, Nicon!");

    let x : Card = Card{suit:Suit::Heart, value:Value::Three};
    let y : Card = Card{suit:Suit::Diamond, value:Value::Four};
    let z : Card = Card{suit:Suit::Heart, value:Value::Ace};
    let w : Card = Card{suit:Suit::Heart, value:Value::King};

    println!("{:?}",x);
    println!("{:?}",evaluate_trick(x,y,z,w,));
    create_deck().print_deck();
}

//Use Value
fn evaluate_trick(leader:Card, second:Card, responder:Card, fourth:Card)->RelativePlayer {

    let (winner_a, card_a) =
        if compare_card(leader, second,leader.suit) {(RelativePlayer::Leader,leader)}
        else {(RelativePlayer::Second,second)};

    let (winner_b, card_b) =
        if compare_card(responder, fourth,leader.suit) {(RelativePlayer::Responder,responder)}
        else {(RelativePlayer::Fourth,fourth)};

    if compare_card(card_a, card_b, leader.suit) { winner_a }
    else { winner_b }
}

fn compare_card(first:Card, second:Card, startingsuit:Suit)->bool {
    //Return TRUE if first card wins.
    //Return FALSE if second card wins.
    if first.suit == second.suit {
        first.value > second.value
    }
    else if first.suit == Suit::Spade{true}
    else if second.suit == Suit::Spade{false}
    else if second.suit == startingsuit{false}
    else {true}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Suit { Club, Diamond, Heart, Spade }

#[derive(Debug, PartialOrd, Eq, PartialEq, Ord, Copy, Clone)]
enum Value {Two,Three,Four,Five,Six,Seven,Eight,Nine,Ten,Jack,Queen,King,Ace}

#[derive(Debug)]
enum RelativePlayer { Leader, Second, Responder, Fourth }

#[derive(Debug)]
enum Player { North, East, South, West }

#[derive(Debug, Copy, Clone)]
struct Card{suit:Suit, value:Value}

#[derive(Clone)]
struct Deck ([Card; 52]);

impl Deck {

    fn print_deck(&self){
        for x in self.0.iter() {
            println!("{:?}", x);
        }
    }
}


fn create_deck()->Deck {
    let mut ofcards = [Card{suit:Suit::Club, value:Value::Two};52];

    let values = [Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven, Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King, Value::Ace];
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    for (i,(&s,&v)) in suits.iter().cartesian_product(values.iter()).enumerate(){
        ofcards[i] = Card{suit:s,value:v};
    }
    Deck(ofcards)
}
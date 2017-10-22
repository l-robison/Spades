

fn main() {
    println!("Hello, Nicon!");

    let x : Card = Card(Suit::Club, Value::Three);

    println!("{:?}",x);
}

#[derive(Debug)]
enum Suit { Club, Diamond, Heart, Spade }

#[derive(Debug)]
enum Value {Two,Three,Four,Five,Six,Seven,Eight,Nine,Ten,Jack,Queen,King,Ace}


#[derive(Debug)]
struct Card(Suit, Value);

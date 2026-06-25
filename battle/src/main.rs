use rand::distr::StandardUniform;
use rand::prelude::Distribution;
use rand::{Rng, RngExt};
use std::fmt::{Display, Formatter};
use std::io::stdin;

enum Color {
    /// Pique
    Spade,

    /// Coeur
    Heart,

    /// Carreau
    Diamond,

    /// Trefle
    Club,
}

struct Card {
    value: u8,
    color: Color,
}

fn main() {
    let mut rng = rand::rng();
    let cards = 54;

    loop {
        println!("Press enter to draw a card");
        let mut enter = String::new();
        stdin().read_line(&mut enter).expect("Invalid input.");
        println!("You entered: -{}-", enter);
        let player_card = Card {
            value: rng.random_range(1..11),
            color: rng.random(),
        };
        println!("Votre carte : {player_card}");

        let computer_card = Card {
            value: rng.random_range(1..11),
            color: rng.random(),
        };
        println!("Alfred carte : {computer_card}");

        if (player_card.value > computer_card.value) {
            println!("Tu as gagné Youpi GG");
        } else {
            println!("Tu as perdu... noob");
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} de {}", self.value, self.color)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Color::Club => '♣',
            Color::Diamond => '♦',
            Color::Spade => '♠',
            Color::Heart => '♥',
        };
        write!(f, "{}", char)
    }
}

impl Distribution<Color> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let random_color = rng.random_range(0..4);
        match random_color {
            0 => Color::Spade,
            1 => Color::Diamond,
            2 => Color::Heart,
            _ => Color::Club,
        }
    }
}

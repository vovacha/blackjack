use std::io;

enum Result {
    WON,
    LOST,
    UNKNOWN
}

struct Card {
    name: String,
    value: u8,
}

trait VecCardExt {
    fn count_points(&self) -> u8;
    fn stringify(&self) -> String;
}

impl VecCardExt for Vec<Card> {

    fn count_points(&self) -> u8 {
        let total = self.iter().map(|s| s.value).sum();
        if total > 21 {
            let aces = self.iter().filter(|&a| *&a.name == "A").count();
            return total - aces as u8 * 10
        }
        total
    }
    
    fn stringify(&self) -> String {
        self.iter().map(|c| c.name.to_string()).collect::<Vec<_>>().join(",")
    }
}

struct Dealer {
    deck: Vec<Card>,
    cards: Vec<Card>,
    player_cards: Vec<Card>,
}


impl Dealer {
    pub fn new() -> Dealer {
        let mut deck = Vec::with_capacity(52);
        for i in 2..15 {
            if i < 11 {
                // 2-10
                // TODO: seems suboptimal
                for _ in 0..4 { deck.push( Card{name: i.to_string(), value: i} ) }
            } else if i == 11 {
                // Jack
                for _ in 0..4 { deck.push( Card{name: String::from("J"), value: 10} ) }
            } else if i == 12 {
                // Queen
                for _ in 0..4 { deck.push( Card{name: String::from("Q"), value: 10} ) }
            } else if i == 13 {
                // King
                for _ in 0..4 { deck.push( Card{name: String::from("K"), value: 10} ) }
            } else if i == 14 {
                // Ace
                for _ in 0..4 { deck.push( Card{name: String::from("A"), value: 11} ) }
            } 
        }
        Dealer { deck, cards: vec![], player_cards: vec![] }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        self.deck.shuffle(&mut thread_rng());
    } 

    fn game_init(&mut self) {
        for _ in 0..2 {
            self.cards.push(self.deck.pop().unwrap());
            self.player_cards.push(self.deck.pop().unwrap());
        }
    }

    fn game_dealer(&mut self) -> Result {
        loop {
            let points = self.cards.count_points();
            if points < 17 {
                let card = self.deck.pop().unwrap();
                println!("Dealer's card is {}", card.name);
                self.cards.push(card);
                continue }
            else if points == 21 { return Result::WON }
            else if points > 21 { return Result::LOST }
            else { return Result::UNKNOWN }
        }
    }

    fn game_player(&mut self) -> Result {
        loop {
            let points = self.player_cards.count_points();
            if points == 21 { return Result::WON }
            else if points > 21 { return Result::LOST }
            else {
                let mut choice = String::new();
                println!("Do you want one more card: y/n");
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");
                choice = choice.trim().to_lowercase();
    
                match &choice[..] {
                    "y" => {
                        let card = self.deck.pop().unwrap();
                        println!("Your card is {}", card.name);
                        self.player_cards.push(card);
                        continue
                    },
                    "n" => return Result::UNKNOWN,
                    _ => {
                        println!("Wrong choice: y/n");
                        continue
                    },
                }
            }
        }
    }

    fn play(&mut self) {
        self.shuffle();
        self.game_init();
        self.print_msg("Game started!");

        match self.game_player() {
            Result::WON => self.print_msg("Blackjack! You won!"),
            Result::LOST => self.print_msg("You lose."),
            Result::UNKNOWN => {
                match self.game_dealer() {
                    Result::WON => self.print_msg("Blackjack! Computer won!"),
                    Result::LOST => self.print_msg("You won!"),
                    Result::UNKNOWN => {
                        let player_points = self.player_cards.count_points();
                        let dealer_points = self.cards.count_points();
                        if player_points > dealer_points {
                            self.print_msg("You won!");
                        } else if dealer_points > player_points {
                            self.print_msg("You lose.");
                        } else {
                            self.print_msg("That's a draw.");
                        }
                    }
                }
            }
        }
    }

    fn print_msg(&self, message: &str) {
        println!("{}", message);
        println!("Dealer's cards are: {}", self.cards.stringify());
        println!("Your cards are: {}", self.player_cards.stringify());
    }

}

fn main() {
    Dealer::new().play();
}

// TODO: Add tests
// TODO: Add docs

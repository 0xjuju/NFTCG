
#[allow(dead_code)]
#[allow(unused_variables)]
pub mod template {
    

    trait IsGame {}
    trait IsPlayer {}

    trait IsZone {}

    trait GameRules {
        fn new_game(player1: Player, player2: Player) -> Self;
        fn end_game(&mut self) -> GameResult;

        // Move a card from one zone to another
        fn move_card(from: &mut Vec<Card>, to: &mut Vec<Card>) -> bool {  
            if let Some(card) = from.pop() {
                to.push(card);
                return true
            }
            false
        }
    }

    trait HandContent {
        fn new() -> Self;
    }

    trait PlayerOptions {
        fn new_player(&self, name: String, avatar: Avatar, deck: Deck, hand: Hand) -> Self;
        fn hand_size(&self) -> u8;
        fn draw_card(&mut self);
    }

    enum CardType {
        Event(EventType),
        Equipment(EquipmentType),
        Character(String),
    }

    // Tyoe of equipment for CardType::Equipment
    enum EquipmentType {
        Weapon,
        Shield,
        Vehicle,
        Armor,
        Item,
    }

    // type for CardType::Event
    enum EventType {
        Instant,
        Normal
    }

    enum GameResult {
        Winner(Player),
        Draw

    }

    // Location of card in game
    enum Location {
        Deck,
        Field,
        Discard,
        Hand,
        Exiled,
    }

    // Different catchphrases for Avatar
    enum Phrase {
        Win(String),
        Lose(String),
        GameStart(String),
    }

    // type of reserve for Avatar
    enum ReserveType {
        Ammo(u16),
        Magic(u8),
    }
    
    /// Structs Defined

    // The Main Character for the game
    pub struct Avatar {
        name: String,
        health: u8,
        power: u8,
        speed: u8,
        defense: u8,
        reserves: ReserveType,
        special: SpecailAbility,
        catchphrase: Phrase,
    }


    pub struct Card {
        name: String,
        card_type: CardType,  
        location: Location,
        ability: String, 
    }

    pub struct Deck {
        cards: Vec<Card>,
        max_size: u8
    }

    pub struct Field {}

    pub struct Game {
        field: Field,
        player1: Player,
        player2: Player,
        winner: Option<Player>,
    }

    impl GameRules for Game {
        fn new_game(player1: Player, player2: Player) -> Self {
            let field = Field {};
            Self {
                field,
                player1,
                player2,
                winner: None
            }
        }

        fn end_game(&mut self)  -> GameResult  {
            GameResult::Draw
        }
    }
    

    pub struct Hand {
        cards: Vec<Card>,
    }

    impl HandContent for Hand {
        fn new() -> Self {
            Self {
                cards: Vec::new()
            }
        }
    }

    pub struct Player {
        name: String,
        avatar: Avatar,
        deck: Deck,
        hand: Hand,
    }

    impl IsPlayer for Player {}
    impl PlayerOptions for Player where Player: IsPlayer {
        fn new_player(&self, name: String, avatar: Avatar, deck: Deck, hand: Hand) -> Self {
            Self {
                name,
                avatar,
                deck,
                hand
            }
        }

        fn draw_card(&mut self) {
            let card_drawn = Game::move_card(&mut self.deck.cards, &mut self.hand.cards);
           
        }

        fn hand_size(&self) -> u8 {
            self.hand.cards.len() as u8
        }
    }

    // Unique ability for Avatar. Activated once per game
    pub struct SpecailAbility {
        name: String,
        text: String,
        used: bool,
    }

    #[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_game_setup() {
        let field = Field {};

        let deck1 = Deck {
            max_size: 40,
            cards: vec![
                Card {
                    name: "mirror".to_string(),
                    card_type: CardType::Event(EventType::Normal),
                    ability: "copy permanent".to_string(),
                    location: Location::Deck,
                },

                Card {
                    name: "See".to_string(),
                    card_type: CardType::Event(EventType::Normal),
                    ability: "Look at opponent's hand.".to_string(),
                    location: Location::Deck,
                },

                Card {
                    name: "shadow".to_string(),
                    card_type: CardType::Character("sidekick".to_string()),
                    ability: "+1 speed to general".to_string(),
                    location: Location::Deck,
                },
            ]

        };

        let deck2 = Deck {
            max_size: 40,
            cards: vec![
                Card {
                    name: "mirror".to_string(),
                    card_type: CardType::Event(EventType::Normal),
                    ability: "copy permanent".to_string(),
                    location: Location::Deck,
                },

                Card {
                    name: "See".to_string(),
                    card_type: CardType::Event(EventType::Normal),
                    ability: "Look at opponent's hand.".to_string(),
                    location: Location::Deck,
                },

                Card {
                    name: "shadow".to_string(),
                    card_type: CardType::Character("sidekick".to_string()),
                    ability: "+1 speed to general".to_string(),
                    location: Location::Deck,
                },
            ]

        };

        let avatar1 = Avatar {
            name: "Clout the Blue Whale".to_string(),
            speed: 25,
            health: 255,
            defense: 155,
            power: 225,
            reserves: ReserveType::Magic(0),
            special: SpecailAbility { name: "swallow".to_string(), text: "-100 to general".to_string(), used: false },
            catchphrase: Phrase::GameStart("aaarrrrooooooo".to_string())


        };

        let avatar2 = Avatar {
            name: "Sharky the quick-tipped".to_string(),
            speed: 246,
            health: 150,
            defense: 75,
            power: 245,
            reserves: ReserveType::Magic(0),
            special: SpecailAbility { name: "bop and move".to_string(), text: "Opposing General misses next attack, take two attacks".to_string(), used: false },
            catchphrase: Phrase::GameStart("You couldn't hit a slug swinging like that!".to_string())
        };

        let hand1 = Hand::new();
        let hand2 = Hand::new();

        let p1 = Player {
            name: "Player1".to_string(),
            deck: deck1,
            hand: hand1,
            avatar: avatar1,


        };

        let p2 = Player {
            name: "Player1".to_string(),
            deck: deck2,
            hand: hand2,
            avatar: avatar2,


        };


        let mut game = Game::new_game(p1, p2 ); 
        assert_eq!(game.player1.hand.cards.len(), 0);
        assert_eq!(game.player1.deck.cards.len(), 3);

        game.player1.draw_card();
        assert_eq!(game.player1.hand_size(), 1);
        assert_eq!(game.player1.deck.cards.len(), 2);


        
    }

}

}




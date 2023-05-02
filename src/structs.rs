pub mod GameStructs {
    use crate::enums::GameEnums::*;
    use crate::traits::GameTraits::*;

    // The Main Character for the game
    pub struct Avatar {
        name: String,
        battlefields: Vec<String>,
        action_counter: u8,
        health: u8,
        power: u8,
        speed: u8,
        defense: u8,
        reserves: ReserveType,
        special: SpecailAbility,
        catchphrase: Phrase,
    }

    pub struct Battlefield {
        field: String
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

    pub struct Discard {}

    pub struct Field {}

    pub struct Game {
        field: Field,
        battlefield: Battlefield,
        player1: Player,
        player2: Player,
        turn: u8,
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
}
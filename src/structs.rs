
#[allow(dead_code)]
#[allow(unused_variables)]
pub mod game_structs {
    use rand::Rng;

    use crate::enums::game_enums::*;
    use crate::traits::game_traits::*;
    use crate::state::state_vars::*;



    pub struct Action {
        counter: u8,
        max: u8,
    }

    impl ActionTrait for Action {
        fn new(counter: u8, max: u8) -> Self {
            Self {
                counter,
                max
            }
        }
    }

    // The Main Character for the game
    pub struct Avatar {
        name: String,
        battlefields: Vec<String>,
        actions: Action,
        health: u8,
        power: u8,
        speed: u8,
        specials: Vec<SpecialAbility>,
        catchphrase: Phrase,
    }

    impl AvatarTrait for Avatar {
        fn deploy() {
            unimplemented!()
        }

        fn new(name: String, battlefields: Vec<String>, actions: Action, health: u8, power: u8, speed: u8, specials: Vec<SpecialAbility>, catchphrase: Phrase) -> Self {
            
            Self {
                name, battlefields, actions, health, power, speed, specials, catchphrase
            }
        }
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

    impl CardTrait for Card {
        fn new(name: String, card_type: CardType, ability: String) -> Self {
            Self {
                name,
                card_type,
                location: Location::Limbo,
                ability
            }
        }
    }

    pub struct Deck {
        cards: Vec<Card>,
        max_size: u8
    }

    impl DeckTrait for Deck {
        fn new(cards: Cards, max_size: u8) -> Self {    

            Self {
                cards, 
                max_size
            } 
        }
    }

    pub struct Discard {
        cards: Vec<Card>
    }

    impl DiscardTrait for Discard {
        fn new() -> Self {
            Self {
                cards: Vec::new()
            }
        }
    }

    pub struct Field {}

    pub struct Game {
        pub field: Field,
        pub battlefield: Battlefield,
        pub player1: Player,
        pub player2: Player,
        pub turn: u8,
        pub winner: Option<Player>,
    }
    
    impl GameRules for Game {


        fn end_game(&mut self)  -> GameResult  {
            unimplemented!()
        }

        fn end_turn(&mut self) {
            unimplemented!()
        }
        
        fn flip_coin() -> CoinFlip {
            let  mut rng = rand::thread_rng();
            let num = rng.gen_range(1..=2);

            match num {
                1 => CoinFlip::Heads,
                2 => CoinFlip::Tails,
                _ => unreachable!("Unexpected value")
            }
        }


        fn new_game(player1: Player, player2: Player) -> Self {

            // choose which player starts with the battlefield
            let coin_flip = Self::flip_coin();
            let battlefield_owner = match coin_flip {
                CoinFlip::Heads => &player1.avatar,
                CoinFlip::Tails => &player2.avatar
            };

            let bf = &battlefield_owner.battlefields[0];
            let battlefield = Battlefield { field: bf.to_string() };

            let field = Field {};

            Self {
                field,
                battlefield,
                player1,
                player2,
                turn: 0,
                winner: None
            }
        }

        fn new_turn(&mut self) {
            unimplemented!()
        }

        fn reset_action_counters(&mut self) {
            unimplemented!()
        }


    }
    

    pub struct Hand {
        cards: Vec<Card>,
    }

    impl HasCards for Hand {
        fn get_cards(&mut self) -> &mut Cards {
            &mut self.cards
        }
    }

    impl HandTrait for Hand {

        fn new() -> Self {
            Self {
                cards: Vec::new()
            }
        }

        fn reveal(&self) -> Vec<Card> {
            unimplemented!()
        }
    }

    pub struct Player {
        pub name: String,
        pub avatar: Avatar,
        pub discard: Discard,
        pub deck: Deck,
        pub hand: Hand,
    }

    impl PlayerOptions for Player {

        fn deck_size(&self) -> u8 {
            self.deck.cards.len() as u8
        }

        fn discard_card(player: Player) {
            unimplemented!()
        }
        
        fn draw_card(&mut self, location: TopOrBottom) {
            let card = self.deck.cards.pop();

            match card {
                Some(card) => Self::move_card(card, &mut self.hand, location),
                None => ()
            }
        }

        fn draw_cards(&mut self, mut num: u8, location: TopOrBottom) {
            
            while num > 0 {
                self.draw_card(location);
                num -= 1;
            }
        }

        fn hand_size(&self) -> u8 {
            self.hand.cards.len() as u8
        }

        fn new_player(name: String, avatar: Avatar, deck: Deck) -> Self {
            let hand = Hand::new();
            let discard = Discard::new();

            Self {
                name,
                avatar,
                deck,
                hand,
                discard
            }
        }

        fn shuffle_deck(&mut self) {
            unimplemented!()
        }

        fn shuffle_hand(&mut self) {
            unimplemented!()
        }

    }

    // Unique ability for Avatar. Activated once per game
    pub struct SpecialAbility {
        name: String,
        text: String,
        used: bool,
    }

    impl SpecialAbilityTrait for SpecialAbility {
        fn new(name: String, text: String) -> Self {
            Self {
                name,
                text,
                used: false 
            }
        }
    }
}
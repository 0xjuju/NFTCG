
#[allow(dead_code)]
#[allow(unused_variables)]
pub mod game_structs {

    use rand::Rng;
    use std::cmp::{Eq, PartialEq};
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::enums::game_enums::*;
    use crate::traits::game_traits::*;
    use crate::state::state_vars::*;


    #[derive(Clone, Eq, PartialEq)]
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
    #[derive(Clone, Eq, PartialEq)]
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

    #[derive(Clone)]
    pub struct Battlefield {
        field: String
    }


    #[derive(Clone, Eq, PartialEq)]
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

    #[derive(Clone, Eq, PartialEq)]
    pub struct Deck {
        pub cards: Cards,
        pub max_size: u8
    }

    impl DeckTrait for Deck {
        fn new(cards: Cards, max_size: u8) -> Self {    

            Self {
                cards, 
                max_size
            } 
        }
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct DiscardPile {
        pub cards: Cards
    }

    impl DiscardTrait for DiscardPile {
        fn new() -> Self {
            Self {
                cards: Vec::new()
            }
        }
    }

    #[derive(Clone)]
    pub struct Field {}

    #[derive(Clone)]
    pub struct Game {
        pub field: Field,
        pub battlefield: Battlefield,
        pub player1: Rc<RefCell<Player>>,
        pub player2: Rc<RefCell<Player>>,
        pub turn: u8,
        pub winner: Option<Player>,
    }

    impl GameRules for Game {

        fn begin_game(&mut self) {
            unimplemented!()
        }

        fn end_game(&self, loser: Player, condition: VictoryCondition)  -> GameResult  {
            unimplemented!()
            // match condition {
            //     VictoryCondition::None => GameResult::Draw,
            //     _ => match self.player1 == loser {
            //         true => GameResult::Winner(&self.player2),
            //         false => GameResult::Winner(&self.player1)
            //    }
            // }
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


        fn new_game(player1: Rc<RefCell<Player>>, player2: Rc<RefCell<Player>>) -> Self {
            let p1_borrowed = player1.borrow();
            let p2_borrowed = player2.borrow();
            // choose which player starts with the battlefield
            let coin_flip = Self::flip_coin();
            let battlefield_owner = match coin_flip {
                CoinFlip::Heads => &p1_borrowed.avatar,
                CoinFlip::Tails => &p2_borrowed.avatar
            };

            let bf = &battlefield_owner.battlefields[0];
            let battlefield = Battlefield { field: bf.to_string() };

            let field = Field {};

            Self {
                field,
                battlefield,
                player1: player1.clone(),
                player2: player2.clone(),
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
    

    #[derive(Clone, Eq, PartialEq)]
    pub struct Hand {
        pub cards: Cards,
    }


    impl HandTrait for Hand {

        fn new() -> Self {
            Self {
                cards: Vec::new()
            }
        }

        fn reveal(&self) -> Cards {
            unimplemented!()
        }
    }

    #[derive(Clone)]
    pub struct Player {
        pub name: String,
        pub opponent: Option<Rc<RefCell<Player>>>,
        pub avatar: Avatar,
        pub discard_pile: DiscardPile,
        pub deck: Deck,
        pub hand: Hand,
    }

    impl PlayerOptions for Player {

        fn deck_size(&self) -> u8 {
            self.deck.cards.len() as u8
        }

        fn discard_card_from_hand(&mut self, target: Target) {

            match target {
                Target::SELF => {
                    let card = self.hand.card_selector();
                    self.move_to_discard(card);
                },
                Target::Oppoenet => {
                    let mut player = self.opponent.as_ref().unwrap().borrow_mut();
                    let card = player.hand.card_selector();
                    player.move_to_discard(card);
                },
                _ => panic!("Unexpected value")
            }
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

        fn move_to_discard(&mut self, card: Card) {
            Self::move_card(card, &mut self.discard_pile, TopOrBottom::Top);
        }

        fn new_player(name: String, avatar: Avatar, deck: Deck) -> Self {
            let hand = Hand::new();
            let discard_pile = DiscardPile::new();

            Self {
                name,
                opponent: None,
                avatar,
                deck,
                hand,
                discard_pile
            }
        }

        fn set_opponent(&mut self, opponent: Rc<RefCell<Player>>) {
            self.opponent = Some(opponent);
        }

        fn shuffle_deck(&mut self) {
            unimplemented!()
        }

        fn shuffle_hand(&mut self) {
            unimplemented!()
        }

    }

    #[derive(Clone, Eq, PartialEq)]
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


pub mod game_traits {
    use rand;
    use rand::prelude::SliceRandom;

    use crate::enums::game_enums::*;
    use crate::state::state_vars::*;
    use crate::structs::game_structs::*;


    /// Any trait that implements is expected to be card zone with a field like `cards: Vec<Card>'
    pub trait HasCards {

        // number of cards in held
        fn card_count(&self) -> u8;

        // Search through cards and select specific card
        fn card_selector(&mut self) -> Card;

        // Retun list of cards held by zone
        fn get_cards(&mut self) -> &mut Cards;
    }

    // impl using macro since when can't make default implementations from trait directly
    macro_rules! impl_has_cads_for_generics {
        ( $( $t:ty ),+ $(,)? ) => ($(
            impl HasCards for $t {

                fn card_count(&self) -> u8 {
                    self.cards.len() as u8
                }
        
                fn card_selector(&mut self) -> Card {
                    unimplemented!()
                }
        
                fn get_cards(&mut self) -> &mut Cards {
                    &mut self.cards
                }
            }
        )+)
    }

    impl_has_cads_for_generics!(
        DiscardPile,
        Hand,
        Deck
    );

    pub trait IsAvatar {}
    pub trait IsGame {}
    pub trait IsZone {}

    /// Actions are owned by avatars and manages how many moves an Avatar can make on their turn
    pub trait ActionTrait {
        fn new(counter: u8, max: u8) -> Self;
    }

    // Avatars are owned by Player types, and is the main actor for in-game actions like attacking, health etc.
    pub trait AvatarTrait {
        fn deploy();
        fn new(name: String, battlefields: Vec<String>, actions: Action, health: u8, power: u8, speed: u8, specials: Vec<SpecialAbility>, catchphrase: Phrase) -> Self;
    } 
    
    // Battlefields are owned by Avatars. Used to decide the location of the game battlefield. Owner of Battlefield acts first each turn
    pub trait BattlefieldTrait {

        // change battlefield to the first one owned by avatar
        fn change_battlefield(&mut self, avatar: Avatar);

        // matches owner to the current battlefield
        fn owner(&self) -> Player;
    }

    pub trait CardTrait {
        fn new(name: String, card_type: CardType, ability: String) -> Self;
    }

    pub trait DeckTrait {
        fn new(cards: Cards, max_size: u8) -> Self;
    }

    pub trait DiscardTrait {
        fn new() -> Self;
    }

    /// Define the game's setup and ending conditions. Also manages the state of the game and turn transitions
    pub trait GameRules {
        fn begin_game(&mut self);
        fn end_game(&self, loser: Player, condition: VictoryCondition) -> GameResult;
        fn end_turn(&mut self);
        fn flip_coin() -> CoinFlip;
        fn new_game(player1: Player, player2: Player) -> Self;
        fn new_turn(&mut self);
        fn reset_action_counters(&mut self);

        fn randomize_cards(cards: &mut Vec<Card>) {
            cards.shuffle(&mut rand::thread_rng())
        }
    }

    pub trait HandTrait {
        fn new() -> Self;
        fn reveal(&self) -> Vec<Card>;

    }

    pub trait PlayerOptions {
        fn deck_size(&self) -> u8;
        fn draw_card(&mut self, location: TopOrBottom);
        fn draw_cards(&mut self, num: u8, location: TopOrBottom);
        fn hand_size(&self) -> u8;
        fn move_to_discard(&mut self, card: Card);
        fn new_player(name: String, avatar: Avatar, deck: Deck) -> Self;
        fn set_opponent(&mut self, opponent: Player);
        fn shuffle_deck(&mut self);
        fn shuffle_hand(&mut self);


        // Move a card to a specific card zone. Top or bottom
        fn move_card<T: HasCards>(card: Card, zone: &mut T, location: TopOrBottom) {
            use TopOrBottom::*;
            match location {
                Top => zone.get_cards().push(card),
                Bottom => zone.get_cards().insert(0, card),
            }    
        }
    }

    // Special ability owned by Avatar.
    pub trait SpecialAbilityTrait {
        fn new(name: String, text: String) -> Self;
    }
}


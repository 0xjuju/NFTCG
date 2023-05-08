

pub mod game_traits {
    use rand;
    use rand::prelude::SliceRandom;

    use crate::enums::game_enums::*;
    use crate::state::state_vars::*;
    use crate::structs::game_structs::*;

    pub trait HasCards {
        fn get_cards(&mut self) -> &mut Cards;
    }

    pub trait IsAvatar {}
    pub trait IsGame {}
    pub trait IsZone {}

    pub trait ActionTrait {
        fn new(counter: u8, max: u8) -> Self;
    }

    pub trait AvatarTrait {
        fn deploy();
        fn new(name: String, battlefields: Vec<String>, actions: Action, health: u8, power: u8, speed: u8, specials: Vec<SpecialAbility>, catchphrase: Phrase) -> Self;
    } 
    
    pub trait BattlefieldTrait {
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

    /// Define main rule for game
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
        fn discard_card(player: Player);
        fn draw_card(&mut self, location: TopOrBottom);
        fn draw_cards(&mut self, num: u8, location: TopOrBottom);
        fn hand_size(&self) -> u8;
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

    pub trait SpecialAbilityTrait {
        fn new(name: String, text: String) -> Self;
    }
}


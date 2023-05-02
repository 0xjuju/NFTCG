
pub mod GameTraits {
    use crate::structs::GameStructs::*;
    use crate::enums::GameEnums::*;

    pub trait IsAvatar {}
    pub trait IsGame {}
    pub trait IsPlayer {}

    pub trait IsZone {}

    pub trait BattlefieldTrait {

        // matches owner to the current battlefiel
        fn owner(&self) -> Player;
    }

    /// Define main rule for game
    pub trait GameRules {
        fn change_battlefield(&mut self, avatar: Avatar);
        fn end_game(&mut self) -> GameResult;
        fn new_game(player1: Player, player2: Player) -> Self;
        fn new_turn(&mut self);
        
        
        // Move a card from one zone to another
        fn move_card(from: &mut Vec<Card>, to: &mut Vec<Card>) -> bool {  
            if let Some(card) = from.pop() {
                to.push(card);
                return true
            }
            false
        }
    }

    pub trait HandContent {
        fn new() -> Self;
    }

    pub trait PlayerOptions {
        fn new_player(&self, name: String, avatar: Avatar, deck: Deck, hand: Hand) -> Self;
        fn hand_size(&self) -> u8;
        fn draw_card(&mut self);
    }
}


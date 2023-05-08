pub mod game_enums {
    use crate::structs::game_structs::*;


    pub enum CoinFlip {
        Heads,
        Tails,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub enum CardType {
        Event(EventType),
        Equipment(EquipmentType),
        Character(String),
    }

    // Tyoe of equipment for CardType::Equipment
    #[derive(Clone, Eq, PartialEq)]
    pub enum EquipmentType {
        Weapon,
        Shield,
        Vehicle,
        Armor,
        Item,
    }

    #[derive(Clone, Eq, PartialEq)]
    // type for CardType::Event
    pub enum EventType {
        Instant,
        Normal
    }

    pub enum GameResult<'a> {
        Winner(&'a Player),
        Draw

    }

    #[derive(Eq, PartialEq)]
    // Location of card in game
    #[derive(Copy, Clone)]
    pub enum Location {
        Deck,
        Field,
        Discard,
        Hand,
        Limbo,
    }

    #[derive(Clone, Eq, PartialEq)]
    // Different catchphrases for Avatar
    pub enum Phrase {
        Win(String),
        Lose(String),
        GameStart(String),
    }

    #[derive(Copy, Clone)]
    pub enum TopOrBottom {
        Top,
        Bottom
    }

    pub enum VictoryCondition {
        AvatarDefeated,
        Deckout,
        None
    }

}
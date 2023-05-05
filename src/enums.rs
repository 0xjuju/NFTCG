pub mod game_enums {
    use crate::structs::game_structs::*;


    pub enum CoinFlip {
        Heads,
        Tails,
    }

    pub enum CardType {
        Event(EventType),
        Equipment(EquipmentType),
        Character(String),
    }

    // Tyoe of equipment for CardType::Equipment
    pub enum EquipmentType {
        Weapon,
        Shield,
        Vehicle,
        Armor,
        Item,
    }

    // type for CardType::Event
    pub enum EventType {
        Instant,
        Normal
    }

    pub enum GameResult {
        Winner(Player),
        Draw

    }


    // Location of card in game
    #[derive(Copy, Clone)]
    pub enum Location {
        Deck,
        Field,
        Discard,
        Hand,
        Limbo,
    }

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

}
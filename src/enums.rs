pub mod GameEnums {
    use crate::structs::GameStructs::*;

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
    pub enum Location {
        Deck,
        Field,
        Discard,
        Hand,
        Exiled,
    }

    // Different catchphrases for Avatar
    pub enum Phrase {
        Win(String),
        Lose(String),
        GameStart(String),
    }
    // type of reserve for Avatar
    pub enum ReserveType {
        Ammo(u16),
        Magic(u8),
    }
}
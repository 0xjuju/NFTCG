
pub mod Template {

    enum CardType {
        Event,
        Equipment,
        

    }


    enum Location {
        Field,
        Discard,
        Hand,
        Exiled,
    }

    // Different catchphrases for Avatar
    enum Phrase {
        win(String),
        Lose(String),
        GameStart(String),
    }

    // type of reserve for Avatar
    enum ReserveType {
        Ammo(u16),
        Magic(u8),
    }

    // Unique ability for Avatar. Activated once per game
    struct SpecailAbility {
        name: String,
        text: String,
        used: bool,
    }

    // The Main Character for the game
    pub struct Avatar {
        name: String,
        health: u8,
        power: u8,
        speed: u8,
        defense: u8,
        reserves: ReserveType,
        special: SpecialAbility,
        catchphrase: Phrase,
    }

    pub struct Card {
        name: String,
        card_type: String,  
        location: Location,    
    }

    pub struct Deck {
        quantity: u8,
    }
}

#[cfg(test)]
mod tests {

}

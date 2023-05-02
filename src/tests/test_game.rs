#[cfg(test)]
mod tests {
    use crate::enums::GameEnums::*;
    use crate::structs::GameStructs::*;
    use crate::traits::GameTraits::*;

    #[test]
    fn test_game_setup() {
        let field = Field {};

        let deck1 = Deck {
            max_size: 40,
            cards: vec![
                Card {
                    name: "mirror".to_string(),
                    card_type: CardType::Event(EventType::Normal),
                    ability: "copy permanent".to_string(),
                    location: Location::Deck,
                },

                Card {
                    name: "See".to_string(),
                    card_type: CardType::Event(EventType::Normal),
                    ability: "Look at opponent's hand.".to_string(),
                    location: Location::Deck,
                },

                Card {
                    name: "shadow".to_string(),
                    card_type: CardType::Character("sidekick".to_string()),
                    ability: "+1 speed to general".to_string(),
                    location: Location::Deck,
                },
            ]

        };

        let deck2 = Deck {
            max_size: 40,
            cards: vec![
                Card {
                    name: "mirror".to_string(),
                    card_type: CardType::Event(EventType::Normal),
                    ability: "copy permanent".to_string(),
                    location: Location::Deck,
                },

                Card {
                    name: "See".to_string(),
                    card_type: CardType::Event(EventType::Normal),
                    ability: "Look at opponent's hand.".to_string(),
                    location: Location::Deck,
                },

                Card {
                    name: "shadow".to_string(),
                    card_type: CardType::Character("sidekick".to_string()),
                    ability: "+1 speed to general".to_string(),
                    location: Location::Deck,
                },
            ]

        };

        let avatar1 = Avatar {
            name: "Clout the Blue Whale".to_string(),
            homefield: "Sea".to_string(),
            speed: 25,
            health: 255,
            defense: 155,
            power: 225,
            reserves: ReserveType::Magic(0),
            special: SpecailAbility { name: "swallow".to_string(), text: "-100 to general".to_string(), used: false },
            catchphrase: Phrase::GameStart("aaarrrrooooooo".to_string())


        };

        let avatar2 = Avatar {
            name: "Sharky the quick-tipped".to_string(),
            homefield: "shipwreck".to_string(),
            speed: 246,
            health: 150,
            defense: 75,
            power: 245,
            reserves: ReserveType::Magic(0),
            special: SpecailAbility { name: "bop and move".to_string(), text: "Opposing General misses next attack, take two attacks".to_string(), used: false },
            catchphrase: Phrase::GameStart("You couldn't hit a slug swinging like that!".to_string())
        };

        let hand1 = Hand::new();
        let hand2 = Hand::new();

        let p1 = Player {
            name: "Player1".to_string(),
            deck: deck1,
            hand: hand1,
            avatar: avatar1,


        };

        let p2 = Player {
            name: "Player1".to_string(),
            deck: deck2,
            hand: hand2,
            avatar: avatar2,


        };


        let mut game = Game::new_game(p1, p2 ); 
        assert_eq!(game.player1.hand.cards.len(), 0);
        assert_eq!(game.player1.deck.cards.len(), 3);

        game.player1.draw_card();
        assert_eq!(game.player1.hand_size(), 1);
        assert_eq!(game.player1.deck.cards.len(), 2);


        
    }

}
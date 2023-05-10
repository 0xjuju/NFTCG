

pub mod build_data {
    
    use NFTCG::enums::game_enums::*;
    use NFTCG::structs::game_structs::*;
    use NFTCG::traits::game_traits::*;


    pub fn mach_game() -> Game {
        let max_deck_size = 40;

        let deck1 = Deck::new(
            vec![
                Card::new(
                    "mirror".to_string(),
                    CardType::Event(EventType::Normal),
                    "copy permanent".to_string(),
                ),
                
                Card::new(
                    "See".to_string(),
                    CardType::Event(EventType::Normal),
                    "Look at opponent's hand.".to_string(),
                ),

                Card::new(
                    "shadow".to_string(),
                    CardType::Character("sidekick".to_string()),
                    "+1 speed to general".to_string(),
                ),
            ],

            max_deck_size

        );

        let deck2 = Deck::new(
            vec![
                Card::new(
                    "mirror".to_string(),
                    CardType::Event(EventType::Normal),
                    "copy permanent".to_string(),
                ),

                Card::new(
                    "See".to_string(),
                    CardType::Event(EventType::Normal),
                    "Look at opponent's hand.".to_string(),
                ),

                Card::new(
                    "shadow".to_string(),
                    CardType::Character("sidekick".to_string()),
                    "+1 speed to general".to_string(),
                ),
            ],

            max_deck_size

        );


        let av1_actions = Action::new(
            1,
            1
        );

        let av2_actions = Action::new(
            1,
            1
        );
        
        let avatar1 = Avatar::new(
            "Clout the Blue Whale".to_string(),
            vec!["Sea".to_string(), "Deep Sea".to_string()],
            av1_actions,
            255,
            225,
            70,
            vec![SpecialAbility::new( "swallow".to_string(), "-100 to general".to_string())],
            Phrase::GameStart("aaarrrrooooooo".to_string()),

        );
        
        let avatar2 = Avatar::new(
            "Torpedo Sharky".to_string(),
            vec!["Shipwreck".to_string()],
            av2_actions,
            150,
            175,
            240,
            vec![SpecialAbility::new("Double Strike".to_string(), "50 dmg, take addition action".to_string())],
            Phrase::GameStart("You move like a Lambo. So slow!!!".to_string()),

        );

        let mut p1 = Player::new_player (
            "Player1".to_string(),
            avatar1,
            deck1,
        );

        let mut p2 = Player::new_player (
            "Player2".to_string(),
            avatar2,
            deck2,
        );

        p1.set_opponent(p2.clone());
        p2.set_opponent(p1.clone());

        let game = Game::new_game(p1, p2);

        
        game

    }
}





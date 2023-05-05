mod build_test_data;

#[cfg(test)]
mod tests {
    use crate::build_test_data::build_data::*;
    use NFTCG::enums::game_enums::*;
    use NFTCG::traits::game_traits::*;


    #[test]
    fn draw_cards_from_zones() {
        use TopOrBottom::*;

        let mut game = mach_game();

        // Draw card from top of deck
        assert_eq!(game.player1.hand_size(), 0);
        assert_eq!(game.player1.deck_size(), 3);
        game.player1.draw_card(Top);
        assert_eq!(game.player1.hand_size(), 1);
        assert_eq!(game.player1.deck_size(), 2);

        // draw from bottom
        game.player1.draw_card(Bottom);
        assert_eq!(game.player1.hand_size(), 2);
        assert_eq!(game.player1.deck_size(), 1);

        // draw multiple cards
        let mut game = mach_game();
        assert_eq!(game.player1.hand_size(), 0);
        assert_eq!(game.player1.deck_size(), 3);
        game.player1.draw_cards(2, Top);
        assert_eq!(game.player1.hand_size(), 2);
        assert_eq!(game.player1.deck_size(), 1);

    }

    #[test]
    fn setup() {
        let game = mach_game();
        assert_eq!(game.player1.hand_size(), 0);
        assert_eq!(game.player1.deck_size(), 3);


    }

}
mod build_test_data;

#[cfg(test)]
mod tests {
    use crate::build_test_data::build_data::*;
    use NFTCG::enums::game_enums::*;
    use NFTCG::traits::game_traits::*;


    #[test]
    fn draw_discard_cards_from_zones() {
        use TopOrBottom::*;

        let game = mach_game();

        let mut player1 = game.player1.borrow_mut();
        // let mut player2 = game.player2.borrow_mut();

        // Draw card from top of deck
        assert_eq!(player1.hand_size(), 0);
        assert_eq!(player1.deck_size(), 3);
        player1.draw_card(Top);
        assert_eq!(player1.hand_size(), 1);
        assert_eq!(player1.deck_size(), 2);

        // draw from bottom
        player1.draw_card(Bottom);
        assert_eq!(player1.hand_size(), 2);
        assert_eq!(player1.deck_size(), 1);

        // draw multiple cards
        let game = mach_game();
        let player1 = game.player1;
        let player2 = game.player2;
        
        assert_eq!(player1.borrow().hand_size(), 0);
        assert_eq!(player1.borrow().deck_size(), 3);
        player1.borrow_mut().draw_cards(2, Top);
        assert_eq!(player1.borrow().hand_size(), 2);
        assert_eq!(player1.borrow().deck_size(), 1);

        // Move card to discard pile
        assert_eq!(player1.borrow().discard_pile.card_count(), 0);
        let card = player1.borrow_mut().hand.cards.pop().unwrap();
        player1.borrow_mut().move_to_discard(card);
        assert_eq!(player1.borrow().discard_pile.card_count(), 1);

        // Discard card from either players hand to their respective discard pile
        player2.borrow_mut().draw_cards(2, TopOrBottom::Top);
        assert_eq!(player2.borrow().hand_size(), 2);
        assert_eq!(player2.borrow().discard_pile.card_count(), 0);

        player1.borrow_mut().discard_card_from_hand(Target::Oppoenet);
        assert_eq!(player1.borrow().discard_pile.card_count(), 1);

        assert_eq!(player2.borrow().hand.card_count(), 1);
        assert_eq!(player2.borrow().discard_pile.card_count(), 1);




    }

    #[test]
    fn setup() {
        let game = mach_game();
        let player1 = game.player1.borrow_mut();
        assert_eq!(player1.hand_size(), 0);
        assert_eq!(player1.deck_size(), 3);


    }

}
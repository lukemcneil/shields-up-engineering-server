use client::get_user_action;
use game::{GameState, UserAction};

mod cards;
mod client;
mod game;
mod tests;

fn main() {
    let mut game_state = GameState::start_state();
    let mut players_turn = game_state.players_turn;
    let mut turns = 0;
    let mut action_count = 0;
    let mut effect_count = 0;
    let mut pass_count = 0;
    let mut stop_resolving_count = 0;
    loop {
        let user_action_with_player = get_user_action(&game_state);
        assert_eq!(game_state.get_total_cards(), 25);
        let game_state_before = game_state.clone();
        match game_state.receive_user_action(user_action_with_player.clone()) {
            Ok(()) => {
                assert_ne!(game_state_before, game_state);
                match &user_action_with_player.user_action {
                    UserAction::ChooseAction { .. } => action_count += 1,
                    UserAction::ResolveEffect { .. } => effect_count += 1,
                    UserAction::Pass { .. } => pass_count += 1,
                    UserAction::StopResolvingEffects => stop_resolving_count += 1,
                }
                println!("did user action {:?}", user_action_with_player);
            }
            Err(_e) => {
                // println!("{:?}", _e);
                assert_eq!(game_state_before, game_state);
            }
        }
        if players_turn != game_state.players_turn {
            turns += 1;
            players_turn = game_state.players_turn;
        }
        if game_state.player1.hull_damage >= 500 || game_state.player2.hull_damage >= 500 {
            println!("game over after {turns} turns");
            println!("actions: {action_count}, effects: {effect_count}, pass: {pass_count}, stop_resolving: {stop_resolving_count}");
            return;
        }
    }
}

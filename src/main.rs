#[macro_use]
extern crate rocket;

use game::{GameState, UserActionError, UserActionWithPlayer};
use rocket::get;

mod cards;
mod client;
mod game;
mod tests;

#[get("/")]
fn play_game(ws: ws::WebSocket) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};
    let mut game_state = GameState::start_state();
    game_state.player1.life_support.hot_wires = vec![game_state.deck.pop().unwrap()];
    game_state.player1.shield_generator.hot_wires = vec![
        game_state.deck.pop().unwrap(),
        game_state.deck.pop().unwrap(),
    ];

    game_state.player2.weapons_system.hot_wires = vec![
        game_state.deck.pop().unwrap(),
        game_state.deck.pop().unwrap(),
    ];

    game_state.player1.life_support.overloads = 3;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let _ = stream
                .send(ws::Message::Text(
                    serde_json::to_string(&game_state).unwrap(),
                ))
                .await;
            while let Some(message) = stream.next().await {
                if let ws::Message::Text(text) = message? {
                    match serde_json::from_str::<UserActionWithPlayer>(&text) {
                        Ok(user_action_with_player) => {
                            let result = game_state.receive_user_action(user_action_with_player);
                            let _ = stream
                                .send(ws::Message::Text(serde_json::to_string(&result).unwrap()))
                                .await;
                            if result.is_ok() {
                                let _ = stream
                                    .send(ws::Message::Text(
                                        serde_json::to_string(&game_state).unwrap(),
                                    ))
                                    .await;
                            }
                        }
                        Err(_) => {
                            let _ = stream
                                .send(ws::Message::Text(
                                    serde_json::to_string(&Err::<(), UserActionError>(
                                        UserActionError::MalformedUserActionWithPlayer,
                                    ))
                                    .unwrap(),
                                ))
                                .await;
                        }
                    }
                } else {
                    let _ = stream
                        .send(ws::Message::Text(
                            serde_json::to_string(&Err::<(), UserActionError>(
                                UserActionError::SentNonTextMessage,
                            ))
                            .unwrap(),
                        ))
                        .await;
                }
            }
            Ok(())
        })
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![play_game])
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            ..Default::default()
        })
}

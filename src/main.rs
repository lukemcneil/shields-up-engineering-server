#[macro_use]
extern crate rocket;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use game::{GameState, UserActionError, UserActionWithPlayer};
use rocket::{get, State};

mod cards;
mod client;
mod game;
mod tests;

#[get("/game/<game_name>")]
fn play_game(
    ws: ws::WebSocket,
    game_name: &str,
    games_state: &State<Arc<Mutex<Games>>>,
) -> ws::Channel<'static> {
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

    let mut games = games_state.lock().unwrap();
    if !games.0.contains_key(game_name) {
        games
            .0
            .insert(game_name.to_string(), Arc::new(Mutex::new(game_state)));
    }

    use rocket::futures::{SinkExt, StreamExt};
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut game_state = GameState::start_state(); // TODO: get the game state to send initial state
            let _ = stream
                .send(ws::Message::Text(
                    serde_json::to_string(&game_state).unwrap(),
                ))
                .await;
            while let Some(message) = stream.next().await {
                if let ws::Message::Text(text) = message? {
                    println!("received: {}", text);
                    match serde_json::from_str::<UserActionWithPlayer>(&text) {
                        Ok(user_action_with_player) => {
                            // TODO: probably need to do something in here to lock on the mutex for the game state to apply the action to the state and send back the new state
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

#[get("/")]
fn test() -> String {
    "shields up engineering".to_string()
}

#[derive(Default)]
struct Games(HashMap<String, Arc<Mutex<GameState>>>);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![play_game, test])
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            ..Default::default()
        })
        .manage(Arc::new(Mutex::new(Games::default())))
}

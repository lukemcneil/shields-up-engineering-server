#[macro_use]
extern crate rocket;

use std::{collections::HashMap, sync::Arc};

use game::{GameState, UserActionError, UserActionWithPlayer};
use rocket::{futures::lock::Mutex, get, State};

mod cards;
mod client;
mod game;
mod tests;

#[get("/game/<game_name>")]
async fn play_game(
    ws: ws::WebSocket,
    game_name: &str,
    games_state: &State<Arc<Mutex<Games>>>,
) -> ws::Channel<'static> {
    let mut games = games_state.lock().await;
    if !games.0.contains_key(game_name) {
        games
            .0
            .insert(game_name.to_string(), GameState::start_state());
    }

    let games_state = Arc::clone(games_state);
    let game_name = game_name.to_string();
    use rocket::futures::{SinkExt, StreamExt};
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut games = games_state.lock().await;
            let game_state = games.0.get_mut(&game_name).unwrap();
            let _ = stream
                .send(ws::Message::Text(
                    serde_json::to_string(&game_state).unwrap(),
                ))
                .await;
            drop(games);
            while let Some(message) = stream.next().await {
                if let ws::Message::Text(text) = message? {
                    println!("received: {}", text);
                    match serde_json::from_str::<UserActionWithPlayer>(&text) {
                        Ok(user_action_with_player) => {
                            let mut games = games_state.lock().await;
                            let game_state = games.0.get_mut(&game_name).unwrap();
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
struct Games(HashMap<String, GameState>);

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

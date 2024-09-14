#[macro_use]
extern crate rocket;

use std::{collections::HashMap, sync::Arc};

use game::{GameState, UserActionError, UserActionWithPlayer};
use rocket::futures::{SinkExt, StreamExt};
use rocket::tokio::sync::broadcast::{self, Sender};
use rocket::{futures::lock::Mutex, get, tokio::select, State};
use ws::{stream::DuplexStream, Message};

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
        let (state_updated_sender, _) = broadcast::channel(1);
        games.0.insert(
            game_name.to_string(),
            (GameState::start_state(), state_updated_sender),
        );
    }

    let games_state = Arc::clone(games_state);
    let game_name = game_name.to_string();
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let mut games = games_state.lock().await;
            let (game_state, state_updated_sender) = games.0.get_mut(&game_name).unwrap();
            let _ = stream
                .send(ws::Message::Text(
                    serde_json::to_string(&game_state).unwrap(),
                ))
                .await;
            let mut state_updated_receiver = state_updated_sender.subscribe();
            drop(games);
            loop {
                select! {
                    x = stream.next() => {
                        if let Some(message) = x {
                            handle_message_from_client(message?, games_state.clone(), &mut stream, &game_name).await;
                        } else {
                            break
                        }
                    }
                    _ = state_updated_receiver.recv() => {
                        let mut games = games_state.lock().await;
                        let (game_state, _) = games.0.get_mut(&game_name).unwrap();
                        let _ = stream
                            .send(ws::Message::Text(
                                serde_json::to_string(&game_state).unwrap(),
                            ))
                            .await;
                    }
                }
            }
            Ok(())
        })
    })
}

async fn handle_message_from_client(
    message: Message,
    games_state: Arc<Mutex<Games>>,
    stream: &mut DuplexStream,
    game_name: &str,
) {
    if let ws::Message::Text(text) = message {
        println!("received: {}", text);
        match serde_json::from_str::<UserActionWithPlayer>(&text) {
            Ok(user_action_with_player) => {
                let mut games = games_state.lock().await;
                let (game_state, state_updated_sender) = games.0.get_mut(game_name).unwrap();
                let result = game_state.receive_user_action(user_action_with_player);
                let _ = stream
                    .send(ws::Message::Text(serde_json::to_string(&result).unwrap()))
                    .await;
                if result.is_ok() {
                    state_updated_sender.send(()).unwrap();
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

#[get("/")]
fn test() -> String {
    "shields up engineering".to_string()
}

#[derive(Default)]
struct Games(HashMap<String, (GameState, Sender<()>)>);

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

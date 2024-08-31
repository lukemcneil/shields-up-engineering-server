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
                            match game_state.receive_user_action(user_action_with_player) {
                                Ok(_) => {
                                    let _ = stream
                                        .send(ws::Message::Text(
                                            serde_json::to_string(&game_state).unwrap(),
                                        ))
                                        .await;
                                }
                                Err(user_action_error) => {
                                    let _ = stream
                                        .send(ws::Message::Text(
                                            serde_json::to_string(&user_action_error).unwrap(),
                                        ))
                                        .await;
                                }
                            }
                        }
                        Err(_) => {
                            let _ = stream
                                .send(ws::Message::Text(
                                    serde_json::to_string(
                                        &UserActionError::MalformedUserActionWithPlayer,
                                    )
                                    .unwrap(),
                                ))
                                .await;
                        }
                    }
                } else {
                    let _ = stream
                        .send(ws::Message::Text(
                            serde_json::to_string(&UserActionError::SentNonTextMessage).unwrap(),
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
    rocket::build().mount("/", routes![play_game])
}

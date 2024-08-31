#[macro_use]
extern crate rocket;

use client::get_user_action;
use game::GameState;
use rocket::get;

mod cards;
mod client;
mod game;
mod tests;

#[get("/echo")]
fn echo(ws: ws::WebSocket) -> ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};
    let mut game_state = GameState::start_state();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            let _ = stream
                .send(ws::Message::Text(
                    serde_json::to_string(&game_state).unwrap(),
                ))
                .await;
            while let Some(_message) = stream.next().await {
                loop {
                    let user_action_with_player = get_user_action(&game_state);
                    if let Ok(()) = game_state.receive_user_action(user_action_with_player.clone())
                    {
                        let _ = stream
                            .send(ws::Message::Text(
                                serde_json::to_string(&user_action_with_player).unwrap(),
                            ))
                            .await;
                        let _ = stream
                            .send(ws::Message::Text(
                                serde_json::to_string(&game_state.turn_state).unwrap(),
                            ))
                            .await;
                        let _ = stream
                            .send(ws::Message::Text(
                                serde_json::to_string(&game_state.actions_left).unwrap(),
                            ))
                            .await;
                        break;
                    }
                }
            }

            Ok(())
        })
    })
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, echo])
}

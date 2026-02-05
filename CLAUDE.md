# Shields Up Engineering Server

Multiplayer card game server where two players manage spaceship systems (Fusion Reactor, Life Support, Weapons, Shield Generator) in a turn-based strategic card game.

## Build & Run

```bash
cargo build          # compile
cargo run            # start server on 0.0.0.0:5000
cargo test           # run all tests
```

## Architecture

- **Rust 2021** with Rocket web framework + WebSocket support
- Server listens on port 5000, games are created dynamically at `/game/<game_name>`
- Game state is synchronized to all connected clients via broadcast channels
- Full game state is serialized as JSON over WebSocket

## Source Layout

- `src/main.rs` — Rocket server, WebSocket endpoint, message routing
- `src/game.rs` — Core game engine: state machine, actions, effect resolution (~940 lines)
- `src/cards.rs` — Card definitions (10 types x 3 copies = 30 card deck)
- `src/client.rs` — AI client for automated testing (random action selection)
- `src/tests.rs` — Integration and unit tests

## Key Concepts

- **Turn state machine**: `ChoosingAction` ↔ `ResolvingEffects`
- **Actions per turn**: 3 (some actions cost more, e.g. Fusion Reactor activation costs 2)
- **Win condition**: First player to 3 hull damage loses (TODO: not yet enforced in code)
- **Short circuits**: At 5+, systems overload on pass (system with most hot-wires gets overloaded)
- **State rollback**: Invalid actions revert game state to pre-action snapshot

## WebSocket Protocol

- Clients send `UserActionWithPlayer` as JSON (contains `player` and `user_action`)
- Server responds with `Result<(), UserActionError>` for each action
- On successful action, updated `GameState` is broadcast to all clients

## Web Client

A browser-based client lives in `shields-up-engineering-client/` (sibling directory to the server).

```bash
# Start the server (from server dir)
cargo run                                    # runs on port 8000

# Start the client (from client dir)
cd ../shields-up-engineering-client
python3 -m http.server 3000                  # serves on port 3000
```

- Open two browser tabs to `http://localhost:3000`
- Tab 1: select Player 1, enter game name, connect
- Tab 2: select Player 2, enter same game name, connect
- Vanilla HTML/CSS/JS — no build step, no framework
- Card images in `cards/` (symlink to `../cards/`)

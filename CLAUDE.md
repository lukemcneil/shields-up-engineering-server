# Shields Up Engineering Server

Multiplayer card game server where two players manage spaceship systems (Fusion Reactor, Life Support, Weapons, Shield Generator) in a turn-based strategic card game.

## Build & Run

```bash
cargo build          # compile
cargo run            # start server on 0.0.0.0:8000 (Rocket default)
cargo test           # run all tests
```

## Architecture

- **Rust 2021** with Rocket web framework + WebSocket support
- Server listens on port 8000, games are created dynamically at `/game/<game_name>`
- Game state is synchronized to all connected clients via broadcast channels
- Full game state is serialized as JSON over WebSocket

## Source Layout

- `src/main.rs` — Rocket server, WebSocket endpoint, message routing
- `src/game.rs` — Core game engine: state machine, actions, effect resolution (~940 lines)
- `src/cards.rs` — Card definitions (100 cards total, 30 unique types with varying copy counts)
- `src/client.rs` — AI client for automated testing (random action selection)
- `src/tests.rs` — Integration and unit tests

## Game Constants

| Mechanic | Value |
|----------|-------|
| Max Hull Damage (loss) | 5 (TODO: not yet enforced in code) |
| Starting Shields | 2 |
| Short Circuit Overload Threshold | 5 (auto-overload on pass) |
| Short Circuit Warning Threshold | 10 |
| Short Circuit Death Threshold | 12 |
| Actions Per Turn | 3 |
| Max Hand Size | 5 (must discard on pass) |
| Deck Size | 100 cards |
| Starting Hand Size | 3 cards |
| ReduceShortCircuits Amount | 2 per action |

### Starting Energy & System Costs

| System | Starting Energy | Max Energy | Activation Cost (actions) | Energy Cost |
|--------|----------------|------------|--------------------------|-------------|
| Fusion Reactor | 0 | 5 | 2 | N/A (distributes energy) |
| Life Support | 2 | 3 | 1 | 2 |
| Shield Generator | 1 | 3 | 1 | 1 |
| Weapons System | 2 | 3 | 1 | 2 |

### Action Costs

| Action | Cost |
|--------|------|
| PlayInstantCard | 0 actions |
| HotWireCard | 1 action |
| ActivateSystem (Fusion Reactor) | 2 actions |
| ActivateSystem (other systems) | 1 action |
| DiscardOverload | 1 action |
| ReduceShortCircuits | 1 action |

## Key Concepts

- **Turn state machine**: `ChoosingAction` ↔ `ResolvingEffects`
- **Win condition**: First player to 5 hull damage loses (TODO: not yet enforced in code)
- **Short circuits**: At 5+, when player passes turn, system with most hot-wires gets overloaded, SC reduced by 5 (repeats while >= 5)
- **Overloads**: Disable a system — energy drains to Fusion Reactor, system can't activate or receive energy. Removed via DiscardOverload action (1 action, removes 1 overload)
- **State rollback**: Invalid actions revert game state to pre-action snapshot

### Fusion Reactor Activation (Special)

- Costs 2 actions (unique among systems)
- Opens energy distribution dialog — player must distribute ALL available Fusion Reactor energy across the 4 systems
- Cannot send energy to overloaded systems
- Energy cost formula for other systems: `max(1, base_cost + UseMoreEnergy - UseLessEnergy from hot-wires)`

### Hot-Wire System

- Cards can be permanently attached to a system instead of played as instants
- Card must match system type, be generic (no system), or system must have `UseSystemCards` effect
- Costs short circuits and/or hand card discards (defined per card)
- Hot-wire effects persist for the rest of the game
- Effects modify system behavior: `StoreMoreEnergy`, `UseMoreEnergy`, `UseLessEnergy`, `DrawPowerFrom(System)`, `UseSystemCards(System)`

### Effect Types

- **Mandatory** (must resolve before StopResolvingEffects): `GainShortCircuit`, `OpponentDiscard`
- **Combat**: `Attack` (-1 shield or +1 hull damage), `Shield` (+1 shield), `BypassShield` (direct hull damage)
- **Resources**: `Draw`, `GainAction`, `MoveEnergy`, `MoveEnergyTo(System)`
- **Disruption**: `OpponentGainShortCircuit`, `OpponentLoseShield`, `OpponentGainOverload`, `OpponentMoveEnergy`, `OpponentDiscard`
- **Utility**: `DiscardOverload`, `LoseShortCircuit`, `PlayHotWire`
- **Modifiers** (hot-wire only): `StoreMoreEnergy`, `UseMoreEnergy`, `UseLessEnergy`, `DrawPowerFrom(System)`, `UseSystemCards(System)`

## WebSocket Protocol

- Clients send `UserActionWithPlayer` as JSON: `{ "player": "Player1"|"Player2", "user_action": ... }`
- Server responds with `Result<(), UserActionError>` for each action (`{"Ok":null}` or `{"Err":"..."}`)
- On successful action, updated `GameState` is broadcast to all clients

### Action Messages

```json
// Play a card instantly (0 actions)
{ "ChooseAction": { "action": { "PlayInstantCard": { "card_index": 0 } } } }

// Hot-wire a card to a system (1 action)
{ "ChooseAction": { "action": { "HotWireCard": { "card_index": 0, "system": "FusionReactor", "indices_to_discard": [] } } } }

// Activate a system
{ "ChooseAction": { "action": { "ActivateSystem": { "system": "LifeSupport", "energy_to_use": null, "energy_distribution": null } } } }

// Fusion Reactor activation (requires energy_distribution map)
{ "ChooseAction": { "action": { "ActivateSystem": { "system": "FusionReactor", "energy_to_use": null, "energy_distribution": { "FusionReactor": 0, "LifeSupport": 2, "ShieldGenerator": 1, "Weapons": 2 } } } } }

// Discard an overload from a system
{ "ChooseAction": { "action": { "DiscardOverload": { "system": "Weapons" } } } }

// Reduce short circuits by 2
{ "ChooseAction": { "action": "ReduceShortCircuits" } }

// Resolve effects
{ "ResolveEffect": { "resolve_effect": "Attack" } }
{ "ResolveEffect": { "resolve_effect": "Shield" } }
{ "ResolveEffect": { "resolve_effect": "GainShortCircuit" } }

// Stop resolving optional effects
"StopResolvingEffects"

// Pass turn (with optional discard indices if hand > 5)
{ "Pass": { "card_indices_to_discard": [] } }
```

## Web Client

A browser-based client lives in `shields-up-engineering-client/` (sibling directory to the server).

```bash
# Start the server (from server dir)
cargo run                                    # runs on port 8000

# Start the client (from client dir)
cd ../shields-up-engineering-client
npx live-server --port=3000                  # serves on port 3000
```

- Open two browser tabs to `http://localhost:3000`
- Tab 1: select Player 1, enter game name, connect
- Tab 2: select Player 2, enter same game name, connect
- Vanilla HTML/CSS/JS — no build step, no framework
- Card images in `cards/` directory
- Client connects to server WebSocket at `ws://localhost:8000/game/<game_name>`
- URL hash stores game/player for auto-reconnect on refresh (e.g. `#game7/Player2`)

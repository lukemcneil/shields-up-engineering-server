// mod input;
mod tests;

fn print_list_and_indices(name: &str, things: Vec<String>) {
    print!("{} - [", name);
    let x: Vec<String> = things
        .iter()
        .enumerate()
        .map(|(i, thing)| format!("{}({})", thing, i))
        .collect();
    println!("{}]", x.join(", "));
}

use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
struct Card {
    instant_effects: Vec<Effect>,
    hot_wire_effects: Vec<Effect>,
    hot_wire_cost: Vec<Effect>,
    system: Option<System>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Effect {
    GainShortCircuit,
    LoseShortCircuit,
    StoreMoreEnergy,
    UseMoreEnergy,
    UseLessEnergy,
    Shield,
    Attack,
    DiscardOverload,
    GainAction,

    PlayHotWire,
    Discard,
    Draw,

    OpponentDiscard,
    OpponentGainShortCircuit,
    OpponentLoseShield,
    // OpponentMoveEnergy,
    OpponentGainOverload,
    // DrawPowerFrom(System),
    // MoveEnergy,
    // MoveEnergyTo(System),
    // UseSystemCards(System),
}

impl Effect {
    fn has_immediate_effect(&self) -> bool {
        match self {
            Effect::GainShortCircuit
            | Effect::LoseShortCircuit
            | Effect::Shield
            | Effect::Attack
            | Effect::DiscardOverload
            | Effect::GainAction
            | Effect::PlayHotWire
            | Effect::Discard
            | Effect::Draw
            | Effect::OpponentDiscard
            | Effect::OpponentGainShortCircuit
            | Effect::OpponentLoseShield
            | Effect::OpponentGainOverload => true,
            Effect::StoreMoreEnergy | Effect::UseMoreEnergy | Effect::UseLessEnergy => false,
        }
    }
}

#[derive(Debug, Clone)]
struct SystemState {
    system: System,
    energy: i32,
    overloads: i32,
    hot_wires: Vec<Card>,
}

impl SystemState {
    fn with_energy(energy: i32, system: System) -> Self {
        Self {
            energy,
            system,
            overloads: 0,
            hot_wires: vec![],
        }
    }

    fn get_hot_wire_effects(&self) -> Vec<Effect> {
        let mut system_effects: Vec<Effect> = self.system.starting_effects();
        for hot_wire_card in &self.hot_wires {
            system_effects.append(&mut hot_wire_card.hot_wire_effects.clone());
        }
        system_effects
    }

    fn get_allowed_energy(&self) -> i32 {
        self.get_hot_wire_effects()
            .iter()
            .filter(|&&effect| effect == Effect::StoreMoreEnergy)
            .count() as i32
    }

    fn get_energy_used(&self) -> i32 {
        self.get_hot_wire_effects()
            .iter()
            .filter_map(|effect| match effect {
                Effect::UseMoreEnergy => Some(1),
                Effect::UseLessEnergy => Some(-1),
                _ => None,
            })
            .sum::<i32>()
            .max(1)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum System {
    FusionReactor,
    LifeSupport,
    Weapons,
    ShieldGenerator,
}

impl System {
    fn starting_effects(&self) -> Vec<Effect> {
        match self {
            System::FusionReactor => vec![
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
            ],
            System::LifeSupport => vec![
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::Draw,
                Effect::Draw,
                Effect::UseMoreEnergy,
                Effect::UseMoreEnergy,
            ],
            System::Weapons => vec![
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::Attack,
                Effect::UseMoreEnergy,
                Effect::UseMoreEnergy,
            ],
            System::ShieldGenerator => vec![
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::StoreMoreEnergy,
                Effect::Shield,
                Effect::UseMoreEnergy,
            ],
        }
    }
}

#[derive(Debug, Clone)]
struct PlayerState {
    hull_damage: i32,
    shields: i32,
    short_circuits: i32,
    hand: Vec<Card>,
    fusion_reactor: SystemState,
    life_support: SystemState,
    shield_generator: SystemState,
    weapons_system: SystemState,
}

impl PlayerState {
    fn start_state() -> Self {
        Self {
            hull_damage: 0,
            shields: 2,
            short_circuits: 0,
            hand: vec![
                Card {
                    instant_effects: vec![
                        Effect::GainShortCircuit,
                        Effect::GainShortCircuit,
                        Effect::GainShortCircuit,
                        Effect::GainShortCircuit,
                        Effect::GainShortCircuit,
                        // Effect::UseLessEnergy,
                        // Effect::DiscardOverload,
                        // Effect::GainAction,
                        // Effect::PlayHotWire,
                        // Effect::Discard,
                        // Effect::OpponentDiscard,
                        // Effect::OpponentGainShortCircuit,
                        // Effect::OpponentGainOverload,
                        // Effect::OpponentLoseShield,
                    ],
                    hot_wire_effects: vec![
                        Effect::UseLessEnergy,
                        Effect::GainAction,
                        Effect::GainAction,
                        Effect::Draw,
                        Effect::StoreMoreEnergy,
                        Effect::StoreMoreEnergy,
                        Effect::StoreMoreEnergy,
                    ],
                    hot_wire_cost: vec![
                        Effect::GainShortCircuit,
                        Effect::GainShortCircuit,
                        Effect::GainShortCircuit,
                        Effect::GainShortCircuit,
                        Effect::GainShortCircuit,
                    ],
                    system: None,
                },
                Card::default(),
                Card::default(),
                Card::default(),
                Card::default(),
            ],
            fusion_reactor: SystemState::with_energy(0, System::FusionReactor),
            life_support: SystemState::with_energy(2, System::LifeSupport),
            shield_generator: SystemState::with_energy(1, System::ShieldGenerator),
            weapons_system: SystemState::with_energy(2, System::Weapons),
        }
    }

    fn get_system_state(&mut self, system: System) -> &mut SystemState {
        match system {
            System::FusionReactor => &mut self.fusion_reactor,
            System::LifeSupport => &mut self.life_support,
            System::Weapons => &mut self.weapons_system,
            System::ShieldGenerator => &mut self.shield_generator,
        }
    }

    // fn resolve_effects(
    //     &mut self,
    //     mut effects: Vec<Effect>,
    //     choose_order: bool,
    //     opponent_state: &mut PlayerState,
    //     shared_state: &mut SharedState,
    // ) -> ResolveEffectResult {
    //     effects = effects
    //         .iter()
    //         .filter(|e| e.has_immediate_effect())
    //         .cloned()
    //         .collect();
    //     effects.sort();
    //     loop {
    //         if effects.is_empty() {
    //             break;
    //         }
    //         let effect = if choose_order {
    //             println!("choose an effect");
    //             print_list_and_indices(
    //                 "Effects",
    //                 effects.iter().map(|e| format!("{:?}", e)).collect(),
    //             );
    //             let index = choose_card_index(effects.len()).unwrap();
    //             effects.remove(index)
    //         } else {
    //             effects.pop().unwrap()
    //         };
    //         match effect {
    //             Effect::GainShortCircuit => {
    //                 self.short_circuits += 1;
    //                 if self.short_circuits == 12 {
    //                     println!("you lost by getting 12 short circuits");
    //                     exit(0);
    //                 }
    //             }
    //             Effect::LoseShortCircuit => self.short_circuits = (self.short_circuits - 1).max(0),
    //             Effect::Shield => {
    //                 if self.shields < self.shield_generator.get_allowed_energy() {
    //                     self.shields += 1;
    //                 } else {
    //                     println!("already at max shields")
    //                 }
    //             }
    //             Effect::Attack => {
    //                 if opponent_state.shields > 0 {
    //                     opponent_state.shields -= 1;
    //                 } else {
    //                     opponent_state.hull_damage += 1;
    //                     if opponent_state.hull_damage >= 5 {
    //                         println!("opponent lost by taking 5 damage");
    //                         exit(0);
    //                     }
    //                 }
    //             }
    //             Effect::DiscardOverload => {
    //                 let system_state = self.get_system_state(choose_system());
    //                 if system_state.overloads == 0 {
    //                     println!("no overloads to discard")
    //                 } else {
    //                     system_state.overloads -= 1;
    //                 }
    //             }
    //             Effect::GainAction => (),
    //             Effect::PlayHotWire => {
    //                 // let card_index = choose_card_index(self.hand.len());
    //                 // match card_index {
    //                 //     Some(card_index) => {
    //                 //         let _ = self.try_to_do_action(
    //                 //             Action::HotWireCard {
    //                 //                 card_index,
    //                 //                 system: choose_system(),
    //                 //             },
    //                 //             opponent_state,
    //                 //             shared_state,
    //                 //         );
    //                 //     }
    //                 //     None => println!("no card to hot wire"),
    //                 // }
    //             }
    //             Effect::Discard => {
    //                 self.discard(1, shared_state);
    //             }
    //             Effect::Draw => {
    //                 if shared_state.deck.is_empty() {
    //                     shared_state.deck.append(&mut shared_state.discard_pile);
    //                     shared_state.deck.shuffle(&mut thread_rng());
    //                 }
    //                 match shared_state.deck.pop() {
    //                     Some(card) => self.hand.push(card),
    //                     None => println!("no card to draw"),
    //                 };
    //             }
    //             Effect::OpponentDiscard => {
    //                 opponent_state.discard(1, shared_state);
    //             }
    //             Effect::OpponentGainShortCircuit => {
    //                 opponent_state.short_circuits += 1;
    //                 if opponent_state.short_circuits == 12 {
    //                     println!("opponent lost by getting 12 short circuits");
    //                     exit(0);
    //                 }
    //             }
    //             Effect::OpponentLoseShield => {
    //                 opponent_state.shields = (opponent_state.shields - 1).max(0)
    //             }
    //             Effect::OpponentGainOverload => opponent_state.overload_system(choose_system()),
    //             Effect::StoreMoreEnergy | Effect::UseMoreEnergy | Effect::UseLessEnergy => (),
    //             // Effect::OpponentMoveEnergy => todo!(),
    //             // Effect::DrawPowerFrom(_) => (),
    //             // Effect::MoveEnergy => todo!(),
    //             // Effect::MoveEnergyTo(_) => todo!(),
    //             // Effect::UseSystemCards(_) => todo!(),
    //         }
    //     }
    //     ResolveEffectResult::Resolved
    // }

    // fn check_hand_size(&mut self, shared_state: &mut SharedState) {
    //     if self.hand.len() > 5 {
    //         println!(
    //             "over hand limit need to discard {} cards",
    //             self.hand.len() - 5
    //         );
    //         self.discard(self.hand.len() - 5, shared_state);
    //     }
    // }

    // fn discard(&mut self, amount: usize, shared_state: &mut SharedState) -> bool {
    //     for _ in 0..amount {
    //         if self.hand.is_empty() {
    //             return false;
    //         }
    //         println!("choose a card to discard");
    //         let index = choose_card_index(self.hand.len()).expect("just checked hand is not empty");
    //         let discarded_card = self.hand.remove(index);
    //         println!("discarded {:?}", discarded_card);
    //         shared_state.discard_pile.push(discarded_card);
    //     }
    //     true
    // }

    fn check_system_overload(&mut self) {
        while self.short_circuits >= 5 {
            println!("have 5 or more short circuits, system overload, choose a system to disable");
            // TODO: this is supposed to go on the system with most hotwires
            // self.overload_system(choose_system());
            self.short_circuits -= 5;
        }
    }

    fn overload_system(&mut self, system: System) {
        let system_state = self.get_system_state(system);
        system_state.overloads += 1;
        let energy_to_move = system_state.energy;
        system_state.energy = 0;
        self.fusion_reactor.energy += energy_to_move;
    }
}

#[derive(PartialEq, Eq)]
enum ResolveEffectResult {
    Resolved,
    CouldNotDiscard,
}

impl Action {
    fn action_points(&self) -> i32 {
        match self {
            Action::HotWireCard { .. } => 1,
            Action::PlayInstantCard { .. } => 0,
            Action::ActivateSystem { system, .. } => match system {
                System::FusionReactor => 2,
                System::LifeSupport => 1,
                System::Weapons => 1,
                System::ShieldGenerator => 1,
            },
            Action::DiscardOverload { .. } => 1,
            Action::ReduceShortCircuits => 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Action {
    HotWireCard {
        card_index: usize,
        system: System,
    },
    PlayInstantCard {
        card_index: usize,
    },
    ActivateSystem {
        system: System,
        energy_distribution: Option<BTreeMap<System, i32>>,
    },
    DiscardOverload {
        system: System,
    },
    ReduceShortCircuits,
}

enum UserAction {
    ChooseAction { action: Action },
    ResolveEffect { effect: Effect },
    Pass { card_indices_to_discard: Vec<usize> },
}

struct UserActionWithPlayer {
    player: Player,
    user_action: UserAction,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Player {
    Player1,
    Player2,
}

#[derive(Clone, Debug)]
enum TurnState {
    ChoosingAction,
    ResolvingEffects { effects: Vec<Effect> },
}

#[derive(Clone, Debug)]
struct GameState {
    player1: PlayerState,
    player2: PlayerState,
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
    players_turn: Player,
    actions_left: i32,
    turn_state: TurnState,
}

impl GameState {
    fn start_state() -> Self {
        let player1 = PlayerState::start_state();
        let player2 = PlayerState::start_state();
        Self {
            players_turn: Player::Player1,
            turn_state: TurnState::ChoosingAction,
            player1,
            player2,
            deck: vec![Card::default(), Card::default()],
            discard_pile: vec![],
            actions_left: 3,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum UserActionError {
    NotYourTurn,
    NotEnoughCardsToDiscard,
    NotEnoughActionsLeft,
    SystemHasNoOverload,
    MissingEnergyDistribution,
    InvalidEnergyDistribution,
    CannotPutEnergyOnDisabledSystem,
    InvalidCardIndex,
    CannotActivateOverloadedSystem,
    NotEnoughEnergyToActivate,
    InvalidUserAction,
}

impl GameState {
    fn choose_action(&mut self, action: Action, player: Player) -> Result<(), UserActionError> {
        if action.action_points() > self.actions_left {
            return Err(UserActionError::NotEnoughActionsLeft);
        }
        let result = match action.clone() {
            Action::HotWireCard { card_index, system } => {
                let my_state = self.my_state(player);
                if card_index >= my_state.hand.len() {
                    return Err(UserActionError::InvalidCardIndex);
                }
                let card = my_state.hand.remove(card_index);
                let discard_count = card
                    .hot_wire_effects
                    .iter()
                    .filter(|&&e| e == Effect::Discard)
                    .count();
                if discard_count > my_state.hand.len() {
                    return Err(UserActionError::NotEnoughCardsToDiscard);
                }
                my_state
                    .get_system_state(system)
                    .hot_wires
                    .push(card.clone());
                self.turn_state = TurnState::ResolvingEffects {
                    effects: card.hot_wire_cost,
                };
                Ok(())
            }
            Action::PlayInstantCard { card_index } => {
                let my_state = self.my_state(player);
                if card_index >= my_state.hand.len() {
                    return Err(UserActionError::InvalidCardIndex);
                }
                let card = my_state.hand.remove(card_index);
                self.discard_pile.push(card.clone());
                self.turn_state = TurnState::ResolvingEffects {
                    effects: card.instant_effects,
                };
                Ok(())
            }
            Action::ActivateSystem {
                system,
                energy_distribution,
            } => {
                let my_state = self.my_state(player);
                if my_state.get_system_state(system).overloads > 0 {
                    return Err(UserActionError::CannotActivateOverloadedSystem);
                }
                if system == System::FusionReactor {
                    if let Some(energy_distribution) = energy_distribution {
                        let allocated_energy = energy_distribution.values().sum::<i32>();
                        if allocated_energy != my_state.fusion_reactor.get_allowed_energy() {
                            return Err(UserActionError::InvalidEnergyDistribution);
                        }
                        for (system, energy) in energy_distribution {
                            let system_state = my_state.get_system_state(system);
                            if system_state.overloads > 0 && energy > 0 {
                                return Err(UserActionError::CannotPutEnergyOnDisabledSystem);
                            }
                            system_state.energy = energy;
                        }
                    } else {
                        return Err(UserActionError::MissingEnergyDistribution);
                    }
                } else {
                    let system_state = my_state.get_system_state(system);
                    let energy_used = system_state.get_energy_used();
                    if energy_used > system_state.energy {
                        return Err(UserActionError::NotEnoughEnergyToActivate);
                    }
                    system_state.energy -= energy_used;
                    my_state.fusion_reactor.energy += energy_used;
                }
                self.turn_state = TurnState::ResolvingEffects {
                    effects: my_state.get_system_state(system).get_hot_wire_effects(),
                };
                Ok(())
            }
            Action::DiscardOverload { system } => {
                let my_state = self.my_state(player);
                let system_state = my_state.get_system_state(system);
                if system_state.overloads > 0 {
                    system_state.overloads -= 1;
                    Ok(())
                } else {
                    Err(UserActionError::SystemHasNoOverload)
                }
            }
            Action::ReduceShortCircuits => {
                self.turn_state = TurnState::ResolvingEffects {
                    effects: vec![Effect::LoseShortCircuit, Effect::LoseShortCircuit],
                };
                Ok(())
            }
        };
        if result.is_ok() {
            self.actions_left -= action.action_points();
        }
        result
    }

    fn receive_user_action(
        &mut self,
        user_action_with_player: UserActionWithPlayer,
    ) -> Result<(), UserActionError> {
        let game_state_before = self.clone();
        let player = user_action_with_player.player;
        let result = if self.players_turn == player {
            match (&self.turn_state, user_action_with_player.user_action) {
                (TurnState::ChoosingAction, UserAction::ChooseAction { action }) => {
                    self.choose_action(action, player)
                }
                (TurnState::ResolvingEffects { effects }, UserAction::ResolveEffect { effect }) => {
                    todo!()
                }
                (
                    TurnState::ChoosingAction,
                    UserAction::Pass {
                        card_indices_to_discard,
                    },
                ) => {
                    self.actions_left = 3;
                    match self.players_turn {
                        Player::Player1 => self.players_turn = Player::Player2,
                        Player::Player2 => self.players_turn = Player::Player1,
                    }
                    // TODO: discard cards and check short circuits
                    return Ok(());
                }
                (TurnState::ChoosingAction, UserAction::ResolveEffect { .. }) => {
                    Err(UserActionError::InvalidUserAction)
                }
                (TurnState::ResolvingEffects { .. }, UserAction::ChooseAction { .. }) => {
                    Err(UserActionError::InvalidUserAction)
                }
                (TurnState::ResolvingEffects { .. }, UserAction::Pass { .. }) => {
                    Err(UserActionError::InvalidUserAction)
                }
            }
        } else {
            println!("making a user action when it's not your turn");
            Err(UserActionError::NotYourTurn)
        };
        if result.is_err() {
            *self = game_state_before;
        }
        self.remove_effects_without_immediate_effects();
        result
    }

    fn remove_effects_without_immediate_effects(&mut self) {
        if let TurnState::ResolvingEffects { effects } = &mut self.turn_state {
            effects.retain(Effect::has_immediate_effect);
        }
    }

    fn my_state(&mut self, player: Player) -> &mut PlayerState {
        match player {
            Player::Player1 => &mut self.player1,
            Player::Player2 => &mut self.player2,
        }
    }

    fn opponent_state(&mut self, player: Player) -> &mut PlayerState {
        match player {
            Player::Player1 => &mut self.player2,
            Player::Player2 => &mut self.player1,
        }
    }
}

fn main() {
    let mut game_state = GameState::start_state();
    let result = game_state.receive_user_action(UserActionWithPlayer {
        player: Player::Player1,
        user_action: UserAction::ChooseAction {
            action: Action::ReduceShortCircuits,
        },
    });
    assert!(result.is_ok());
    // loop {
    //     game_state
    // }
}

use rand::Rng;

use crate::*;

fn random_option(options: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..options)
}

fn random_system() -> System {
    match random_option(4) {
        0 => System::FusionReactor,
        1 => System::ShieldGenerator,
        2 => System::LifeSupport,
        3 => System::Weapons,
        _ => panic!(),
    }
}

pub fn get_user_action(game_state: &GameState) -> UserActionWithPlayer {
    let player = game_state.players_turn;
    let my_state = game_state.my_state_immut(player);
    let user_action = match &game_state.turn_state {
        TurnState::ChoosingAction => {
            if game_state.actions_left == 0 {
                let mut card_indices_to_discard = vec![];
                if my_state.hand.len() > 5 {
                    card_indices_to_discard = (0..(my_state.hand.len() - 5)).collect();
                }
                return UserActionWithPlayer {
                    player,
                    user_action: UserAction::Pass {
                        card_indices_to_discard,
                    },
                };
            }
            let action = match random_option(5) {
                0 => {
                    let system = random_system();
                    let mut energy_distribution = None;
                    if system == System::FusionReactor {
                        let mut dist = BTreeMap::new();
                        let total = my_state.fusion_reactor.get_allowed_energy();
                        dist.insert(System::Weapons, 2);
                        dist.insert(System::LifeSupport, 2);
                        dist.insert(System::ShieldGenerator, 1);
                        dist.insert(System::FusionReactor, total - 5);
                        energy_distribution = Some(dist);
                    }
                    Action::ActivateSystem {
                        system,
                        energy_to_use: None,
                        energy_distribution,
                    }
                }
                1 => Action::DiscardOverload {
                    system: random_system(),
                },
                2 => {
                    if my_state.hand.is_empty() {
                        return get_user_action(game_state);
                    }
                    let card = my_state.hand.first().unwrap();
                    let cards_to_discard = card.hot_wire_cost.cards_to_discard;
                    Action::HotWireCard {
                        card_index: 0,
                        system: random_system(),
                        indices_to_discard: (1..=cards_to_discard).collect(),
                    }
                }
                3 => {
                    if my_state.hand.is_empty() {
                        return get_user_action(game_state);
                    };
                    Action::PlayInstantCard { card_index: 0 }
                }
                4 => Action::ReduceShortCircuits,
                _ => panic!(),
            };
            UserAction::ChooseAction { action }
        }
        TurnState::ResolvingEffects { effects } => {
            for effect in effects {
                if let Some(resolve_effect) = effect.get_resolution(game_state) {
                    let player = if let ResolveEffect::OpponentDiscard { .. } = resolve_effect {
                        player.other_player()
                    } else {
                        player
                    };
                    let user_action = UserAction::ResolveEffect { resolve_effect };
                    if game_state
                        .clone()
                        .receive_user_action(UserActionWithPlayer {
                            player,
                            user_action: user_action.clone(),
                        })
                        .is_ok()
                    {
                        return UserActionWithPlayer {
                            player,
                            user_action,
                        };
                    }
                }
            }
            UserAction::StopResolvingEffects
        }
    };
    UserActionWithPlayer {
        player,
        user_action,
    }
}

impl Effect {
    fn get_resolution(&self, game_state: &GameState) -> Option<ResolveEffect> {
        let player = game_state.players_turn;
        let my_state = game_state.my_state_immut(player);
        match self {
            Effect::GainShortCircuit => Some(ResolveEffect::GainShortCircuit),
            Effect::LoseShortCircuit => Some(ResolveEffect::LoseShortCircuit),
            Effect::Shield => Some(ResolveEffect::Shield),
            Effect::Attack => Some(ResolveEffect::Attack),
            Effect::DiscardOverload => Some(ResolveEffect::DiscardOverload {
                system: random_system(),
            }),
            Effect::GainAction => Some(ResolveEffect::GainAction),
            Effect::PlayHotWire => {
                if my_state.hand.is_empty() {
                    return None;
                }
                let card = my_state.hand.first().unwrap();
                let cards_to_discard = card.hot_wire_cost.cards_to_discard;
                Some(ResolveEffect::PlayHotWire {
                    card_index: 0,
                    system: random_system(),
                    indices_to_discard: (1..=cards_to_discard).collect(),
                })
            }
            Effect::Draw => Some(ResolveEffect::Draw),
            Effect::OpponentGainShortCircuit => Some(ResolveEffect::OpponentGainShortCircuit),
            Effect::OpponentLoseShield => Some(ResolveEffect::OpponentLoseShield),
            Effect::OpponentMoveEnergy => Some(ResolveEffect::OpponentMoveEnergy {
                from_system: random_system(),
                to_system: random_system(),
            }),
            Effect::OpponentGainOverload => Some(ResolveEffect::OpponentGainOverload {
                system: random_system(),
            }),
            Effect::MoveEnergy => Some(ResolveEffect::MoveEnergy {
                from_system: random_system(),
                to_system: random_system(),
            }),
            Effect::MoveEnergyTo(to_system) => Some(ResolveEffect::MoveEnergyTo {
                from_system: random_system(),
                to_system: *to_system,
            }),
            Effect::OpponentDiscard => Some(ResolveEffect::OpponentDiscard { card_index: 0 }),
            Effect::BypassShield => Some(ResolveEffect::BypassShield),
            Effect::StoreMoreEnergy
            | Effect::UseMoreEnergy
            | Effect::UseLessEnergy
            | Effect::UseSystemCards(_)
            | Effect::DrawPowerFrom(_) => None,
        }
    }
}

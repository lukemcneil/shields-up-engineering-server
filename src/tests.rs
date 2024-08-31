#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{client::get_user_action, game::*};

    impl GameState {
        pub fn get_total_cards(&self) -> usize {
            self.player1.hand.len()
                + self.player2.hand.len()
                + self.player1.fusion_reactor.hot_wires.len()
                + self.player2.fusion_reactor.hot_wires.len()
                + self.player1.life_support.hot_wires.len()
                + self.player2.life_support.hot_wires.len()
                + self.player1.weapons_system.hot_wires.len()
                + self.player2.weapons_system.hot_wires.len()
                + self.player1.shield_generator.hot_wires.len()
                + self.player2.shield_generator.hot_wires.len()
                + self.deck.len()
                + self.discard_pile.len()
        }
    }

    #[test]
    fn test_client() {
        let mut game_state = GameState::start_state();
        let mut players_turn = game_state.players_turn;
        let mut turns = 0;
        let mut action_count = 0;
        let mut effect_count = 0;
        let mut pass_count = 0;
        let mut stop_resolving_count = 0;
        loop {
            let user_action_with_player = get_user_action(&game_state);
            assert_eq!(game_state.get_total_cards(), 25);
            let game_state_before = game_state.clone();
            match game_state.receive_user_action(user_action_with_player.clone()) {
                Ok(()) => {
                    assert_ne!(game_state_before, game_state);
                    match &user_action_with_player.user_action {
                        UserAction::ChooseAction { .. } => action_count += 1,
                        UserAction::ResolveEffect { .. } => effect_count += 1,
                        UserAction::Pass { .. } => pass_count += 1,
                        UserAction::StopResolvingEffects => stop_resolving_count += 1,
                    }
                    println!("did user action {:?}", user_action_with_player);
                }
                Err(_e) => {
                    // println!("{:?}", _e);
                    assert_eq!(game_state_before, game_state);
                }
            }
            if players_turn != game_state.players_turn {
                turns += 1;
                players_turn = game_state.players_turn;
            }
            if game_state.player1.hull_damage >= 50 || game_state.player2.hull_damage >= 50 {
                println!("game over after {turns} turns");
                println!("actions: {action_count}, effects: {effect_count}, pass: {pass_count}, stop_resolving: {stop_resolving_count}");
                return;
            }
        }
    }

    #[test]
    fn test_bad_user_action() {
        let mut game_state = GameState::start_state();
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                resolve_effect: ResolveEffect::Attack,
            },
        });
        assert!(result.is_err_and(|e| e == UserActionError::InvalidUserAction));
    }

    #[test]
    fn test_pass() {
        let mut game_state = GameState::start_state();
        game_state.player1.fusion_reactor.overloads = 1;
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::DiscardOverload {
                    system: System::FusionReactor,
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.player1.fusion_reactor.overloads, 0);
        assert_eq!(game_state.actions_left, 2);

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::Pass {
                card_indices_to_discard: vec![5],
            },
        });
        assert!(result.is_ok());
        assert!(game_state.players_turn == Player::Player2);
        assert_eq!(game_state.actions_left, 3);
    }

    #[test]
    fn test_activate_fusion_reactor() {
        let mut game_state = GameState::start_state();
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::ActivateSystem {
                    system: System::FusionReactor,
                    energy_to_use: None,
                    energy_distribution: None,
                },
            },
        });
        assert_eq!(result, Err(UserActionError::MissingEnergyDistribution));

        game_state.player1.shield_generator.overloads = 1;
        let mut bad_energy_distribution = BTreeMap::new();
        bad_energy_distribution.insert(System::FusionReactor, 1);
        bad_energy_distribution.insert(System::Weapons, 3);
        bad_energy_distribution.insert(System::ShieldGenerator, 1);
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::ActivateSystem {
                    system: System::FusionReactor,
                    energy_to_use: None,
                    energy_distribution: Some(bad_energy_distribution),
                },
            },
        });
        assert_eq!(
            result,
            Err(UserActionError::CannotPutEnergyOnDisabledSystem)
        );

        game_state.player1.shield_generator.overloads = 0;
        let mut bad_energy_distribution2 = BTreeMap::new();
        bad_energy_distribution2.insert(System::FusionReactor, 1);
        bad_energy_distribution2.insert(System::Weapons, 3);
        bad_energy_distribution2.insert(System::ShieldGenerator, 2);
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::ActivateSystem {
                    system: System::FusionReactor,
                    energy_to_use: None,
                    energy_distribution: Some(bad_energy_distribution2),
                },
            },
        });
        assert_eq!(result, Err(UserActionError::InvalidEnergyDistribution));

        let mut energy_distribution = BTreeMap::new();
        energy_distribution.insert(System::FusionReactor, 1);
        energy_distribution.insert(System::Weapons, 3);
        energy_distribution.insert(System::ShieldGenerator, 1);
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::ActivateSystem {
                    system: System::FusionReactor,
                    energy_to_use: None,
                    energy_distribution: Some(energy_distribution),
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 1);
        match game_state.turn_state {
            TurnState::ChoosingAction => assert!(false),
            TurnState::ResolvingEffects { effects } => {
                assert_eq!(effects, vec![])
            }
        }
    }

    #[test]
    fn test_activate_weapons() {
        let mut game_state = GameState::start_state();
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::ActivateSystem {
                    system: System::Weapons,
                    energy_to_use: None,
                    energy_distribution: None,
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 2);
        match &game_state.turn_state {
            TurnState::ChoosingAction => assert!(false),
            TurnState::ResolvingEffects { effects } => {
                assert_eq!(effects, &vec![Effect::Attack]);
            }
        }

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                resolve_effect: ResolveEffect::Attack,
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.turn_state, TurnState::ChoosingAction);
        assert_eq!(game_state.actions_left, 2);
        assert_eq!(game_state.player2.shields, 1);
    }

    #[test]
    fn test_shields() {
        let mut game_state = GameState::start_state();
        game_state.player1.hand = vec![
            Card {
                instant_effects: vec![],
                hot_wire_effects: vec![Effect::Shield],
                hot_wire_cost: HotWireCost {
                    short_circuits: 7,
                    cards_to_discard: 1,
                },
                system: Some(System::ShieldGenerator),
            },
            Card::default(),
        ];
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::HotWireCard {
                    card_index: 0,
                    system: System::ShieldGenerator,
                    indices_to_discard: vec![1],
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.player1.shield_generator.hot_wires.len(), 1);
        assert_eq!(game_state.actions_left, 2);
        assert_eq!(game_state.player1.short_circuits, 7);
        assert_eq!(game_state.player1.hand.len(), 0);

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::ActivateSystem {
                    system: System::ShieldGenerator,
                    energy_to_use: None,
                    energy_distribution: None,
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 1);
        match &game_state.turn_state {
            TurnState::ChoosingAction => assert!(false),
            TurnState::ResolvingEffects { effects } => {
                assert_eq!(effects, &vec![Effect::Shield, Effect::Shield]);
            }
        }

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                resolve_effect: ResolveEffect::Shield,
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 1);
        assert_eq!(game_state.player1.shields, 3);

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                resolve_effect: ResolveEffect::Shield,
            },
        });
        assert_eq!(result, Err(UserActionError::AlreadyAtMaxShields));

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::StopResolvingEffects,
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.turn_state, TurnState::ChoosingAction);
        assert_eq!(game_state.actions_left, 1);
    }

    #[test]
    fn test_move_energy() {
        let mut game_state = GameState::start_state();
        game_state.player1.hand = vec![Card {
            instant_effects: vec![
                Effect::MoveEnergy,
                Effect::MoveEnergyTo(System::ShieldGenerator),
                Effect::OpponentMoveEnergy,
            ],
            hot_wire_effects: vec![],
            hot_wire_cost: HotWireCost {
                short_circuits: 0,
                cards_to_discard: 0,
            },
            system: None,
        }];
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::PlayInstantCard { card_index: 0 },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 3);

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                resolve_effect: ResolveEffect::MoveEnergy {
                    from_system: System::ShieldGenerator,
                    to_system: System::Weapons,
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.player1.shield_generator.energy, 0);
        assert_eq!(game_state.player1.weapons_system.energy, 3);

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                resolve_effect: ResolveEffect::MoveEnergyTo {
                    from_system: System::LifeSupport,
                    to_system: System::ShieldGenerator,
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.player1.shield_generator.energy, 1);
        assert_eq!(game_state.player1.weapons_system.energy, 3);
        assert_eq!(game_state.player1.life_support.energy, 1);

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                resolve_effect: ResolveEffect::OpponentMoveEnergy {
                    from_system: System::LifeSupport,
                    to_system: System::ShieldGenerator,
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.player1.shield_generator.energy, 1);
        assert_eq!(game_state.player1.weapons_system.energy, 3);
        assert_eq!(game_state.player1.life_support.energy, 1);
        assert_eq!(game_state.player2.shield_generator.energy, 2);
        assert_eq!(game_state.player2.weapons_system.energy, 2);
        assert_eq!(game_state.player2.life_support.energy, 1);
    }

    #[test]
    fn test_use_system_cards() {
        let mut game_state = GameState::start_state();
        game_state.player1.hand = vec![
            Card {
                instant_effects: vec![],
                hot_wire_effects: vec![Effect::UseSystemCards(System::ShieldGenerator)],
                hot_wire_cost: HotWireCost {
                    short_circuits: 0,
                    cards_to_discard: 0,
                },
                system: Some(System::Weapons),
            },
            Card {
                instant_effects: vec![],
                hot_wire_effects: vec![],
                hot_wire_cost: HotWireCost {
                    short_circuits: 0,
                    cards_to_discard: 0,
                },
                system: Some(System::ShieldGenerator),
            },
        ];

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::HotWireCard {
                    card_index: 0,
                    system: System::Weapons,
                    indices_to_discard: vec![],
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 2);

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::HotWireCard {
                    card_index: 0,
                    system: System::Weapons,
                    indices_to_discard: vec![],
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 1);
    }

    #[test]
    fn test_draw_power_from() {
        let mut game_state = GameState::start_state();
        game_state.player1.hand = vec![Card {
            instant_effects: vec![],
            hot_wire_effects: vec![Effect::DrawPowerFrom(System::LifeSupport)],
            hot_wire_cost: HotWireCost {
                short_circuits: 0,
                cards_to_discard: 0,
            },
            system: None,
        }];

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::HotWireCard {
                    card_index: 0,
                    system: System::Weapons,
                    indices_to_discard: vec![],
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 2);

        let mut energy_to_use = BTreeMap::new();
        energy_to_use.insert(System::Weapons, 1);
        energy_to_use.insert(System::LifeSupport, 1);
        let result: Result<(), UserActionError> =
            game_state.receive_user_action(UserActionWithPlayer {
                player: Player::Player1,
                user_action: UserAction::ChooseAction {
                    action: Action::ActivateSystem {
                        system: System::Weapons,
                        energy_to_use: Some(energy_to_use),
                        energy_distribution: None,
                    },
                },
            });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 1);
        assert_eq!(game_state.player1.fusion_reactor.energy, 2);
        assert_eq!(game_state.player1.weapons_system.energy, 1);
        assert_eq!(game_state.player1.life_support.energy, 1);
        assert_eq!(game_state.player1.shield_generator.energy, 1);
    }

    #[test]
    fn test_bypass_shield() {
        let mut game_state = GameState::start_state();
        game_state.player1.hand = vec![Card {
            instant_effects: vec![],
            hot_wire_effects: vec![Effect::BypassShield, Effect::BypassShield],
            hot_wire_cost: HotWireCost {
                short_circuits: 0,
                cards_to_discard: 0,
            },
            system: None,
        }];

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ChooseAction {
                action: Action::HotWireCard {
                    card_index: 0,
                    system: System::Weapons,
                    indices_to_discard: vec![],
                },
            },
        });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 2);

        let result: Result<(), UserActionError> =
            game_state.receive_user_action(UserActionWithPlayer {
                player: Player::Player1,
                user_action: UserAction::ChooseAction {
                    action: Action::ActivateSystem {
                        system: System::Weapons,
                        energy_to_use: None,
                        energy_distribution: None,
                    },
                },
            });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.actions_left, 1);
        match &game_state.turn_state {
            TurnState::ChoosingAction => assert!(false),
            TurnState::ResolvingEffects { effects } => {
                assert_eq!(
                    effects,
                    &vec![Effect::Attack, Effect::BypassShield, Effect::BypassShield]
                );
            }
        }

        let result: Result<(), UserActionError> =
            game_state.receive_user_action(UserActionWithPlayer {
                player: Player::Player1,
                user_action: UserAction::ResolveEffect {
                    resolve_effect: ResolveEffect::BypassShield,
                },
            });
        assert_eq!(result, Ok(()));
        assert_eq!(game_state.player2.hull_damage, 1);
        assert_eq!(game_state.player2.shields, 2);

        let result: Result<(), UserActionError> =
            game_state.receive_user_action(UserActionWithPlayer {
                player: Player::Player1,
                user_action: UserAction::ResolveEffect {
                    resolve_effect: ResolveEffect::BypassShield,
                },
            });
        assert_eq!(
            result,
            Err(UserActionError::CannotResolveBypassShieldWithoutAttack)
        );
    }
}

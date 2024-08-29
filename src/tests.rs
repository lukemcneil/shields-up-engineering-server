#[cfg(test)]
mod tests {
    use crate::*;

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
}

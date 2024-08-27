#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_bad_user_action() {
        let mut game_state = GameState::start_state();
        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                effect: Effect::Attack,
            },
        });
        assert!(result.is_err_and(|e| e == UserActionError::InvalidUserAction));

        let result = game_state.receive_user_action(UserActionWithPlayer {
            player: Player::Player1,
            user_action: UserAction::ResolveEffect {
                effect: Effect::Attack,
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
                card_indices_to_discard: vec![],
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
    fn test_activate_weaponse() {
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
        match game_state.turn_state {
            TurnState::ChoosingAction => assert!(false),
            TurnState::ResolvingEffects { effects } => {
                assert_eq!(effects, vec![Effect::Attack]);
            }
        }
    }
}

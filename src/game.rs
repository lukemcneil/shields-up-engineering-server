use std::collections::BTreeMap;

use cards::get_deck;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::cards;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub instant_effects: Vec<Effect>,
    pub hot_wire_effects: Vec<Effect>,
    pub hot_wire_cost: HotWireCost,
    pub system: Option<System>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotWireCost {
    pub short_circuits: i32,
    pub cards_to_discard: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Effect {
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
    // Discard, // this only appears as a cost to hot wire a card
    Draw,

    OpponentDiscard,
    OpponentGainShortCircuit,
    OpponentLoseShield,
    OpponentMoveEnergy,
    OpponentGainOverload,
    DrawPowerFrom(System),
    MoveEnergy,
    MoveEnergyTo(System),
    UseSystemCards(System),
    BypassShield,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolveEffect {
    GainShortCircuit,
    LoseShortCircuit,
    Shield,
    Attack,
    DiscardOverload {
        system: System,
    },
    GainAction,

    PlayHotWire {
        card_index: usize,
        system: System,
        indices_to_discard: Vec<usize>,
    },
    Draw,
    OpponentDiscard {
        card_index: usize,
    },
    OpponentGainShortCircuit,
    OpponentLoseShield,
    OpponentMoveEnergy {
        from_system: System,
        to_system: System,
    },
    OpponentGainOverload {
        system: System,
    },
    // DrawPowerFrom(System),
    MoveEnergy {
        from_system: System,
        to_system: System,
    },
    MoveEnergyTo {
        from_system: System,
        to_system: System,
    }, // UseSystemCards(System),
    BypassShield,
}

impl ResolveEffect {
    fn effect_this_resolves(&self) -> Effect {
        match self {
            ResolveEffect::Attack => Effect::Attack,
            ResolveEffect::GainShortCircuit => Effect::GainShortCircuit,
            ResolveEffect::LoseShortCircuit => Effect::LoseShortCircuit,
            ResolveEffect::Shield => Effect::Shield,
            ResolveEffect::DiscardOverload { .. } => Effect::DiscardOverload,
            ResolveEffect::GainAction => Effect::GainAction,
            ResolveEffect::PlayHotWire { .. } => Effect::PlayHotWire,
            ResolveEffect::Draw => Effect::Draw,
            ResolveEffect::OpponentGainShortCircuit => Effect::OpponentGainShortCircuit,
            ResolveEffect::OpponentLoseShield => Effect::OpponentLoseShield,
            ResolveEffect::OpponentGainOverload { .. } => Effect::OpponentGainOverload,
            ResolveEffect::OpponentMoveEnergy { .. } => Effect::OpponentMoveEnergy,
            ResolveEffect::MoveEnergy { .. } => Effect::MoveEnergy,
            ResolveEffect::MoveEnergyTo { to_system, .. } => Effect::MoveEnergyTo(*to_system),
            ResolveEffect::OpponentDiscard { .. } => Effect::OpponentDiscard,
            ResolveEffect::BypassShield => Effect::BypassShield,
        }
    }
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
            | Effect::Draw
            | Effect::OpponentGainShortCircuit
            | Effect::OpponentLoseShield
            | Effect::OpponentGainOverload
            | Effect::OpponentMoveEnergy
            | Effect::MoveEnergy
            | Effect::MoveEnergyTo(_)
            | Effect::OpponentDiscard
            | Effect::BypassShield => true,
            Effect::StoreMoreEnergy
            | Effect::UseMoreEnergy
            | Effect::UseLessEnergy
            | Effect::UseSystemCards(_)
            | Effect::DrawPowerFrom(_) => false,
        }
    }

    fn must_resolve(&self) -> bool {
        match self {
            Effect::GainShortCircuit | Effect::OpponentDiscard => true,
            Effect::LoseShortCircuit
            | Effect::StoreMoreEnergy
            | Effect::UseMoreEnergy
            | Effect::UseLessEnergy
            | Effect::Shield
            | Effect::Attack
            | Effect::DiscardOverload
            | Effect::GainAction
            | Effect::PlayHotWire
            | Effect::Draw
            | Effect::OpponentGainShortCircuit
            | Effect::OpponentLoseShield
            | Effect::OpponentGainOverload
            | Effect::OpponentMoveEnergy
            | Effect::MoveEnergy
            | Effect::MoveEnergyTo(_)
            | Effect::UseSystemCards(_)
            | Effect::DrawPowerFrom(_)
            | Effect::BypassShield => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemState {
    pub system: System,
    pub energy: i32,
    pub overloads: i32,
    pub hot_wires: Vec<Card>,
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

    pub fn get_allowed_energy(&self) -> i32 {
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

    fn get_allowed_system_cards(&self) -> Vec<System> {
        let mut additional: Vec<System> = self
            .get_hot_wire_effects()
            .iter()
            .filter_map(|effect| match effect {
                Effect::UseSystemCards(system) => Some(*system),
                _ => None,
            })
            .collect();
        additional.push(self.system);
        additional
    }

    fn get_allowed_systems_to_draw_energy_from(&self) -> Vec<System> {
        let mut additional: Vec<System> = self
            .get_hot_wire_effects()
            .iter()
            .filter_map(|effect| match effect {
                Effect::DrawPowerFrom(system) => Some(*system),
                _ => None,
            })
            .collect();
        additional.push(self.system);
        additional
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum System {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerState {
    pub hull_damage: i32,
    pub shields: i32,
    pub short_circuits: i32,
    pub hand: Vec<Card>,
    pub fusion_reactor: SystemState,
    pub life_support: SystemState,
    pub shield_generator: SystemState,
    pub weapons_system: SystemState,
}

impl PlayerState {
    fn start_state() -> Self {
        Self {
            hull_damage: 0,
            shields: 2,
            short_circuits: 0,
            hand: vec![],
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    HotWireCard {
        card_index: usize,
        system: System,
        indices_to_discard: Vec<usize>,
    },
    PlayInstantCard {
        card_index: usize,
    },
    ActivateSystem {
        system: System,
        energy_to_use: Option<BTreeMap<System, i32>>,
        energy_distribution: Option<BTreeMap<System, i32>>,
    },
    DiscardOverload {
        system: System,
    },
    ReduceShortCircuits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserAction {
    ChooseAction { action: Action },
    ResolveEffect { resolve_effect: ResolveEffect },
    Pass { card_indices_to_discard: Vec<usize> },
    StopResolvingEffects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActionWithPlayer {
    pub player: Player,
    pub user_action: UserAction,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn other_player(&self) -> Self {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TurnState {
    ChoosingAction,
    ResolvingEffects { effects: Vec<Effect> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub player1: PlayerState,
    pub player2: PlayerState,
    pub deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub players_turn: Player,
    pub actions_left: i32,
    pub turn_state: TurnState,
}

impl GameState {
    pub fn start_state() -> Self {
        let mut player1 = PlayerState::start_state();
        let mut player2 = PlayerState::start_state();
        let mut deck = get_deck();
        player1.hand = deck.drain(0..3).collect();
        player2.hand = deck.drain(0..3).collect();
        Self {
            players_turn: Player::Player1,
            turn_state: TurnState::ChoosingAction,
            player1,
            player2,
            deck,
            discard_pile: vec![],
            actions_left: 3,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum UserActionError {
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
    InvalidDiscardIndices,
    WrongNumberOfDiscardIndices,
    NoMatchingEffectToResolve,
    NoShortCircuitToRemove,
    AlreadyAtMaxShields,
    NoOverloadToDiscard,
    NoShieldsToLose,
    DiscardingCardPlayed,
    CannotHotWireCardOnThisSystem,
    StillHaveSomeEffectsThatMustBeResolved,
    NoCardToDraw,
    NoEnergyToMoveOnSystem,
    SystemAlreadyHasMaxEnergy,
    ActivePlayerCannotResolveOpponentDiscard,
    CannotDrawPowerFromSystem,
    IncorrectAmountOfEnergyToUse,
    CannotResolveBypassShieldWithoutAttack,
}

impl GameState {
    fn hot_wire_card(
        &mut self,
        card_index: usize,
        system: System,
        mut indices_to_discard: Vec<usize>,
        player: Player,
    ) -> Result<(), UserActionError> {
        let my_state = self.my_state(player);
        if card_index >= my_state.hand.len() {
            return Err(UserActionError::InvalidCardIndex);
        }
        let card = my_state.hand.remove(card_index);
        let system_state = my_state.get_system_state(system);
        if let Some(card_system) = card.system {
            let allowed_system_cards = system_state.get_allowed_system_cards();
            if !allowed_system_cards.contains(&card_system) {
                return Err(UserActionError::CannotHotWireCardOnThisSystem);
            }
        }
        system_state.hot_wires.push(card.clone());
        my_state.short_circuits =
            (my_state.short_circuits + card.hot_wire_cost.short_circuits).max(0);
        if card.hot_wire_cost.cards_to_discard > my_state.hand.len() {
            return Err(UserActionError::NotEnoughCardsToDiscard);
        }
        for i in &mut indices_to_discard {
            if *i == card_index {
                return Err(UserActionError::DiscardingCardPlayed);
            }
            if *i > card_index {
                *i -= 1;
            }
        }
        if indices_to_discard.len() != card.hot_wire_cost.cards_to_discard {
            return Err(UserActionError::WrongNumberOfDiscardIndices);
        }
        self.discard(player, indices_to_discard)?;
        Ok(())
    }

    fn choose_action(&mut self, action: Action, player: Player) -> Result<(), UserActionError> {
        if action.action_points() > self.actions_left {
            return Err(UserActionError::NotEnoughActionsLeft);
        }
        let my_state = self.my_state(player);
        let result = match action.clone() {
            Action::HotWireCard {
                card_index,
                system,
                indices_to_discard,
            } => self.hot_wire_card(card_index, system, indices_to_discard, player),
            Action::PlayInstantCard { card_index } => {
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
                energy_to_use,
                energy_distribution,
            } => {
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
                    let energy_to_use = energy_to_use.unwrap_or_else(|| {
                        let mut default_energy_to_use = BTreeMap::new();
                        default_energy_to_use.insert(system, system_state.get_energy_used());
                        default_energy_to_use
                    });
                    if energy_to_use.values().sum::<i32>() != system_state.get_energy_used() {
                        return Err(UserActionError::IncorrectAmountOfEnergyToUse);
                    }
                    let allowed_systems = system_state.get_allowed_systems_to_draw_energy_from();
                    for (system_getting_energy_from, energy_used) in energy_to_use {
                        if !allowed_systems.contains(&system_getting_energy_from) && energy_used > 0
                        {
                            return Err(UserActionError::CannotDrawPowerFromSystem);
                        }
                        let from_system_state =
                            my_state.get_system_state(system_getting_energy_from);
                        if energy_used > from_system_state.energy {
                            return Err(UserActionError::NotEnoughEnergyToActivate);
                        }
                        from_system_state.energy -= energy_used;
                        my_state.fusion_reactor.energy += energy_used;
                    }
                }
                self.turn_state = TurnState::ResolvingEffects {
                    effects: my_state.get_system_state(system).get_hot_wire_effects(),
                };
                Ok(())
            }
            Action::DiscardOverload { system } => {
                let system_state = my_state.get_system_state(system);
                if system_state.overloads > 0 {
                    system_state.overloads -= 1;
                    Ok(())
                } else {
                    Err(UserActionError::SystemHasNoOverload)
                }
            }
            Action::ReduceShortCircuits => {
                my_state.short_circuits = (my_state.short_circuits - 2).max(0);
                Ok(())
            }
        };
        if result.is_ok() {
            self.actions_left -= action.action_points();
        }
        result
    }

    fn resolve_effect(
        &mut self,
        effects_to_resolve: &mut Vec<Effect>,
        resolve_effect: ResolveEffect,
        player: Player,
    ) -> Result<(), UserActionError> {
        match effects_to_resolve
            .iter()
            .position(|&e| e == resolve_effect.effect_this_resolves())
        {
            Some(mut i) => {
                let my_state = self.my_state(player);
                match resolve_effect {
                    ResolveEffect::Attack => {
                        let opponent_state = self.opponent_state(player);
                        if opponent_state.shields > 0 {
                            opponent_state.shields -= 1;
                        } else {
                            opponent_state.hull_damage += 1;
                        }
                    }
                    ResolveEffect::GainShortCircuit => my_state.short_circuits += 1,
                    ResolveEffect::LoseShortCircuit => {
                        if my_state.short_circuits > 0 {
                            my_state.short_circuits -= 1;
                        } else {
                            return Err(UserActionError::NoShortCircuitToRemove);
                        }
                    }
                    ResolveEffect::Shield => {
                        let max_shields = my_state.shield_generator.get_allowed_energy();
                        if my_state.shields < max_shields {
                            my_state.shields += 1;
                        } else {
                            return Err(UserActionError::AlreadyAtMaxShields);
                        }
                    }
                    ResolveEffect::DiscardOverload { system } => {
                        let system_state = my_state.get_system_state(system);
                        if system_state.overloads > 0 {
                            system_state.overloads -= 1;
                        } else {
                            return Err(UserActionError::NoOverloadToDiscard);
                        }
                    }
                    ResolveEffect::GainAction => self.actions_left += 1,
                    ResolveEffect::PlayHotWire {
                        card_index,
                        system,
                        indices_to_discard,
                    } => self.hot_wire_card(card_index, system, indices_to_discard, player)?,
                    ResolveEffect::Draw => {
                        if self.deck.is_empty() {
                            self.deck.append(&mut self.discard_pile);
                            self.deck.shuffle(&mut thread_rng());
                        }
                        match self.deck.pop() {
                            Some(card) => self.my_state(player).hand.push(card),
                            None => return Err(UserActionError::NoCardToDraw),
                        };
                    }
                    ResolveEffect::OpponentGainShortCircuit => {
                        self.opponent_state(player).short_circuits += 1
                    }
                    ResolveEffect::OpponentLoseShield => {
                        let opponent_state = self.opponent_state(player);
                        if opponent_state.shields > 0 {
                            opponent_state.shields -= 1;
                        } else {
                            return Err(UserActionError::NoShieldsToLose);
                        }
                    }
                    ResolveEffect::OpponentGainOverload { system } => {
                        self.opponent_state(player)
                            .get_system_state(system)
                            .overloads += 1
                    }
                    ResolveEffect::OpponentMoveEnergy {
                        from_system,
                        to_system,
                    } => self.move_energy(from_system, to_system, player.other_player())?,
                    ResolveEffect::MoveEnergy {
                        from_system,
                        to_system,
                    }
                    | ResolveEffect::MoveEnergyTo {
                        from_system,
                        to_system,
                    } => self.move_energy(from_system, to_system, player)?,
                    ResolveEffect::OpponentDiscard { .. } => {
                        return Err(UserActionError::ActivePlayerCannotResolveOpponentDiscard)
                    }
                    ResolveEffect::BypassShield => {
                        match effects_to_resolve.iter().position(|&e| e == Effect::Attack) {
                            Some(index_of_attack) => {
                                effects_to_resolve.remove(index_of_attack);
                                if i >= index_of_attack {
                                    i -= 1;
                                }
                                self.opponent_state(player).hull_damage += 1;
                            }
                            None => {
                                return Err(UserActionError::CannotResolveBypassShieldWithoutAttack)
                            }
                        }
                    }
                }
                effects_to_resolve.remove(i);
                Ok(())
            }
            None => Err(UserActionError::NoMatchingEffectToResolve),
        }
    }

    fn move_energy(
        &mut self,
        from_system: System,
        to_system: System,
        player: Player,
    ) -> Result<(), UserActionError> {
        let my_state = self.my_state(player);
        let from_system_state = my_state.get_system_state(from_system);
        if from_system_state.energy <= 0 {
            return Err(UserActionError::NoEnergyToMoveOnSystem);
        }
        from_system_state.energy -= 1;

        let to_system_state = my_state.get_system_state(to_system);
        if to_system_state.overloads > 0 {
            return Err(UserActionError::CannotPutEnergyOnDisabledSystem);
        }
        if to_system_state.energy == to_system_state.get_allowed_energy() {
            return Err(UserActionError::SystemAlreadyHasMaxEnergy);
        }
        to_system_state.energy += 1;
        Ok(())
    }

    pub fn receive_user_action(
        &mut self,
        user_action_with_player: UserActionWithPlayer,
    ) -> Result<(), UserActionError> {
        let game_state_before = self.clone();
        let player = user_action_with_player.player;
        let result = if self.players_turn == player {
            match (self.turn_state.clone(), user_action_with_player.user_action) {
                (TurnState::ChoosingAction, UserAction::ChooseAction { action }) => {
                    self.choose_action(action, player)
                }
                (
                    TurnState::ResolvingEffects { mut effects },
                    UserAction::ResolveEffect { resolve_effect },
                ) => {
                    self.resolve_effect(&mut effects, resolve_effect, player)?;
                    if effects.is_empty() {
                        self.turn_state = TurnState::ChoosingAction;
                    } else {
                        self.turn_state = TurnState::ResolvingEffects { effects };
                    }
                    Ok(())
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
                    if self.my_state(player).hand.len() > 5 {
                        let cards_to_discard = self.my_state(player).hand.len() - 5;
                        if cards_to_discard != card_indices_to_discard.len() {
                            return Err(UserActionError::WrongNumberOfDiscardIndices);
                        }
                        self.discard(player, card_indices_to_discard)?;
                    }
                    // TODO: check short circuits
                    return Ok(());
                }
                (TurnState::ResolvingEffects { effects }, UserAction::StopResolvingEffects) => {
                    if effects.iter().any(Effect::must_resolve) {
                        return Err(UserActionError::StillHaveSomeEffectsThatMustBeResolved);
                    }
                    self.turn_state = TurnState::ChoosingAction;
                    Ok(())
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
                (TurnState::ChoosingAction, UserAction::StopResolvingEffects) => {
                    Err(UserActionError::InvalidUserAction)
                }
            }
        } else {
            match (self.turn_state.clone(), user_action_with_player.user_action) {
                (
                    TurnState::ResolvingEffects { mut effects },
                    UserAction::ResolveEffect { resolve_effect },
                ) => {
                    if let ResolveEffect::OpponentDiscard { card_index } = resolve_effect {
                        match effects
                            .iter()
                            .position(|&e| e == resolve_effect.effect_this_resolves())
                        {
                            Some(i) => {
                                let my_state = self.my_state(player);
                                if card_index >= my_state.hand.len() {
                                    return Err(UserActionError::InvalidCardIndex);
                                }
                                let card = my_state.hand.remove(card_index);
                                self.discard_pile.push(card);
                                effects.remove(i);
                                if effects.is_empty() {
                                    self.turn_state = TurnState::ChoosingAction;
                                } else {
                                    self.turn_state = TurnState::ResolvingEffects { effects };
                                }
                                Ok(())
                            }
                            None => return Err(UserActionError::NoMatchingEffectToResolve),
                        }
                    } else {
                        Err(UserActionError::NotYourTurn)
                    }
                }
                _ => Err(UserActionError::NotYourTurn),
            }
        };
        if result.is_err() {
            *self = game_state_before;
        }
        self.remove_effects_without_immediate_effects();
        self.remove_opponent_discards_if_no_cards();
        // TODO: check if someone won
        result
    }

    fn discard(
        &mut self,
        player: Player,
        mut card_indices: Vec<usize>,
    ) -> Result<(), UserActionError> {
        card_indices.sort();
        card_indices.reverse();
        for i in card_indices {
            let my_state = self.my_state(player);
            if i >= my_state.hand.len() {
                return Err(UserActionError::InvalidDiscardIndices);
            }
            let discarded_card = my_state.hand.remove(i);
            self.discard_pile.push(discarded_card);
        }
        Ok(())
    }

    fn remove_effects_without_immediate_effects(&mut self) {
        if let TurnState::ResolvingEffects { effects } = &mut self.turn_state {
            effects.retain(Effect::has_immediate_effect);
        }
    }

    fn remove_opponent_discards_if_no_cards(&mut self) {
        if self.opponent_state(self.players_turn).hand.is_empty() {
            if let TurnState::ResolvingEffects { effects } = &mut self.turn_state {
                effects.retain(|&e| e != Effect::OpponentDiscard);
            }
        }
    }

    fn my_state(&mut self, player: Player) -> &mut PlayerState {
        match player {
            Player::Player1 => &mut self.player1,
            Player::Player2 => &mut self.player2,
        }
    }

    pub fn my_state_immut(&self, player: Player) -> &PlayerState {
        match player {
            Player::Player1 => &self.player1,
            Player::Player2 => &self.player2,
        }
    }

    fn opponent_state(&mut self, player: Player) -> &mut PlayerState {
        match player {
            Player::Player1 => &mut self.player2,
            Player::Player2 => &mut self.player1,
        }
    }
}

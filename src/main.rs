mod input;

use std::collections::BTreeMap;

use input::{choose_action, choose_card_index, choose_system};

#[derive(Debug, Clone, Default)]
struct Card {
    instant_effects: Vec<Effect>,
    hot_wire_effects: Vec<Effect>,
    hot_wire_cost: Vec<Effect>,
    system: Option<System>,
}

#[derive(Debug, Clone, Copy)]
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
    SpendAction,

    PlayHotWire,
    Discard,
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
}

#[derive(Default, Debug, Clone)]
struct SystemState {
    energy: i32,
    overloads: i32,
    hot_wires: Vec<Card>,
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
            System::FusionReactor => vec![],
            System::LifeSupport => vec![Effect::Draw, Effect::Draw],
            System::Weapons => vec![Effect::Attack],
            System::ShieldGenerator => vec![Effect::Shield],
        }
    }
}

#[derive(Debug, Clone)]
struct PlayerState {
    hull_damage: i32,
    shields: i32,
    short_circuits: i32,
    hand: Vec<Card>,
    systems: BTreeMap<System, SystemState>,
    actions: i32,
}

impl PlayerState {
    fn start_state() -> Self {
        let mut systems = BTreeMap::new();
        systems.insert(
            System::FusionReactor,
            SystemState {
                energy: 5,
                ..Default::default()
            },
        );
        systems.insert(System::LifeSupport, SystemState::default());
        systems.insert(System::Weapons, SystemState::default());
        systems.insert(System::ShieldGenerator, SystemState::default());
        Self {
            hull_damage: 0,
            shields: 2,
            short_circuits: 0,
            hand: vec![
                Card::default(),
                Card::default(),
                Card::default(),
                Card::default(),
                Card::default(),
                Card::default(),
                Card::default(),
            ],
            systems,
            actions: 3,
        }
    }

    fn do_turn(&mut self, opponent_state: &mut PlayerState, shared_state: &mut SharedState) {
        self.actions = 3;
        while let Some(action) = choose_action(self) {
            if let Err(e) = self.try_to_do_action(action, opponent_state, shared_state) {
                println!("failed to do action {:?} because {}", action, e);
            }
        }
        self.check_hand_size(shared_state);
        self.check_system_overload();
    }

    fn resolve_effects(
        &mut self,
        effects: &[Effect],
        opponent_state: &mut PlayerState,
    ) -> ResolveEffectResult {
        for effect in effects {
            match effect {
                Effect::GainShortCircuit => todo!(),
                Effect::LoseShortCircuit => todo!(),
                Effect::StoreMoreEnergy => todo!(),
                Effect::UseMoreEnergy => todo!(),
                Effect::UseLessEnergy => todo!(),
                Effect::Shield => todo!(),
                Effect::Attack => todo!(),
                Effect::DiscardOverload => todo!(),
                Effect::GainAction => todo!(),
                Effect::SpendAction => todo!(),
                Effect::PlayHotWire => todo!(),
                Effect::Discard => todo!(),
                Effect::Draw => todo!(),
                Effect::OpponentDiscard => todo!(),
                Effect::OpponentGainShortCircuit => todo!(),
                Effect::OpponentLoseShield => todo!(),
                Effect::OpponentMoveEnergy => todo!(),
                Effect::OpponentGainOverload => todo!(),
                Effect::DrawPowerFrom(_) => todo!(),
                Effect::MoveEnergy => todo!(),
                Effect::MoveEnergyTo(_) => todo!(),
                Effect::UseSystemCards(_) => todo!(),
            }
        }
        ResolveEffectResult::Resolved
    }

    fn try_to_do_action(
        &mut self,
        action: Action,
        opponent_state: &mut PlayerState,
        shared_state: &mut SharedState,
    ) -> Result<(), String> {
        if action.action_points() > self.actions {
            return Err(format!(
                "do not have enough action points to do {:?}",
                action
            ));
        }
        let my_state_before = self.clone();
        let opponent_state_before = opponent_state.clone();
        let result = match action {
            Action::HotWireCard { card_index, system } => {
                let card = self.hand.remove(card_index);
                if let Some(card_system) = card.system {
                    if system != card_system {
                        return Err(format!(
                            "cannot hot wire {:?} card on {:?}",
                            card_system, system
                        ));
                    }
                }
                if self.resolve_effects(&card.hot_wire_cost, opponent_state)
                    == ResolveEffectResult::CouldNotDiscard
                {
                    return Err("don't have enough cards to discard".to_string());
                }
                self.systems.get_mut(&system).unwrap().hot_wires.push(card);
                Ok(())
            }
            Action::PlayInstantCard { card_index } => {
                let card = self.hand.remove(card_index);
                if self.resolve_effects(&card.instant_effects, opponent_state)
                    == ResolveEffectResult::CouldNotDiscard
                {
                    return Err("don't have enough cards to discard".to_string());
                }
                shared_state.discard_pile.push(card);
                Ok(())
            }
            Action::ActivateSystem { system } => {
                let system_state = self.systems.get_mut(&system).unwrap();
                let mut system_effects = system.starting_effects();
                for hot_wire_card in &system_state.hot_wires {
                    system_effects.append(&mut hot_wire_card.hot_wire_effects.clone());
                }
                self.resolve_effects(&system_effects, opponent_state);
                Ok(())
            }
            Action::DiscardOverload { system } => {
                let system_state = self.systems.get_mut(&system).unwrap();
                if system_state.overloads == 0 {
                    Err(format!("no overloads to remove on {:?}", system))
                } else {
                    system_state.overloads -= 1;
                    Ok(())
                }
            }
            Action::ReduceShortCircuits => {
                self.resolve_effects(
                    &[Effect::LoseShortCircuit, Effect::LoseShortCircuit],
                    opponent_state,
                );
                Ok(())
            }
        };
        match result {
            Ok(_) => {
                self.actions -= action.action_points();
            }
            Err(_) => {
                *self = my_state_before;
                *opponent_state = opponent_state_before
            }
        };
        result
    }

    fn check_hand_size(&mut self, shared_state: &mut SharedState) {
        if self.hand.len() > 5 {
            println!(
                "over hand limit need to discard {} cards",
                self.hand.len() - 5
            );
            self.discard(self.hand.len() - 5, shared_state);
        }
    }

    fn discard(&mut self, amount: usize, shared_state: &mut SharedState) -> bool {
        for _ in 0..amount {
            if self.hand.is_empty() {
                return false;
            }
            println!("choose a card to discard");
            let index = choose_card_index(self.hand.len()).expect("just checked hand is not empty");
            let discarded_card = self.hand.remove(index);
            println!("discarded {:?}", discarded_card);
            shared_state.discard_pile.push(discarded_card);
        }
        true
    }

    fn check_system_overload(&mut self) {
        while self.short_circuits >= 5 {
            println!("have 5 or more short circuits, system overload, choose a system to disable");
            self.overload_system(choose_system());
            self.short_circuits -= 5;
        }
    }

    fn overload_system(&mut self, system: System) {
        let system_state = self.systems.get_mut(&system).unwrap();
        system_state.overloads += 1;
        let energy_to_move = system_state.energy;
        system_state.energy = 0;

        let fusion_reactor_state = self.systems.get_mut(&System::FusionReactor).unwrap();
        fusion_reactor_state.energy += energy_to_move;
    }
}

#[derive(PartialEq, Eq)]
enum ResolveEffectResult {
    Resolved,
    CouldNotDiscard,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    HotWireCard { card_index: usize, system: System },
    PlayInstantCard { card_index: usize },
    ActivateSystem { system: System },
    DiscardOverload { system: System },
    ReduceShortCircuits,
}

impl Action {
    fn action_points(&self) -> i32 {
        match self {
            Action::HotWireCard { .. } => 1,
            Action::PlayInstantCard { .. } => 0,
            Action::ActivateSystem { system } => match system {
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

struct SharedState {
    deck: Vec<Card>,
    discard_pile: Vec<Card>,
}

fn main() {
    let mut state1 = PlayerState::start_state();
    let mut state2 = PlayerState::start_state();
    let mut shared_state = SharedState {
        deck: vec![],
        discard_pile: vec![],
    };
    state1.do_turn(&mut state2, &mut shared_state);
    println!("{:?}", state1);
}

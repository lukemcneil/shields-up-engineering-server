mod input;

use std::collections::BTreeMap;

use input::{choose_action, choose_system};

#[derive(Debug, Clone)]
struct Card {
    instant_effects: Vec<Effect>,
    hot_wire_effects: Vec<Effect>,
    hot_wire_cost: Vec<Effect>,
    system: Option<System>,
}

#[derive(Debug, Clone)]
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
            hand: vec![],
            systems,
            actions: 3,
        }
    }

    fn do_turn(&mut self, opponent_state: &mut PlayerState, shared_state: &mut SharedState) {
        self.actions = 3;
        while let Some(action) = choose_action(self) {
            if let Err(e) = self.try_to_do_action(action, opponent_state) {
                println!("failed to do action {:?} because {}", action, e);
            }
        }
        self.check_hand_size(shared_state);
        self.check_system_overload();
    }

    fn try_to_do_action(
        &mut self,
        action: Action,
        opponent_state: &mut PlayerState,
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
            Action::HotWireCard { card_index } => todo!(),
            Action::PlayInstantCard { card_index } => todo!(),
            Action::ActivateSystem { system } => todo!(),
            Action::DiscardOverload { system } => Err("todo".to_string()),
            Action::ReduceShortCircuits => {
                self.short_circuits = (self.short_circuits - 2).max(0);
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

    fn check_hand_size(&mut self, shared_state: &mut SharedState) {}

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

#[derive(Debug, Clone, Copy)]
enum Action {
    HotWireCard { card_index: usize },
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

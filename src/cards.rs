use rand::{seq::SliceRandom, thread_rng};

use crate::game::{Card, Effect, HotWireCost, System};

pub fn get_deck() -> Vec<Card> {
    let mut deck = vec![
        attack_01(),
        attack_02(),
        draw_01(),
        draw_06(),
        generic_01(),
        generic_02(),
        generic_07(),
        power_05b(),
        shields_01(),
        shields_02(),
        // copy 2
        attack_01(),
        attack_02(),
        draw_01(),
        draw_06(),
        generic_01(),
        generic_02(),
        generic_07(),
        power_05b(),
        shields_01(),
        shields_02(),
        // copy 3
        attack_01(),
        attack_02(),
        draw_01(),
        draw_06(),
        generic_01(),
        generic_02(),
        generic_07(),
        power_05b(),
        shields_01(),
        shields_02(),
    ];
    deck.shuffle(&mut thread_rng());
    deck
}

fn attack_01() -> Card {
    Card {
        instant_effects: vec![Effect::Attack],
        hot_wire_effects: vec![Effect::Attack, Effect::UseMoreEnergy],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::Weapons),
        name: "attack_01".to_string(),
    }
}

fn attack_02() -> Card {
    Card {
        instant_effects: vec![Effect::Attack],
        hot_wire_effects: vec![
            Effect::Attack,
            Effect::Attack,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
            Effect::UseMoreEnergy,
            Effect::UseMoreEnergy,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: Some(System::Weapons),
        name: "attack_02".to_string(),
    }
}

fn draw_01() -> Card {
    Card {
        instant_effects: vec![
            Effect::Draw,
            Effect::Draw,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![Effect::Draw, Effect::GainShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: Some(System::LifeSupport),
        name: "draw_01".to_string(),
    }
}

fn draw_06() -> Card {
    Card {
        instant_effects: vec![Effect::GainAction, Effect::LoseShortCircuit],
        hot_wire_effects: vec![Effect::OpponentGainShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 2,
            cards_to_discard: 0,
        },
        system: Some(System::LifeSupport),
        name: "draw_06".to_string(),
    }
}

fn generic_01() -> Card {
    Card {
        instant_effects: vec![Effect::GainAction, Effect::LoseShortCircuit],
        hot_wire_effects: vec![Effect::UseLessEnergy],
        hot_wire_cost: HotWireCost {
            short_circuits: 2,
            cards_to_discard: 0,
        },
        system: None,
        name: "generic_01".to_string(),
    }
}

fn generic_02() -> Card {
    Card {
        instant_effects: vec![Effect::GainAction, Effect::LoseShortCircuit],
        hot_wire_effects: vec![Effect::LoseShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: None,
        name: "generic_02".to_string(),
    }
}

fn generic_07() -> Card {
    Card {
        instant_effects: vec![
            Effect::LoseShortCircuit,
            Effect::LoseShortCircuit,
            Effect::LoseShortCircuit,
            Effect::LoseShortCircuit,
        ],
        hot_wire_effects: vec![Effect::StoreMoreEnergy],
        hot_wire_cost: HotWireCost {
            short_circuits: -1,
            cards_to_discard: 0,
        },
        system: None,
        name: "generic_07".to_string(),
    }
}

fn power_05b() -> Card {
    Card {
        instant_effects: vec![Effect::GainAction, Effect::LoseShortCircuit],
        hot_wire_effects: vec![Effect::LoseShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: Some(System::FusionReactor),
        name: "power_05b".to_string(),
    }
}

fn shields_01() -> Card {
    Card {
        instant_effects: vec![Effect::Shield, Effect::LoseShortCircuit],
        hot_wire_effects: vec![Effect::Shield, Effect::UseMoreEnergy],
        hot_wire_cost: HotWireCost {
            short_circuits: -1,
            cards_to_discard: 0,
        },
        system: Some(System::ShieldGenerator),
        name: "shields_01".to_string(),
    }
}

fn shields_02() -> Card {
    Card {
        instant_effects: vec![Effect::Shield, Effect::LoseShortCircuit],
        hot_wire_effects: vec![
            Effect::Shield,
            Effect::Shield,
            Effect::UseMoreEnergy,
            Effect::UseMoreEnergy,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::ShieldGenerator),
        name: "shields_02".to_string(),
    }
}

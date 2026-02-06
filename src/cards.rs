use rand::{seq::SliceRandom, thread_rng};

use crate::game::{Card, Effect, HotWireCost, System};

pub fn get_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(100);

    // attack_01 x4
    for _ in 0..4 { deck.push(attack_01()); }
    // attack_02 x2
    for _ in 0..2 { deck.push(attack_02()); }
    // attack_05 x2
    for _ in 0..2 { deck.push(attack_05()); }
    // attack_06 x2
    for _ in 0..2 { deck.push(attack_06()); }
    // attack_07 x2
    for _ in 0..2 { deck.push(attack_07()); }
    // attack_08 x2
    for _ in 0..2 { deck.push(attack_08()); }
    // attack_10 x1
    deck.push(attack_10());
    // attack_11 x1
    deck.push(attack_11());

    // draw_01 x6
    for _ in 0..6 { deck.push(draw_01()); }
    // draw_06 x4
    for _ in 0..4 { deck.push(draw_06()); }
    // draw_06b x2
    for _ in 0..2 { deck.push(draw_06b()); }
    // draw_08 x2
    for _ in 0..2 { deck.push(draw_08()); }
    // draw_10 x1
    deck.push(draw_10());
    // draw_11 x1
    deck.push(draw_11());

    // generic_01 x8
    for _ in 0..8 { deck.push(generic_01()); }
    // generic_02 x3
    for _ in 0..3 { deck.push(generic_02()); }
    // generic_04 x3
    for _ in 0..3 { deck.push(generic_04()); }
    // generic_04c x3
    for _ in 0..3 { deck.push(generic_04c()); }
    // generic_04d x3
    for _ in 0..3 { deck.push(generic_04d()); }
    // generic_06 x7
    for _ in 0..7 { deck.push(generic_06()); }
    // generic_07 x9
    for _ in 0..9 { deck.push(generic_07()); }

    // power_04 x2
    for _ in 0..2 { deck.push(power_04()); }
    // power_05 x4
    for _ in 0..4 { deck.push(power_05()); }
    // power_05b x4
    for _ in 0..4 { deck.push(power_05b()); }
    // power_06 x2
    for _ in 0..2 { deck.push(power_06()); }
    // power_07 x2
    for _ in 0..2 { deck.push(power_07()); }
    // power_08 x2
    for _ in 0..2 { deck.push(power_08()); }

    // shields_01 x4
    for _ in 0..4 { deck.push(shields_01()); }
    // shields_02 x2
    for _ in 0..2 { deck.push(shields_02()); }
    // shields_05 x2
    for _ in 0..2 { deck.push(shields_05()); }
    // shields_06 x2
    for _ in 0..2 { deck.push(shields_06()); }
    // shields_07 x2
    for _ in 0..2 { deck.push(shields_07()); }
    // shields_08 x2
    for _ in 0..2 { deck.push(shields_08()); }
    // shields_10 x1
    deck.push(shields_10());
    // shields_11 x1
    deck.push(shields_11());

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

fn attack_05() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainOverload,
            Effect::DiscardOverload,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![Effect::MoveEnergyTo(System::LifeSupport)],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::Weapons),
        name: "attack_05".to_string(),
    }
}

fn attack_06() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainOverload,
            Effect::DiscardOverload,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![Effect::MoveEnergyTo(System::ShieldGenerator)],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::Weapons),
        name: "attack_06".to_string(),
    }
}

fn attack_07() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainOverload,
            Effect::DiscardOverload,
            Effect::GainShortCircuit,
            Effect::BypassShield,
        ],
        hot_wire_effects: vec![
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
            Effect::UseMoreEnergy,
            Effect::UseMoreEnergy,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 4,
            cards_to_discard: 0,
        },
        system: Some(System::Weapons),
        name: "attack_07".to_string(),
    }
}

fn attack_08() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainOverload,
            Effect::DiscardOverload,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::Attack,
            Effect::Attack,
            Effect::Attack,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
            Effect::UseMoreEnergy,
            Effect::UseMoreEnergy,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 4,
            cards_to_discard: 0,
        },
        system: Some(System::Weapons),
        name: "attack_08".to_string(),
    }
}

fn attack_10() -> Card {
    Card {
        instant_effects: vec![
            Effect::PlayHotWire,
            Effect::PlayHotWire,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::UseSystemCards(System::LifeSupport),
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::Weapons),
        name: "attack_10".to_string(),
    }
}

fn attack_11() -> Card {
    Card {
        instant_effects: vec![
            Effect::LoseShortCircuit,
            Effect::LoseShortCircuit,
            Effect::DiscardOverload,
        ],
        hot_wire_effects: vec![
            Effect::UseSystemCards(System::ShieldGenerator),
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::Weapons),
        name: "attack_11".to_string(),
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

fn draw_06b() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainShortCircuit,
            Effect::OpponentGainShortCircuit,
            Effect::LoseShortCircuit,
        ],
        hot_wire_effects: vec![Effect::OpponentGainShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 2,
            cards_to_discard: 0,
        },
        system: Some(System::LifeSupport),
        name: "draw_06b".to_string(),
    }
}

fn draw_08() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainShortCircuit,
            Effect::OpponentGainShortCircuit,
            Effect::LoseShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::OpponentGainShortCircuit,
            Effect::OpponentGainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 3,
            cards_to_discard: 1,
        },
        system: Some(System::LifeSupport),
        name: "draw_08".to_string(),
    }
}

fn draw_10() -> Card {
    Card {
        instant_effects: vec![
            Effect::PlayHotWire,
            Effect::PlayHotWire,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::UseSystemCards(System::Weapons),
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::LifeSupport),
        name: "draw_10".to_string(),
    }
}

fn draw_11() -> Card {
    Card {
        instant_effects: vec![
            Effect::LoseShortCircuit,
            Effect::LoseShortCircuit,
            Effect::DiscardOverload,
        ],
        hot_wire_effects: vec![
            Effect::UseSystemCards(System::ShieldGenerator),
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::LifeSupport),
        name: "draw_11".to_string(),
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

fn generic_04() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainOverload,
            Effect::DiscardOverload,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::DrawPowerFrom(System::LifeSupport),
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: None,
        name: "generic_04".to_string(),
    }
}

fn generic_04c() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainOverload,
            Effect::DiscardOverload,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::DrawPowerFrom(System::ShieldGenerator),
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: None,
        name: "generic_04c".to_string(),
    }
}

fn generic_04d() -> Card {
    Card {
        instant_effects: vec![
            Effect::DiscardOverload,
            Effect::DiscardOverload,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::DrawPowerFrom(System::Weapons),
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: None,
        name: "generic_04d".to_string(),
    }
}

fn generic_06() -> Card {
    Card {
        instant_effects: vec![
            Effect::PlayHotWire,
            Effect::LoseShortCircuit,
            Effect::LoseShortCircuit,
        ],
        hot_wire_effects: vec![Effect::StoreMoreEnergy, Effect::StoreMoreEnergy],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: None,
        name: "generic_06".to_string(),
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

fn power_04() -> Card {
    Card {
        instant_effects: vec![
            Effect::MoveEnergy,
            Effect::MoveEnergy,
            Effect::LoseShortCircuit,
        ],
        hot_wire_effects: vec![Effect::OpponentMoveEnergy, Effect::GainShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: Some(System::FusionReactor),
        name: "power_04".to_string(),
    }
}

fn power_05() -> Card {
    Card {
        instant_effects: vec![
            Effect::MoveEnergy,
            Effect::MoveEnergy,
            Effect::LoseShortCircuit,
        ],
        hot_wire_effects: vec![Effect::LoseShortCircuit, Effect::LoseShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 3,
            cards_to_discard: 0,
        },
        system: Some(System::FusionReactor),
        name: "power_05".to_string(),
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

fn power_06() -> Card {
    Card {
        instant_effects: vec![
            Effect::MoveEnergy,
            Effect::MoveEnergy,
            Effect::LoseShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::OpponentMoveEnergy,
            Effect::OpponentMoveEnergy,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 3,
            cards_to_discard: 0,
        },
        system: Some(System::FusionReactor),
        name: "power_06".to_string(),
    }
}

fn power_07() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentDiscard,
            Effect::OpponentDiscard,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![Effect::DiscardOverload],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 1,
        },
        system: Some(System::FusionReactor),
        name: "power_07".to_string(),
    }
}

fn power_08() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentDiscard,
            Effect::OpponentDiscard,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![Effect::GainAction, Effect::GainShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 1,
        },
        system: Some(System::FusionReactor),
        name: "power_08".to_string(),
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

fn shields_05() -> Card {
    Card {
        instant_effects: vec![Effect::GainAction, Effect::LoseShortCircuit],
        hot_wire_effects: vec![
            Effect::OpponentDiscard,
            Effect::OpponentDiscard,
            Effect::UseMoreEnergy,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 3,
            cards_to_discard: 0,
        },
        system: Some(System::ShieldGenerator),
        name: "shields_05".to_string(),
    }
}

fn shields_06() -> Card {
    Card {
        instant_effects: vec![Effect::GainAction, Effect::LoseShortCircuit],
        hot_wire_effects: vec![Effect::OpponentDiscard, Effect::GainShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: Some(System::ShieldGenerator),
        name: "shields_06".to_string(),
    }
}

fn shields_07() -> Card {
    Card {
        instant_effects: vec![Effect::OpponentLoseShield, Effect::LoseShortCircuit],
        hot_wire_effects: vec![Effect::OpponentDiscard, Effect::GainShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: Some(System::ShieldGenerator),
        name: "shields_07".to_string(),
    }
}

fn shields_08() -> Card {
    Card {
        instant_effects: vec![Effect::OpponentLoseShield, Effect::LoseShortCircuit],
        hot_wire_effects: vec![Effect::OpponentDiscard, Effect::GainShortCircuit],
        hot_wire_cost: HotWireCost {
            short_circuits: 0,
            cards_to_discard: 0,
        },
        system: Some(System::ShieldGenerator),
        name: "shields_08".to_string(),
    }
}

fn shields_10() -> Card {
    Card {
        instant_effects: vec![
            Effect::PlayHotWire,
            Effect::PlayHotWire,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::UseSystemCards(System::LifeSupport),
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::ShieldGenerator),
        name: "shields_10".to_string(),
    }
}

fn shields_11() -> Card {
    Card {
        instant_effects: vec![
            Effect::LoseShortCircuit,
            Effect::LoseShortCircuit,
            Effect::DiscardOverload,
        ],
        hot_wire_effects: vec![
            Effect::UseSystemCards(System::Weapons),
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: Some(System::ShieldGenerator),
        name: "shields_11".to_string(),
    }
}

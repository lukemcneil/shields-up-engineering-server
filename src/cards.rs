use rand::{seq::SliceRandom, thread_rng};

use crate::game::{Card, Effect, HotWireCost, System};

pub fn get_deck() -> Vec<Card> {
    let mut deck = vec![
        get_card1(),
        get_card1(),
        get_card1(),
        get_card1(),
        get_card1(),
        get_card1(),
        get_card1(),
        get_card1(),
        get_card1(),
        get_card1(),
        get_card1(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card2(),
        get_card3(),
        get_card3(),
        get_card3(),
        get_card3(),
        get_card3(),
        get_card3(),
        get_card3(),
        get_card3(),
        get_card3(),
        get_card3(),
    ];
    deck.shuffle(&mut thread_rng());
    deck
}

fn get_card1() -> Card {
    Card {
        instant_effects: vec![
            Effect::PlayHotWire,
            Effect::GainShortCircuit,
            Effect::GainShortCircuit,
            Effect::MoveEnergy,
            Effect::MoveEnergyTo(System::Weapons),
            Effect::OpponentDiscard,
            Effect::OpponentDiscard,
            Effect::Draw,
            Effect::Draw,
        ],
        hot_wire_effects: vec![
            Effect::StoreMoreEnergy,
            Effect::StoreMoreEnergy,
            Effect::UseSystemCards(System::Weapons),
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: None,
        name: "power_08".to_string(),
    }
}

fn get_card2() -> Card {
    Card {
        instant_effects: vec![Effect::Shield, Effect::GainShortCircuit, Effect::Attack],
        hot_wire_effects: vec![
            Effect::UseLessEnergy,
            Effect::BypassShield,
            Effect::Attack,
            Effect::DiscardOverload,
            Effect::Draw,
            Effect::Draw,
            Effect::OpponentDiscard,
            Effect::OpponentDiscard,
            Effect::DrawPowerFrom(System::Weapons),
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 0,
        },
        system: None,
        name: "generic_01".to_string(),
    }
}

fn get_card3() -> Card {
    Card {
        instant_effects: vec![
            Effect::OpponentGainOverload,
            Effect::DiscardOverload,
            Effect::GainShortCircuit,
        ],
        hot_wire_effects: vec![
            Effect::BypassShield,
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
        system: Some(System::LifeSupport),
        name: "attack_01".to_string(),
    }
}

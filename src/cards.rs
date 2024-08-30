use rand::{seq::SliceRandom, thread_rng};

use crate::{Card, Effect, HotWireCost, System};

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
    }
}

fn get_card2() -> Card {
    Card {
        instant_effects: vec![Effect::Shield, Effect::GainShortCircuit, Effect::Attack],
        hot_wire_effects: vec![
            Effect::UseLessEnergy,
            Effect::Attack,
            Effect::DiscardOverload,
            Effect::Draw,
            Effect::Draw,
            Effect::OpponentDiscard,
            Effect::OpponentDiscard,
        ],
        hot_wire_cost: HotWireCost {
            short_circuits: 1,
            cards_to_discard: 1,
        },
        system: None,
    }
}

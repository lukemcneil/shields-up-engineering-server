use std::{collections::BTreeMap, io};

use crate::{Action, PlayerState, System, SystemState};

pub fn choose_card_index(cards_len: usize) -> Option<usize> {
    if cards_len == 0 {
        return None;
    }
    if cards_len == 1 {
        println!("only one choice, choosing 0");
        return Some(0);
    } else {
        println!("choose a card from index {} to {}", 0, cards_len - 1);
    }
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let number: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number!");
                continue;
            }
        };
        if number < cards_len {
            return Some(number);
        } else {
            println!("That's not a valid index for");
        }
    }
}

pub fn choose_system() -> System {
    loop {
        println!("Choose a system:\n\t0 - Fusion Reactor\n\t1 - Life Support\n\t2 - Weapons System\n\t3 - Shield Generator");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let number: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number!");
                continue;
            }
        };
        match number {
            0 => return System::FusionReactor,
            1 => return System::LifeSupport,
            2 => return System::Weapons,
            3 => return System::ShieldGenerator,
            _ => println!("That's not a valid choice!"),
        }
    }
}

pub fn choose_action(my_state: &PlayerState) -> Option<Action> {
    loop {
        println!("Actions left: {}", my_state.actions);
        println!("Choose an action:\n\t0 - Hot Wire\n\t1 - Play Instant\n\t2 - Activate System\n\t3 - Discard Disabled Token\n\t4 - Reduce Short Circuits By 2\n\t5 - Pass");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let number: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number!");
                continue;
            }
        };
        match number {
            0 => match choose_card_index(my_state.hand.len()) {
                Some(card_index) => {
                    return Some(Action::HotWireCard {
                        card_index,
                        system: choose_system(),
                    })
                }
                None => println!("Cannot hot wire card, don't have any cards"),
            },
            1 => match choose_card_index(my_state.hand.len()) {
                Some(card_index) => return Some(Action::PlayInstantCard { card_index }),
                None => println!("Cannot play instant card, don't have any cards"),
            },
            2 => {
                return Some(Action::ActivateSystem {
                    system: choose_system(),
                })
            }
            3 => {
                return Some(Action::DiscardOverload {
                    system: choose_system(),
                })
            }
            4 => return Some(Action::ReduceShortCircuits),
            5 => return None,
            _ => println!("That's not a valid choice!"),
        }
    }
}

fn choose_energy_amount(system_state: &SystemState, max_energy: i32) -> i32 {
    if system_state.overloads > 0 {
        println!("cannot put any energy on because overloaded");
        return 0;
    }
    if max_energy == 0 {
        println!("do not have any energy left to allocate");
        return 0;
    }
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let number: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a valid number!");
                continue;
            }
        };
        if number <= max_energy as usize {
            return number as i32;
        } else {
            println!("That's more than the allowed energy");
        }
    }
}

pub fn choose_energy_distribution(
    my_state: &PlayerState,
    mut total_energy: i32,
) -> BTreeMap<System, i32> {
    let mut energy_distribution = BTreeMap::new();
    println!("distribute {total_energy} energy");

    let allowed_energy = my_state.life_support.get_allowed_energy();
    println!(
        "how much should go on Life Support of remaining {total_energy} allowed {allowed_energy}",
    );
    let allocated_energy =
        choose_energy_amount(&my_state.life_support, allowed_energy.min(total_energy));
    total_energy -= allocated_energy;
    energy_distribution.insert(System::LifeSupport, allocated_energy);

    let allowed_energy = my_state.shield_generator.get_allowed_energy();
    println!(
        "how much should go on Shield Generator of remaining {total_energy} allowed {allowed_energy}",
    );
    let allocated_energy =
        choose_energy_amount(&my_state.shield_generator, allowed_energy.min(total_energy));
    total_energy -= allocated_energy;
    energy_distribution.insert(System::ShieldGenerator, allocated_energy);

    let allowed_energy = my_state.weapons_system.get_allowed_energy();
    println!(
        "how much should go on Weapons System of remaining {total_energy} allowed {allowed_energy}",
    );
    let allocated_energy =
        choose_energy_amount(&my_state.weapons_system, allowed_energy.min(total_energy));
    total_energy -= allocated_energy;
    energy_distribution.insert(System::Weapons, allocated_energy);

    println!("leaving {total_energy} energy on Fusion Reactor");
    energy_distribution.insert(System::FusionReactor, total_energy);
    energy_distribution
}

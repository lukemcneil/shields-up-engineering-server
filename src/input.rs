use std::io;

use crate::{Action, PlayerState, System};

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

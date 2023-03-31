use rand::Rng;
use std::{io, time, thread::sleep};
use std::process::exit;

use clearscreen::{clear, is_windows_10};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

struct PlayCard {
    value: u8,
    c_type: PCardType,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
enum PCardType {
    Ace, Jack, Queen, King, Normal,
}

fn main() {
    let mut card_deck: Vec<PlayCard> = Vec::new();
    let mut player_deck: Vec<PlayCard> = Vec::new();
    let mut dealer_deck: Vec<PlayCard> = Vec::new();

    init_cards(&mut card_deck);
    run_loop(&mut card_deck, &mut player_deck, &mut dealer_deck);
}

fn run_loop(card_deck: &mut Vec<PlayCard>, player_deck: &mut Vec<PlayCard>, dealer_deck: &mut Vec<PlayCard>) {
    let mut running = true;

    'game_loop: while running {
        add_card(player_deck, card_deck); // player
        add_card(player_deck, card_deck); // player
        add_card(dealer_deck, card_deck); // dealer

        let mut game_result = result(player_deck, dealer_deck);

        if game_result.0 {
            clear().expect("Failed to clear screen!");
            send_state("Blackjack! You won.", player_deck, dealer_deck);
            restart(player_deck, dealer_deck);
            continue;
        }

        'playing: loop {
            let mut input = String::new();

            send_state("Do you want to hit or stay", player_deck, dealer_deck);
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().to_lowercase().as_str() {
                "hit" => {
                    add_card(player_deck, card_deck);
                    if get_total_value(player_deck) > 21 {
                        clear().expect("Failed to clear screen!");
                        send_state("Bust! You lost.", player_deck, card_deck);
                        restart(player_deck, dealer_deck);
                        break 'game_loop;
                    }
                    clear().expect("Failed to clear screen!");
                }
                "stay" => { break 'playing; }
                _ => {
                    clear().expect("Failed to clear screen!");
                    println!("Invalid input! Use 'hit' or 'stay'");
                    println!();
                    continue 'playing;
                }
            }
        }

        clear().expect("Failed to clear screen!");
        println!("Dealer: {}", get_total_value(dealer_deck));
        sleep(time::Duration::from_millis(1000));
        while get_total_value(dealer_deck) < 16 {
            add_card(dealer_deck, card_deck);

            clear().expect("Failed to clear screen!");
            println!("Dealer: {}", get_total_value(dealer_deck));
            sleep(time::Duration::from_millis(
                if (get_total_value(dealer_deck)) < 16 { 500 } else { 1000 },
            ));
        }
        clear().expect("Failed to clear screen!");

        game_result = result(player_deck, dealer_deck);
        if game_result.0 {
            send_state(if get_total_value(dealer_deck) > 21 { "Bust! You won." } else { "You won." }, player_deck, dealer_deck);
        } else {
            send_state("You lost!", player_deck, dealer_deck);
        }
        restart(player_deck, dealer_deck);
    }
}

/// Returns if deck has won or lost
///
/// # Arguments
///
/// * bool.0 - Has the deck won
/// * bool.1 - Has the deck won, with a black jack
/// * bool.2 - Did the deck bust
fn result(player_deck: &mut Vec<PlayCard>, dealer_deck: &mut Vec<PlayCard>) -> (bool, bool, bool) {
    let dealer_value = get_total_value(dealer_deck);
    let player_value = get_total_value(player_deck);

    /* Did player bust? */
    if player_value > 21 {
        return (false, false, true);
    }
    /* Did player hit blackjack? */
    if player_value == 21 && player_deck.len() == 2 {
        return (true, true, false);
    }
    /* Did dealer hit blackjack? */
    if dealer_value == 21 && dealer_deck.len() == 2 {
        return (false, true, false);
    }

    (dealer_value >= 16 && player_value <= 21 && (player_value > dealer_value || dealer_value > 21), false, player_value > 21)
}

fn send_state(message: &str, player_deck: &mut Vec<PlayCard>, dealer_deck: &mut Vec<PlayCard>) {
    println!("{message}");
    println!(" - dealer: {}", get_total_value(dealer_deck));
    println!(" - you: {}", get_total_value(player_deck));
}

fn get_total_value(deck: &mut Vec<PlayCard>) -> u8 {
    let mut value: u8 = 0;
    let mut aces: u8 = 0;

    for card in deck.iter() {
        if card.c_type == PCardType::Ace {
            aces += 1;
            continue;
        }
        value += card.value;
    }

    for _ in 0..aces {
        value += if value + 11 > 21 { 1 } else { 11 };
    }

    value
}

fn restart(player_deck: &mut Vec<PlayCard>, dealer_deck: &mut Vec<PlayCard>) {
    let mut input = String::new();

    println!();
    println!("Do you want to play again?");
    io::stdin().read_line(&mut input).unwrap();
    if !input.to_lowercase().starts_with("y") {
        println!("Thank you for playing!");
        exit(0);
    }

    clear().expect("Failed to clear screen!");
    player_deck.clear();
    dealer_deck.clear();
}

fn add_card(target_deck: &mut Vec<PlayCard>, card_deck: &mut Vec<PlayCard>) {
    let card = card_deck.get(rand::thread_rng().gen_range(0..12)).unwrap();
    target_deck.push(PlayCard {
        value: card.value,
        c_type: card.c_type,
    });
}

fn init_cards(card_deck: &mut Vec<PlayCard>) {
    for i in 2..11 {
        card_deck.push(PlayCard {
            value: i,
            c_type: PCardType::Normal,
        });
    }
    for special in PCardType::iter() {
        if special != PCardType::Normal {
            card_deck.push(PlayCard {
                value: 10,
                c_type: special,
            });
        }
    }
}
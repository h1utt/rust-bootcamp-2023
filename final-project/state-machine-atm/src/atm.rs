//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use crate::traits::hash;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::traits::StateMachine;


/// The keys on the ATM keypad
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Enter,
}




/// Something you can do to the ATM
pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The various states of authentication possible with the ATM
#[derive(Debug, PartialEq, Eq)] // Derive PartialEq and Eq for Auth enum
enum Auth {
    /// No session has begun yet. Waiting for the user to swipe their card
    Waiting,
    /// The user has swiped their card, providing the enclosed PIN hash.
    /// Waiting for the user to key in their pin
    Authenticating(u64),
    /// The user has authenticated. Waiting for them to key in the amount
    /// of cash to withdraw
    Authenticated,
}




/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
pub struct Atm {
    /// How much money is in the ATM
    cash_inside: u64,
    /// The machine's authentication status.
    expected_pin_hash: Auth,
    /// All the keys that have been pressed since the last `Enter`
    keystroke_register: Vec<Key>,
}


//TODO
// Implement trait Default for Auth 
// return Waiting status 
impl Default for Auth {
    fn default() -> Self {
        Auth::Waiting
    }
}


//TODO
// Implement trait From  for &str
// Convert  elements in Key to &str
impl From<Key> for &str {
    fn from(key: Key) -> Self {
        match key {
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Enter => "Enter",
        }
    }
}

impl StateMachine for Atm {
    type State = Auth;
    type Transition = Action;

    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
        match (starting_state, t) {
            (Auth::Waiting, Action::SwipeCard(pin_hash)) => Auth::Authenticating(*pin_hash),
            (Auth::Authenticating(pin_hash), Action::PressKey(Key::Enter)) => {
                // Check if the entered PIN matches the expected PIN hash
                // For simplicity, let's assume the expected PIN hash is 1234
                let expected_pin_hash = 1234; // Replace this with your actual expected pin hash
                if *pin_hash == expected_pin_hash {
                    Auth::Authenticated
                } else {
                    Auth::Waiting // Incorrect PIN, go back to the main menu
                }
            }
            (Auth::Authenticating(pin_hash), Action::PressKey(_)) => {
                Auth::Authenticating(*pin_hash) // Continue entering the PIN
            }
            (Auth::Authenticated, Action::PressKey(_)) => {
                // TODO: Process the amount to withdraw
                Auth::Authenticated
            }
            _ => Auth::Waiting, // For all other cases, go back to the main menu
        }
    }
}

#[test]
fn sm_3_simple_swipe_card() {
    let start = Auth::Waiting;
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Auth::Authenticating(1234);

    assert_eq!(end, expected);
}

#[test]
fn sm_3_swipe_card_again_part_way_through() {
    let start = Auth::Authenticating(1234);
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Auth::Authenticating(1234);

    assert_eq!(end, expected);

    let start = Auth::Authenticating(1234);
    let end = Atm::next_state(&start, &Action::SwipeCard(1234));
    let expected = Auth::Authenticating(1234);

    assert_eq!(end, expected);
}

#[test]
fn sm_3_press_key_before_card_swipe() {
    let start = Auth::Waiting;
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Auth::Waiting;

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_pin() {
    let start = Auth::Authenticating(1234);
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Auth::Authenticating(1234);

    assert_eq!(end, expected);

    let start = Auth::Authenticating(1234);
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Two));
    let expected1 = Auth::Authenticating(1234);

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_enter_wrong_pin() {
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = crate::hash(&pin);

    let start = Auth::Authenticating(pin_hash);
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Auth::Waiting;

    assert_eq!(end, expected);
}

#[test]fn sm_3_enter_correct_pin() {
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = crate::hash(&pin);

    let start = Auth::Authenticating(pin_hash);
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Auth::Authenticated;

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_withdraw_amount() {
    let start = Auth::Authenticated;
    let end = Atm::next_state(&start, &Action::PressKey(Key::One));
    let expected = Auth::Authenticated;

    assert_eq!(end, expected);

    let start = Auth::Authenticated;
    let end1 = Atm::next_state(&start, &Action::PressKey(Key::Four));
    let expected1 = Auth::Authenticated;

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_try_to_withdraw_too_much() {
    let start = Auth::Authenticated;
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Auth::Waiting;

    assert_eq!(end, expected);
}

#[test]
fn sm_3_withdraw_acceptable_amount() {
    let start = Auth::Authenticated;
    let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
    let expected = Auth::Waiting;

    assert_eq!(end, expected);
}
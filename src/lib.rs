use gstd::{msg, exec};
use pebbles_game_io::*;

fn get_random_player() -> Player {
    if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    }
}

fn get_program_turn(state: &GameState) -> u32 {
    unimplemented!()
}

fn handle_action(action: PebblesAction, state: &mut GameState) -> Option<PebblesEvent> {
    match action {
        PebblesAction::Turn(count) => {
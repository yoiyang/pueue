use anyhow::Result;

use pueue::network::message::Message;
use pueue::network::protocol::*;
use pueue::state::State;

pub mod edit;
pub mod local_follow;
pub mod restart;
pub mod wait;

// This is a helper function for easy retrieval of the current daemon state.
// The current daemon state is often needed in more complex commands.
pub async fn get_state(stream: &mut GenericStream) -> Result<State> {
    // Create the message payload and send it to the daemon.
    send_message(Message::Status, stream).await?;

    // Check if we can receive the response from the daemon
    let message = receive_message(stream).await?;

    match message {
        Message::StatusResponse(state) => Ok(*state),
        _ => unreachable!(),
    }
}

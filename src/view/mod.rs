mod bbcode;
mod room;

use self::bbcode::BBCode;
use crate::state::State;
use anyhow::Result;

pub fn render(state: &State) -> Result<String> {
    let mut bb = BBCode::new();

    for room in state.src.rooms.values() {
        room::render(&mut bb, state, room)?;
    }

    Ok(bb.to_string())
}

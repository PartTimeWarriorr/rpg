#![allow(clippy::unnecessary_wraps)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use ggez::{
    event,
    GameResult,
};

use rpg::state::*;

use std::{
    path,
    env,
};

pub fn main() -> GameResult {

    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}

// this is supposed to be a game where you
// have to connect the dots (metaphorically)
// and solve riddles on an old computer terminal
// in an abandonded antique labratory

mod commands;
mod cli;
mod texts;
mod style;
mod typewriter;
mod filesystem;
mod game_progress;

use style::*;

fn main() {
    set_style(&Style::NORMAL);
    typewriter::typewriter(texts::INTRO, 80, [50, 250, 500], true);
    cli::run();
    set_style(&Style::NORMAL);
    typewriter::typewriter(texts::OUTRO, 80, [50, 250, 500], true);
}

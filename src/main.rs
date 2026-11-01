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


fn main() {
    typewriter::typewriter(texts::INTRO, 80, [50, 250, 500], true);
    cli::run();
}

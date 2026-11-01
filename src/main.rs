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

fn main() {
//    typewriter::typewriter(&texts::INTRO.green().italic(), 100);
    cli::run();
}

// this file contains the text styling
// i want a function to make text formatting easy

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Red, Green, Yellow, Blue, White, Default
}

#[derive(Clone, Copy, Debug)]
pub enum Weight {
    Normal, Bold, Dim
}

#[derive(Clone, Copy, Debug)]
pub enum Decoration {
    Normal, Italic//, Underline
}

#[derive(Clone, Copy, Debug)]
pub struct Style {
    pub color: Color,
    pub weight: Weight,
    pub decoration: Decoration
}

impl Style {
    pub const PROMPT: Self = Self {
        color: Color::Green,
        weight: Weight::Bold,
        decoration: Decoration::Normal
    };
    pub const HEADING: Self = Self {
        color: Color::Yellow,
        weight: Weight::Bold,
        decoration: Decoration::Normal
    };
    pub const CMD_NAME: Self = Self {
        color: Color::Green,
        weight: Weight::Bold,
        decoration: Decoration::Italic
    };
    pub const ERROR: Self = Self {
        color: Color::Red,
        weight: Weight::Normal,
        decoration: Decoration::Normal
    };
    pub const NORMAL: Self = Self {
        color: Color::Blue,
        weight: Weight::Normal,
        decoration: Decoration::Normal
    };
    pub const DIRECTORY: Self = Self {
        color: Color::Blue,
        weight: Weight::Bold,
        decoration: Decoration::Normal
    };
    pub const FILE: Self = Self {
        color: Color::Yellow,
        weight: Weight::Normal,
        decoration: Decoration::Italic
    };
    pub const MESSAGE: Self = Self {
        color: Color::Default,
        weight: Weight::Dim,
        decoration: Decoration::Normal
    };
    pub const CREDENTIALS: Self = Self {
        color: Color::White,
        weight: Weight::Bold,
        decoration: Decoration::Italic
    };
}

pub fn set_style(style: &Style) {
    let color_code = match style.color {
        Color::Red      => 31,
        Color::Green    => 32,
        Color::Yellow   => 33,
        Color::Blue     => 34,
        Color::White    => 37,
        Color::Default  => 39
    };

    let weight_code = match style.weight {
        Weight::Normal  => 22,
        Weight::Bold    => 1,
        Weight::Dim     => 2
    };

    let decoration_code = match style.decoration {
        Decoration::Normal      => 23,
//        Decoration::Underline   => 4,
        Decoration::Italic      => 3
    };
    
    clear_style();
    print!("\x1b[{};{};{}m", color_code, weight_code, decoration_code);
}

pub fn clear_style() {
    print!("\x1b[0m");
}

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

#[allow(dead_code)]
pub fn test() {
    // should iterate through all combinations to check them out once
    const COLORS: [Color; 6] = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::White,
        Color::Default
    ];
    const WEIGHTS: [Weight; 3] = [
        Weight::Normal,
        Weight::Bold,
        Weight::Dim
    ];
    const DECS: [Decoration; 2] = [
        Decoration::Normal,
        Decoration::Italic,
    ];

    for c in &COLORS {
        for w in &WEIGHTS {
            for d in &DECS {
                set_style(&Style {color: *c, weight: *w, decoration: *d});
                println!("This text is {:?}, {:?} and {:?}.",
                    c, w, d);
            }
        }
    }
}

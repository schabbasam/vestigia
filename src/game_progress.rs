
pub struct GameProgress {
    pub credentials_unlocked: bool,
    pub research_unlocked: bool
}

impl GameProgress {
    pub fn new() -> Self {
        Self {
            credentials_unlocked: false,
            research_unlocked: false
        }
    }
}

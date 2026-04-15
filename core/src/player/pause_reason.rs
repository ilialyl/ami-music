#[derive(Debug, Default, PartialEq)]
pub enum PauseReason {
    #[default]
    None,
    User,
    Exhaustion,
}

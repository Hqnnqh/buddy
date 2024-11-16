use std::ops::Not;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum State {
    Idle,

    // special state for initiating run to ensure proper timing
    InitiatingRun,
    Running,

    InitiatingClick,
    Click,
}

impl Not for State {
    type Output = State;

    fn not(self) -> Self::Output {
        match self {
            State::Running | State::InitiatingRun | State::InitiatingClick | State::Click => {
                State::Idle
            }
            State::Idle => State::InitiatingRun,
        }
    }
}

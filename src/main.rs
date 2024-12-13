use std::{error::Error, process::Termination};

/// "because of the dead hand of the mathematics of human behavior that can
/// neither be stopped, swerved, nor delayed."
async fn trantorian(member: Foundation) -> Result<State, Box<dyn Error>> {
    let mut state = State::Init(member);

    state = loop {
        state = tokio::select! {
            res = handle_state_upgrade(&state) => res?,
            res = handle_forced_cancellation(&state) => break res?,
        }
    };

    Ok(state)
}

#[derive(PartialEq, Eq, Debug)]
pub enum Foundation {
    HarrySeldon,
    SalvadorHardin,
    Magnifico,
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug)]
struct TaskDesc(String);

#[derive(PartialEq, Eq, Debug)]
#[allow(dead_code)]
enum State {
    Init(Foundation),
    Task(TaskDesc),
    Pause(Option<TaskDesc>),
    Done,
}

impl Termination for State {
    fn report(self) -> std::process::ExitCode {
        match self {
            State::Done => std::process::ExitCode::SUCCESS,
            _ => std::process::ExitCode::FAILURE,
        }
    }
}

async fn handle_state_upgrade(_state: &State) -> Result<State, Box<dyn Error>> {
    unimplemented!()
}

async fn handle_forced_cancellation(_state: &State) -> Result<State, Box<dyn Error>> {
    unimplemented!()
}

#[tokio::main]
async fn main() -> Result<State, Box<dyn Error>> {
    trantorian(Foundation::Magnifico).await
}

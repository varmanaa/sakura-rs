pub mod latency;

use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

pub fn get_commands() -> Vec<Command> {
    vec![latency::LatencyCommand::create_command().into()]
}

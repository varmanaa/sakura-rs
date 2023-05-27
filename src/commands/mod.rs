pub mod check_message;
pub mod config;
pub mod info;
pub mod latency;

use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        check_message::CheckMessageCommand::create_command(),
        config::ConfigCommand::create_command().into(),
        info::InfoCommand::create_command().into(),
        latency::LatencyCommand::create_command().into(),
    ]
}

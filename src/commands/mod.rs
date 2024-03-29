pub mod check;
pub mod check_message;
pub mod config;
pub mod counts;
pub mod info;
pub mod latency;
pub mod stats;

use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        check::CheckCommand::create_command().into(),
        check_message::CheckMessageCommand::create_command(),
        config::ConfigCommand::create_command().into(),
        counts::CountsCommand::create_command().into(),
        info::InfoCommand::create_command().into(),
        latency::LatencyCommand::create_command().into(),
        stats::StatsCommand::create_command().into(),
    ]
}

use std::collections::HashMap;

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::{Context},
    framework::standard::{
        CommandResult, Args,
    },
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            Message,
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
            command::CommandOptionType,
        },
        user::User,
    },
};

use crate::{
    database_handler, DBContainer, loa_contents::LOA_CONTENTS, user_info::*,
    command_handler::{
        command_handler::*,
        command_data::*,
        command_return::{CommandReturn, ControlInteraction},
    }
};

struct UserReset;

pub fn command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(UserReset)
}

#[async_trait]
impl CommandInterface for UserReset {
    async fn run(
        &self, 
        ctx: &Context, 
        command: &ApplicationCommandInteraction, 
        options: &[CommandDataOption]
    ) -> CommandReturn {
        database_handler::user_delete(ctx.data.read().await.get::<DBContainer>().unwrap(), command.user.tag()).await.unwrap();
        CommandReturn::String(format!("{}의 보유 캐릭터 목록 삭제됨", command.user.tag()))
    }

    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand
    ) -> &'b mut CreateApplicationCommand {
        command
            .name("초기화")
            .description("등록된 유저 캐릭터 목록 삭제")
    }
}

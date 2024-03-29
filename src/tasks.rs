use std::{sync::Arc, time::Duration};

use time::OffsetDateTime;
use tokio::time::sleep;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    types::{cache::GuildUpdate, context::Context, Result},
    utility::message::get_invite_codes,
};

#[cold]
pub async fn handle_tasks(context: Arc<Context>) -> Result<()> {
    // Wait while the jobs run
    sleep(Duration::from_secs(60)).await;

    let scheduler: JobScheduler = JobScheduler::new().await?;
    let unchecked_invites_task_context = context.clone();
    let recycle_invites_task_context = context.clone();

    scheduler
        .add(Job::new_async("*/20 1-59 * * * *", move |_uuid, _lock| {
            let unchecked_invites_task_context = unchecked_invites_task_context.clone();

            Box::pin(async move {
                handle_unchecked_invites_task(unchecked_invites_task_context)
                    .await
                    .unwrap();
            })
        })?)
        .await?;

    scheduler
        .add(Job::new_async("0 0 * * * *", move |_uuid, _lock| {
            let recycle_invites_task_context = recycle_invites_task_context.clone();

            Box::pin(async move {
                handle_recycle_invites_task(recycle_invites_task_context)
                    .await
                    .unwrap();
            })
        })?)
        .await?;

    // Start the scheduler
    scheduler.start().await?;

    Ok(())
}

async fn handle_unchecked_invites_task(context: Arc<Context>) -> Result<()> {
    if let Ok(unchecked_invite_codes) = context.database.get_unchecked_invites().await {
        for unchecked_invite_code in unchecked_invite_codes {
            let (is_permalink, is_valid, expires_at) = if let Ok(response) = context
                .http
                .invite(&unchecked_invite_code)
                .with_expiration()
                .await
            {
                let invite = response.model().await?;
                let mut is_permalink = invite.expires_at.is_none()
                    && invite.max_age.is_none()
                    && invite.max_uses.is_none();

                if let Some(invite_guild) = invite.guild {
                    if let Some(vanity_url_code) = invite_guild.vanity_url_code {
                        is_permalink = is_permalink && vanity_url_code.eq(&unchecked_invite_code);
                    }
                }

                let expires_at = invite.expires_at.map(|timestamp| {
                    OffsetDateTime::from_unix_timestamp(timestamp.as_secs()).unwrap()
                });

                (is_permalink, true, expires_at)
            } else {
                (false, false, None)
            };

            context
                .database
                .insert_checked_invite(
                    &unchecked_invite_code,
                    is_permalink,
                    is_valid,
                    expires_at,
                    OffsetDateTime::now_utc(),
                )
                .await?;
        }
    }

    Ok(())
}

async fn handle_recycle_invites_task(context: Arc<Context>) -> Result<()> {
    context.database.remove_old_invites().await?;

    let removed_ids = context.database.remove_old_messages().await?;

    for (guild_id, channel_ids) in removed_ids.into_iter() {
        let Some(cached_guild) = context.cache.get_guild(guild_id) else {
            continue;
        };
        let cached_guild_invite_check_category_ids =
            cached_guild.invite_check_category_ids.read().clone();

        context.cache.update_guild(
            guild_id,
            GuildUpdate {
                in_check: Some(true),
                ..Default::default()
            },
        );

        for channel_id in channel_ids {
            sleep(Duration::from_millis(500)).await;

            let Some(channel) = context.cache.get_channel(channel_id) else {
                continue;
            };
            let Some(parent_id) = channel.parent_id else {
                continue;
            };

            if !cached_guild_invite_check_category_ids.contains(&parent_id) {
                continue;
            }

            let messages = context
                .http
                .channel_messages(channel_id)
                .limit(10)?
                .await?
                .model()
                .await?;

            for message in messages {
                let invite_codes = get_invite_codes(message.content, message.embeds);

                for invite_code in invite_codes.iter() {
                    context
                        .database
                        .insert_unchecked_invite(invite_code)
                        .await?;
                }

                context
                    .database
                    .insert_message(guild_id, channel_id, message.id, parent_id, invite_codes)
                    .await?;
            }
        }

        context.cache.update_guild(
            guild_id,
            GuildUpdate {
                in_check: Some(false),
                ..Default::default()
            },
        );
    }

    Ok(())
}

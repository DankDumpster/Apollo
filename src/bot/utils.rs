use log::{error, warn};
use regex::Regex;
use serenity::model::channel::{Channel, Message};
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::Result as SerenityResult;
use time::Duration;

pub(crate) async fn reply<T: std::fmt::Display>(ctx: &Context, msg: &Message, content: T) {
    if let Err(why) = msg.channel_id.say(&ctx, &content).await {
        warn!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why,
        );
    }
}

pub(crate) fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        error!("Error sending message: {:?}", why);
        eprintln!("{}", why)
    }
}

pub(crate) fn convert_time_into_str(diff: Duration) -> String {
    let mins = (diff.num_minutes() - 60 * diff.num_hours()).to_string();
    let min = if mins.len() == 1 {
        format!("0{}", mins)
    } else {
        mins
    };
    let hour = if diff.num_hours().to_string().len() == 1 {
        format!("0{}", diff.num_hours())
    } else {
        diff.num_hours().to_string()
    };
    format!("{}:{}", hour, min)
}

pub(crate) async fn get_channel_forced(ctx: &Context, channel_id: u64) -> Option<Channel> {
    return match ctx.cache.channel(channel_id).await {
        Some(channel) => Some(channel),
        None => {
            if let Ok(channel) = ctx.http.get_channel(channel_id).await {
                Some(channel)
            } else {
                return None;
            }
        }
    };
}

pub(crate) async fn get_user_forced(ctx: &Context, user_id: u64) -> Option<User> {
    return match ctx.cache.user(user_id).await {
        Some(user) => Some(user),
        None => match ctx.http.get_user(user_id).await {
            Ok(user) => Some(user),
            Err(_) => None,
        },
    };
}

/*
pub(crate) async fn reply_embed<T>(ctx: &Context, msg: &Message, embed: T) {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, &embed).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    }
}
*/

pub(crate) async fn parse_channel(ctx: &Context, channel_name: String) -> Option<Channel> {
    let channel: Channel;
    if let Ok(id) = channel_name.parse::<u64>() {
        let channel = match ctx.http.get_channel(id).await {
            Ok(c) => c,
            Err(_e) => return None,
        };
        Some(channel)
    } else if channel_name.starts_with("<#") && channel_name.ends_with('>') {
        let re = Regex::new("[<#>]").unwrap();
        let channel_id = re.replace_all(&channel_name, "").into_owned();

        channel = match ctx
            .http
            .get_channel(channel_id.parse::<u64>().unwrap())
            .await
        {
            Ok(m) => m,
            Err(_e) => return None,
        };

        Some(channel.to_owned())
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

#[allow(dead_code)]
pub fn truncate_string(s: &mut String, max_chars: usize) {
    let bytes = truncate(&s, max_chars).len();
    s.truncate(bytes);
    s.push_str("...")
}

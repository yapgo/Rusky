use crate::constants::colors::DISCORD_BLUE;
use crate::constants::emotes::*;
use crate::util::calculator_command as calc_util;
use crate::util::{
    discord_time::get_relative_time_string,
    discord_user::{format_client_status, get_client_status},
    image::random_default_avatar,
};
use futures::lock::Mutex;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::futures::StreamExt;
use serenity::model::interactions::ButtonStyle;
use serenity::model::interactions::InteractionData;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::sync::Arc;
use std::time::Duration;
#[command]
#[description("informações sobre um usuário")]
#[aliases("whois")]
#[only_in(guilds)]
#[example("@yxqsnz")]
pub async fn userinfo(context: &Context, message: &Message, _args: Args) -> CommandResult {
    if let Some(user) = message.mentions.first() {
        let statuses = format_client_status(
            &get_client_status(&message.guild(context).await.unwrap(), &user.id).await,
        );
        message
            .channel_id
            .send_message(context, |builder| {
                builder.reference_message(message).embed(|builder| {
                    builder
                        .title(&format!("Informações de {}", user.tag()))
                        .thumbnail(user.avatar_url().unwrap_or_else(random_default_avatar))
                        .description(format!(
                            "{} **·** Tag: `{}`\n{} **·** Conta criada: {}\n{} **·** Dispositivo: `{}`",
                            DETECTIVE_EMOTE,
                            user.tag(),
                            DATE_EMOTE,
                            get_relative_time_string(user.created_at().timestamp()),
                            COMPUTER_EMOTE,
                            statuses,

                        ))
                        .color(DISCORD_BLUE)
                })
            })
            .await?;
    } else {
        message
            .reply(
                context,
                &format!("{} **·** Por favor mencione um usuário!", ERROR_EMOTE),
            )
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("calculadora", "calculator")]
#[description("Uma calculadora")]
pub async fn calc(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let _msg = message
        .channel_id
        .send_message(context, |b| {
            b.components(|components| {
                components
                    .create_action_row(|row| {
                        row.create_button(|button| {
                            button
                                .label("(")
                                .style(ButtonStyle::Primary)
                                .custom_id("calculator_bkl")
                        })
                        .create_button(|button| {
                            button
                                .label(")")
                                .style(ButtonStyle::Primary)
                                .custom_id("calculator_bkr")
                        })
                        .create_button(|button| {
                            button
                                .label("^")
                                .style(ButtonStyle::Primary)
                                .custom_id("calculator_wed")
                        })
                        .create_button(|button| {
                            button
                                .label("D")
                                .style(ButtonStyle::Danger)
                                .custom_id("calculator_del")
                        })
                        .create_button(|button| {
                            button
                                .label("C")
                                .style(ButtonStyle::Danger)
                                .custom_id("calculator_cls")
                        })
                    })
                    .create_action_row(|row| {
                        row.create_button(|button| {
                            button
                                .label("7")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_7")
                        })
                        .create_button(|button| {
                            button
                                .label("8")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_8")
                        })
                        .create_button(|button| {
                            button
                                .label("9")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_9")
                        })
                        .create_button(|button| {
                            button
                                .label("-")
                                .style(ButtonStyle::Primary)
                                .custom_id("calculator_min")
                        })
                        .create_button(|button| {
                            button
                                .label(" ")
                                .style(ButtonStyle::Secondary)
                                .custom_id("nothing-1")
                                .disabled(true)
                        })
                    })
                    .create_action_row(|row| {
                        row.create_button(|button| {
                            button
                                .label("4")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_4")
                        })
                        .create_button(|button| {
                            button
                                .label("5")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_5")
                        })
                        .create_button(|button| {
                            button
                                .label("6")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_6")
                        })
                        .create_button(|button| {
                            button
                                .label("*")
                                .style(ButtonStyle::Primary)
                                .custom_id("calculator_ply")
                        })
                        .create_button(|button| {
                            button
                                .label(" ")
                                .style(ButtonStyle::Secondary)
                                .custom_id("nothing0")
                                .disabled(true)
                        })
                    })
                    .create_action_row(|row| {
                        row.create_button(|button| {
                            button
                                .label("1")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_1")
                        })
                        .create_button(|button| {
                            button
                                .label("2")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_2")
                        })
                        .create_button(|button| {
                            button
                                .label("3")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_3")
                        })
                        .create_button(|button| {
                            button
                                .label("/")
                                .style(ButtonStyle::Primary)
                                .custom_id("calculator_div")
                        })
                        .create_button(|button| {
                            button
                                .label(" ")
                                .style(ButtonStyle::Secondary)
                                .custom_id("nothing1")
                                .disabled(true)
                        })
                    })
                    .create_action_row(|row| {
                        row.create_button(|button| {
                            button
                                .label(".")
                                .style(ButtonStyle::Primary)
                                .custom_id("calculator_pon")
                        })
                        .create_button(|button| {
                            button
                                .label("0")
                                .style(ButtonStyle::Secondary)
                                .custom_id("calculator_0")
                        })
                        .create_button(|button| {
                            button
                                .label("=")
                                .style(ButtonStyle::Success)
                                .custom_id("calculator_res")
                        })
                        .create_button(|button| {
                            button
                                .label("+")
                                .style(ButtonStyle::Primary)
                                .custom_id("calculator_pls")
                        })
                        .create_button(|button| {
                            button
                                .label(" ")
                                .style(ButtonStyle::Secondary)
                                .custom_id("nothing2")
                                .disabled(true)
                        })
                    })
            })
            .content("** **")
            .reference_message(message)
        })
        .await?;
    let collector = _msg
        .await_component_interactions(context)
        .timeout(Duration::from_secs(60 * 2))
        .await;
    let context = &context;

    let tks = Arc::new(Mutex::new(vec![]));
    let _: Vec<_> = collector
        .then(|it| async {
            if let Some(data) = &it.data {
                if let InteractionData::MessageComponent(button) = data {
                    let splited = button.custom_id.split("_").collect::<Vec<_>>();
                    if !splited.is_empty() {
                        let action = &splited[1];

                        let tk = calc_util::parse_str(&action.to_string());
                        match tk {
                            calc_util::Token::Result => {
                                let tks = tks.lock().await;
                                let mut ns = fasteval::EmptyNamespace;
                                let eval = fasteval::ez_eval(&calc_util::parse_tks(&tks), &mut ns);
                                let _ = it
                                    .create_interaction_response(context, |it| {
                                        it.kind(InteractionResponseType::UpdateMessage)
                                            .interaction_response_data(|data| {
                                                data.create_embed(|embed| {
                                                    embed
                                                        .field(
                                                            "Entrada",
                                                            format!(
                                                                "`{}`",
                                                                calc_util::parse_tks(&tks)
                                                            ),
                                                            true,
                                                        )
                                                        .field(
                                                            "Saida",
                                                            format!("`{}`", {
                                                                if let Ok(ev) = eval {
                                                                    ev.to_string()
                                                                } else {
                                                                    "Expressão inválida".to_string()
                                                                }
                                                            }),
                                                            true,
                                                        )
                                                })
                                            })
                                    })
                                    .await;
                            }
                            _ => {
                                //let _ = message
                                //    .reply(
                                //       context,
                                //        format!(
                                //            "custom_id: `{:?}`\nToken: `{:?}`",
                                //            button.custom_id, tk
                                //        ),
                                //    )
                                //    .await;

                                {
                                    let mut tks = tks.lock().await;
                                    tks.push(tk);

                                    let _ = it
                                        .create_interaction_response(context, |it| {
                                            it.kind(InteractionResponseType::UpdateMessage)
                                                .interaction_response_data(|data| {
                                                    data.create_embed(|embed| {
                                                        embed
                                                            .field(
                                                                "Entrada",
                                                                format!(
                                                                    "`{}`",
                                                                    calc_util::parse_tks(&tks)
                                                                ),
                                                                true,
                                                            )
                                                            .field("Saida", "`???`", true)
                                                    })
                                                })
                                        })
                                        .await;
                                }
                            }
                        }
                    }
                }
            }
            it
        })
        .collect()
        .await;

    Ok(())
}

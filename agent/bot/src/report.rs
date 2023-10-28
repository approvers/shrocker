use bitflags::bitflags;
use serenity::{model::prelude::Message, prelude::Context};

use anyhow::{Result, Context as _};

bitflags! {
    pub struct ReportPreference: u8 {
        const UPDATE_REPLY         = 0b00000001;
        const EXPLICIT_NEW_MESSAGE = 0b00000010;
        const NOTIFY_HARDER        = 0b00000100;
    }
}

pub trait Reporter {
    async fn report(&mut self, msg: &str, pref: impl Into<Option<ReportPreference>>) -> Result<()>;

    fn empathize(&self, msg: &str) -> String {
        msg.to_string()
    }

    async fn error(&mut self, msg: &str, pref: impl Into<Option<ReportPreference>>) -> Result<()> {
        let (title, desc) = msg.split_once('\n').unwrap_or((msg, ""));
        let title = self.empathize(title);

        self.report(&format!("🛑 {title}\n{desc}"), pref).await
    }

    async fn processing(&mut self, msg: &str, pref: impl Into<Option<ReportPreference>>) -> Result<()> {
        let (title, desc) = msg.split_once('\n').unwrap_or((msg, ""));
        let title = self.empathize(title);

        self.report(&format!("⏳ {title}\n{desc}"), pref).await
    }

    async fn success(&mut self, msg: &str, pref: impl Into<Option<ReportPreference>>) -> Result<()> {
        let (title, desc) = msg.split_once('\n').unwrap_or((msg, ""));
        let title = self.empathize(title);

        self.report(&format!("✅ {title}\n{desc}"), pref).await
    }
}

pub struct SerenityReporter<'ctx> {
    initial_reply: Option<Message>,
    ctx: &'ctx Context,
    msg: &'ctx Message,
}

impl<'ctx> SerenityReporter<'ctx> {
    pub fn new(
        ctx: &'ctx Context,
        msg: &'ctx Message,
    ) -> Self {
        Self { initial_reply: None, ctx, msg }
    }
}

impl<'ctx> Reporter for SerenityReporter<'ctx> {
    async fn report(&mut self, msg: &str, pref: impl Into<Option<ReportPreference>>) -> Result<()> {
        let pref = pref.into().unwrap_or(ReportPreference::UPDATE_REPLY);

        if pref.intersects(ReportPreference::UPDATE_REPLY) {
            return self.report_by_updating_reply(msg).await
        }

        if pref.intersects(ReportPreference::EXPLICIT_NEW_MESSAGE) {
            return self.report_by_creating_new_message(msg).await;
        }

        Ok(())
    }

    fn empathize(&self, msg: &str) -> String {
        format!("**{msg}**")
    }
}

impl<'ctx> SerenityReporter<'ctx> {
    async fn report_by_creating_new_message(&mut self, msg: &str) -> Result<()> {
        self.msg.channel_id
            .say(self.ctx, msg)
            .await
            .context("Could not create new message as the reply of the message during report")?;

        Ok(())
    }

    async fn report_by_updating_reply(&mut self, msg: &str) -> Result<()> {
        let Some(ref mut message) = self.initial_reply else {
            self.initial_reply = Some(
                self.msg
                    .reply(self.ctx, msg)
                    .await
                    .context("Could not reply to message during report")?
            );

            return Ok(())
        };

        message
            .edit(self.ctx, |edit| edit.content(msg))
            .await
            .context("Could not edit the already posted reply during report")?;

        Ok(())
    }
}


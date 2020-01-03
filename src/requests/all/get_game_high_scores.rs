use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, GameHighScore},
    Bot,
};

/// Use this method to get data for high score tables. Will return the score of
/// the specified user and several of his neighbors in a game. On success,
/// returns an Array of GameHighScore objects.This method will currently return
/// scores for the target user, plus two of his closest neighbors on each side.
/// Will also return the top three users if the user and his neighbors are not
/// among them. Please note that this behavior is subject to change.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetGameHighScores<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,

    /// Target user id
    user_id: i32,
}

#[async_trait::async_trait]
impl Request for GetGameHighScores<'_> {
    type Output = Vec<GameHighScore>;

    async fn send(&self) -> ResponseResult<Vec<GameHighScore>> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "getGameHighScores",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> GetGameHighScores<'a> {
    pub(crate) fn new(
        bot: &'a Bot,
        chat_or_inline_message: ChatOrInlineMessage,
        user_id: i32,
    ) -> Self {
        Self {
            bot,
            chat_or_inline_message,
            user_id,
        }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }
}

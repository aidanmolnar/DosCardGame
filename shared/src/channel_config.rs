use std::time::Duration;

use bevy_renet::renet::{RenetConnectionConfig, ReliableChannelConfig};

pub const LOBBY_CHANNEL_ID: u8 = 0;
pub const GAME_CHANNEL_ID: u8 = 1;

// Renet channel configuration used by client and server
// Two reliable channels: 
//  one for lobby update messages (ex. changes in player count) used all the time
//  one for game messages (ex. which card someone plays) used once game actually starts
pub fn connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        send_channels_config: 
            vec![
                ReliableChannelConfig {
                    channel_id: LOBBY_CHANNEL_ID,
                    message_resend_time: Duration::ZERO,
                    ..Default::default()
                }.into(),
                ReliableChannelConfig {
                    channel_id: GAME_CHANNEL_ID,
                    message_resend_time: Duration::ZERO,
                    ..Default::default()
                }.into()
            ],
        receive_channels_config: 
            vec![
                ReliableChannelConfig {
                    channel_id: LOBBY_CHANNEL_ID,
                    message_resend_time: Duration::ZERO,
                    ..Default::default()
                }.into(),
                ReliableChannelConfig {
                    channel_id: GAME_CHANNEL_ID,
                    message_resend_time: Duration::ZERO,
                    ..Default::default()
                }.into()
            ],
        ..Default::default()
    }
}



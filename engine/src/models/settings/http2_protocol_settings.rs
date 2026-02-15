use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Http2ProtocolSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_connection_window_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_window_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frame_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_header_list_size: Option<u32>,
}

impl Default for Http2ProtocolSettings {
    fn default() -> Self {
        Self {
            initial_connection_window_size: None,
            stream_window_size: None,
            max_frame_size: None,
            max_header_list_size: None,
        }
    }
}

impl Http2ProtocolSettings {
    pub fn global() -> Self {
        Self {
            initial_connection_window_size: Some(
                Constants::DEFAULT_HTTP_INITIAL_CONNECTION_WINDOW_SIZE,
            ),
            stream_window_size: Some(Constants::DEFAULT_HTTP2_STREAM_WINDOW_SIZE),
            max_frame_size: Some(Constants::DEFAULT_HTTP2_MAX_FRAME_SIZE),
            max_header_list_size: Some(Constants::DEFAULT_HTTP2_MAX_HEADER_LIST_SIZE),
        }
    }

    pub fn merge(&mut self, other: &Self) {
        if other.initial_connection_window_size.is_some() {
            self.initial_connection_window_size = other.initial_connection_window_size;
        }
        if other.stream_window_size.is_some() {
            self.stream_window_size = other.stream_window_size;
        }
        if other.max_frame_size.is_some() {
            self.max_frame_size = other.max_frame_size;
        }
        if other.max_header_list_size.is_some() {
            self.max_header_list_size = other.max_header_list_size;
        }
    }

    // Getters
    pub fn get_initial_connection_window_size(&self) -> u32 {
        self.initial_connection_window_size
            .unwrap_or(Constants::DEFAULT_HTTP_INITIAL_CONNECTION_WINDOW_SIZE)
    }

    pub fn get_stream_window_size(&self) -> u32 {
        self.stream_window_size
            .unwrap_or(Constants::DEFAULT_HTTP2_STREAM_WINDOW_SIZE)
    }

    pub fn get_max_frame_size(&self) -> u32 {
        self.max_frame_size
            .unwrap_or(Constants::DEFAULT_HTTP2_MAX_FRAME_SIZE)
    }

    pub fn get_max_header_list_size(&self) -> u32 {
        self.max_header_list_size
            .unwrap_or(Constants::DEFAULT_HTTP2_MAX_HEADER_LIST_SIZE)
    }
}

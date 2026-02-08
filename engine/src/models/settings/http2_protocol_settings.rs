use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Http2ProtocolSettings {
    #[serde(default = "default_http2_initial_connection_window_size")]
    pub initial_connection_window_size: u32,
    
    #[serde(default = "default_http2_stream_window_size")]
    pub stream_window_size: u32,
    
    #[serde(default = "default_http2_max_frame_size")]
    pub max_frame_size: u32,
    
    #[serde(default = "default_http2_max_header_list_size")]
    pub max_header_list_size: u32,
}

impl Default for Http2ProtocolSettings {
    fn default() -> Self {
        Self {
            initial_connection_window_size: Constants::DEFAULT_HTTP_INITIAL_CONNECTION_WINDOW_SIZE,
            stream_window_size: Constants::DEFAULT_HTTP2_STREAM_WINDOW_SIZE,
            max_frame_size: Constants::DEFAULT_HTTP2_MAX_FRAME_SIZE,
            max_header_list_size: Constants::DEFAULT_HTTP2_MAX_HEADER_LIST_SIZE,
        }
    }
}

fn default_http2_initial_connection_window_size() -> u32 { Constants::DEFAULT_HTTP_INITIAL_CONNECTION_WINDOW_SIZE }
fn default_http2_stream_window_size() -> u32 { Constants::DEFAULT_HTTP2_STREAM_WINDOW_SIZE }
fn default_http2_max_frame_size() -> u32 { Constants::DEFAULT_HTTP2_MAX_FRAME_SIZE }
fn default_http2_max_header_list_size() -> u32 { Constants::DEFAULT_HTTP2_MAX_HEADER_LIST_SIZE }

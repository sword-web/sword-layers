use crate::socketio::SocketIoServerConfig;
use socketioxide::{ParserConfig, SocketIo, TransportType, layer::SocketIoLayer};
use std::collections::HashSet;

pub struct SocketIoServerLayer;

impl SocketIoServerLayer {
    pub fn new(config: SocketIoServerConfig) -> (SocketIoLayer, SocketIo) {
        let mut layer_builder = SocketIo::builder();

        if let Some(ack_timeout) = config.ack_timeout {
            layer_builder = layer_builder.ack_timeout(ack_timeout.parsed);
        }
        if let Some(connect_timeout) = config.connect_timeout {
            layer_builder = layer_builder.connect_timeout(connect_timeout.parsed);
        }
        if let Some(max_buffer_size) = config.max_buffer_size {
            layer_builder = layer_builder.max_buffer_size(max_buffer_size);
        }
        if let Some(max_payload) = config.max_payload {
            layer_builder = layer_builder.max_payload(max_payload.parsed as u64);
        }
        if let Some(ping_interval) = config.ping_interval {
            layer_builder = layer_builder.ping_interval(ping_interval.parsed);
        }
        if let Some(ping_timeout) = config.ping_timeout {
            layer_builder = layer_builder.ping_timeout(ping_timeout.parsed);
        }
        if let Some(req_path) = config.req_path {
            layer_builder = layer_builder.req_path(req_path);
        }
        if let Some(transports) = config.transports {
            let parsed_transports = transports
                .into_iter()
                .collect::<HashSet<_>>()
                .iter()
                .filter_map(|t| match t.as_str() {
                    "polling" => Some(TransportType::Polling),
                    "websocket" => Some(TransportType::Websocket),
                    _ => None,
                })
                .collect::<Vec<_>>();

            match parsed_transports.len() {
                1 => layer_builder = layer_builder.transports([parsed_transports[0]]),
                2 => {
                    layer_builder =
                        layer_builder.transports([parsed_transports[0], parsed_transports[1]])
                }
                _ => {}
            };
        }
        if let Some(parser) = config.parser {
            match parser.as_str() {
                "common" => layer_builder = layer_builder.with_parser(ParserConfig::common()),
                "msgpack" => layer_builder = layer_builder.with_parser(ParserConfig::msgpack()),
                _ => {}
            }
        }
        if let Some(ws_read_buffer_size) = config.ws_read_buffer_size {
            layer_builder = layer_builder.ws_read_buffer_size(ws_read_buffer_size);
        }

        layer_builder.build_layer() as (SocketIoLayer, SocketIo)
    }
}

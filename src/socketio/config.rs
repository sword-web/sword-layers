use crate::utils::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketIoServerConfig {
    /// Whether to enable the Socket.IO server.
    /// Defaults to false.
    pub enabled: bool,

    /// The amount of time the server will wait for an acknowledgement
    /// from the client before closing the connection.
    ///
    /// Defaults to 5 seconds.
    pub ack_timeout: Option<TimeConfig>,

    /// The amount of time before disconnecting a client that has not
    /// successfully joined a namespace.
    ///
    /// Defaults to 45 seconds.
    pub connect_timeout: Option<TimeConfig>,

    /// The maximum number of packets that can be buffered per connection
    /// before being emitted to the client. If the buffer if full the emit()
    /// method will return an error.
    ///
    /// Defaults to 128 packets.
    pub max_buffer_size: Option<usize>,

    /// The maximum size of a payload in bytes. If a payload is bigger than
    /// this value the emit() method will return an error.
    ///
    /// Defaults to 100 kb.
    pub max_payload: Option<ByteConfig>,

    /// The interval at which the server will send a ping packet to the client.
    /// Defaults to 25 seconds.
    pub ping_interval: Option<TimeConfig>,

    /// The amount of time the server will wait for a ping response from the
    /// client before closing the connection.
    ///
    /// Defaults to 20 seconds.
    pub ping_timeout: Option<TimeConfig>,

    /// The path to listen for socket.io requests on.
    /// Defaults to "/socket.io".
    pub req_path: Option<String>,

    /// The transports to allow for connections.
    /// Valid options are "polling" and "websocket".
    pub transports: Option<Vec<String>>,

    /// The parser to use for encoding and decoding messages.
    /// Valid options are "common" and "msgpack".
    pub parser: Option<String>,

    /// The size of the read buffer for the websocket transport.
    /// You can tweak this value depending on your use case.
    ///
    /// Defaults to 4KiB.
    ///
    /// Setting it to a higher value will improve performance on heavy read scenarios
    /// but will consume more memory.
    pub ws_read_buffer_size: Option<usize>,
}

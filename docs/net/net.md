This is the documentation for the networking protocol.

Rules:
* Unless specified otherwise, all data is sent using **network endianness**.
* Unless specified otherwise, all text strings are using **UTF-8** encoding.

# Handshake
The handshake starts the connection with the server. These bytes must be the first sent to the server.

There are two types of handshakes, client connections and server polling. Server polling is for fetching data like player count, while client connections are regular game clients connecting to the game.

## Client connection
* `2` bytes: `NT`, ID bytes.
* `1` byte: Length of version string as an `u8`.
* `n` bytes: Version text string.
* `4` bytes: Length of nickname as `u32`.
* `n` bytes: Nickname string.

After this, the server will send either a one or a zero as `u8`. 0 if the connection is rejected, 1 if the connection is accepted, 0 if rejected.

## Server polling
* `2` bytes: `SY`, ID bytes.
* `1` byte: Length of version string as an `u8`.
* `n` bytes: Version text string.
* **TODO**, server polling reserved for later designing.

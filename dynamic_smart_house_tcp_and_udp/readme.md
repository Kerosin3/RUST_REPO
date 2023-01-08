# Smart house socket connetion via Tcp
## Usage
1. launch the server via command ``cargo run -p server_shouse``
2. launch client app, spicifying a valid dev name and optional enable\disable CLI command.
### Examples
1. `` cargo run -p client_application -- -d smart_socket_#0 -e true `` to enable a smart socket with a name smart_socket_#0
2. `` cargo run -p client_application -- -d smart_socket_#0 `` to retrieve current power consumption via socket smart_socket_#0
3. `` cargo run -p client_application -- --h `` help

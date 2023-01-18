# Smart house socket connetion via TCP, termometer update via UDP
## Usage
1. launch the server via command ``cargo run -p server_shouse``
2. launch client app, specifying a valid dev name and optional enable\disable CLI command.
### Examples
1. `` cargo run -p client_application -- -d termometer_#0 -e true `` to enable termometer with a name termometer_#0
2. `` cargo run -p client_application -- -d termometer_#0 `` to retrieve current temperature via UDP data source imitator
3. `` cargo run -p client_application -- --h `` help

## Notes
Termometer data is being updated every 1 second.

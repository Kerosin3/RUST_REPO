# Smart house socket connetion via TCP, termometer update via UDP C language API
## Usage
1. Launch the server via command ``cargo run -p server_shouse``
2. Compile C api lib by building `cargo build -p client_api`
3. Compile C executable in folder ``client_api`` by creating ``build`` in the ``client_api`` dir, cd into, then ``cmake ..``, and ``make`` 
4. Run ``RUSTCAPP`` to test whether app is working.

## Notes
Termometer data is being updated every 0.1 second.

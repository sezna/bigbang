# Websocket Bigbang Example
This example will run a 3d simulation in your browser using three.js. There are two components that must be run: the server and the client. The server can be run with `cargo run --bin websocket_3d_server`. The frontend in this folder must be served manually, either by something like `miniserve .` or `python -m SimpleHTTPServer <port>`. Then, browse to where you are hosting the frontend and view the simulation.

A PR to make the server also host the frontend like the `2d_js` example would be welcome. :)
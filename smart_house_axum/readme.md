# Smarthouse lib via axum (base on hyper) web framework.  
## Usage
1. launch the server via command ``cargo run``
2. launch example client app via command `` cargo run --example client ``
### Info
1. Use query request with [devname,status] field to ``http::/localhost:8080/device`` to turn on and turn off a device 
2. Use POST with JSON with field [devname] to retrieve current information about one of added devices, retrieve info in form of JSON with [devname,info] fields.
## Notes
Termometer's and socket's data are being updated every 100 mili seconds.

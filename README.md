# ais-filter

Created while I was experimeting with AIS data using a [dAISy Rapberry Pi HAT](https://shop.wegmatt.com/products/daisy-hat-ais-receiver).

This builds on the [ais crate](https://crates.io/crates/ais), and:
* Takes a stream of AIS messages from STDIN (e.g., piped in from `/dev/serial0`)
* Parsing each line using the `ais` crate
* Discarding invalid messages
* Optionally discarding duplicates, with a configurable sample rate (e.g. 60 seconds)
* Writing the resulting messages to STDOUT

This can be used as a filter for invalid or duplicate messages, where you can then pipe the result to your output of choice (say, AIShub or MarineTraffic).

This was developed as part of an article where I was exploring how far you can take processing AIS messages using Unix pipelines and standard or simple Linux tools.

## Similar projects

The [ais](https://crates.io/crates/ais) crate itself comes with a CLI, although currently it just parses messages from STDIN and prints their contents.

[AIS-catcher](https://github.com/jvde-github/AIS-catcher) is an excellent tool that integrates with a bunch of AIS inputs and outputs.

I wanted something simpler than `AIS-catcher`, focussed on file/network IO. For more complex usecases or more specialised hardware, I'd encourage using `AIS-catcher` which is a more established and mature project.

## Future work

Right now this only pipes STDIN to STDOUT. I'd like to lean on Rust's async ecosystem to expand this into a broker that can take one or more inputs, combine them, and push them out to multiple output destinations with minimal memory/CPU overhead.

More complex filters: e.g. filter specific message types or MMSI values. TBD what a reasonable configuration mechanism is - whether it's more CLI parameters, or a config file, or a customised DSL.

Provide a TCP binding so that data can be sent to a broker while also allowing access to multiple clients on the local network.

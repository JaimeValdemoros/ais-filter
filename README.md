# ais-filter

Created while I was experimeting with AIS data using a [dAISy Rapberry Pi HAT](https://shop.wegmatt.com/products/daisy-hat-ais-receiver).

This builds on the [ais crate](https://crates.io/crates/ais), and:
* Takes a stream of AIS messages from STDIN (e.g., piped in from `/dev/serial0`)
* Parsing each line using the `ais` crate
* Discarding invalid messages
* Discarding duplicates, with a configurable sample rate (e.g. 60 seconds)
* Copying the resulting messages to STDOUT

This can be used as a filter for invalid or duplicate messages, where you can then pipe the result to your output of choice (say, AIShub or MarineTraffic).

## Similar projects

[AIS-catcher](https://github.com/jvde-github/AIS-catcher) is an excellent tool that integrates with a bunch of AIS inputs and outputs.

I wanted something simpler, focussed on file/network IO. For more complex usecases, I'd encourage using AIS-catcher which is a more established and mature project.

## Future work

Right now this only pipes STDIN to STDOUT.
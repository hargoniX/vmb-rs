# vmb-config
This crate implements a small parser for the vmb config file format as specified here:
http://vmb.sourceforge.net/configuration.html.

## examples
You can run the led example in the `examples/` directory as follows: `cargo run --example=led -- examples/led.vmb`.
The output should be something along the lines of:
```
Ok(LedConfig { address: Some(1234), path: Some("/absolute/path/to/here/vmb-config/examples/led.vmb"), filename: Some("led.vmb") })
```

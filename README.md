# gst-openaichat: A GStreamer element implementing conversation with the OpenAI Chat API.

Accepts text buffers on its sink pad, sends them to the OpenAI Chat API, and produces the responses as text buffers on its source pad.

## Installation

gst-openaichat is written in Rust and uses the cargo-c helper. Set up a Rust development environment (e.g. using [rustup](https://rustup.rs)) and then:

```
cargo install cargo-c

git clone https://github.com/avstack/gst-openaichat
cd gst-openaichat
cargo cinstall
```

## Example usage (chat with GPT in your console)

```
OPENAI_API_KEY=... gst-launch-1.0 --quiet fdsrc ! 'text/x-raw,format=utf8' ! openaichat model=gpt-3.5-turbo ! fdsink
```

Combine it with our [whisper](https://github.com/avstack/gst-whisper) and [ttssink](https://github.com/avstack/gst-ttssink) elements and have a spoken conversation with GPT! (Use headphones, or GPT may respond to itself.)

```
OPENAI_API_KEY=... gst-launch-1.0 --quiet autoaudiosrc ! audioconvert ! audioresample ! queue ! whisper ! openaichat model=gpt-3.5-turbo ! ttssink
```

## License

gst-openaichat is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any kinds of contributions are welcome as a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in these crates by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements

gst-openaichat development is sponsored by [AVStack](https://avstack.io/). We provide globally-distributed, scalable, managed Jitsi Meet backends.

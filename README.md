# `fry`

A very simple, dumb, no-alloc, no-std TTS.
This comes with a handful of extreme limitation:

1. It may only work on text of a fixed-sized buffer (maximum 32 characters at a time).
2. It only spells words, it does not actually speak them.
3. It uses `espeak` to generate the sound files, and `sox` to modify the output to be of a fixed length.
4. It has no output capability. It is up to the user of the library to know where to dump this data.
5. The audio produced is only 16-bit signed PCM with one channel.

## Building new files

If you'd like to produce new files for the library to use (this is required to change the speed of the speech, for example), then you can use the scripts in `data` to create new files.
Please note that you will have to manually update variables in both the scripts and the library to accomodate any changes made to the data files.

### `generate_base.sh`

This script creates new files using `espeak` and the list of english alphabetic characters to create `[a-z].wav`
You may modify the arguments to espeak to produce faster sounds via the `-s` flag for settings "words per minute".

Check out `man espeak` or `man espeak-ng` for more details.

### `calc.py`

This Python file uses the `sox` command, along with some basic math to calculate the output for adding `0` padding to the WAV file so all files have exactly the same length (in bytes and time).
Then, strip the headers so that the WAV data is simply [raw PCM data](https://en.wikipedia.org/wiki/Pulse-code_modulation).
It is up the user what they will do with this data.

If `mediainfo` displays different information than this for `[a-z].wav`, then you may need to change the settings in the constants of `calc.py` to produce the right sized padded/raw files.

## TODO

* [ ] Add some tests to verify that bit patterns are indeed concatonated correctly.
* [ ] Simplify build process for each letter `.wav` files.
* [ ] Eliminate non-Rust dependencies for building the `.wav` files.
  * [ ] `sox` (`calc.py`)
  * [ ] `python` (`calc.py`)
  * [ ] `espeak` (`generate-base.sh`)
  * [ ] `bash` (`generate-base.sh`)
* [ ] Actually use a TTS engine instead of manually craming in spelling by letter.
* [ ] Add `std` and `alloc` features for when they are available to the consumer.
* [ ] Add compile-time or test-time tests that that the following match both the wav file and some constant in the `lib.rs` file:
  * [ ] Verify bit arrangement (LE, BE)
  * [ ] Verify number of channels (mono, stereo)
  * [ ] Verify sample rate (22050, 41000, etc.)
  * [ ] Verify PCM width (s16, u16, s32, s8, etc.)


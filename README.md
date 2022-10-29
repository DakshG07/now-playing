# now-playing

A Windows utility written in Rust to be blazingly fast, simple, and easy to use. It gets the currently playing media using WinRT and displays its attributes. It also exposes commands to control the playback.

## Usage

```
Gets information about currently playing media on Windows.

Usage: now-playing.exe [COMMAND]

Commands:
  title     Print the title of the media
  artist    Print the artist of the media
  position  Print the current position in the media (may be delayed for a few seconds due to WinRT restrictions)
  duration  Print the length of the media
  status    Print the status of the media
  play      Play the media if it was previously paused
  pause     Pause the media if it was previously playing
  toggle    Toggle the state of the media between playing and paused
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

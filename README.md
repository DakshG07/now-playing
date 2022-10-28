# now-playing
A simple utility that utilizes WinRT to get the Song Title and artist. Also gets current playback features.

## Usage
```
Gets information about currently playing media on Windows.

Usage: now-playing.exe [COMMAND]

Commands:
  title     Print the title of the song
  artist    Print the atist of the song
  position  Print the current position in the song (may be delayed for a few seconds due to winRT restrictions)
  duration  Print the length of the song
  play      Play the music if it was previously paused
  pause     Pause the music if it was previously playing
  toggle    Toggle the state of the music: playing or paused
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Todo

### /playing and paused states
```
now-playing --play [text to return if playing]
now-playing --pause [text to return if paused]
```
These can be used on play/pause commands to get current status. Will also be used to change the returned value of `toggle`, `play`, and `pause`.

## Status
```
now-playing status
```
see if music is playing or pausing

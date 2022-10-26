# now-playing
A simple utility that utilizes WinRT to get the Song Title and artist. Also gets current playback features.

## Usage

```
now-playing
```
Simply run the command to get the current playing song, time, and artist. Only includes the components it can find.

```
now-playing play
```
Plays the current music. Not yet implemented.

```
now-playing pause
```
Pauses the music. Not yet implemented.

```
now-playing toggle
```
Pauses/plays the music. Not yet implemented.

```
now-playing position
```
Gets current timestamp.

```
now-playing title
```
Gets current song.

```
now-playing artist
```
Gets current artist.

```
now-playing status
```
Checks if playback is played or paused. Not yet implemented.

Also has the following flags:
```
--play [text to return if playing]
--pause [text to return if paused]
```
These can be used on play/pause commands to get current status. Will also be used to change the returned value of `toggle`, `play`, and `pause`.

Not yet implemented.

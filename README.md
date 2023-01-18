# spotify-id3

Simple command line utility for patching local `.mp3`-files with `ID3v2.3` metadata fetched from the official Spotify API.

```bash
> spotify-id3 ./library/3EDKoVbMaOWMzMGvgITUN1.mp3 --write

WORS spotify:track:3EDKoVbMaOWMzMGvgITUN1
TPE1 DJOKO
TIT2 Lesson
TALB Lesson 1 EP
TDRL 2020
TRCK 4
TBPM 126.994
TKEY Fb
TCON german tech house, minimal tech house

TXXX acousticness       0.068
TXXX danceability       0.808
TXXX energy             0.829
TXXX instrumentalness   0.866
TXXX liveness           0.061
TXXX speechiness        0.073
TXXX valence            0.465

Successfully wrote ID3v2.3 tag to ./library/3EDKoVbMaOWMzMGvgITUN1.mp3!
```

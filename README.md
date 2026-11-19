# Flick Time

`Flick` or "frame tick" is a measurement of time representing `1 / 705_600_000`  of a second. Invented by [Christopher Horvath](https://github.com/blackencino) in 2016 as an alternative to measuring time in software and hardware using floating point math. The magic of the `Flick` is that it can be stored as an integer and accurately represent almost all time formats used in media playback and rendering.

## Intent

While this measurement of time may not fit most needs, it does fit extremely well in the world of rendering, game, and media software. It has particular use for deterministic measurement of time when working with common media frame rates and audio sample rates. I am building this crate as a part of my own personal project that involves synchronized media playback and rendering and believe my implementation is more idiomatic for `rust` than other crates and other projects may find it useful.

## History

The `Flick` was first proposed on FaceBook by Horvath and then refined with contributions by Kearsley Higgins, Francisco De La Torre, Benjy Feen, Eric Landreneau, and Sebastian Sylwan. The original repository can be found in the archived [Flicks](https://github.com/facebookarchive/Flicks) repository on GitHub.

## Common Conversions

| Hz |   Flicks | Note |
| --- | --- | --- |
| 8 | `88_200_000` | Animation on 3's |
| 12 | `58_800_000` | Animation on 2's |
| 23.976  | `29_429_400` | NTSC 3:2 pulldown |
| 24 | `29_400_000` | Film |
| 25 | `28_224_000` | PAL Broadcast |
| 29.97 | `23_543_520` | NTSC Broadcast 30p |
| 30 | `23_520_000` | 30 Fps |
| 48 | `14_700_000` | 2x Film |
| 50 | `14_112_000` | 2x PAL Broadcast |
| 59.94 | `11_771_760` | NTSC Broadcast 60p |
| 60 | `11_760_000` | Standard Refresh Rate |
| 90 | `7_840_000` | LED \ Monitor Refresh |
| 120 | `5_880_000` | Monitor Refresh |
| 144 | `4_900_000` | Monitor Refresh |
| 240 | `2_940_000` | Monitor Refresh |
| 44,100 | `16_000` | NTSC Audio |
| 48,000 | `14_700` | CD Quality Audio |
| 88,200 | `8_000` | NTSC High Quality Audio |
| 96,000 | `7_350` | High Quakity Audio |
| 192,000 | `3_675` | Audio Mastering |


# Flick Time

`Flick` or "frame tick" is a measurement of time representing `1 / 705_600_000`  of a second. Invented by [Christopher Horvath](https://github.com/blackencino) in 2016 as an alternative to measuring time in software and hardware using floating point math. The magic of the `Flick` is that it can be stored as an integer and accurately represent almost all time formats used in media playback and rendering.

## Intent

While this measurement of time may not fit most needs, it does fit extremely well in the world of rendering, game, and media software. It has particular use for deterministic measurement of time when working with common media frame rates and audio sample rates. I am building this crate as a part of my own personal project that involves synchronized media playback and rendering and believe my implementation is more idiomatic for `rust` than other crates and other projects may find it useful.

## History

The `Flick` was first proposed on FaceBook by Horvath and then refined with contributions by Kearsley Higgins, Francisco De La Torre, Benjy Feen, Eric Landreneau, and Sebastian Sylwan. The original repository can be found in the [Flicks](https://github.com/facebookarchive/Flicks) repository on GitHub.

## Common Conversions

**Common Frame / Refresh Rates**

| Hz/FPS |   Flicks |
| --- | --- |
| 23.976  | `29_429_400` |
| 24 | `29_400_000` |
| 29.97 | `23_543_520` |
| 30 | `23_520_000`
| 48 | `14_700_000` |
| 50 | `14_112_000` |
| 59.94 | `11_771_760` |
| 60 | `11_760_000` |
| 90 | `7_840_000` |
| 120 | `5_880_000` |
| 144 | `4_900_000` |
| 240 | `2_940_000` |


**Common Audio Sample Rates**
| Hz | Flicks |
| --- | --- |
| 44,100 | `16_000` |
| 48,000 | `14_700` |
| 88,200 | `8_000` |
| 96,000 | `7_350` |
| 192,000 | `3_675` |



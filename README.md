# ngrrram

`ngrrram` is a CLI tool to practice typing ngrams (`n` adjacent symbols in
particular order) to improve your typing speed and/or learn new keyboard
layouts effectively.

![a showcase of the ngrrram ui](./.github/showcase.gif)

Certain letter combination occur a lot more often than others, thus practicing
these in particular makes sense. This type of practice is often recommended for
example by [Ben Vallack](https://www.youtube.com/watch?v=sI-a64EVPPU) and Josh
Kaufman, who [learned Colemak in just 20 hours](https://first20hours.com)
making strong use of ngrams.

There already exists a good tool for this type of practice called
[ngram-type](https://github.com/ranelpadon/ngram-type). This tool is *heavily*
inspired by `ngram-type` and I want to thank Ranel Padon for creating it.

However, ngram-type does not support emulating different keyboard layouts,
which I found important so I would not have to switch my whole system to a
layout I was still learning.

Also, some people might prefer local/offline CLI based solutions over web based
ones.

## Installation

### Releases

Precompiled [releases](https://github.com/wintermute-cell/ngrrram/releases) for
linux, windows and macos are available.

### AUR (maintained by [JinEnMok](https://github.com/JinEnMok))

Use `yay` or any other AUR helper to install for an Archlinux system:

```bash
yay -S ngrrram-git
or
yay -S ngrrram-bin
```

### From Source

Make sure you have the rust tooling installed, then simply run:

```bash
cargo build --release
```

The executable will then be located at `./target/release/ngrrram`

### Looking for help packaging!

For now `ngrrram` is not available in any package repo. If you'd like to help
by packaging for your platform, I'd gladly accept!

## Usage

`ngrrram` is not very complex. It offers a few customization options as command
flags, but starts with the recommended defaults if unconfigured.

Options:
```
Usage: ngrrram [OPTIONS]

Options:
  -n, --n <2|3|4|w|file>  use bi-(2), tri-(3), tetragrams(4), (w)ords or comma separated wordlist file. [default: 2]
  -t, --top <1-200>       use the top X ngrams ordered by usage. [default: 50]
  -c, --combi <1-200>     how many different ngrams to use in a single lesson. [default: 2]
  -r, --rep <number>      how often to repeat *each* different ngram in a lesson. [default: 3]
  -w, --wpm <number>      the wpm threshold at which the lesson is considered a success. [default: 40]
  -a, --acc <0-100>       the accuracy in percent at which the lesson is considered a success. [default: 94]
      --emu-in <layout>   your current keyboard layout. only needed if you want to emulate a different layout. see docs for supported layouts. [default: ]
      --emu-out <layout>  the layout you want to emulate. only needed if you want to emulate a different layout. see docs for supported layouts. [default: ]
      --show-ortho        show keyboard in ortholinear format
      --nokb              pass this flag to disable the keyboard layout display.
      --cat               the most important flag. don't practice alone.
  -h, --help              Print help
```

If you start `ngrrram` without parameters, it uses these recommended defaults:
```bash
ngrrram --n 2 --top 50 --combi 2 --rep 3 --wpm 40 --acc 100
```

## Layout Emulation

To emulate a different keyboard layout in `ngrrram`, you must pass the flags
`--emu-in` and `--emu-out`, the first one describing your current layout, and
the second one being the one you want to emulate.

Available layouts are:

- `qwerty`    (Qwerty)
- `qwertz`    (Qwertz)
- `azerty`    (Azerty)
- `dvorak`    (Dvorak)
- `colemak`   (Colemak)
- `colemakdh` (ColemakDH)

> Having to provide an input layout is sub-optimal. I'm not sure how to get
> layout independent scancodes in rust; Could not get `device_query` to work.
> If you know a solution, please tell me.

## Random Notes
- The WPM timer for each lesson only starts once you type the first letter of
  that lesson; no need to stress.
- Every 5 non-space characters are considered a "word" for the WPM calculation.
  Otherwise WPM would unnaturally skyrocket with smaller ngrams.

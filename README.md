# ngrrram

`ngrrram` is a CLI tool to practice typing ngrams (`n` adjacent symbols in
particular order) to improve your typing speed and/or learn new keyboard
layouts effectively.

![a showcase of the ngrrram ui](./showcase.mp4)

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

### Looking for help packaging!

For now `ngrrram` is not available in any package repo. If you'd like to help
by packaging for your platform, I'd gladly accept!

### From Source

Make sure you have the rust tooling installed, then simply run:

```
cargo build --release
```

The executable will then be located at `./target/release/ngrrram`

## Usage

`ngrrram` is not very complex. It offers a few customization options as command
flags, but starts with the recommended defaults if unconfigured.

Options:
```
--n <2|3|4|w|FILENAME>       use bigrams(2) trigrams(3) tetragrams(4) or words(w), or specify your own comma separated wordlist as a file.
--top <1-200>                use the top X ngrams ordered by usage.
--combi <1-200>              how many different ngrams to use in a single lesson.
--rep <number>               how often to repeat *each* different ngram in a lesson.
--wpm <number>               the wpm threshold at which the lesson is considered a success.
--acc <0-100>                the accuracy in percent at which the lesson is considered a success.
--emu_in <layout>            see section (## Layout Emulation).
--emu_out <layout>           see section (## Layout Emulation).
--nokb                       pass this flag to disable the keyboard layout display.
--cat                        the most important flag. don't practice alone.
```

If you start `ngrrram` without parameters, it uses these recommended defaults:
```bash
ngrrram --n 2 --top 50 --combi 2 --rep 3 --wpm 40 --acc 100
```

## Layout Emulation

To emulate a different keyboard layout in `ngrrram`, you must pass the flags
`--emu_in` and `--emu_out`, the first one describing your current layout, and
the second one being the one you want to emulate.

Available layouts are:

- `qwerty`    (Qwerty)
- `qwertz`    (Qwertz)
- `azerty`    (Azerty)
- `dvorak`    (Dvorak)
- `colemak`   (Colemak)
- `colemakdh` (ColemakDH)

> Having to provide an input layout is sub-optimal. I'm not sure how to get
> layout independant scancodes in rust; Could not get `device_query` to work.
> If you know a solution, please tell me.

## Random Notes
- The WPM timer for each lesson only starts once you type the first letter of
  that lesson; no need to stress.
- Every 5 non-space characters are considered a "word" for the WPM calculation.
  Otherwise WPM would unnaturally skyrocket with smaller ngrams.

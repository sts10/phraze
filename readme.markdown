# Phraze
[![Crates.io](https://img.shields.io/crates/v/phraze?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fphraze)](https://crates.io/crates/phraze)
[![Crates.io number of downloads](https://img.shields.io/crates/d/phraze)](https://crates.io/crates/phraze)
[![](https://deps.rs/repo/github/sts10/phraze/status.svg)](https://deps.rs/repo/github/sts10/phraze)
[![Crates.io](https://img.shields.io/crates/l/phraze)](./LICENSE.txt)
[![Packaging status](https://repology.org/badge/tiny-repos/phraze.svg)](https://repology.org/project/phraze/versions)

Generate random passphrases.

```bash
$ phraze
curse-argues-valves-unfair-punk-ritual-inlet
```

## Features

* 🎚️  Allows user to set a minimum entropy, freeing them from having to figure how many words from a given list they need to create a strong passphrase
* 🎯 Includes a variety of built-in word lists, all of which are uniquely decodable, ensuring that passphrase entropy estimates remain accurate when no separator is used
* ⚡ Fast: Takes about 2 milliseconds to generate a passphrase
* 🔣 Can insert numbers, symbols, and/or capital letters if necessary (e.g. `phraze -s _b -t`)
* 🛁 Default word list is (hopefully) free of profane words
* 🧺 Choose from a number of included word lists or provide your own
* 🛠️  Written in [Rust](https://www.rust-lang.org/)

## Installing

### Using Rust and Cargo
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install phraze --locked` (Run this same command to upgrade Phraze to latest available version.)

Uninstall Phraze by running `cargo uninstall phraze`.

### Homebrew

```shell
brew update
brew tap sts10/phraze
brew install phraze
```

Uninstall with `brew uninstall phraze && brew untap sts10/phraze`

(Special thanks to [@phoggy](https://github.com/phoggy) for help getting this Homebrew installation working.)

### NixOS/nix

[![nixpkgs unstable package](https://repology.org/badge/version-for-repo/nix_unstable/phraze.svg)](https://repology.org/project/phraze/versions)

Phraze is available within `nixpkgs`, and can be used:

- System-wide on a NixOS system using `environment.systemPackages =
[pkgs.phraze]`
- Per-user using home-manager `home.packages = [pkgs.phraze]`
- One-off with `nix run`

    ```shell
    # Run one-shot via nix
    $ nix run nixpkgs#phraze -- -S -s _b -t
    Commuter=Scripts_Motorway9Battle&Results,Trouble-Policy@Tools
    ```

- Via a temporary nix shell without installing.
    
    ```shell
    # Drop into a nix shell with phraze available
    $ nix shell nixpkgs#phraze
    $ which phraze
    /nix/store/i1car5jf8w6vxglfi2gdrzsbzmi2vrrh-phraze-0.3.11/bin/phraze
    $ phraze -S -s _b -t
    Meditation)Skin0Invalid!Donations6Targeted(Housed8Tossed#Synagogue
    $ exit
    ```

### Latest release
Alternatively, you can get binaries from [the GitHub releases page](https://github.com/sts10/phraze/releases).

## How to use

Running `phraze`, without specifying options, will generate a passphrase with at least 80 bits of entropy with a hyphen separating each word.

### Changing the strength of the passphrase

By default, Phraze generates a passphrase with at least 80 bits of entropy. Entropy is an estimate of the "strength" of the passphrase. Higher entropy means a stronger passphrase.

You can change this "strength" of the passphrase Phraze generates, making it either weaker or stronger, **3 different ways**:

**1. Set a Strength Count.** Use `-S` to increase minimum entropy from 80 bits to 100 bits. Each additional `S` adds another 20 bits of minimum entropy (e.g. `-SS` => 120 bit minimum; `-SSS` => 140 bit minimum, etc.).
```text
$ phraze -SS
determined-pervasive-entirety-incumbent-trophy-emergence-spatial-wondering-destroyed-gamma
```

**2. Set a specific minimum entropy.** Use `--minimum-entropy` to specify your own minimum amount of entropy, in bits, that your passphrase must have.
```text
$ phraze --minimum-entropy 100
toured-warrior-skeleton-shear-hosts-injuries-relied-sadness
```

**3. Set number words.** Use `--words` to specify the exact number of words for Phraze to use.
```text
$ phraze --words 5 # passphrase will have 5 words, overriding the default minimum entropy setting of 80 bits
determines-generated-frozen-excluded-sleeping
```

Note that you can only use one of these strength-changing methods at a time.

If you want to know how much entropy your generated passphrase has, add the `-v`/`--verbose` flag.
```text
$ phraze -v -S
Passphrase has an estimated 104.00 bits of entropy (8 words from a list of 8192 words)
seventy-cost-freight-suspended-misery-objections-represents-buying
```

### Changing the separator between words
By default, Phraze separates words with a hyphen ("-"). You can change that with the `--sep` (or `-s`) option.

`--sep` accepts special inputs `_n` (random numbers), `_s` (random symbols), and `_b` (mix of both). Note that separator choice does _not_ effect entropy calculations.
```text
$ phraze --sep ' '
optimism daughters figures grim processors became decreasing
$ phrase --sep _s
fax/household>validation_replied-upgrade,remind?reasoning
```

If you don't want a separator at all, use `-s ''`:
```text
$ phraze -s ''
theftinversiondebtsquietlysuspensionannualchocolate
```

You can make all the word Title Case by using `--title-case`:
```text
$ phraze --sep '' --title-case
GoverningDominateAnswersReceptorsAllocatedClientModify
```

If your passphrase needs to have a symbol, a number and an uppercase character in your passphrase, you can use Title Case (`-t`) and use random symbols and numbers as word separators (`-s _b`):
```text
$ phraze -t -s _b
Welcome&Song}Barker)Concrete;Commune$Shouted2Ensuing
```

### Changing the word list that Phraze uses
By default, Phraze uses [a 8192-word list](https://github.com/sts10/phraze/blob/main/word-lists/orchard-street-medium.txt) called the Orchard Street Medium List (which gives 13 bits of entropy per word).

You can specify a different list with `--list`/`-l`, with a choice of a handful of lists included with Phraze.

Each included list has a corresponding one-letter code (see below or run `phrase --help` for a full list). For example, `--list s` causes Phraze to use the [EFF **s**hort list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).
```text
$ phraze --list s
duck-slip-swoop-stray-wink-stump-whiff-slot
```
(Note that we need 8 words from the EFF Short List to meet the default minimum entropy of 80 bits.)

### Using your own list
If you prefer, you can have Phraze generate a passphrase using your own word list. Use the `--custom-list` option.
```text
$ phraze --custom-list path/to/word/list
```
Before generating a passphrase from a given custom list, Phraze will remove any and all trailing white space, duplicate words, and blank words in the inputted list. Phraze will also check for uniform [Unicode normalization](https://www.unicode.org/faq/normalization.html).

### Copying passphrase to clipboard
You can pipe Phraze's outputted passphrase to other tools. For example, you can copy generated passphrase to xclip (a common Linux clipboard tool):
```bash
$ phraze | xclip -selection clipboard -rmlastnl
```

## Usage
```text
Usage: phraze [OPTIONS]

Options:
  -S, --strength...
          Strengthen your passphrase the easy way: Each -S flag increases minimum 
          entropy by 20 bits (above the default of 80 bits)

  -e, --minimum-entropy <MINIMUM_ENTROPY>
          Set minimum amount of entropy in bits for generated passphrase. If 
          neither minimum_entropy or number_of_words is specified, Phraze will 
          default to an 80-bit minimum

  -w, --words <NUMBER_OF_WORDS>
          Set exactly how many words to use in generated passphrase. If neither 
          number_of_words or minimum_entropy is specified, Phraze will default to 
          an 80-bit minimum

  -n, --passphrases <N_PASSPHRASES>
          Number of passphrases to generate
          
          [default: 1]

  -s, --sep <SEPARATOR>
          Word separator. Can accept single quotes around the separator. To not use a 
          separator, use empty single quotes ''.
          
          There are special values that will trigger generated separators:
          
          _n: separators will be random numbers
          
          _s: separators will be random symbols
          
          _b: separators will be a mix of random numbers and symbols
          
          [default: -]

  -l, --list <LIST_CHOICE>
          Choose a word list to use.
          
          Options:
          
          m: Orchard Street Medium List (8,192 words) [DEFAULT]
          
          l: Orchard Street Long List (17,576 words)
          
          e: EFF Long List (7,776 words)
          
          n: Mnemonicode list (1,633 words). Good if you know you're going to be 
          speaking passphrases out loud.
          
          s: EFF Short List (1,296 words)
          
          q: Orchard Street QWERTY List (1,296 words). Optimized to minimize travel 
          distance on QWERTY keyboard layout.
          
          a: Orchard Street Alpha List (1,296 words). Optimized to minimize travel 
          distance on alphabetical keyboard layout
          
          [default: m]

  -c, --custom-list <CUSTOM_LIST_FILE_PATH>
          Provide a text file with a list of words to randomly generate passphrase 
          from. Should be a text file with one word per line.

  -t, --title-case
          Use Title Case for words in generated passphrases

  -v, --verbose
          Print estimated entropy of generated passphrase, in bits, along with the 
          passphrase itself

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Included word lists

By default, Phraze uses [the Orchard Street Medium wordlist](https://github.com/sts10/orchard-street-wordlists/blob/main/lists/orchard-street-medium.txt), which has 8,192 words. This means each word adds 13.0 bits of entropy to a passphrase.

However, Phraze comes with other word lists built-in. Run `phraze --help` to view word list options. You can choose a different word list by using the `-l`/`--list` option. All of the word lists included with Phraze are uniquely decodable, which means they're safe to use without a separator between words.

* Orchard Street Medium list: 8,192 words; 13 bits of entropy per word. This is the **DEFAULT** list Phraze will use if no list is specified by the user.
* [Orchard Street Long list](https://github.com/sts10/orchard-street-wordlists/blob/main/lists/orchard-street-long.txt): 17,576 words; 14.1 bits of entropy per word. Use `l`.
* [EFF Long List](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases): 7,776 words; 12.93 bits of entropy per word. Use `e`.
* [Mnemonicode](https://github.com/singpolyma/mnemonicode) list: 1,633 words; 10.67 bits of entropy per word. Words are easy to pronounce out loud. Use `n`.
* [EFF Short List 1](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases): 1,296 words; 10.3 bits of entropy per word. Use `s`.
* Orchard Street QWERTY List: 1,296 words; 10.3 bits of entropy per word. Use `q`.
* Orchard Street Alpha List: 1,296 words; 10.3 bits of entropy per word. Use `a`.

### Notes on the Orchard Street QWERTY and Alpha lists
These two lists are optimized to minimize travel distance when inputting passphrases into TVs or video game consoles. They both have 1,296 words (10.3 bits per word).

The Orchard Street QWERTY List that is optimized for QWERTY keyboard layouts. Use this list if your keyboard layout looks like this:

```txt
qwertyuiop
asdfghjkl
zxcvbnm
```

The Orchard Street Alpha List that is optimized for alphabetical keyboard layouts. Use this list if your keyboard layout looks like this:

```txt
abcdef
ghijkl
mnopqr
stuvwx
yz
```

### Technical details of all included word lists

This list information was generated using [Word List Auditor](https://github.com/sts10/wla).

#### Orchard Street Medium List
```txt
Lines found               : 8192
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 8192
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : false
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (add)
Length of longest word    : 10 characters (worthwhile)
Mean word length          : 7.07 characters
Entropy per word          : 13.000 bits
Efficiency per character  : 1.839 bits
Assumed entropy per char  : 4.333 bits
Shortest edit distance    : 1
Mean edit distance        : 6.966
Longest shared prefix     : 9
Unique character prefix   : 10

Sample passphrases:
popular-claiming-sailing-spiritual-homeland-pay-keyboard
provided-plant-summarized-therapy-married-involves-rocks
worked-athlete-caucus-slight-discretion-tightly-occasional
medal-ranks-habit-labor-genre-saved-remainder
spectator-municipal-longest-colleagues-demolition-enzyme-widespread
```

#### Orchard Street Long list
```txt
Lines found               : 17576
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 17576
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : false
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (add)
Length of longest word    : 15 characters (troubleshooting)
Mean word length          : 7.98 characters
Entropy per word          : 14.101 bits
Efficiency per character  : 1.767 bits
Assumed entropy per char  : 4.700 bits
Shortest edit distance    : 1
Mean edit distance        : 7.915
Longest shared prefix     : 14
Unique character prefix   : 15

Sample passphrases:
exponent-sync-memorandum-vaulted-stiffened-reverted
camps-interdependence-worsening-choral-somebody-obey
immensely-casinos-plundered-warns-vinegar-event
bottled-charge-linkage-husbands-cuisine-weave
gospel-graders-relegated-exits-determine-ducked
```

#### EFF Long List
```txt
Lines found               : 7776
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 7776
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : true
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (aim)
Length of longest word    : 9 characters (zoologist)
Mean word length          : 6.99 characters
Entropy per word          : 12.925 bits
Efficiency per character  : 1.849 bits
Assumed entropy per char  : 4.308 bits
Shortest edit distance    : 1
Mean edit distance        : 6.858
Longest shared prefix     : 8
Unique character prefix   : 9

Sample passphrases:
audible-encounter-defection-democracy-canister-pencil-comma
dwindling-gangway-driving-grumbly-stoke-scanning-stimulant
overpay-dial-manlike-purposely-demeanor-unified-likeness
edition-fernlike-synthetic-aloe-filing-wrangle-spiny
tattle-reapply-borough-stature-cuddle-crummiest-flatten
```

#### Mnemonicode list
Note: I swapped the word "beatles" for "beetle", so this isn't exactly the same as the canonical Mnemonicode word list.
```txt
Lines found               : 1633
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 1633
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : true
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (ego)
Length of longest word    : 7 characters (william)
Mean word length          : 5.75 characters
Entropy per word          : 10.673 bits
Efficiency per character  : 1.857 bits
Assumed entropy per char  : 3.558 bits
Shortest edit distance    : 1
Mean edit distance        : 5.552
Longest shared prefix     : 6
Unique character prefix   : 7

Sample passphrases:
bodies-novelist-poor-feminine-plates-ideology-emeritus
specific-lighting-orbit-math-weakness-embarked-rang
session-somebody-sector-keyboards-ambassador-circle-contrasts
strand-mankind-punished-woke-deities-keyboard-camping
glass-homeless-feature-fee-preparing-interfaces-nations
```

#### EFF Short List
Note: I swapped out the word "yo-yo" for the word "zen".
```txt
Lines found               : 1296
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 1296
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : true
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (aim)
Length of longest word    : 5 characters (zippy)
Mean word length          : 4.54 characters
Entropy per word          : 10.340 bits
Efficiency per character  : 2.278 bits
Assumed entropy per char  : 3.447 bits
Shortest edit distance    : 1
Mean edit distance        : 4.366
Longest shared prefix     : 4
Unique character prefix   : 5

Sample passphrases:
flame-chump-stood-slurp-saint-spent-path-putt
sax-sweep-guide-snore-knee-pod-cadet-twist
reset-mouse-track-taco-movie-oak-recap-purse
hump-dug-wifi-skid-panty-rake-vocal-stoop
silo-utter-pest-snap-zoom-crate-suds-batch
```

#### Orchard Street QWERTY List
```txt
Lines found               : 1296
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 1296
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : false
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (add)
Length of longest word    : 8 characters (referred)
Mean word length          : 4.24 characters
Entropy per word          : 10.340 bits
Efficiency per character  : 2.441 bits
Assumed entropy per char  : 3.447 bits
Shortest edit distance    : 1
Mean edit distance        : 4.170
Longest shared prefix     : 6
Unique character prefix   : 7

Sample passphrases:
think-watt-bad-unity-strip-troop-three-crab
graded-mast-mom-semi-chop-dash-far-view
dam-fare-root-quite-pill-hitter-guide-muse
man-tomb-jar-trim-tip-bits-faded-dig
young-ten-threw-shy-zero-grew-ready-dead
```

#### Orchard Street Alpha List
```txt
Lines found               : 1296
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 1296
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : false
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (add)
Length of longest word    : 7 characters (stopped)
Mean word length          : 4.12 characters
Entropy per word          : 10.340 bits
Efficiency per character  : 2.509 bits
Assumed entropy per char  : 3.447 bits
Shortest edit distance    : 1
Mean edit distance        : 4.043
Longest shared prefix     : 6
Unique character prefix   : 7

Sample passphrases:
pigs-sue-stay-week-woke-sued-pass-mayo
month-guns-half-lists-seek-pony-pine-foe
jet-troop-hung-fond-wind-lit-long-dams
loops-peer-quit-push-hank-over-doing-pain
gave-model-coil-lent-deep-lam-chin-tall
```

## Source of randomness

Phraze uses the [rand crate](https://github.com/rust-random/rand)'s [ThreadRng](https://rust-random.github.io/rand/rand/rngs/struct.ThreadRng.html) as its cryptographically secure pseudo-random number generator (CSPRNG) for generating passphrases.

According to [the rand crate documentation at the time of this writing, "ThreadRng uses the same CSPRNG as StdRng, ChaCha12"](https://rust-random.github.io/rand/rand/rngs/struct.ThreadRng.html), meaning it uses 12 rounds of the ChaCha stream cipher. That `StdRng` uses ChaCha12 is [relatively clear in the source code](https://docs.rs/rand/latest/src/rand/rngs/std.rs.html#9-15). (See [this issue](https://github.com/rust-random/rand/issues/932) for arguments in favor of using 12 rounds rather than 20.)

## Why another random passphrase generator?
There are already a few good passphrase generators, including [passphraseme](https://github.com/micahflee/passphraseme) and [Pgen](https://github.com/ctsrc/Pgen).

Admittedly, part of my motivation to create Phraze was to highlight my [Orchard Street Wordlists](https://github.com/sts10/orchard-street-wordlists), which I think are pretty good!

## For developers
I'm trying to do development work on the `development` git branch, then merge the work into the `main` branch when it feels like time for a new release.

In general I welcome both pull requests and issues. See included [LICENSE.txt](./LICENSE.txt) file. This project doesn't have a formal Code of Conduct yet (it may in the future), but informally just try to be kind to each other.

Check **license compatibility** of Phraze's dependencies: `cargo deny check licenses` (requires that you [have cargo-deny installed locally](https://github.com/EmbarkStudios/cargo-deny#install-cargo-deny)). See below for more on how Phraze is licensed.

### Testing and benchmarking Phraze
Run `cargo test` to run Phraze's tests.

Phraze uses [Criterion](https://github.com/bheisler/criterion.rs) for benchmarking. You can run the benchmarks for yourself with `cargo bench`.

## Licensing

Phraze's source code is licensed under the Mozilla Public License v2.0. See included [LICENSE.txt](./LICENSE.txt) file or [this online version of the license](https://www.mozilla.org/en-US/MPL/2.0/).

### Word list licensing

Phraze includes a number of word lists, which are licensed in a variety of ways.

* The Mnemonicode word list is [copyrighted](https://github.com/singpolyma/mnemonicode/blob/master/mn_wordlist.c) by Oren Tirosh under [the MIT License](https://mit-license.org/).
* The word lists from the Electronic Frontier Foundation (EFF) are [distributed under the Creative Commons Attribution 4.0 International License (CC-BY)](https://www.eff.org/copyright).
* All Orchard Street Wordlists are available under [the Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/).

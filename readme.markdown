# Phraze

Generate random passphrases. 

## Examples

```
$ phraze
listeners-strikeouts-duchess-shrine-platform-advise-fellowship
```

Use `-w` to specify the number of words for Phraze to use.
```
$ phraze -w 5
routines-factory-threats-exact-destroyer
```

Specify a separator between words with `-s`:
```
$ phraze -s ' '
marrow responded beauty syndrome compromise cognitive prompted
```

Use `-t` to make all word in passphrase Title Case
```
$ phraze -s '' -t
ReinforcedMetalProphecyHeavyArtilleryEnoughStaying
```

Use `-l` to specify which word list to use. `-l l` uses the Orchard Street Long list.
```
$ phraze -l l
prices-leisurely-monument-shame-taller-troupe-compulsory
```

Copy generated passphrase to xclip (Linux clipboard) (Passphrase won't be printed to terminal):
```
$ phraze | xclip -selection clipboard
```

## Usage
```
Usage: phraze [OPTIONS]

Options:
  -w, --words <NUMBER_OF_WORDS>
          Set how many words in generated passphrase
          
          [default: 7]

  -s, --sep <SEPARATOR>
          Word separator. Can accept single quotes around the separator.
          
          There are special values that will trigger generated separators:
          
          _n: separators will be random numbers
          
          _s: separators will be random symbols
          
          _b: separators will be both random numbers and symbols
          
          [default: -]

  -l, --list <LIST_CHOICE>
          Choose a word list to use. Options:
          
          m -> Orchard Street Medium List (7,776 words)
          
          l -> Orchard Street Long List (17,576 words)
          
          q -> Orchard Street QWERTY list (1,296 words). Optimized to minimize travel distance on QWERTY keyboard layouts.
          
          a -> Orchard Street Alpha list (1,296 words). Optimized to minimize travel distance on alphabetical keyboard layouts.

  -t, --title-case
          Use Title Case for words in generated usernames

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Installation

### Using Rust and cargo (suggested method)
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/phraze --branch main` (Run this same command to upgrade Phraze.)

Uninstall Phraze by running `cargo uninstall phraze`.

### Releases on GitHub
You can also check for [recent releases on GitHub](https://github.com/sts10/phraze/releases).

## Word lists used

Phraze uses the [Orchard Street Lists](https://github.com/sts10/orchard-street-wordlists). These word lists are licensed separately. See that project's repository for licensing information concerning the word lists. All of the Orchard Street Wordlists lists are uniquely decodable, so they are safe to use without a separator between words.

By default, Phraze uses the Orchard Street Medium list, which has 7,776 words. That means each word adds 12.93 bits of entropy to a passphrase.

However you can use other word lists with the `-l`/`--list` option.

### Orchard Street Long list
Passing `l` to the list flag causes Phraze to use the Orchard Street Long List. This list has 17,576 words on it. This means that each word adds 14.1 bits of entropy to a passphrase. Thus, a 7-word passphrase has 98.7 bits of entropy.

### Orchard Street QWERTY list
Passing `q` to the list flag causes Phraze to use the Orchard Street QWERTY list that is optimized for QWERTY keyboard layouts. This list only has 1,296 words on it. This means that each word adds 10.3 bits of entropy to a passphrase.

Use this option if you're going to input this passphrase into a TV or video game console with a keyboard layout that looks like this:

```txt
qwertyuiop
asdfghjkl
zxcvbnm
```

### Orchard Street Alpha list
Passing `a` to the list flag causes Phraze to use the Orchard Street Alpha list that is optimized for alphabetical keyboard layouts. This list only has 1,296 words on it. This means that each word adds 10.3 bits of entropy to a passphrase.

Use this option if you're going to input this passphrase into a TV or video game console with a keyboard layout that looks like this:

```txt
abcdef
ghijkl
mnopqr
stuvwx
yz
```

## Source of randomness

Phraze uses the [rand crate](https://github.com/rust-random/rand), specifically the [SliceRandom's `choose` method](https://docs.rs/rand/latest/rand/seq/trait.SliceRandom.html#tymethod.choose).

## Why another random passphrase generator?

There are already a few good passphrase generators, including [passphraseme](https://github.com/micahflee/passphraseme) and [pgen](https://github.com/ctsrc/Pgen). 

Admittedly, I created Phraze in part to highlight my [Orchard Street Wordlists](https://github.com/sts10/orchard-street-wordlists). However I also wanted a Rust option that was simple and easy to read.

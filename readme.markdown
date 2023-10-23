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

Use `-t` to make all the words in passphrase Title Case
```
$ phraze -s '' -t
ReinforcedMetalProphecyHeavyArtilleryEnoughStaying
```

If you need to have a symbol, a number and an uppercase character in your passphrase, you can try
```
$ phraze -t -s _b
Segments6Lining{Cubs,Elementary8Exchanges0Fourteen7Slide
```

Use `-l` to specify which word list to use. For example, `-l l` uses the Orchard Street Long list.
```
$ phraze -l l
prices-leisurely-monument-shame-taller-troupe-compulsory
```

Copy generated passphrase to xclip (Linux clipboard) (Passphrase won't be printed to terminal):
```
$ phraze | xclip -selection clipboard
```

## Usage
```text
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
          
          m: Orchard Street Medium List (7,776 words)
          
          l: Orchard Street Long List (17,576 words)
          
          e: EFF long list (7,776 words)
          
          s: EFF short list (1,296 words)
          
          q: Orchard Street QWERTY list (1,296 words). Optimized to minimize travel distance on QWERTY keyboard layouts.
          
          a: Orchard Street Alpha list (1,296 words). Optimized to minimize travel distance on an alphabetical keyboard layout

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

<!-- Phraze uses the [Orchard Street Lists](https://github.com/sts10/orchard-street-wordlists). These word lists are licensed separately. See that project's repository for licensing information concerning the word lists. All of the Orchard Street Wordlists lists are uniquely decodable, so they are safe to use without a separator between words. -->

By default, Phraze uses the "Medium" list from the [Orchard Street Wordlists](https://github.com/sts10/orchard-street-wordlists), which has 7,776 words. That means each word adds 12.93 bits of entropy to a passphrase.

However, other lists are available for use. you can select a different word list by using the `-l`/`--list` option. All of these lists are uniquely decodable, which means they're safe to use without a separator between words.

* Orchard Street Medium list: 7,776 words; 12.93 bits of entropy per word. This is the **DEFAULT** list Phraze will use if no list is selected.

* Orchard Street Long list: 17,576 words; 14.1 bits of entropy per word. Use `l`.
* [EFF long list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases): 7,776 words; 12.93 bits of entropy per word. Use `e`.
* [Mnemonicode](https://github.com/schollz/mnemonicode) list: 1,633 words; 10.67 bits of entropy per word. Words are easy to pronounce out loud. Use `n`.
* [EFF short list 1](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases): 1,296 words; 10.3 bits of entropy per word. Use `s`.
* Orchard Street QWERTY list: 1,296 words; 10.3 bits of entropy per word. Use `q`. 
* Orchard Street Alpha list: 1,296 words; 10.3 bits of entropy per word. Use `a`.

### Notes on the Orchard Street QWERTY and Alpha lists
These two lists are optimized to minimize travel distance when inputting passphrases into TVs or video game consoles. They both have 1,296 words (10.3 bits per word).

The Orchard Street QWERTY list that is optimized for QWERTY keyboard layouts. Use this list if your keyboard layout looks like this:

```txt
qwertyuiop
asdfghjkl
zxcvbnm
```

The Orchard Street Alpha list that is optimized for alphabetical keyboard layouts. Use this list if your keyboard layout looks like this:

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

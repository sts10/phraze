# Phraze

Generate random passphrases.

## How to use Phraze, with examples

Run without arguments, Phraze will generate a 7-word passphrase that has 90.4 bits of entropy.
```
$ phraze
listeners-strikeouts-duchess-shrine-platform-advise-fellowship
```

If you want to change the strength of the passphrase, I recommend that you use `-e` to specify the minimum amount of entropy, in bits, that your passphrase must have. If not specified, Phraze will give 80 bits or more of entropy.
```
$ phraze -e 85
rugby-legally-sweeping-economist-thirty-achieving-sliding
```

If you want to specify the number words INSTEAD of minimum entropy, you can use `-w` to specify the number of words for Phraze to use. Cannot be used with `-e`/minimum entropy option.
```
$ phraze -w 5
routines-factory-threats-exact-destroyer
```

By default, Phraze separates words with a hyphen ("-"). You can change that with `-s`. Can accept special inputs `_n` (random numbers), `_s` (random symbols), and `_b` (mix of both). Note that separator choice does _not_ effect entropy calculations.
```
$ phraze -s ' '
marrow responded beauty syndrome compromise cognitive prompted
$ phrase -s _s
marble]outward{party_shuttle*killing\softball}spider
```

You can make all the word Title Case by using `-t`:
```
$ phraze -s '' -t
ReinforcedMetalProphecyHeavyArtilleryEnoughStaying
```

If you need to have a symbol, a number and an uppercase character in your passphrase, you can try:
```
$ phraze -t -s _b
Segments6Lining{Cubs,Elementary8Exchanges0Fourteen7Slide
```

Use `-l` to specify which word list to use. For example, `-l l` uses the Orchard Street Long list. (Note that we need only 6 words from this list to meet the default minimum entropy of 80 bits.)
```
$ phraze -l l
bundles-gross-whatsoever-precepts-standardized-household
```

You can pipe Phraze's outputted passphrase to other tools For example, you can copy generated passphrase to xclip (Linux clipboard):
```
$ phraze | xclip -selection clipboard
```

## Usage
```text
Usage: phraze [OPTIONS]

Options:
  -e, --minimum_entropy <MINIMUM_ENTROPY>
          Set minimum amount of entropy for generated passphrase. If neither minimum_entropy
          or number_of_words is specified, Phraze will default to an 80-bit minimum

  -w, --words <NUMBER_OF_WORDS>
          Set how many words in generated passphrase. If neither number_of_words or
          minimum_entropy is specified, Phraze will default to an 80-bit minimum

  -s, --sep <SEPARATOR>
          Word separator. Can accept single quotes around the separator.

          There are special values that will trigger generated separators:

          _n: separators will be random numbers

          _s: separators will be random symbols

          _b: separators will be a mix of random numbers and symbols

          [default: -]

  -l, --list <LIST_CHOICE>
          Choose a word list to use.

          Options:

          m: Orchard Street Medium List (7,776 words) [DEFAULT]

          l: Orchard Street Long List (17,576 words)

          e: EFF long list (7,776 words)

          n: Mnemonicode list (1,633 words). Good if you know you're going to be speaking
             passphrases out loud.

          s: EFF short list (1,296 words)

          q: Orchard Street QWERTY list (1,296 words). Optimized to minimize travel distance
             on QWERTY keyboard layouts.

          a: Orchard Street Alpha list (1,296 words). Optimized to minimize travel distance on
             an alphabetical keyboard layout

          [default: m]

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

By default, Phraze uses a word list from the [Orchard Street Wordlists](https://github.com/sts10/orchard-street-wordlists), specifically the
["Medium" list](https://github.com/sts10/orchard-street-wordlists/blob/main/lists/orchard-street-medium.txt), which has 7,776 words. That means each word adds 12.93 bits of entropy to a passphrase.

However, other lists are available to Phraze users. You can select a different word list by using the `-l`/`--list` option. All of these lists are uniquely decodable, which means they're safe to use without a separator between words.

* Orchard Street Medium list: 7,776 words; 12.93 bits of entropy per word. This is the **DEFAULT** list Phraze will use if no list is specified by the user.

* Orchard Street Long list: 17,576 words; 14.1 bits of entropy per word. Use `l`.
* [EFF long list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases): 7,776 words; 12.93 bits of entropy per word. Use `e`.
* [Mnemonicode](https://github.com/singpolyma/mnemonicode) list: 1,633 words; 10.67 bits of entropy per word. Words are easy to pronounce out loud. Use `n`.
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

## Details about each word list

This list information was generated using [Word List Auditor](https://github.com/sts10/wla).

### Orchard Street Medium
```txt
Lines found               : 7776
Free of exact duplicates  : true
Free of fuzzy duplicates  : true
Free of blank lines       : true
Unique words found        : 7776
No start/end whitespace   : true
No non-ASCII characters   : true
Unicode normalized        : true
Free of prefix words      : false
Free of suffix words      : false
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (add)
Length of longest word    : 10 characters (worthwhile)
Mean word length          : 7.05 characters
Entropy per word          : 12.925 bits
Efficiency per character  : 1.832 bits
Assumed entropy per char  : 4.308 bits
Shortest edit distance    : 1
Mean edit distance        : 6.954
Longest shared prefix     : 9
Unique character prefix   : 10
```

### Orchard Street Long list
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
Free of suffix words      : false
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
```

### EFF long list
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
Free of suffix words      : false
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
```

### Mnemonicode list
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
Free of suffix words      : false
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
```

### EFF short list
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
Free of suffix words      : false
Uniquely decodable        : true
Above brute force line    : true
Length of shortest word   : 3 characters (aim)
Length of longest word    : 5 characters (zippy)
Mean word length          : 4.54 characters
Entropy per word          : 10.340 bits
Efficiency per character  : 2.277 bits
Assumed entropy per char  : 3.447 bits
Shortest edit distance    : 1
Mean edit distance        : 4.367
Longest shared prefix     : 4
Unique character prefix   : 5
```

### Orchard Street QWERTY list
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
Free of suffix words      : false
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
```

### Orchard Street Alpha list
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
Free of suffix words      : false
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
```


## Source of randomness

Phraze uses the [rand crate](https://github.com/rust-random/rand), specifically the [SliceRandom's `choose` method](https://docs.rs/rand/latest/rand/seq/trait.SliceRandom.html#tymethod.choose).

## Why another random passphrase generator?

There are already a few good passphrase generators, including [passphraseme](https://github.com/micahflee/passphraseme) and [pgen](https://github.com/ctsrc/Pgen).

Admittedly, I created Phraze in part to highlight my [Orchard Street Wordlists](https://github.com/sts10/orchard-street-wordlists). However I also wanted a Rust option that was simple and easy to read.

### Some nice features of Phraze

✅ Allows user to set a minimum entropy, freeing them from having to figure how many words from a given list they need to create a strong passphrase
✅ Fast: Takes less than 2 milliseconds to generate a passphrase
✅ Only uses uniquely decodable word lists, ensuring that passphrase entropy estimates are accurate, even if no separator is used
✅ Word lists are (hopefully) free of profane words
✅ Numbers, symbols, and capital letters can be used if a service requires that in a password (`-s _b -t` flags)
✅ Written in [Rust](https://www.rust-lang.org/)

## Word list Licensing

The Mnemonicode word list is [copyrighted](https://github.com/singpolyma/mnemonicode/blob/master/mn_wordlist.c) by Oren Tirosh <oren@hishome.net> under [the MIT License](https://mit-license.org/).

The word lists from the Electronic Frontier Foundation (EFF) are [distributed under the Creative Commons Attribution 3.0 License](https://www.eff.org/copyright).

The Orchard Street Wordlists are available under [the Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/).

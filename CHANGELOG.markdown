# v0.3.1

**New in this release:** Mostly small stuff I wanted to improve after the large changes in the v0.3.0 release.

* b87cd56 - make verbose flag a bit more verbose 
* 4e8710a - make word all lowercase before making it title case, in case word is all uppercase or similar
* 2ad1a62 - uses an enum for separator types
* Various tweaks to README

**Full Changelog**: https://github.com/sts10/phraze/compare/v0.3.0...v0.3.1

# v0.3.0

## What's Changed
* Adds option for user to use their own, "custom" word list in https://github.com/sts10/phraze/pull/14
* Add option to set the number of passphrases to generate in https://github.com/sts10/phraze/pull/12
* Make list in main function, simplifying generate_passphrase in https://github.com/sts10/phraze/pull/13

**Full Changelog**: https://github.com/sts10/phraze/compare/v0.2.0...v0.3.0

# v0.2.0

## What's Changed
* Adds a "strong count" setting by @sts10 in https://github.com/sts10/phraze/pull/11
* b98c7c9 - switches in new Orchard Street Medium List, which has 8,192 words rather than 7,776
* Improves README, highlighting features (with emoji!)
* 5e21159 - fixes `--minimum-entropy` flag to have a hyphen rather than a _
* General code clean-up.

**Full Changelog**: https://github.com/sts10/phraze/compare/v0.1.8...v0.2.0

# v0.1.8

## What's Changed
* Use a build script to improve performance https://github.com/sts10/phraze/pull/2
* Simplify build.rs with write macro by @wezm in https://github.com/sts10/phraze/pull/10
* Add Criterion benchmarking 1f2e3af
* Adds a test for reading in proper length 6d3ec59 

## New Contributors
* @wezm made their first contribution in https://github.com/sts10/phraze/pull/10

# v0.1.6

* Removes KeePassXC word list (for now) due to licensing concerns (see #5)
* Adds Mnemonicode word list
* Adds some notes on word list licensing to the README.

# v0.1.5

## Empty word bug fix
This release fixes what could be considered a bug that negatively affected security present in earlier versions/releases of this tool. 

Previously, the `make_list` function used `.split('\n')` to read in lines of the word lists. This caused Phraze to read in **a blank or empty line** at the end of each file, such that one word on every word list was `""`. This meant that a, say, 7-word passphrase could contain a "blank" word, dropping entropy down to that of a 6-word passphrase. Not good!

Now, we use [the smarter `lines()` method](https://doc.rust-lang.org/std/primitive.str.html#method.lines):
```rust
/// Read in the appropriate word list, give the desired list enum
fn make_list(list_to_use: List) -> Vec<&'static str> {
    match list_to_use {
        List::Medium => include_str!("../word-lists/orchard-street-medium.txt")
            .lines()
            .collect(),
        List::Long => include_str!("../word-lists/orchard-street-long.txt")
            .lines()
            .collect(),
        List::Qwerty => include_str!("../word-lists/orchard-street-qwerty.txt")
            .lines()
            .collect(),
        List::Alpha => include_str!("../word-lists/orchard-street-alpha.txt")
            .lines()
            .collect(),
        List::Eff => include_str!("../word-lists/eff-long.txt").lines().collect(),
    }
}
```

This seems to solve the issue, however I'll keep it in mind going forward. 

## Other recent changes
* Adds option to add _random_ separators of numbers, symbols, or both between words, thanks to @jc00ke's PR: #3 ! 
* Ensures that random number function can return a 9 by using an inclusive `gen_range()`
* Includes [EFF long list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) as a list option, the first list that is not an Orchard Street Wordlist. Note that the EFF long list is free of prefix words, and thus uniquely decodable and therefore safe to generate passphrases without a separator between words. 
* Makes list parsing error handling slightly more graceful
# v0.3.24
* Upgrade rand dependency to latest version (v0.10.0)

# v0.3.24
Fresh release using the latest version of cargo-dist (v0.28.2).

* 1dbad74 - update allowed licenses -- I'm not too concerned about 'Unicode' licenses... 
* e156195 - dist init with latest version of cargo-dist, preparing for a new release 
* da2a6a2 - updates CHANGELOG with last release's release notes 
* 09ae148 - cargo lock 

# v0.3.23
Since [the cargo-dist project seems to have paused development due to lack of funding](https://github.com/axodotdev/cargo-dist/issues/1807#issuecomment-2768419143), we're doing this release the old-fashioned way. 

If you **need an OS-specific binary**, [v0.3.18](https://github.com/sts10/phraze/releases/tag/v0.3.18) is not very different from this version (see changelog below).

**CHANGES**
* 395b94b - cargo update and bump version  
* 7ba1108 - capitalizes the word 'list' consistently 
* 210f16c - takes a suggestion from clippy 
* c685b7a - couple word swaps in the built-in word lists
* 62fe7a9 - update EFF copyright information

I'm also hoping this simpler release, with fewer files attached to it, will work better with a Homebrew cask (see #30 for more). 


# v0.3.20
* Some word swaps in two of the built-in word lists

# v0.3.19

* Upgrade to [Rust edition 2024](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html) (mostly minor code reformatting).

# v0.3.18

* 49cb652 - upgrades rand crate dependency to v0.9.0, which required some statement changes and some new method names. See [#29](https://github.com/sts10/phraze/pull/29).
* ab5ed13 - updates two of the built-in word lists, removing the awkward word 'hank' 
* 6312ad8 - Uses `?` for conciseness in file reader, at suggestion of new version of Clippy 
* 45758ab - makes a clarification in licensing section of README

# v0.3.17

* Attempts to use cargo-dist to create a musl binary (see issue: #28)

# v0.3.15

* Upgrade some dependencies like clap, as well as the version of cargo-dist used to create releases. Sorry it's not more exciting!

# v0.3.14

* Upgrade clap dependency to version 4.5.13.

# v0.3.13

* Add man pages and shell completions (#27), thanks to @donovanglover.
* Updates version of cargo-dist

# v0.3.12

* Adds installation instructions for NixOS/nix (#25). Thanks for @x123!
* A few more word swaps in the Orchard Street word lists (9702125, 986d4d3, 9cfa359)
## Other, smaller README changes/fixes
* 0391f7d - adds a new badge to top of README showing number of downloads from crates.io, now that it's approaching 2,000 (woohoo!)
* 92574c6 - fix example Bash script n README for copying outputted passphrase to clipboard on Linux 
* 8d14563 - add note about doing work on the 'develop' git branch between releases to the readme

# v0.3.11

Nothing huge in this small update.
* Remove abbreviation "comm" from all included Orchard Street Wordlists.
* Fix some grammatical inconsistencies and a typo in help text.

# v0.3.10

* A few word swaps in various Orchard Street Wordlists, removing words like "sire" and "peter". See cbf70a0 and 5bc34d3.
* Use latest version of cargo-dist crate (0.17). 

# v0.3.8

* Updates Orchard Street Alpha wordlist (swaps "berg" for "baby")
* Upgrades `clap` and `unicode-normalization` dependency versions
* Uses cargo-dist v0.15.0 to create binaries for releases 
* Various of tweaks to README and documentation

# v0.3.6

* Pull latest versions of Orchard Street Word Lists
* Use cargo-dist v 0.8.0 to cut release

# v0.3.5 

* Some improvements to help text and Readme.

# v0.3.4

* Re-organizes README
* Clarifies README section about the PRNG method Phraze uses
* Adds some module-level documentation comments.
* Uses cargo-dist v0.5.0 to create releases, including this one.

# v0.3.3

Big thanks to @westonal for some nice code improvements in this release!

* Unify the types of built-in and custom word lists; by @westonal  #[16](https://github.com/sts10/phraze/pull/16)
* Use include_lines! macro rather than code generation; by @westonal [#17](https://github.com/sts10/phraze/pull/17)
* First release to use [cargo-dist](https://opensource.axo.dev/cargo-dist/). 
* Adds some more metadata to Cargo.toml 3bbeee6bfba42b9c65250c2c960ba25835c828f9 

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

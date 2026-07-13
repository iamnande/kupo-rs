# kupo-rs

> a small offering to the great moogle archives, kupo!

`kupo` is a tiny Rust CLI that remembers where I last left my SD cards.

it's a little storage companion for managing removable artifacts - especially
camera SD cards and the precious photos & video they might carry.

the dream:

1. insert an SD card
2. let `kupo` discover it
3. open the stash
4. safely copy the treasures within
5. return the card to its quiet slumber

## commands

open a stash:

```sh
kupo stash open
```

close a stash:

```sh
kupo stash close
```

`open` is our ritual of bringing a stash into the realm.

`close` is our ritual of returning our stash safely to its liminal plane once
the treasures have been collected.

## uhh.. what's a stash?

a `stash` is a removable volume containing artifacts (photos, videos, etc.)
gathered during adventures, kupo!

today, `kupo` is focused on building the linux storage foundations needed for
reliable discovery and management of these volumes.

## our quest

`kupo` is still quite young, just a wee bebe.

the current journey includes:

- discovering removable devices
- identifying known SD cards
- managing mounts safely
- replacing shell commands with linux APIs
- automatically backing up camera artifacts

## why?

> important memories deserve a moogle, kupo!

plugging in cards, mounting them, copying files, and remembering which ones have
already been backed up is a quest better handled by a tiny helper creature.

## joining the expedition

built with rust, linux storage APIs, and a healthy dose of curiosity.

`kupo` is currently a personal project and an exploration of linux storage
primitives.

if you enjoy tinkering with rust, linux, filesystems, or tiny automation
helpers - you're welcome to come along, kupo!

# task-hookrs

`task-hookrs` is a rust library for writing
[taskwarrior](https://taskwarrior.org) hooks.

* [crate](https://crates.io/crates/task-hookrs/)
* [Documentation](https://matthiasbeyer.github.io/task-hookrs/task_hookrs/index.html)
* [travis-ci](https://travis-ci.org/matthiasbeyer/task-hookrs)

[![Build Status](https://travis-ci.org/matthiasbeyer/task-hookrs.svg?branch=master)](https://travis-ci.org/matthiasbeyer/task-hookrs)
[![GitHub tag](https://img.shields.io/github/tag/matthiasbeyer/task-hookrs.svg?maxAge=2592000)]()
[![Crates.io](https://img.shields.io/crates/v/task-hookrs.svg?maxAge=2592000)]()
[![Crates.io](https://img.shields.io/crates/d/task-hookrs.svg?maxAge=2592000)]()
[![Crates.io](https://img.shields.io/crates/dv/task-hookrs.svg?maxAge=2592000)]()
[![Crates.io](https://img.shields.io/crates/l/task-hookrs.svg?maxAge=2592000)]()

`task-hookrs` is able to import and export the JSON taskwarrior understands, so
you can write taskwarrior hooks without having to deal with the JSON
import/export foo by simply using this crate.

# Vacation notice

**Notice:** I, the author of task-hookrs, will be on vacation from mid-May 2018
until early 2019. I hope I can continue develop task-hookrs during that time, but I
cannot guarantee that. I hope I can continue development of task-hookrs after that
and I certainly plan to do so.

With task-hookrs, the situation is a bit different as with my other
repositories. @maralorn has commit access and is actively developing this
library (as of the time of writing this note). I'm sure they will respond much
faster than I can - and I trust them to do the right thing.

@TheNeikos as commit rights to this repository and will respond to issues and
PRs.

# License

    task-hookrs - A Rust library for writing taskwarrior hooks and general interfacing with taskwarrior
    Copyright (C) 2016-2018 Matthias Beyer

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

For more information and the full license text, see
[the LICENSE file](./LICENSE).

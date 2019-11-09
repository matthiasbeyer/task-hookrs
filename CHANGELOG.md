# Changelog

This changelog was started with the 0.4.0 release.

## Next

## 0.7.0

* Bugfix: "imask" field in taskwarrior is a float, change type to f64.
  This is a breaking API change, thus the minor version is bumped.
* CI setup was fixed to actually fail when an error happens during
  compilation/testing. We had luck that it worked before without issue.
* All rustc versions from 1.32 until beta are tested now via travis, for maximum
  backwards compat

## 0.6.0

* Switched error handling to `failure`
* Added TaskBuilder
* Added support for user defined attributes (short UDA) via the task-hookrs::uda module.
  (This reintroduced the dependency to the "log" crate.)

## 0.5.0

* The "uuid" dependency was updated from 0.5 to 0.6

## 0.4.0

* Dependencies updated
* Unused "log" dependency removed
* Interface changed: The provided types are only data containers with
  read/write functionality, no aggregate functionality anymore.
  This mainly means that `Task::add_annotation()` and
  `Task::add_annotations()` were removed, but all other data-accessor
  functions got a mutable variant.

## 0.3.0

* Dependencies updated
* Update serde so we do not need to de/ser on our own anymore
* Added examples
* Test task importing in travis-ci

## 0.2.2

* Dependencies updated

## 0.2.1

* Relicensed to MPL2.0

## 0.2.0

* Annotation support
* Documentation
* Dead code removed

## 0.1.0

* Initial version

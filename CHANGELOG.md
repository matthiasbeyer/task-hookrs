# Changelog

This changelog was started with the 0.4.0 release.

## Next

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

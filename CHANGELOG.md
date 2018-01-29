# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* [sv-bindings]
  * Added documentation (36b47a4)
  * Added search_path_from_plusargs() (36b47a4)
* [rvs-parser] - Added support for variables that begin with an uppercase
  letter (2525d67)
* [development]
    * Added code coverage collection (dd2b968, 47ef6fa)
    * Applied rustfmt (8461a9c)

### Fixed

* [rvs-c-api] - Fixed calling rvs_parse with filenames that contain the
  string 'import' (e0f35fa)

### Changed

* [rvs-parser]
  * Grammar
    * Removed optional leading underscore from hexadecimal literals
      (5d0d443)
    * Removed optional trailing comma for ranges (9b8370b)
    * Improved grammar errors (7bcd9d6)
  * AST
    * Renamed WeightedSample to Weighted (cb0f710)
    * Replace VariableInst, EnumItemInst, and EnumItem with RIdentifier
    * (2525d67)[

* [rvs]
  * Renamed WeightedSample to Weighted (cb0f710)
  * Changed Sequence arguments to be re-evaluated on every cycle (ef1fabb)
  * Change Sequence syntax
    * From: `Sequence(count)`, `Sequence(offset, count)`,
      `Sequence(offset, increment, count)`
    * To: `Sequence(last)`, `Sequence(first, last)`,
      `Sequence(first, last, increment)`

## [0.2.0] - 2017-06-20

First release.

[Unreleased]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.2.0...HEAD

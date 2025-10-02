# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.0

### Added

- Ordinal time unit parsing with patterns like "1st day of next month", "last day of the year", "15th hour of tomorrow"
- Support for spelled ordinals ("first", "second", "third", etc.) in addition to numeric ordinals ("1st", "2nd", "3rd")
- "The" time unit references like "second day of the year", "third week of the month"
- Configuration option to choose first day of the week (Sunday or Monday) via `from_human_time_with_config()`
- Support for spelled numbers in durations ("two years ago", "three months ago")
- Complex month+duration patterns ("april 2 years from now", "december 3 years ago")
- Comprehensive spelled number support (one through hundred)

## [0.3.1]

### Changed

- Improved documentation of the main function

### Fixed

- Fixed README code example

## [0.3.0]

### Changed

- The library no longer uses the local time zone and instead uses naive times.
  Handling of time zones is left up to the consumer of the library.
- Internal: Input text is not being parsed into a custom AST before being
  processed. This should make it easier to reason about how the code works.

## [0.2.0]

## Added

- Allow the pattern like 7 days ago at today, which is parsed to [Ago] [AtLiteral] [HumanTime].
- Impl Display for ParseResult to allow directly println without match

## Fixed

- Fixed issue with 'months ago' parsing

## [0.1.2]

### Added

- Implement RelativeSpecifier ~ Week ~ Weekday
- Add 'Overmorrow'

### Changed

- Fix panic on invalid iso date

## [0.1.1]

### Added

- Allow return of naive dates

## [0.1.0]

Initial release

[0.3.1]: https://github.com/technologicalMayhem/human-date-parser/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/technologicalMayhem/human-date-parser/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/technologicalMayhem/human-date-parser/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/technologicalMayhem/human-date-parser/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/technologicalMayhem/human-date-parser/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/technologicalMayhem/human-date-parser/releases/tag/v0.1.0

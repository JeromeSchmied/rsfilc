# [RsFilc](https://codeberg.org/jark/rsfilc): `Kréta` client in [Rust](https://rust-lang.org)
> `Kréta` is an awful hungarian electronic school administration system

## Installation

-   [Rust](https://rustup.rs)
-   `cargo install --locked --git "https://codeberg.org/jark/rsfilc"`

## Usage

`rsfilc --help`

## API

-   [x] Kréta API URL fetching (no clue what they do)
-   [x] school fetching from [ReFilc API](https://api.refilc.hu/v1/public/school-list)

-   [x] user info fetch (json)

    -   [x] token
    -   [x] basic information
    -   [x] evaluations/grades
    -   [x] timetable
    -   [x] messages
    -   [x] announced test
    -   [x] absences

-   [x] usable user info (in `structs`)

    -   [x] token
    -   [x] basic information
    -   [x] evaluations/grades
    -   [x] timetable
    -   [x] messages
    -   [x] announced test
    -   [x] absences

## CLI

-   [x] basic usage of API
-   [x] filtering what to show
-   [x] multi-user feature
-   [x] somehow rendering `html` that messages return
-   [ ] caching everything so that life remains enjoyable

## TUI

-   [ ] multiple pages

    -   [ ] evaluations
    -   [ ] timetable
    -   [ ] ...

-   [ ] beautifully displaying data
    -   [ ] timetable in nice table
    -   [ ] plotting evaluations
    -   [ ] ...

## Acknowledgements

-   The API is written according to [this great documentation](https://github.com/bczsalba/ekreta-docs-v3).
-   Code ideas from [ReFilc](https://github.com/refilc/naplo).

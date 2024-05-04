# [RsFilc](https://codeberg.org/jark/rsfilc): [`E-Kréta`](https://www.e-kreta.hu/) console client in [Rust](https://rust-lang.org)

> `E-Kréta` is an awful Hungarian electronic school administration system

> [Magyar leírás](README.hu.md)

[![dependency status](https://deps.rs/repo/codeberg/jark/rsfilc/status.svg)](https://deps.rs/repo/codeberg/jark/rsfilc)

> # IMPORTANT!
>
> When upgrading from `v0.5.21`, credentials have to be recreated.
> It's necessary, as from `v0.5.22`, base64 encoding is used for storing passwords.
>
> 1. You have to manually find and delete them. A user called Alice would find `credentials` under:
>     - linux: `/home/alice/.config/rsfilc/credentials.toml`
>     - windows: `C:\Users\Alice\AppData\Roaming\rsfilc\credentials.toml`
>     - mac: `/Users/Alice/Library/Application Support/rsfilc/credentials.toml`
> 2. recreate all users with `rsfilc user --create`

## Installation

-   [Rust](https://rustup.rs)
-   `cargo install --locked rsfilc`

    > for latest, beta builds: `cargo install --locked --git "https://codeberg.org/jark/rsfilc"`

### Dependencies

#### optional, recommended

-   [w3m](https://w3m.sourceforge.net/): for enjoyable (html) message previews
-   [lynx](https://lynx.browser.org/): for enjoyable (html) message previews

### Shell completions:

   <details>
   <summary>Bash</summary>

> Add this to the <ins>**end**</ins> of your config file (usually `~/.bashrc`):
>
> ```sh
> eval "$(rsfilc completions bash)"
> ```

   </details>

   <details>
   <summary>Zsh</summary>

> Add this to the <ins>**end**</ins> of your config file (usually `~/.zshrc`):
>
> ```sh
> eval "$(rsfilc completions zsh)"
> ```
>
> For completions to work, the above line must be added _after_ `compcompletions` is
> called. You may have to rebuild your completions cache by running
> `rm ~/.zcompdump*; compcompletions`.

   </details>

   <details>
   <summary>Fish</summary>

> Add this to the <ins>**end**</ins> of your config file (usually `~/.config/fish/config.fish`):
>
> ```fish
> rsfilc completions fish | source
> ```

   </details>

   <details>
   <summary>PowerShell</summary>

> Add this to the <ins>**end**</ins> of your config file (find it by running `echo $profile` in PowerShell):
>
> ```powershell
> Invoke-Expression (& { (rsfilc completions powershell | Out-String) })
> ```

   </details>

   <details>
   <summary>Elvish</summary>

> Add this to the <ins>**end**</ins> of your config file (usually `~/.elvish/rc.elv`):
>
> ```sh
> eval (rsfilc completions elvish | slurp)
> ```
>
> **Note**
> RsFilc only supports elvish v0.18.0 and above.

   </details>

## Usage

`rsfilc --help`

## Features

### cross-platform: not tested thoroughly but should run on

-   linux
-   windows
-   macOS
-   android via Termux
-   everything else that Rust supports

## API

-   [x] Kréta API URL fetching (no clue what they do)
-   [x] school fetching from [reFilc API](https://api.refilc.hu/v1/public/school-list)

-   [x] user info fetch (json)

    -   [x] token
    -   [x] basic information
    -   [x] evaluations/grades
    -   [x] timetable
    -   [x] messages
        -   [x] attachments
    -   [x] announced test
    -   [x] absences

-   [x] usable user info (in `structs`)

    -   [x] token
    -   [x] basic information
    -   [x] evaluations/grades
    -   [x] timetable
    -   [x] messages
        -   [x] attachments
    -   [x] announced test
    -   [x] absences

## CLI

-   [x] basic usage of API
-   [x] filtering what to show
-   [x] multi-user feature
-   [x] somehow rendering `html` that messages return
-   [x] render `html` messages with `w3m` or `lynx` if possible
-   [x] shell completions: [bash, zsh, fish, elvish, powershell]
-   [x] logger: `fern` maybe
-   [ ] helpful crashes
-   [ ] caching everything so that life remains enjoyable
    -   [ ] token
    -   [ ] timetable
    -   [ ] evals
    -   [ ] user info
    -   [ ] absences
    -   [ ] announced tests
    -   [ ] messages
-   [x] encoding passwords
-   [ ] changing passwords

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
-   Code ideas from [reFilc](https://github.com/refilc/naplo).
-   Shell completions section got from [zoxide](https://github.com/ajeetdsouza/zoxide)

# [RsFilc](https://codeberg.org/jark/rsfilc): `Kréta` client in [Rust](https://rust-lang.org)

> `Kréta` is an awful hungarian electronic school administration system

> [Magyar leírás](README.hu.md)

## Installation

-   [Rust](https://rustup.rs)
-   `cargo install --locked --git "https://codeberg.org/jark/rsfilc"`

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
-   [ ] helpful crashes
-   [ ] render `html` messages with `w3m` or `lynx` if possible
-   [x] shell completions: [bash, zsh, fish, elvish, powershell]
-   [ ] caching everything so that life remains enjoyable
-   [ ] encoding passwords
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
-   Code ideas from [ReFilc](https://github.com/refilc/naplo).
-   Shell completions section got from [zoxide](https://github.com/ajeetdsouza/zoxide)

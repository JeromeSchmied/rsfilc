# [RozsdásFilc](https://codeberg.org/jark/rsfilc): [E-Kréta](https://www.e-kreta.hu/) konzol kliens [Rust](https://rust-lang.org)ban

> [English README](README.md)

[![dependency status](https://deps.rs/repo/codeberg/jark/rsfilc/status.svg)](https://deps.rs/repo/codeberg/jark/rsfilc)

## Letöltés

-   [Rust](https://rustup.rs)
-   `cargo install --locked rsfilc`
    > legújabb, béta: `cargo install --locked --git "https://codeberg.org/jark/rsfilc"`

### autókitöltés:

   <details>
   <summary>Bash</summary>

> Add a `~/.bashrc` <ins>**végére**</ins> :
>
> ```sh
> eval "$(rsfilc completions bash)"
> ```

   </details>

   <details>
   <summary>Zsh</summary>

> Add a `~/.zshrc` <ins>**végére**</ins> :
>
> ```sh
> eval "$(rsfilc completions zsh)"
> ```

   </details>

   <details>
   <summary>Fish</summary>

> Add a `~/.config/fish/config.fish` <ins>**végére**</ins>:
>
> ```fish
> rsfilc completions fish | source
> ```

   </details>

   <details>
   <summary>PowerShell</summary>

> Add a <ins>**végére**</ins> a beállításaidnak (így találod `echo $profile` PowerShell-ben):
>
> ```powershell
> Invoke-Expression (& { (rsfilc completions powershell | Out-String) })
> ```

   </details>

   <details>
   <summary>Elvish</summary>

> Add a `~/.elvish/rc.elv` <ins>**végére**</ins>:
>
> ```sh
> eval (rsfilc completions elvish | slurp)
> ```

   </details>

## Használat

`rsfilc --help`

![demo](./rsfilc_demo.gif "using rsfilc")

## Finomságok

### nem rendszerhez kötött: nincs különösebben letesztelve, de elvileg fut

-   linuxon
-   windowson
-   macOSen
-   androidon Termuxon
-   mindenen amit támogat a Rust

## CLI

-   [x] API alapvető használata
-   [x] kért adatok alapvető megjelenítése
-   [x] több fiókos rendszer
-   [x] üzenetek (`html`) elfogadható megjelenítése
-   [x] üzenetek (`html`) megjelenítése `w3m`-mel vagy `lynx`-el ha lehetséges
-   [x] shell autókitöltések: [bash, zsh, fish, elvish, powershell]
-   [x] hibajelentések: esetleg `fern`
-   [ ] segítőkész hibaüüzenetek
-   [x] kb minden cache-elése a valódi élmény érdekében
    -   [x] token
    -   [x] órarend
    -   [x] jegyek
    -   [x] felhasználó adatai
    -   [x] hiányzások
    -   [x] bejelentett számonkérések
    -   [x] üzenetek
-   [x] jelszavak titkosítása mentéshez
-   [ ] üzenetek küldése
-   [ ] osztályátlagok
-   [ ] ügyintézések lekérése
-   [ ] ügyintézések indítása
-   [ ] ...

## TUI

-   [ ] külön oldalak

    -   [ ] jegyek
    -   [ ] órarend
    -   [ ] ...

-   [ ] adatok szép megjelenítése
    -   [ ] órarendnek táblázat
    -   [ ] pl jegyeknek diagram
    -   [ ] ...

## Elismerések

Minden használatba vett `crate`-nek köszönet, [itt](./Cargo.toml) találtatnak.
Tessék egy pillantást vetni az [ekreta-rs](https://codeberg.org/jark/ekreta-rs)-re, mely a használt API kliens az E-Krétahoz
Autókitöltés leírás innen: [zoxide](https://github.com/ajeetdsouza/zoxide)

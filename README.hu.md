# [RozsdásFilc](https://codeberg.org/jark/rsfilc): [E-Kréta](https://www.e-kreta.hu/) konzol kliens [Rust](https://rust-lang.org)ban

> [English README](README.md)

[![dependency status](https://deps.rs/repo/codeberg/jark/rsfilc/status.svg)](https://deps.rs/repo/codeberg/jark/rsfilc)

> # FONTOS!
>
> Ha `v0.5.21`-ről frissítéshez újra kell csinálni a bejelentkezéseket.
> Ez azért szükséges, mivel a `v0.5.22`-től kezdve titkosítással vannak tárolva a kódok.
>
> 1. Ki kell törölni a régi bejelentkezéseket. Alice ezeket erre találná meg:
>     - linux: `/home/alice/.config/rsfilc/credentials.toml`
>     - windows: `C:\Users\Alice\AppData\Roaming\rsfilc\credentials.toml`
>     - mac: `/Users/Alice/Library/Application Support/rsfilc/credentials.toml`
> 2. Hozd létre újból a bejelentkezéseket a `rsfilc user --create` használatával.

## Letöltés

-   [Rust](https://rustup.rs)
-   `cargo install --locked rsfilc`
    > legújabb, béta: `cargo install --locked --git "https://codeberg.org/jark/rsfilc"`

### segédprogramok

#### opcionális, ajánlott

-   [lynx](https://lynx.browser.org/): az élvezhetőbb üzenet-megjelenítés érdekében
-   [w3m](https://w3m.sourceforge.net/): az élvezhetőbb üzenet-megjelenítés érdekében

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

## API

-   [x] Kréta API Url-ek lekérése (nem tudom mire jók)
-   [x] iskolák lekérése [reFilc API](https://api.refilc.hu/v1/public/school-list)-ból

-   [x] felhasználó adatainak lekérése (json)

    -   [x] token
    -   [x] általános információk
    -   [x] jegyek
    -   [x] órarend
    -   [x] üzenetek
        -   [x] csatolmányok
    -   [x] előre bejelentett számonkérések
    -   [x] hiányzások

-   [x] felhasználó adatainak használhatóvá tétele (struktúrák)

    -   [x] token
    -   [x] általános információk
    -   [x] jegyek
    -   [x] órarend
    -   [x] üzenetek
        -   [x] csatolmányok
    -   [x] előre bejelentett számonkérések
    -   [x] hiányzások

## CLI

-   [x] API alapvető használata
-   [x] kért adatok alapvető megjelenítése
-   [x] több fiókos rendszer
-   [x] üzenetek (`html`) elfogadható megjelenítése
-   [x] üzenetek (`html`) megjelenítése `w3m`-mel vagy `lynx`-el ha lehetséges
-   [x] shell autókitöltések: [bash, zsh, fish, elvish, powershell]
-   [x] hibajelentések: esetleg `fern`
-   [ ] segítőkész hibaüüzenetek
-   [ ] kb minden cache-elése a valódi élmény érdekében
    -   [ ] token
    -   [ ] órarend
    -   [ ] jegyek
    -   [ ] felhasználó adatai
    -   [ ] hiányzások
    -   [ ] bejelentett számonkérések
    -   [ ] üzenetek
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

Az API alapvetően [ez alapján a dokumentáció](https://github.com/bczsalba/ekreta-docs-v3) alapján valósult meg.
Kódinspiráció a [reFilc](https://github.com/refilc/naplo)ből.
Autókitöltés leírás innen: [zoxide](https://github.com/ajeetdsouza/zoxide)

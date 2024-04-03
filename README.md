# [RsFilc](https://codeberg.org/jark/rsfilc): Kréta kliens [Rust](https://rust-lang.org)ban

## Letöltés

-   [Rust](https://rustup.rs)
-   `cargo install --locked --git "https://codeberg.org/jark/rsfilc"`

## Használat

`rsfilc --help`

## API

-   [x] Kréta API Url-ek lekérése (nem tudom mire jók)
-   [x] iskolák lekérése [ReFilc API](https://api.refilc.hu/v1/public/school-list)-ból

-   [x] felhasználó adatainak lekérése (json)

    -   [x] token
    -   [x] általános információk
    -   [x] jegyek
    -   [x] órarend
    -   [x] üzenetek
    -   [x] előre bejelentett számonkérések
    -   [x] hiányzások

-   [ ] felhasználó adatainak használhatóvá tétele (struktúrák)

    -   [ ] token
    -   [x] általános információk
    -   [x] jegyek
    -   [x] órarend
    -   [ ] üzenetek
    -   [ ] előre bejelentett számonkérések
    -   [ ] hiányzások

## CLI

-   [ ] API alapvető használata
-   [ ] kért adatok alapvető megjelenítése
-   [x] több fiókos rendszer
-   [ ] kb minden cache-elése a valódi élmény érdekében
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
Kódinspiráció a [ReFilc](https://github.com/refilc/naplo)ből.

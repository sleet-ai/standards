# RULES

general coding rules

- perfer cargo add rather than editing the `Cargo.toml` directly
- run `bunx prettier . --write` and `cargo fmt` to keep code clean, if this touches files that where not a part of the current work. commit these chages separtly, but never undo these chages.
- please run typecheck or `cargo check` after adding code or a feature, and if it is a web app i am usually running myself so you do not have to run it. i usually like to see that it builds, so if there is a fast script that does something run it, if it is a long running script timeout it so you do not just watch it stream data forever.

=====================
<br/>
copyright 2026 by sleet.near

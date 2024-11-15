# b15r

Dies ist ein Crate für das Programmieren des B15F-Boards mit Rust. Es gibt auf [GitLab](git.imn.htwk-leipzig.de/hwlab-public) auch andere Crates.
Im Gegensatz zum [b15py](www.github.com/mmeiler-dev/b15py) braucht das Crate nicht die Header files des B15F. 

## Installation

(bisher so, cargo wird noch gemacht)
```
git clone https://www.github.com/mmeiler-dev/b15r
```

in der `Cargo.toml`

```toml
[dependencies]
b15r = { path = "/relative/path/to/the/cloned/repo/" }
```

## Benutzung

```rust
use b15r::b15f::B15F;

fn main() {
  let mut drv = B15F::get_instance(); // Erstellt die Verbindung zum B15F
}
```

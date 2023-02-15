# obs: an obsidian cli

Connecting your second brain to the terminal

## Usage

1. git clone & do `cargo install --path .`
2. put this in your `.zshrc` (or equivalent)

```bash
obs() {
    local result=$(command obs "$@")
    [ -n "$result" ] && cd -- "$result"
}
```

3. `obs`

## TODO

- [ ] add `obsidian-git` support
- [ ] add clap and `-h`
- [ ] improve this readme

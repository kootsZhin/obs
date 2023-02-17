<h1 id="readme-title" align="center">
    <img src="./assets/obsidian.png" width="200"/>
    <br>
    obs - the Obsidian CLI
</h1>

<h4 id="readme-description" align="center">
    ⚡️ Connecting your second brain to the termainl - blazing fast ⚡️
</h4>

## Note 🚧

- `obs` is under active development and currently only support MacOS, please open up an issue if there's any error while using the tool

## Usage

- `obs` : open up a menu for choosing actions and vault to interact with
- `obs --goto <GOTO>` : `cd` to the directory of vault `<GOTO>`
- `obs --open <OPEN>` : open the vault `<OPEN>` in Obsidian
- `obs --help` : show help

## Getting Started

1. Clone this repo
2. Install `obs`

```bash
cargo install --path .
```

3. Put this in your `.zshrc` (or equivalent)

```bash
obs() {
    local result=$(command obs "$@")
    [ -n "$result" ] && cd -- "$result"
}
```

4. `obs`

## Roadmap

- [x] add clap and `-h`
- [x] imporve `README`
- [ ] add `obsidian-git` support
- [ ] improve error handling

## License

[MIT @ 2023](LICENSE): Distributed under the MIT License.
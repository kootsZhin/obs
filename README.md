<h1 id="readme-title" align="center">
    <img src="./assets/obsidian.png" width="200"/>
    <br>
    obs - the Obsidian CLI
</h1>

<h4 id="readme-description" align="center">
    ⚡️ Connecting your second brain to the termainl - blazing fast ⚡️
</h4>

## Note 🚧

- `obs` is under active development and currently only support MacOS, please report any issue while using!

## Features

- Fast & easy access to vaults from terminal in seconds
- Backup your vault to remote git effortlessly
- Flat learning curve without the need to memorize complciated commands
- Automatically fetch vault list from Obsidian, no extra config needed

<p align="center">
  <img src="assets/demo-1.gif" alt="animated" />
</p>

## Usage

- `obs` : open up a menu for choosing actions and vault to interact with
  - `goto` : goto vault
  - `open` : open vault
  - `backup` : backup vault
- `obs --goto <GOTO>` : `cd` to the directory of vault `<GOTO>`
- `obs --open <OPEN>` : open the vault `<OPEN>` in Obsidian
- `obs --help` : show help

<p align="center">
  <img src="assets/demo-2.gif" alt="animated" />
</p>

## Getting Started

1. Install `obs`

```bash
cargo install obs
```

2. Put this in your `.zshrc` (or equivalent)

```bash
obs() {
    local result=$(command obs "$@")
    [ -n "$result" ] && cd -- "$result"
}
```

3. Start using: `obs`!

## License

[MIT @ 2023](LICENSE): Distributed under the MIT License.

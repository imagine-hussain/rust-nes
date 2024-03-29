# rust-nes

![GitHub repo size](https://img.shields.io/github/repo-size/imagine-hussain/rust-nes)
![Lines of code](https://img.shields.io/tokei/lines/github/imagine-hussain/rust-nes)
![Build](https://img.shields.io/github/actions/workflow/status/imagine-hussain/rust-nes/build.yml)
![Clippy](https://img.shields.io/github/actions/workflow/status/imagine-hussain/rust-nes/build.yml?label=clippy)


A **BLAZINGLY FAST** rust emulator for the NES.

## Install / Usage

1. Clone the repository and navigate to it

```bash
git clone https://github.com/imagine-hussain/rust-nes.git
cd rust-nes
```

2. Compile using `cargo`. If you don't have cargo, install it [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```bash
cargo build --release
```

3. Run.
```bash
cargo run
```

4. Use the `Open` button in the toolbar to bring up a file-explorer and select the rom you wish to load. A git-ignored `/roms` folder has been provided for you, to have a convenient place to store your ROMS.

<img width="538" alt="image" src="https://user-images.githubusercontent.com/93496985/218387295-bd5b08e6-ff58-43ef-9f86-703fb208a7e8.png">



## Status

Non-functional WIP.

### TODO

- [x] Create instruction decoding
- [x] Implement all instructions
- [x] Decode and open `.iNes` files accurately
- [x] Implement Cpu Cycling
- [x] Implement Ppu
- [x] Get a frontend working that renders *something*
- [x] Add a file explorer and allow roms to be loaded in through the GUI.

- [ ] Mappers. Aim is to have the 10 most common mappers complete, to allow
most games to work. 2127 / 2418 = 88%.
    - [ ] Mapper 001 | 677 (28%)
    - [ ] Mapper 004 | 587 (24.28%)
    - [ ] Mapper 002 | 267 (11.04%)
    - [x] Mapper 000 | 247 (10.22%)
    - [ ] Mapper 003 | 155 (6.41%)
    - [ ] Mapper 007 | 75  (3.1%)
    - [ ] Mapper 206 | 44  (1.82%)
    - [ ] Mapper 011 | 31  (1.28%)
    - [ ] Mapper 005 | 24  (0.99%)
    - [ ] Mapper 009 | 20  (0.83%)

- [ ] APU
- [ ] PPU - **CURRENT WORK IN PROGRESS**

- Additional Emulator Features
    - [x] Pause
    - [x] Speed-up / Slow-down Execution
    - [ ] Pause / Inspect
    - [ ] Save Game-State
    - [ ] Scaling
    - [ ] Crt Shader

- [x] Add GitHub CI/CD to ensure builds on Windows are succesful.


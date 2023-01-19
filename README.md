# rust-nes

A **BLAZINGLY FAST** rust emulator for the NES.

## Status

Non-functional WIP.

### TODO

- [x] Create instruction decoding
- [x] Implement all instructions
- [x] Decode and open `.iNes` files accurately
- [x] Implement Cpu
- [x] Implement Ppu
- [x] Get a frontend working that renders *something*

- [ ] Sound
- [ ] Mappers. Aim is to have the 10 most common mappers complete, to allow
most games to work. 2127 / 2418 = 88%.
    - [ ] Mapper 001 | 677 (28%)
    - [ ] Mapper 004 | 587 (24.28%)
    - [ ] Mapper 002 | 267 (11.04%)
    - [ ] Mapper 000 | 247 (10.22%)
    - [ ] Mapper 003 | 155( 6.41%)
    - [ ] Mapper 007 | 75 (3.1%)
    - [ ] Mapper 206 | 44 (1.82%)
    - [ ] Mapper 011 | 31 (1.28%)
    - [ ] Mapper 005 | 24 (0.99%)
    - [ ] Mapper 009 | 20 (0.83%)


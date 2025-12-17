# Argus

**Argus** is an open-source satellite mission control system focused on CCSDS-compliant telemetry and telecommand processing.

This project aims to provide a modern and extensible ground-segment platform for small satellite missions, research projects, and simulation-driven development.

## Goals

- Decode and visualize CCSDS telemetry packets
- Build and route telecommands from a SCOS-2000 MIB
- Support ECSS/PUS
- Enable procedure-driven operations and testing with an embedded scripting engine (either Lua or Python)
- Provide replay features
- Offer a clean desktop UI built with modern tooling
- Link-layer protocols (CLTU, COP-1)
- Mission planning and scheduling

## Architecture

Argus is structured as a modular system with a native backend and a desktop UI.

- **Backend:** Rust (core protocols, mission database, TM/TC services)
- **UI:** Tauri + TypeScript (React)
- **Scripting:** Embedded procedure language (TBD, considering Lua or Python)
- **Transport:** Pluggable adapters (UDP, file replays, simulators)

## Current Status

This project is in the early development phase.

Planned initial milestones:
- [ ] CCSDS Space Packet parsing
- [ ] Mission database schema and validator
- [ ] Telemetry ingest and decode pipeline
- [ ] Basic telemetry viewer UI

## Development

### Prerequisites
- Rust (stable)
- Node.js (LTS)

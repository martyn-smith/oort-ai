Oort.rs candidate
===

Oort is a "programming game" where you write Rust code to control a fleet of spaceships. Your code is responsible for the engines, weapons, radar, and communications of ships ranging from tiny missiles to massive cruisers.

This repository contains WIP code for ship logic, currently including PID tracking, with ultimate aims to include frequency-hopping comms, modular codebase, nature-inspired random search algorithms (accounting for frictionless travel), etc.

Notes
---

  - Currently, oort.rs has no automatic upload based testing, so this is really a dumb template for code, and won't (currently) be expected to build. This could be changed in future.
  - For similar reasons, there's a lot of code duplication.
  - As a small papercut, every player-controlled entity in Oort is a Ship; even the missiles.
    This means the tick() method needs to enumerate Ship type and dispatch, rather than proper type-based behaviour.
    (Although you CAN define the members of the Ship struct and its constructor).
    Similarly library functions for the ship are indeed functions, not methods as one might expect.
  - PID tracking for guns is based on turn() rather than torque().

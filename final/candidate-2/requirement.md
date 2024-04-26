Duck Database
===

* [Duck Database](#)
* [Problem Statement](#problem-statement)
* [Requirements](#requirements)
   * [Core Requirements](#core-requirements)
   * [High Level Requirements](#high-level-requirements)
   * [Micro Requirements](#micro-requirements)
* [Output](#output)
   * [Design Document](#design-document)
   * [Prototype](#prototype)
   * [Tech Stack](#tech-stack)
* [Outcome](#outcome)

# Problem Statement

Duck Database is an reimplementation of NoSQL with similar design with LevelDB. My main job in Duck Database would be write an atomic key-value store.

# Requirements

*The problem statement is something to start with, be creative and dive into the product details and add constraints and features you think would be important.*

## Core Requirements

- `memtable` with ability of atomic insertion(atomic in the context of insertion, not in the of lock free)
- `lsm-block` to merge and sort entries

## High Level Requirements

 - atomic `INSERTION`, foreground merging of `LSM-Tree`
 - provide interface to java using `JNI`

## Micro Requirements

 - lockfree 
 - write unit test to **speed up** development.

# Output

## Design Document

Create a **design document** that draft the boundary of primary component.

Create a **development journal** which document the development process.

Do **not** write unnecessary components(`class`, `interface`) in design document, design document **should** serve the purpose of speeding up development. The main training in this class to is speed up large software development.

## Tech Stack

This is the tech-stack of the original `MDOJ judger`

|Which|Options|
|-----|-----|
|Language|Rust, Java|

# Outcome

**TO BE CONTINUE!**
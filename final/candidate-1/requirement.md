MDOJ sandbox
===

* [MDOJ sandbox](#)
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

Rewrite a secure judger server with optional functionality of streaming output. A judger server is defined as a server that receive user-submitted code, then compile/execute, finally compare the result of execution.

# Requirements

*The problem statement is something to start with, be creative and dive into the product details and add constraints and features you think would be important.*

## Core Requirements

 - secure(permission control)
 - output resource limit with reasonable precision
 - never cause out of memory error

## High Level Requirements

 - follow proto [here](https://github.com/mdcpp/mdoj/blob/master/proto/judger.proto)

## Micro Requirements

 - support at least `lua` and `c` language.
 - add additional language support by `plugin`, hopefully packed in a tarball. 

# Output

## Design Document

Create a **design document** that draft the boundary of primary component.

Create a **development journal** which document the development process.

Do **not** write unnecessary components(`class`, `interface`) in design document, design document **should** serve the purpose of speeding up development. The main training in this class to is speed up large software development.

## Tech Stack

This is the tech-stack of the original `MDOJ judger`

|Which|Options|
|-----|-----|
|Language|Rust, C, Lua|
|GRpc|tonic, tower|
|Coroutine|tokio|
|Logging|env_logger, log|
|permission control|nsjail|
|resource control|cgroups_rs|
|low level interface|libc, rustix|

# Outcome

**TO BE CONTINUE!**
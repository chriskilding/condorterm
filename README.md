# condorterm

Terminal UI (TUI) for the Condor soaring simulator.

## Overview

`condorterm` displays the UDP output from Condor as a set of virtual steam gauges within your terminal.

This may be helpful in the following scenarios:

- **Debugging** (e.g. to check your connection to Condor from a remote machine)
- **Development** (e.g. to compare the behaviour of your custom Condor client code with `condorterm`)

You can also use it while **flying** in Condor, to display an aircraft's instrument panel on a second monitor. (The terminal offers only limited graphics, so other instrument panel solutions are normally better. However, `condorterm` may be useful if you do not have access to the alternatives.)

## Setup

You can install `condorterm` from source like this:

```shell
cargo install
```

You can alternatively run `cargo build` and move the executables from the `target` directory to a place of your choice.

## Usage

Enable UDP data transmission within Condor's configuration (`UDP.ini` and `Simkits.ini`).

TODO Socat / Condor bridge if needed:

```shell
socat ...
```

Run Condor and start a new flight.

Then open `condorterm` and point it at Condor's UDP socket:

```shell
condorterm <host> -p [port]
```

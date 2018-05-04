# PuluRobot-Robot
## Introduction
This repository contains a few tools and a interface for on-robot deployment.
The interface should at one point be able to act as a middle-man between the
on-robot host, and the server. Communication with the robot is currently
at a working stage.

Besides from the interface, this repository contains
 - a setup tool, for simple on-robot configuration
 - a command-line tool, for demonstration and testing purposes of the host
   interface.

Everything is made in Rust, which besides from being memory safe and fast,
also offers an easy way to cross-compile to the Raspberry-Pi.

## Installation

 1. Install the Rust programming language
 2. Clone repository using:
    ```bash
    git clone https://github.com/TheSoftwareFactory/pulurobot-robot.git
    ```
 3. Enter directory `pulurobot-robot`
    ```bash
    cd pulurobot-robot
    ```
 4. Build package, which will also fetch any dependencies
    ```bash
    cargo build
    ```

# Setup Tool
## Run
To start the setup tool run:

```bash
cargo run --bin setup
```

This will create a configuration file if none was found, for setting up the
connection to the robot.

**Note:** It is necessary to be connected to the same network as the robot.

# Console Client
## Run
To start the command-line client run:

```bash
cargo run --bin console
```

A promt will appear, and the following commands should be available:

## Functionality

##### `quit`
Terminates the program

##### `help`
Prints the help message containing available commands

##### `listen`
Streams information broadcasted by the robot (Still needs work). Press `Enter` to stop the stream.

##### `free`
Will unlock the wheels of the robot, to be able to freely move it around

##### `stop`
Will tell the robot to stop whatever it is currently doing

##### `save [a|b]`
Saves robots current coordinates as location A or B

##### `goto [a|b]`
Will try to route to location A or B respectively


# Todo

 - Improve `listen` with more commands, e.g. the `140` (Robot state) code.
 - Communication with server

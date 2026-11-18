# Rust Radar Simulator

A lightweight radar simulation and tracking system written in Rust, built to explore asynchronous programming, concurrent systems, and simulation architecture.

The project models a virtual environment containing moving targets, observed by multiple independently operating radar sensors. These sensors periodically generate observations and communicate them to a central tracking station for processing.

The primary goal is to develop practical experience with Rust, particularly its asynchronous programming model, through a small but extensible engineering project.

## Disclaimer

The current version of this README is agent generated off of a rough idea. It may not be totally accurate.

## Overview

The simulator consists of several interconnected components:

- **World:** A shared simulation environment containing moving targets.
- **Targets:** Simulated objects with positions, velocities, and movement behaviours.
- **Radar Sensors:** Independent sensors that periodically observe targets within the simulated environment.
- **Tracking Station:** A central receiver responsible for collecting, organising, and processing sensor observations.
- **Application:** Coordinates initialisation, asynchronous execution, and shutdown.

Each radar operates independently, potentially with different scan intervals, detection ranges, and measurement characteristics.

Sensor observations are transmitted asynchronously to the tracking station, allowing multiple sensors to operate without waiting for one another.

## Architecture

The initial architecture follows a simple data flow:

```text
                  Application
                       |
                       v
                Simulated World
                /      |      \
           Target A Target B Target C
                       |
           +-----------+-----------+
           |           |           |
           v           v           v
        Radar A     Radar B     Radar C
           |           |           |
           +-----------+-----------+
                       |
                       v
                MPSC Channel
                       |
                       v
                Tracking Station
                       |
                       v
                Output / Display
```

The world provides a common environment for all sensors. Each radar independently samples the world and produces observations, which may include measurement noise or other sensor-specific imperfections.

The tracking station receives these observations and maintains the information needed to monitor detected targets.

The architecture is expected to evolve as new simulation behaviours and tracking capabilities are introduced.

## Technology

- **Language:** Rust
- **Async runtime:** Tokio
- **Communication:** Tokio MPSC channels
- **Simulation:** Custom 2D target and sensor models
- **Interface:** Initially terminal-based

The project aims to minimise dependencies where practical, implementing core simulation and tracking logic directly in Rust.

## Planned Features

### Initial functionality

- Generation of multiple simulated targets
- Basic target motion using position and velocity
- Multiple radar sensors with configurable properties
- Independent asynchronous radar scanning
- Simulated measurement noise
- Asynchronous observation transmission
- Centralised reception and management of observations
- Basic terminal output and monitoring
- Application lifecycle and graceful shutdown

### Potential extensions

- More complex target movement and manoeuvres
- Target creation and removal during simulation
- Missed detections and false positives
- Sensor bias and varying measurement accuracy
- Communication delays and out-of-order observations
- Target association across multiple sensors
- Position estimation and tracking filters
- Real-time terminal visualisation
- Configurable simulation scenarios

These are potential directions rather than fixed commitments.

## Learning Objectives

The project provides opportunities to explore:

- Rust ownership, borrowing, and type design
- Asynchronous functions and futures
- Tokio tasks and cooperative scheduling
- Message passing and communication channels
- Concurrent task coordination
- Error handling and graceful shutdown
- Simulation modelling
- Modular software architecture

A particular emphasis is placed on understanding where asynchronous programming is beneficial and how to separate asynchronous coordination from ordinary synchronous computation.

## Project Philosophy

This is primarily a hands-on learning project.

The implementation is intended to be developed incrementally, with an emphasis on understanding the underlying design and writing the core code manually.

Simplicity, clarity, and learning value take precedence over feature count or simulation fidelity.

The project architecture and feature set are intentionally flexible. Design decisions may change as implementation progresses and new ideas are explored.

## Project Status

**Status:** Early development / initial implementation

The initial focus is on establishing the world model, generating targets, implementing independent radar tasks, and building the asynchronous observation pipeline.

## Scope

This project is not intended to accurately reproduce real-world radar physics, signal processing, or operational radar systems.

Its radar and tracking concepts are simplified abstractions used to explore simulation, asynchronous communication, and software engineering.

More sophisticated modelling may be introduced later where it supports the project's objectives.


# Table of Contents

1.  [Introduction](#org5a01fb2)
2.  [Architecture Overview](#org3c41804):ATTACH:
3.  [Features](#org7f69091)
4.  [Setup Instructions](#org49a18d5)
    1.  [Prerequisites](#orgfae071d)
    2.  [Installation](#org2ea4de7)
    3.  [Configuration](#org998d854)
5.  [Usage](#orgd6ef0e4)
6.  [Contributing](#orge08ef62)
7.  [License](#org1d9ebc8)
8.  [Acknowledgments](#orgf5dc5a2)



<a id="org5a01fb2"></a>

# Introduction

Meepo is an innovative IoT platform designed to enable seamless collaboration between edge devices and cloud services. The platform leverages modern protocols and microservice architectures to support high-concurrency, low-latency communication, making it ideal for industrial IoT applications such as edge computing and real-time data processing.


<a id="org3c41804"></a>

# Architecture Overview     :ATTACH:

The Meepo platform follows a distributed architecture to optimize edge-cloud collaboration:

-   ****Edge Layer****: Comprises IoT devices (e.g., industrial controllers) that collect and process data locally using efficient communication protocols.
-   ****Edge Gateway****: Acts as a bridge, handling data aggregation and forwarding to the cloud, with support for QUIC to ensure reliable, low-latency connections.
-   ****Cloud Layer****: Hosts microservices (e.g., API servers and data storage) to manage device orchestration, data analytics, and user interfaces.
-   ****Communication****: Utilizes QUIC for edge-to-cloud communication, ensuring high performance and connection migration.

For a visual representation, refer to the attached [architecture diagram](https://github.com/yushun1990/meepo/blob/master/resource/design/M-Overall-Archi.png).


<a id="org7f69091"></a>

# Features

-   Real-time data transmission between edge and cloud.
-   Scalable microservice deployment for independent expansion.
-   Support for high-concurrency device connections.
-   Integration with distributed storage for state management.


<a id="org49a18d5"></a>

# Setup Instructions


<a id="orgfae071d"></a>

## Prerequisites

-   Rust (latest stable version)
-   Docker for containerized services
-   Redis for state management


<a id="org2ea4de7"></a>

## Installation

1.  Clone the repository:
    
        git clone https://github.com/yourusername/meepo.git
        cd meepo

2.  Build the project:
    
        cargo build --release

3.  Start Redis:
    
        docker run -d -p 6379:6379 redis

4.  Run the edge and cloud services:
    -   Edge service: `cargo run --bin edge`
    -   Cloud API service: `cargo run --bin api`


<a id="org998d854"></a>

## Configuration

-   Edit `config.toml` to specify QUIC endpoints and Redis connection details.
-   Ensure the architecture diagram is placed in the project root as `architecture_diagram.png`.


<a id="orgd6ef0e4"></a>

# Usage

-   Access the API at `http://localhost:8080/api/edge_data/<device_id>` to query device data.
-   Send commands via `http://localhost:8080/api/edge_command` with a JSON payload.


<a id="orge08ef62"></a>

# Contributing

We welcome contributions to Meepo! To get started:

1.  Fork the repository and create a new branch.
2.  Implement your changes and ensure tests pass.
3.  Submit a Pull Request with a clear description of your changes.
4.  Follow the Rust coding conventions and document new features.


<a id="org1d9ebc8"></a>

# License

Meepo is released under the MIT License. See the `LICENSE` file for details.


<a id="orgf5dc5a2"></a>

# Acknowledgments

Inspired by the need for efficient edge-cloud collaboration in IoT systems.


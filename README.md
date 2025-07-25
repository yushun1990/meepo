
# Meepo - IoT platform written by rust.

Meepo is an innovative IoT platform designed to enable seamless collaboration between edge devices and cloud services. The platform leverages modern protocols and microservice architectures to support high-concurrency, low-latency communication, making it ideal for industrial IoT applications such as edge computing and real-time data processing.

---

## 🔧 Features

-   Real-time data transmission between edge and cloud.
-   Scalable microservice deployment for independent expansion.
-   Support for high-concurrency device connections.
-   Integration with distributed storage for state management.

---

## 📦 Architecture Overview

The Meepo platform follows a distributed architecture to optimize edge-cloud collaboration:

-   ****Edge Layer****: Comprises IoT devices (e.g., industrial controllers) that collect and process data locally using efficient communication protocols.
-   ****Edge Gateway****: Acts as a bridge, handling data aggregation and forwarding to the cloud, with support for QUIC to ensure reliable, low-latency connections.
-   ****Cloud Layer****: Hosts microservices (e.g., API servers and data storage) to manage device orchestration, data analytics, and user interfaces.
-   ****Communication****: Utilizes QUIC for edge-to-cloud communication, ensuring high performance and connection migration.

![architecture diagram](https://github.com/yushun1990/meepo/blob/master/resource/design/M-Overall-Archi.png)

---

## 🛠 Getting Started

### Prerequisites

-   Rust (latest stable version)
-   Docker for containerized services
-   Redis for state management

### Quick start

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

---

## 🚀 Usage

-   Access the API at `http://localhost:8080/api/edge_data/<device_id>` to query device data.
-   Send commands via `http://localhost:8080/api/edge_command` with a JSON payload.

---

## 🤝 Contributing

We welcome contributions to Meepo! To get started:

1.  Fork the repository and create a new branch.
2.  Implement your changes and ensure tests pass.
3.  Submit a Pull Request with a clear description of your changes.
4.  Follow the Rust coding conventions and document new features.

---


## 📄 License

Meepo is released under the MIT License. See the `LICENSE` file for details.

---

### Useful links

- [Axum](https://docs.rs/axum)
- [SQLx](https://docs.rs/sqlx)
- [Utoipa (OpenAPI)](https://docs.rs/utoipa)
- [Tokio](https://tokio.rs/)
- [Validator (crate)](https://docs.rs/validator)

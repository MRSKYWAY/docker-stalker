# Docker Monitoring Dashboard (Rust)

## Overview
Docker_stalker is a monitoring dashboard designed to keep track of Docker containers. Written in Rust, known for its performance and safety features, especially in managing concurrent operations, this project provides a reliable solution for monitoring Docker containers.

## Features
- **Dashboard Interface**: Presents an intuitive interface for monitoring Docker containers.
- **Container Information**: Displays detailed information about Docker containers, including their IDs, images, commands, statuses, ports, and names.
- **Image Details**: Provides insights into Docker images, including repository tags and sizes.
- **Error Handling**: Incorporates robust error handling to manage unexpected situations gracefully.

## Installation
1. Clone the repository:
    ```bash
    git clone https://github.com/MRSKYWAY/docker-stalker.git
    cd docker-stalker
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. Run the dashboard:
    ```bash
    cargo run
    ```

## Usage
You'll be presented with real-time information about your Docker containers.

## Code Overview
The main functionality of the project is implemented in `main.rs`, which utilizes the following Rust crates:
- `serde`: For serializing and deserializing JSON data.
- `reqwest`: For making HTTP requests to interact with Docker APIs.
- `tokio`: For asynchronous execution.

The code is structured to handle container and image information retrieval from Docker's RESTful API. Additionally, it includes unit tests to ensure the reliability of the codebase.

## Testing
The project includes unit tests to verify the functionality of critical components, ensuring that the dashboard behaves as expected under various scenarios. You can run the tests using the following command:
```bash
cargo test

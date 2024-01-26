# ![Rust Logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg) Whois Parallel Client

## Overview

Welcome to the Rust Whois Parallel Client, a powerful and efficient tool designed to perform Whois queries in parallel. This open-source project is written in Rust, leveraging the language's performance and reliability to deliver a fast and concurrent Whois client.

## Features

- **Parallelism**: Execute multiple Whois queries concurrently, speeding up the process of retrieving information from various Whois servers.
- **Customizable**: Easily configure the tool to suit your needs by specifying the number of parallel queries, timeout thresholds, and more.
- **User-friendly**: Simple command-line interface makes it easy for both beginners and experienced users to perform Whois queries effortlessly.
- **Rust Benefits**: Take advantage of Rust's memory safety, zero-cost abstractions, and fearless concurrency to ensure a robust and reliable tool.

## Getting Started

### Prerequisites

- Rust installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/defconhaya/whoisa.git
cd whoisa
cargo build --release
```

### Usage

Run the tool with a list of domains to perform parallel Whois queries:

```bash
./target/release/whoisa ip_list.txt
```

For more options and configuration settings, refer to the help command:

```bash
./target/release/whoisa --help
```

## Contribution Guidelines

We welcome contributions from the community! Whether it's bug reports, feature requests, or code contributions, please check our [contribution guidelines](CONTRIBUTING.md) to get started.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgments

- Thanks to the Rust community for their support and guidance.
- Special shoutout to contributors who have helped make this project better.

Happy querying!
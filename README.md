# ![Rust Logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg) Whois Parallel Client

## Overview

Welcome to the Rust Whois Parallel Client, a powerful and efficient tool designed to perform Whois queries in parallel. This open-source project is written in Rust, leveraging the language's performance and reliability to deliver a fast and concurrent Whois client.

## About

This project is my first venture into Rust development, and I'm excited to learn and contribute to the Rust ecosystem. Please bear in mind that as a newcomer, there might be room for improvement in both the codebase and documentation. Constructive feedback and contributions from the Rust community are highly encouraged!

I want to emphasize that I have limited available time to work on this project due to other commitments. Your understanding and patience are greatly appreciated. If you're interested in contributing or have suggestions, please refer to the [Contribution Guidelines](CONTRIBUTING.md).


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

## TODO

- ~~**Add Progress**: Implement a progress indicator to provide users with real-time feedback on the status of their parallel Whois queries.~~
  
- **Dynamic Capture Fields**: Enhance the tool to dynamically capture and display relevant fields from the Whois response, providing more comprehensive information.

- **Export Output to CSV**: Introduce functionality to export the Whois query results to a CSV file, allowing for easy analysis and integration with other tools.

- **Fix Regex Pattern for Parsing Whois Response**: Address any issues with the current regex pattern used for parsing Whois responses to ensure accurate and reliable information extraction.


## Contribution Guidelines

We welcome contributions from the community! Whether it's bug reports, feature requests, or code contributions, please check our [contribution guidelines](CONTRIBUTING.md) to get started.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgments

- Thanks to the Rust community for their support and guidance.
- Special shoutout to contributors who have helped make this project better.

Happy querying!
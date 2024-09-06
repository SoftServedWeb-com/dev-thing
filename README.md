# Local Node

Local Node is an open source LocalWP but for Node JS Applications. Effortlessly manage your Node JS applications with a simple and intuitive interface.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [Windows](#windows)
  - [macOS](#macos)
  - [Linux](#linux)
- [Running the Application](#running-the-application)
- [Contributing](#contributing)
  - [Setup for Contribution](#setup-for-contribution)
- [License](#license)

## Features

- Easy management of multiple Node.js projects
- One-click launch of Node.js applications
- Install dependencies
- Launch IDEs

## Prerequisites

- Node.js
- npm/pnpm

## Installation

You can download the binaries for your operating system from the [releases page](https://github.com/SoftServedWeb-com/local-node/releases). Follow the instructions below for your platform.

### Windows

1. Download the latest `.exe` file from the [releases page](https://github.com/SoftServedWeb-com/local-node/releases).
2. Run the `.exe` file and follow the installation prompts.
3. Ensure Node.js is installed on your system by running the following in the terminal:

    ```bash
    node -v
    npm -v
    ```

### macOS

1. Download the latest `.dmg` file from the [releases page](https://github.com/SoftServedWeb-com/local-node/releases).
2. Open the `.dmg` file and drag the application into the `Applications` folder.
3. Verify the installation by opening a terminal and running:

    ```bash
    node -v
    npm -v
    ```

### Linux

1. Download the latest `.tar.gz` file from the [releases page](https://github.com/SoftServedWeb-com/local-node/releases).
2. Extract the contents of the archive:

    ```bash
    tar -xvzf local-node-*.tar.gz
    cd local-node
    ```

3. Run the setup script:

    ```bash
    sudo ./install.sh
    ```

4. Check the installation by running:

    ```bash
    node -v
    npm -v
    ```


## Contributing

We welcome contributions to improve Local Node! To get started, please follow the steps below.

### Setup for Contribution

#### Prerequisites

- Node.js
- npm/pnpm
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

1. Fork the repository to your GitHub account.
2. Clone the forked repository to your local machine:

    ```bash
    git clone https://github.com/YOUR_USERNAME/local-node.git
    ```

3. Install the necessary dependencies:

    ```bash
    cd local-node
    pnpm install
    pnpm tauri dev
    ```

4. Create a new branch for your feature or fix:

    ```bash
    git checkout -b feature/your-feature-name
    ```

5. Make your changes and commit them:

    ```bash
    git add .
    git commit -m "Add your commit message"
    ```

6. Push your branch to GitHub:

    ```bash
    git push origin feature/your-feature-name
    ```

7. Open a pull request on the original repository, explaining your changes.


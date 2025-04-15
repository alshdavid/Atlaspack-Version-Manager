# Atlaspack Version Manager 📚

- Install Atlaspack versions
  - Git Branch Name (will compile, requires Rust, Yarn)
  - Git Commit Hash (will compile, requires Rust, Yarn)
  - Local Atlaspack sources
  - ~Super Package (todo maybe?)
- Link Version Into Current Project
- Run Commands with Specific Version of Atlaspack

## Installation

### npm (todo)

```bash
npm install -g @atlaspack/apvm
```

### MacOS

```bash
# Download and extract, add this to your PATH later
curl -L https://github.com/alshdavid/atlaspack-version-manager/releases/download/latest/apvm-macos-arm64.tar.xz | tar -xJvf - -C .
./apvm --help
```

### Linux

```bash
# Download and extract, add this to your PATH later
curl -L https://github.com/alshdavid/atlaspack-version-manager/releases/download/latest/apvm-linux-amd64.tar.xz | tar -xJvf - -C .
./apvm --help
```

## Usage

```bash
# Run this before the CLI will work
eval "$(./apvm env -s bash)"

# Install Atlaspack from "main" branch
apvm install git:main
apvm use git:main

# Proxy commands to active atlaspack
apvm run -- --version 

# Proxy common commands on the top level to active atlaspack 
apvm build
apvm watch

# Use your local Atlaspack sources
export APVM_LOCAL="$HOME/development/atlasian-labs/atlaspack"
apvm use local

# Link Atlaspack into current project
mkdir my-project && cd my-project
npm init -y
npm install @atlaspack/cli

apvm link git:main
apvm link local
```
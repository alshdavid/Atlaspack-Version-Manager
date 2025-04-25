# 📚 Atlaspack Version Manager 📚

Version manager and installer for Atlaspack. This project allows you to:

- Install Atlaspack versions
  - Npm version
  - Git Branch Name (will compile, requires Rust, Yarn)
  - Git Commit Hash (will compile, requires Rust, Yarn)
  - Local Atlaspack sources
- Link version into `node_modules`
- Run commands against specific version of Atlaspack

## Installation

```bash
# Install globally
npm install -g @atlaspack/apvm

# Install into current project
npm install --save-dev @atlaspack/apvm
yarn add -D @atlaspack/apvm
```

## Usage

### CLI

```bash
# Install a version of Atlaspack from npm
apvm install 2.14.0
apvm install 2.15.0

# Set the system default version of Atlaspack
apvm default 2.15.0

# Ignore the .apvmrc or package.json#atlaspack.version
# apvm override 2.14.0

# Link into node_modules the version specified in the project config or default
apvm npm link

# Link into node_modules (overriding project config)
apvm npm link 2.14.0
apvm npm link 2.15.0

# Link into node_modules with backwards compatibility for @atlaspack/* packages
apvm npm link --legacy 2.15.0

# Run Atlaspack commands using the current or default version
apvm atlaspack build
apvm atlaspack --version

# Also works directly with "atlaspack" command
atlaspack build
atlaspack --version

# Npm project flow
npm init -y
npm install @atlassian/apvm
  #> npx apvm npm postinstall

npx apvm atlaspack --version
npx atlaspack --version

# Npm helper commands
apvm npm scan     # Scans node_modules recursively for
                  # all instances of Atlaspack

apvm npm dedupe   # Traverses node_modules recursively
                  # and ensures only one version of
                  # Atlaspack is installed
```

### Config

Config can be specified in a `package.json` or `.apvmrc`
```json5
// package.json
{
  "atlaspack": {
    "version": "2.15.0"
  }
}
```
```yaml
# .apvmrc
2.15.0
```

### Install a version from git

Versions obtained from git will build after being fetched. This takes a while 🙏

```bash
apvm install git:main
apvm install git:my-branch
apvm install git:1fb73643c
```

### Register a locally installed git repo

```bash
# Register your local Atlaspack sources
apvm install local:/Users/username/atlaspack
```

## Installation (Binary)

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

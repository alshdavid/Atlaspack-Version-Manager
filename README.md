# 📚 Atlaspack Version Manager 📚

- Install Atlaspack versions
  - Git Branch Name (will compile, requires Rust, Yarn)
  - Git Commit Hash (will compile, requires Rust, Yarn)
  - Local Atlaspack sources
  - ~Super Package~ (todo maybe?)
- Link Version Into Current Project
- Run Commands with Specific Version of Atlaspack

## Installation

```bash
# Rerun this to update CLI
npm install -g @atlaspack/apvm
```

## Basic usage

### CLI

```bash
# Install Atlaspack from atlassian-labs/atlaspack
# Versions obtained from git will build first
apvm install -o git main          # main branch
apvm install -o git my-branch     # by branch name
apvm install -o git d874396       # by git hash

apvm reinstall -o git main        # redownload and rebuild main branch

apvm global -o git main           # Set the global version to be main

echo "console.log(42)" > index.js
apvm atlaspack build ./index.js   # Run a CLI command using active Atlaspack
```

### Link into node_modules

You can link the `apvm` managed Atlaspack version into `node_modules`. It does this by shimming the Atlaspack packages in node_modules to point to the `apvm` managed version.

```bash
apvm node_modules link -o git main  # Link a version into node_modules
                                    # Does not follow shell/global version
                                    # so this must be rerun on changes

apvm node_modules scan              # Scans node_modules recursively for
                                    # all instances of Atlaspack

apvm node_modules dedupe            # Traverses node_modules recursively
                                    # and ensures only one version of
                                    # Atlaspack is installed
```

### Config

Add an `.apvmrc` file to the root of the project

Pinned to a specific version:
_note: super package doesn't exist so this doesn't work yet_

```
2.13.2
```

Pinned to a specific git hash

```
origin = git
specifier = d874396
```

Uses a git branch. Warning this can change

```
origin = git
specifier = main
```

To use this version execute

```bash
# Install version specified in .apvmrc
apvm install

# Set the version to be the global version
apvm global

# Set the version to be the shell version (requires advanced setup)
apvm use

# Link the version specified in the config into node_modules
apvm node_modules link
```

## Advanced usage

### CLI

```bash
# Run this to enable per-shell/per-directory versions
eval "$(apvm env)"

# Install Atlaspack from atlassian-labs/atlaspack
# Versions obtained from git will build first
apvm install -o git main          # main branch
apvm install -o git my-branch     # branch name
apvm install -o git d874396       # git hash

apvm global -o git main           # Set the global version to be "main"
apvm use -o git my-branch         # Set the shell version to be "my-branch"

apvm atlaspack --version          # Run a CLI command using active Atlaspack
atlaspack --version               # Run a CLI command using active Atlaspack
```

### Use as a development aid

```bash
# Register your local Atlaspack sources
apvm install -o local "$HOME/atlaspack"

# Set the local copy to be the global default
apvm global -o local

# Or set the local copy to be used in the shell
apvm use -o local

# Run commands using local copy
apvm atlaspack --version
atlaspack --version

# Link into node_modules
apvm node_modules link -o local
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

## Notes

#### Building after fetching

Versions obtained from git will build after being fetched. This takes a while 🙏

Distributing a prebuilt "super" package is being discussed which can resolve the install times

#### Eval must be run first

Eval must be run before running any commands or it won't work

#### Bash and Zsh supported

Fish and Windows users beware

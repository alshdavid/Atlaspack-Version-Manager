# How This Works

A directory is created called `APVM_DIR` which defaults to `$HOME/.local/apvm` with the following structure

```
/apvm
  /global
    /bin
      atlaspack -> $(which apvm)
    /static -> ../versions/[kind]/[specifier]
  /sessions
    /[session_id]
      /bin
        atlaspack -> $(which apvm)
      /static -> ../../versions/[kind]/[specifier]
  /versions
   /git
     /[branch_name_base64]
   /local
     /[local_alias_base64]
   /npm
     /[version_number_base64]
```

### Sessions (optional)

This is an optional feature inspired by tools like rustup and nvm and allows configuration per shell, torn down when the shell is closed.

Sessions (and `[session_id]`) are created when calling `eval "$(apvm env)"`.

A `/sessions/[session_id]` folder is created when you call `apvm use [specifier]` the first time

The session folder is deleted when you close your shell.

Because of the split-package nature of Atlaspack, this is not currently supported when linking Atlaspack into `node_modules`.

### Muli-name binary

The `apvm` binary is a multi-name binary.

When called with the name `apvm` the `apvm` commands are available (like `install` `use` etc)

When called with the `atlaspack` name it will proxy to the `atlaspack` binary associated with it or fallback to the system default.

### Linking into node_modules

The `apvm node_modules link` command goes over the `@atlaspack/*` entries in the target Atlaspack version and symlinks them into the target project's `node_modules`.

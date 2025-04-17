# How This Works

A directory is created called `APVM_DIR` which defaults to `$HOME/.local/apvm` with the following structure

```
/apvm
  /default
    /bin
      atlaspack -> $(which apvm)
    /static -> ../versions[kind]/[specifier]
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
   /super
     /[version_number_base64]
```

### Sessions

Sessions (and `[session_id]`) are created when calling `eval "$(apvm env -b bash)"`.

A `/sessions/[session_id]` folder is created when you call `apvm use [specifier]` the first time

The session folder is deleted when you close your shell

### Muli-name binary

The `apvm` binary is a multi-name binary.

When called with the name `apvm` the `apvm` commands are available (like `link` `use` etc)

When called with the `atlaspack` name it will proxy to the `atlaspack` binary associated with it or fallback to the system default.

### Linking into node_modules

The `apvm link` command goes over the `@atlaspack/*` entries in `node_modules` and mutates them to enable the use of the dynamically set Atlaspack version.

It does this by mutating the `package.json` files of the target packages, replacing their `main` entry with `./apvm.cjs` which contains a check for the `APVM_PATH` variable and, if present, reexports the values from there otherwise it imports the existing code.

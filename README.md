# unjust

A tool for managing portable Justfiles across projects.

## Motivation

Unjust does for tasks what [mise](https://github.com/jdx/mise), does for tools.

Unjust allows you to store your Justfiles in a separate repo and use them across different projects without checking them into each project's VCS.
This is useful when:

- You want to use the same Justfile recipes across multiple projects
- You don't want to clutter project repos with your personal Justfiles
- You need to use Justfiles in repos where you don't have permission to commit

## Installation

Install with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) (recommended):

```sh
cargo binstall unjust
```

or `cargo install unjust` to build it yourself.

## Usage

```
unjust [command] [options]

Commands:
  use [--separate-upstream-justfile] [--force|-f] <repo>
      Use a Justfile from remote storage
      
  sync [--force-push] [repo]
      Sync Justfiles with remote storage
      
  init [-t template] [name]
      Initialize a new Justfile for the current repo
      
  list [--paths|-p]
      List available Justfiles
```

### Examples

Use a Justfile from a remote repo:
```
unjust use username/my-justfiles
```

If you're in a forked repo and want to use the upstream's Justfile:
```
unjust use
```

If you want to use your own Justfile even when in a forked repo:
```
unjust use --separate-upstream-justfile
```

Sync all your Justfiles with remote storage:
```
unjust sync
```

Initialize a new Justfile for the current repo:
```
unjust init
```

Use an existing Justfile as a template:
```
unjust init -t my-template
```

List all available Justfiles:
```
unjust list
```

## Project Ethos

The project is designed to have minimal dependencies and be smol and free of syn/std.

- Core uses `standard-paths` for paths
- CLI uses `facet-args` for argument parsing
- Standalone crates with clear responsibilities

## Development

Clone the repository and build with Cargo:

```
git clone https://github.com/yourusername/unjust.git
cd unjust
cargo build
```

Run tests, with just of course:

```
just test
```

## License

MIT or Apache-2.0, at your option.

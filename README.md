# keron

A [keron](https://stargate.fandom.com/wiki/Keron) an energy particle and part of the individual building blocks of [Replicators](https://stargate.fandom.com/wiki/Replicator) in the Stargate universe.

Keron is an opinionated dotfile manager which only does symlinks.

## Architecture

```mermaid
flowchart TD
  find_files(Find all *.keron recipes) --> prechecks(check if all required tools are installed)
  prechecks -- yes --> dry_run
  prechecks -- no --> warn(warn user that not all functions will be executed)
  warn --> dry_run
  dry_run -- yes --> confirmation
  dry_run -- no --> execute
  confirmation -- no --> exit
  confirmation -- yes --> execute
  execute -- successful --> exit
  execute -- failure --> error
```

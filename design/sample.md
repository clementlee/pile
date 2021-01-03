# Sample Commands and What They Do

## Config

### Add storage location

Modify `~/.pile/config.toml` to set storage locations.
Storage locations are globally shared.
Example config:

```toml
[[drive]]
name="bfd"
size="2T"
mountpoint="/Volumes/lol"
```

### Creating a new pile

```bash
pile new --name <name> [future config]
```

> Note to self: a pile is what a dataset used to be. Just create multiple piles.

Future config will include:

- replication policy (number of copies)
- split policy (minimum unit guaranteed to be stored in a storage location)

## Adding files

```bash
pile add --pile <name> --files <files>
```

This will:

1. analyze disk usage of the files being added.
2. check which storage locations are believed to have enough space
3. propose two storage locations to use, or ask for user input (sort candidates by reasonableness)
4. for each location:
    1. check if free space is sufficient. If not, go back to step 3.
    2. copy over files

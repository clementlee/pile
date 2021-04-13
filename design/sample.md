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

The size is an estimate of the total size of the drive (or however much you want to allocate to pile).
Pile will take that size in hard-drive size (powers of ten), and use up to 95% of the available space.

### Creating a new pile

```sh
pile add --name <name> --path <path> [future config]
```

This will:

1. analyze disk usage of the files being added.
2. check which storage locations are believed to have enough space
3. propose two (by replication policy) storage locations to use, or ask for user input (sort candidates by reasonableness)
4. for each location:
    1. check if free space is sufficient. If not, go back to step 3.
    2. copy over files

Also, you can add to existing piles

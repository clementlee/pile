# Sample Commands and What They Do

## Config

### Adding a storage location

```bash
pile storage add --name <name> --mountpoint <mountpoint>
```

Marks the mountpoint as a storage location.

### Adding a dataset

```bash
pile dataset add --name <name>
```

> _Note_: to start, the following limitations apply to datasets.
>
>- Replication policy of 2 copies per file
>- Datasets must fully fit within a single storage location
>

## File handling

### Adding some files

```bash
pile file add --dataset <dataset> [--storage <location>] --files <files>...
```

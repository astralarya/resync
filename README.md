# Resync
Rsync Extended.
Manage remotes programmatically in archival repositories.

Define archive sources and repos via configutation.
Navigate freely through an archive and
stop worrying about typing the right rsync command.
Simply update from all remote sources with:

```bash
resync pull
```

Push files to a configured remote:

```bash
resync push remote1 path/to/dir
```

Clone the archive including configuration:

```bash
resync clone user@remote:path/to/archive
```

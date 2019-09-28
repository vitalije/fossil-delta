# Leo version server

`leo-ver-serv` is a web server which accepts POST requests from Leo. Leo sends snapshots
of its current state, server calculates delta between the previous version of snapshot and
the current one, and stores delta in a database. Server also serves a small web application
which allows user to browse history of known Leo files.

## Installation

If you already have installed `cargo` and `rustc`, it is enaugh to execute the following
command:

```
cargo install leo-ver-serv
```

It requires one or two arguments. The first one is a file containing known Leo files, and
the second argument optionally is a port number on which server should listen.

```
leo-ver-serv ~/.leo/.leoRecentFiles.txt 8088
```

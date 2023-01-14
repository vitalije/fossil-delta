# A Delta algorithm ported to Rust


This algorithm is used in [fossil](https://fossil-scm.org) where it has been implemented in C-99.
This crate contains the same algorithm implemented in Rust.

It exports two functions: `delta(a:AsRef<[u8]>, b:AsRef<[u8]>) -> Vec<u8>` and `deltainv(b:AsRef<[u8]>, d:AsRef<[u8]>) -> Vec<8>`.

```
// creating delta between a and its next version b
let d = delta(a, b);

// applying delta to b to get previous version a
let s = deltainv(b, d);

assert_eq!(s, a);
```

Suppose we have some text value `a`, and user has changed it to value `b`. Using `delta`
function we can get a compressed delta value `d` that we can store and keep it along with
the new text value `b`. If later user wants to see the previous version of text,
we can use `deltainv(b, d)` to get the previous value `a`. If we keep all consequtive deltas
we can use `deltainv` multiple times to get any of the earlier text versions.

The delta value calculated between two utf-8 encoded might not be valid utf-8.

The algorithm is very well described
[here](https://fossil-scm.org/home/doc/trunk/www/delta_encoder_algorithm.wiki) in the fossil wiki.

The code of this repository is best viewed in [Leo editor](https://leoeditor.com). The outline
containing the code is in a single outline file: [fossil-delta-ref.leo](fossil-delta-ref.leo).

This repository contains two more Rust crates, `py-fossil-delta` which exports fossil-delta
functions to Python as an extension module.

The third crate is `leo-ver-serv` which is a binary (executable) web server which accepts
snapshots from Leo, calculates the delta form the previous snapshot and stores all deltas
in a database. On the other side, this server serves a small web application which allows
user to browse history of known Leo files.

Thanks to [Vincent Néel](https://github.com/pikanezi), functions `delta` and `deltainv` now accept both binary and text arguments.

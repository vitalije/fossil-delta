# A Delta algorithm ported to Rust


This algorithm is used in [fossil](https://fossil-scm.org) where it has been implemented in C-99.
This crate contains the same algorithm implemented in Rust.

It exports two functions: `delta(a:&str, b:&str) -> String` and `deltainv(b:&str, d:&str) -> String`.

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

The delta value calculated between two utf-8 encoded strings is itself utf-8 encoded string.

The algorithm is very well described
[here](https://fossil-scm.org/home/doc/trunk/www/delta_encoder_algorithm.wiki) in the fossil wiki.

The code of this repository is best viewed in [Leo editor](https://leoeditor.com). The outline
containing the code is in a single outline file: [fossil-delta-ref.leo](fossil-delta-ref.leo).

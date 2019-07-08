# A Delta algorithm ported to Rust

### Work In Progress !!!

This algorithm is used in [fossil](https://fossil-scm.org) and there it has been implemented in C-99.
I needed it in my Rust projects and here is reimplementation in Rust.

Once it is ported it will expose two public functions:
    - delta:  fn(a:&str, b:&str) -> String
    - deltainv:  fn(a:&str, d:&str) -> String

```
// creating delta between a and its next version b
let d = delta(a, b);

// applying delta to b to get previous version a
let s = deltainv(b, d);

assert_eq!(s, a);
```

Therefore if you have some text value `a`, and user changes that text to value `b`, using
`delta` function you can get a compressed delta value `d` that you can store and keep it
along with the new text value `b`. If later user wants to see the previous version of text,
you can use `deltainv(b, d)` to get the previous value. If you keep all consequtive deltas
you can use `deltainv` multiple times to get any of the earlier text versions.

The Delta value calculated between two utf-8 encoded strings is itself utf-8 encoded string.

The algorithm is described very well
[here](https://fossil-scm.org/home/doc/trunk/www/delta_encoder_algorithm.wiki) in the fossil wiki.

The code of this repository is best viewed in [Leo editor](https://leoeditor.com). The outline
containing the code is in a single outline file: [fossil-delta-ref.leo].

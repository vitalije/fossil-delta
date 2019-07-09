# fossil_delta a Python extension

This crate exports functions `fossil_delta::delta` and `fossil_delta::deltainv` to
Python interepreter. The resulting library built with:

```
cargo build --release
```
on Linux is named `libfossil_delta.so`. Before using in Python it has to be renamed
to `fossil_delta.so`. On Windows it should be probably renamed to `fossil_delta.pyd`
or `fossil_delta.dll`.

# Usage:

```
>>> import fossil_delta
>>> a = """line 1
... yet another (a bit longer) line 2
... yet another (a bit longer) line 3
... yet another (a bit longer) line 4
... yet another (a bit longer) line 5
... yet another (a bit longer) line 6
... yet another (a bit longer) line 7
... yet another (a bit longer) line 8
... yet another (a bit longer) line 9
... yet another (a bit longer) line 10"""
... 
>>> b = """line 1
... yet another (a bit longer) line 2
... yet another (a bit longer) line 3
... yet another (a bit longer) line 4
... yet another (a bit longer) line 5
... yet another (a bit longer) line 6
... yet another (a bit longer) line 7
... yet another (a bit longer) line 8
... and finally last line 10
... yet another (a bit longer) line 9
... yet another (a bit longer) line 10"""
...
>>> d = fossil_delta.delta(a, b)
>>> s = fossil_delta.deltainv(b, d)
>>> s == a
True
>>>
```

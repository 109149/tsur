Tsur (tsur)
------------
Tsur is a character counter tool.

[![Current Version](http://meritbadge.herokuapp.com/tsur)](https://crates.io/crates/tsur) [![Documentation](https://docs.rs/tsur/badge.svg)][docs]

[docs]: https://docs.rs/tsur/

## Examples

```bash
# count all ascii characters in text.txt
# and output in (default) ascending order, by value
tsur -f text.txt count -c

# count all ascii characters in text.txt
# and output in descending order, by values
tsur -f text.txt count -c -s vdesc

# count all ascii characters in text.txt
# and output in descending order, by keys
tsur -f text.txt count -c -s kdesc

# count all ascii characters in text.txt
# and output in ascending order, by keys
tsur -f text.txt count -c -s vasc

# cout all ascii characters in t1.txt t2.txt
# and output in ascending order, by values
tsur -f t1.txt t2.txt count -c
```

## Author
[109149](https://github.com/109149)

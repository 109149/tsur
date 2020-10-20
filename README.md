Tsur (tsur)
------------
Tsur is a character counter tool.

### Examples

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
```

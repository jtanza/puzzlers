# wordsquare

Creates word squares where each column/row is two characters wide.

```
jtanza @ ~/rust/wordsquare (master) $ cargo run /usr/share/dict/words 2  
----cero
talari--
bi--te--
ramp--ed

Puzzle completed in: 1339ms

al--cove
beso--il
--male--
doli----

Puzzle completed in: 3915ms

Completed 2 iterations in 5254ms. Average: 2627ms.

jtanza @ ~/rust/wordsquare (master) $ cat /usr/share/dict/words | grep -wi albedo
albedo
jtanza @ ~/rust/wordsquare (master) $ cat /usr/share/dict/words | grep -wi alcove
alcove
```

Usage:

`cargo run <path-to-newline-separated-dict> <number-of-iterations>`

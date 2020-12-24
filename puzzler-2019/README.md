# wordsquare

Creates word squares where each cell is two characters wide.

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

Approach:

The general algorithm to generating squares is very straight forward and is as follows:
* To start, place a word on the board.
* Next, choose a word at random from our collection of candidates. Split this word into a set where each member is two `chars` from our candidate.
* Then, iteratively generate the intersection of our candidate and the existing words on the board. Iff at any point the resulting set is not disjoint, attempt to place our candidate on the board at the element in which they meet.
* If the newly placed candidate does not invalidate in any way the state of the board, place it on the board. Otherwise, continue to step two.
* Continue until the board is complete. 

Impl:

* Internally we use a struct `Puzzle` to maintain state of our board as it is being generated. This struct containts a hashmap of our already existing words to a vector of `Point`s (i.e. an `x` and a `y`) representing where they are located on the board. This allows us to very quickly locate and update the state of the board given the words that are currently on it. This hashmap is also updated on each mutation of the board so that words that were previously placed but were either overwritten or removed are eligible as candidates on future iterations.




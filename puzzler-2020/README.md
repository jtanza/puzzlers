# puzzler-2020

Find words by connecting letters. 
* Start at any letter in the graph
* Follow the lines to add letters
* Can loop back to reuse letters
* Can’t repeat a letter “in place” (no doubled letters)
* No minimum or maximum length
* Validate using word list (/usr/share/dict/words or similar)


## Usage
```
$ java -jar puzzler-2020-0.1.0-SNAPSHOT-standalone.jar /usr/share/dict/usa
$ # or with leiningen:
$ lein run /usr/share/dict/usa 
```

## Options

Required input includes any list of newline separated words.

## Example  

```
$ lein run /usr/share/dict/usa | grep -vwE '\w{1,5}' | sort --random-sort | head -n 5
colons
browns
ocelot
bronson
nobler
```

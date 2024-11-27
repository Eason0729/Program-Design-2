# Homework 5

> [!WARNING]
> This is obviously an unreal usecase.

## Problem Description

We are ask to create a program to index text files then search on them.

### Index
The input contain multiple documents and its id, which was enough to compute a score for each `(word, document id)` pair.

### search

The input contain a sequence word and search mode, for example:
- `(OR mode, [is, home, mygo])`: search for document ids which contain any of word in `[is, home, mygo]`
- `(AND mode, [query, import, toss])`: search for document ids which contain all word in ` [query, import, toss]`

Additionally, input also provide a number `n`, which ask us to only output top `n` ids with sum of score.

For example:

- `[is, home, mygo]`: sum of individual score on words `[is, home, mygo]`
- `[dup, dup, YZ]`: sum of individual score on words `[dup, YZ]`, note that score for word `dup` is counted twice

Finally, the homework **only** measure the time spent on searching.

## My Solution

### Indexing
build a `BTreeMap<word, Vec<(id, score for each word)>>`, and the vector is sorted by id.

### searching

1. convert `[dup, dup, hugo]` into `[(dup, 2), (hugo, 1)]`.

> [!TIP]
>  It's sorted by id, the advantage include...
>  1. we don't need to maintain a map.
>  2. It's also easiler to find some document missing a word.

2. get the Vec<(id, sum of score)> by iterating through each entry in `BTreeMap<word, Vec<(id, score)>>`
3. sort `Vec<(id, sum of score)>` and find top `n`

## Lesson Learned

- use `Beoing Tree` instead of `Prefix Tree`
  - The cardinality(variety) of word are small(1000), so Prefix Tree is slower
  - Disk has slower random access, Beoing Tree can mitigate that with larger fanout
  
- use `criterion` for benchmark: benchmark point me out many optimization
  - convert `[dup, dup, hugo]` into `[(dup, 2), (hugo, 1)]`
  - use `Binary Heap` if `n` is small enough
  - vectorization is important, and hashmap defeat that(or I don't know how to vectorize and hash map)
  - fast static symbol table acutally don't work for some reason?

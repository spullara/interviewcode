# Interview Code

At Twitter I often asked a simple question, render a tweet given the text and an unordered list of its entities (hashtags, URLs, @mentions). Each
entity is represented by the replacement text, a start index and an end index. The classic solution is to sort the
list of entities by their start index and build up the solution by iterating over the list grabbing text from the
tweet and text from the replacement as you go. It was surprising that there were many people that could not get that
far. If people made mistakes I would ask them to describe 4 unit tests they would write and I would tell them if
those tests would fail or pass. One of the most difficult to write solutions was only described by a candidate, I
later wrote BinaryTree to see how it did and it was pretty good relative to the Classic solution. However, it is
painfully complicated and it took a while to get it to pass the tests.

Since I don't think that anyone else at Twitter is currently using this question and I am unlikely to use it again,
I thought it would be interesting to share it. One of the oddities of the question is that it is almost impossible
to get right because no one ever asks what the indexes in the entities represent. In the API they are indexes into
codepoints in the original tweet text. Many languages require very special handling of codepoints vs unicode vs
ascii text. The solutions were universally focused on the default string behavior of the language they decided to
write them in. The worst solution of the bunch, StringReplacement, was by far the most popular, sadly using that one
at scale would be devastating. If someone got that far with a correct solution, we then would talk about optimizations
you might perform when you need to do billions of them a day.

I would love to see other solutions if you have a novel one.

There is a rust implementation that has been heavily modified by a better Rust programmer than I. It is not quite apples to apples but it shows that you can get good performance out of it. In the Java case you can see that the JIT gets a pretty good workout as it optimizes. Another thing to note is just how slow processing UNICODE (with supplementary characters) is vs good old ascii (or even UNICODE-16).

# Java Results

```
Running interview.RendererBenchmarkTest
Classic: 1748 ns/op
Classic: 1662 ns/op
Classic: 1218 ns/op
Classic: 909 ns/op
Classic: 892 ns/op
Memory: Classic: 1538 bytes/op
Tree: 1037 ns/op
Tree: 912 ns/op
Tree: 924 ns/op
Tree: 874 ns/op
Tree: 940 ns/op
Memory: Tree: 1206 bytes/op
OptimizedClassic: 739 ns/op
OptimizedClassic: 689 ns/op
OptimizedClassic: 671 ns/op
OptimizedClassic: 648 ns/op
OptimizedClassic: 611 ns/op
Memory: OptimizedClassic: 634 bytes/op
OptimizedClassicWithCodePoints: 875 ns/op
OptimizedClassicWithCodePoints: 899 ns/op
OptimizedClassicWithCodePoints: 953 ns/op
OptimizedClassicWithCodePoints: 931 ns/op
OptimizedClassicWithCodePoints: 902 ns/op
Memory: OptimizedClassicWithCodePoints: 676 bytes/op
BinaryTree: 941 ns/op
BinaryTree: 794 ns/op
BinaryTree: 838 ns/op
BinaryTree: 836 ns/op
BinaryTree: 863 ns/op
Memory: BinaryTree: 1332 bytes/op
LinkedListEntities: 1231 ns/op
LinkedListEntities: 1104 ns/op
LinkedListEntities: 1074 ns/op
LinkedListEntities: 1495 ns/op
LinkedListEntities: 1115 ns/op
Memory: LinkedListEntities: 1140 bytes/op
InsertionSort: 862 ns/op
InsertionSort: 761 ns/op
InsertionSort: 707 ns/op
InsertionSort: 682 ns/op
InsertionSort: 700 ns/op
Memory: InsertionSort: 1057 bytes/op
BinarySearchSort: 883 ns/op
BinarySearchSort: 792 ns/op
BinarySearchSort: 807 ns/op
BinarySearchSort: 777 ns/op
BinarySearchSort: 855 ns/op
Memory: BinarySearchSort: 1239 bytes/op
StringReplacement: 1872 ns/op
StringReplacement: 1790 ns/op
StringReplacement: 1778 ns/op
StringReplacement: 1857 ns/op
StringReplacement: 1786 ns/op
Memory: StringReplacement: 6740 bytes/op
StringBuilderReplace: 701 ns/op
StringBuilderReplace: 680 ns/op
StringBuilderReplace: 724 ns/op
StringBuilderReplace: 701 ns/op
StringBuilderReplace: 673 ns/op
Memory: StringBuilderReplace: 981 bytes/op
HashMapScan: 1500 ns/op
HashMapScan: 1563 ns/op
HashMapScan: 1463 ns/op
HashMapScan: 1518 ns/op
HashMapScan: 1510 ns/op
Memory: HashMapScan: 777 bytes/op
ArrayScan: 712 ns/op
ArrayScan: 683 ns/op
ArrayScan: 668 ns/op
ArrayScan: 665 ns/op
ArrayScan: 683 ns/op
Memory: ArrayScan: 1003 bytes/op
```

# Rust Results

```
interviewcode sam$ CARGO_INCREMENTAL="0" cargo bench
     Running target/release/interviewcode-d8a656227704114f

running 6 tests
test rendertest::correctness ... ignored
test rendertest::correctness_ascii ... ignored
test rendertest::correctness_chars ... ignored
test rendertest::bench_replacement       ... bench:       1,053 ns/iter (+/- 404)
test rendertest::bench_replacement_ascii ... bench:         215 ns/iter (+/- 93)
test rendertest::bench_replacement_chars ... bench:         604 ns/iter (+/- 250)

test result: ok. 0 passed; 0 failed; 3 ignored; 3 measured
```

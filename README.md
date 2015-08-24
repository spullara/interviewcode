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

I recently have been playing with Rust and thought I would port this question to it. The performance is pretty good,
just a bit worse (30% or so) than Java 8 running on the same platform. I'm sure that implementation is not ideal.


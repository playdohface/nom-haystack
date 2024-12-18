# Nom Haystack
Combinators that make it easy to use nom-parsers as patterns to be recognized in an input.

## Why?
I found myself using nom in many cases where traditionally Regex would be the appropriate tool, simply because nom parsers are much easier to read and to compose than regex. However, nom does not make it obvious how to use needle-haystack patterns, where the needle is a parser. 

## Shouldn't this be a PR to nom?
Perhaps it will be in the future. But for that some work and discussion would be needed - on whether or not this is actually within the scope of nom at all, and if so, how it should be implemented in light of streaming vs complete etc..
In the meantime I just want to use and share these as convenience extensions to nom, with no guarantees. Any suggestions for improvement or addition however are very welcome.


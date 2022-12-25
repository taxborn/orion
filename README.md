# :sparkles: The Orion programming language
What is intended to be a strongly-typed compile language. With the goal to be generating assembly
(or transpile to C in the early days, who knows, I haven't written the code generator yet)

## TODO
Currently, there is just about everything to do. Currently working on planning the parser and determining the syntax
in the `examples/` directory, but once I make progress I will be able to update this on whatever I am researching

A few other house-keeping things I want to do:
- Setup a Github Action to run the tests on push
- Create (neo)vim highlighting for it (once I stabilize the syntax)
- Be able to generate [benchmarks](https://doc.rust-lang.org/cargo/commands/cargo-bench.html) to keep track of performance over time

## :boom: Getting the project started
During the early stages I'll be creating examples of what I want the language to look like, and
hopefully from there generate an EBNF form of the language.

**Examples:**
- [generic main.ori file](./examples/main.ori)

## References used:
- The [Dragon Book](https://suif.stanford.edu/dragonbook/)
    - Also used the lecture notes from the courses listed at the website
- [Kind functional language](https://github.com/Kindelia/Kind)

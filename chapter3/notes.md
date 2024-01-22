
[This note](file:///home/isaac/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch03-02-data-types.html#integer-overflow),
from chapter 3.2 Data Types,
talks about some integer behavior, including overflow wrapping.
It is considered an error to depend on the behavior of integer wrapping.
However, its behavior is defined as per Rust language specification.
In this way, it is not exactly UB in the way that it would be in C/C++.
But it is quite similar.
I wonder if there are any architectures to which rust will compile,
while its behavior does not comport with this definition.
It just seems like an architecture-dependent fact.


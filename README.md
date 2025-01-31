



# Ataraxia

> Ataraxia 
> 
> n. calmness untroubled by mental or emotional disquiet
> 
> \- _Merriam-Webster Dictionary_

Ataraxia is a simple, minimal programming language designed to be easy to pick up and use,
while providing powerful features to allow you to do whatever you need with it. The goal
of ataraxia is to, just like its namesake, bring a calmness to the user and prevent them
from being troubled by language-related issues, freeing them up to focus on the project
at hand. 

The primary features of Ataraxia are inspired by Lua, which is a simple language that
also provides a large amount of functionality. However, Lua has some strange design choices
(like 1-indexing, global-by-default variables, and the oddity that is `pcall`) which causes
it to be more of an obstruction than it should be. Ataraxia aims to preserve many of the 
positive features of Lua: list-table combined data structures (which I will henceforth
refer to as a `desk`), lack of object-oriented programming, and flexible dynamic typing.
However, it also adds other features from languages like Rust which Lua is lacking, and
implements a more standard default behavior compared to Lua. 

Ataraxia is currently interpreted, but I plan on creating a self-hosted compiler at some
point in the future to resolve any possible performance issues that might arise. 


# Language Specification

Here is the specification of the language once it's finished. It goes without saying that
this is subject to change, and most of these features haven't yet been implemented. 

## Variable Declaration

Inspired by Rust, Ataraxia sports a transitive mutability system for declaring variables.
The output of a literal expression (ints, strings, etc.) is mutable by default. 
```rust
let a = 5;
mut b = 7;

a = 8; // Invalid! returns an error.
b = 12; // Valid

let c = b; // coerce mutable to immutable
c = 17; // Invalid! returns an error. 

mut d = a; // Invalid! returns an error. 
```


## Desks

A desk is a mixed map and list data structure, taken from Lua. `[]` square brackets are 
chosen for this data structure, because `{}` curly braces are reserved for code blocks. 

List elements (integer-indexed keys) can be accessed only through `[]` bracket indexing. 
Map elements (string-indexed keys) can be accessed through both `[]` bracket indexing 
and `.`-based field access, like a `struct` in Rust. 

As mentioned before, mutability in Ataraxia is **transitive**. Therefore, it is impossible
to mutate fields in a desk when the desk itself is immutable. However, when copying a 
`desk`, it is only a memory reference that is copied, since the type is not free. 

Desks possess a `:copy` property for deep copying, but I'll save that until
we get to properties and method call syntax. 
```rust
let list = [1, 2, 3];
let map = [
    a = 4,
    b = 5, 
    d = 6, // trailing commas are allowed, but optional
];

print(list[0]); // 1
print(map.a); // 4
print(map["a"]); // 4

mut mutable_desk = [1, 2, x = 3, y = 4];
let immutable_desk_copy = mutable_desk; // changes when `mutable_desk` is changed

print(mutable_desk.x); // 3
print(immutable_desk_copy.x); // 3

mutable_desk.x = 12;
print(immutable_desk_copy.x); // 12

immutable_desk_copy.x = 17; // Invalid! returns an error. 
```


## Code Blocks

In Ataraxia, code is organized into blocks delimited by `{}` curly braces. Similar to Rust,
Ataraxia is expression-based; however, Ataraxia is even more heavily expression-based
than Rust. In Ataraxia, there are no statements; everything is an expression. 

Blocks can be treated expressions, like so: 
```rust
let foo = {
    some_func();
    some_other_func();
    5
};
```

Similar to Rust, Ataraxia supports implicit returns for the value of a code block. Blocks
also create an inner `scope`, similar to many other languages, and variables are always
scope-local, notably unlike Lua. Shadowing is also supported, and, like C, blocks can also
be used free-standing without anything surrounded to create a subscope at any point in the
program's code. 
```rust
mut foo = 5;
let bar = 6; 
print(foo); // 5
print(bar); // 6
{
    foo = 8;
    let bar = 9; // shadowing happens here
    let baz = 10;
    print(foo); // 8
    print(bar); // 9
    print(baz); // 10
} // baz goes out of scope here

print(foo); // 8
print(bar); // 5
print(baz); // Error: baz doesn't exist
```



## Functions

There are two ways to declare functions in Ataraxia. The first way is a "standard" 
function declaration:
```rust
fn my_func() {
    print("Hello, World!");
}
```
There is also syntax for declaring anonymous functions, like so: 
```rust
let my_func = fn() {
    print("Hello, World!");
};
```
The "standard" function declaration syntax is just syntax sugar that does exactly the
same thing as creating an anonymous function and binding it to an immutable variable.

Like many other languages, you can use a `{}` block following a function to specify the 
expression that is evaluated when the function is called. However, since `{}` blocks
are just expressions, Ataraxia also allows you to use other expressions in place of the
code block, which in many other languages is what a lambda function or a closure is used
for. 

```rust
fn my_func() print("Hello, World!");
let my_func = fn() print("Hello, World!");
```

Since functions take on the role of a closure, indeed, functions in Ataraxia can also close
over other values. 

```rust
let x = 5;
let my_func = fn() x; // closes over `x`
```

However, since `x` here is a primitive type, and not a `desk`, it is copied into `my_func`
instead of being closed over by reference, so changing the value of `x` later will not change
the return value of `my_func`. To change this, use a `Cell` instead, which can be represented
like this in Ataraxia (it's really just a one-element desk): 
```rust
mut x = [5];
let my_func = fn() x[0]; // closes over x

print(my_func()); // 5
x[0] = 8
print(my_func()); // 8
```

Function calls in Ataraxia also have a Lua-inspired syntax shorthand where if they only take
one argument which happens to be a desk literal, the parentheses can be skipped. An example: 

```rust
let my_func = fn(x) print(x[0]);

print[1, 2, 3, 4, 5]; // [1, 2, 3, 4, 5]
my_func[34]; // 34
```

This is particularly useful when creating short methods (or custom operators) to be called on
or with desks. More on that in a moment. 


### A Note on Currying

I am a really big fan of currying. It's intentionally very straightforward to implement
a form of currying syntax through the function syntax Ataraxia has, and method syntax
actually rides on this as well. 
Here's an example, using some more advanced language features: 
```rust
let old_list = [1, 2, 3, 4, 5];
let add = fn(x) fn(y) { // currying happens here
    /* insert other fancy code here */
    x + y
};

let new_list = old_list:iter_list
    :map(add(1))
    :collect;
```
Here, using the fact that you can declare functions without `{}`, the first set of
curly braces is elided, allowing you to create curried functions without lots of
indentation layers. Implicit returns make the second function just returned as a
function object. 


## Methods & Properties

Ataraxia supports a `:` operator, which is the **property access operator**. I've been
using it before already, but the formal definition is this: `a:b` is equivalent to
`a["b"](a)`. 

As a result, you can create properties (and methods, which are callable properties) like
this:
```rust
let example = [
    a_property = fn(self) 5,
    a_method = fn(self) fn(x, y) { // braces are still optional
        self:a_property + x + y
    }
];

example:a_property; // -> 5
let bound_function = example:a_method; // `bound_function` closes over `self` now
bound_function(1, 2); // -> 8
example:a_method(3, 4); // -> 12
```

Due to the way property access works, methods are defined using a single layer of partial
function application. Also see the note on **Currying** above. 

Also mentioned before, the `:copy` property. Since properties are functions that get a
`self` argument, it's relatively straightforward to see how this works.
```rust
mut foo = [1, 2, 3];
mut bar = foo;
mut baz = foo:copy;

foo[0] = 3;
bar[0] == foo[0]; // -> true
baz[0] == foo[0]; // -> false
```

Through the syntax shorthand above, where desk literal arguments don't require parentheses,
the following (rather unusual) implementation of a set union operation is possible: 
```rust
mut set_A = [
    U = fn(self) fn(other) self:copy:insert_all(other:iter_list):unique,
    1, 2, 3,
];

// union operator fr
set_A :U [ 4, 3, 2 ]; // -> [1, 2, 3, 4]
```


## Operator Overloading

Operator overloading is currently not possible, but I am thinking about adding it in the future. 


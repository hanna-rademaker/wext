# Hello World

## Complete code example
This is the complete "Hello World" example, creating a heading and logging "Hello World!" to the browser console:
```rust
{{#include ../../../examples/hello_world/src/main.rs}}
```
Now let's take this apart.

## Logging to browser console
The first three lines inside the `main` function define conditions for logging:
```rust
{{#include ../../../examples/hello_world/src/main.rs:6:8}}
```
Errors that lead to `panic`, as well as logs from `rust` should be printed to the browser console. Only messages of level `info` or above (errors, warnings) will be logged.

The following
```rust
{{#include ../../../examples/hello_world/src/main.rs:10}}
```
prints `"Hello World!"` to the console.

## Creating an html element
Now we encounter the first use of `wext`, where we create an empty `div` element:
```rust
{{#include ../../../examples/hello_world/src/main.rs:12}}
```
The equivalent `javascript` code is this:
```javascript
const root = document.createElement("div");
```
## Modifying properties of an html element
Just like the `div` above, we can create other html elements. Here we create a level 1 heading and set its `textContent` to `"Hello World"` using `wext`'s `h1()` and `txt()` function:
```rust
{{#include ../../../examples/hello_world/src/main.rs:13}}
```
This is the equivalent `javascript` code:
```javascript
const heading = document.createElement("h1");
heading.textContent = "Hello World";
```
## Attaching elements to each other
In the last two lines we use `wext`'s `child()` function to attach the created elements to each other and to the `body` of the DOM:
```rust
{{#include ../../../examples/hello_world/src/main.rs:14:15}}
```
The `child()` function can also be chained, meaning we can also combine these two lines to one:
```rust
    body().child(&root.child(&heading));
```
Note that `body()` is a function of the `gloo` package, not of `wext`. 
The equivalent `javascript` code for these lines is:
```javascript
document.body.appendChild(root).appendChild(heading);
```
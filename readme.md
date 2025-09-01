An in development graphing programming language meant for REPL's

The syntax is rust inspired whilst introducing changes to keep it faster to write.

Types are optional but always inferred, a function without any types can be seen as a generic function and not a function which takes any type.

```
// tuples
let a = (3, 1);
// math on arbitrary types
let foo = (a + 1) * 2;
// pattern bindings
let (x, y) = foo;
```

Match expressions are also supported using the following style
```
fn fib(0) = 1;
fn fib(1) = 1;
// optional types ensure the input is always positive
// return type inferred to be u64
fn fib(x: u64) = fib(x-1) + fib(x-2)
```

# TODOS:
- Validity checks
    - Build symbol table
    - Check if used variables/function/types exist in any reachable scope
- Types
    - Type inference
    - Typechecking
- Bounds
    - Implicitly generate bounds for all functions: `fn foo(x) = 2 * x + 1` generates the bound `typeof(x) * AnyInt + AnyInt` which ensures the type of x is allowed this operation
    - Bounds bubble algorithm: bubble up bounds in generic functions until a specific implementation is used
- Interpreter
    
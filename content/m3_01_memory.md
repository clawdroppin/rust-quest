@@lesson m3-memory How Memory Works: Stack and Heap
@@text
This module explains Rust's most famous idea: **ownership**. It's what makes Rust different from every other popular language — and it's the reason for all those `&` symbols. To understand it, we first need to understand how programs use **memory**.

## What is memory?

Your computer's memory (RAM) is like an enormous row of numbered boxes, each holding one byte. Every variable in your program lives somewhere in those boxes. When you write `let x = 5;`, a few boxes are set aside to hold `5`, and the name `x` refers to them.

A running program has two main areas of memory: the **stack** and the **heap**.

## The stack: fast and tidy

The stack works like a stack of plates. Each time a function is called, a new "plate" (called a **stack frame**) is put on top, holding that function's local variables. When the function returns, its plate is taken off — and all its variables disappear with it.

```text
 main calls greet, which calls shout:

 ┌──────────────┐
 │ shout: msg   │  ← top: the running function
 ├──────────────┤
 │ greet: name  │
 ├──────────────┤
 │ main: x, y   │
 └──────────────┘
```

The stack is **very fast** and automatic. But it has a rule: everything on it must have a **size known in advance**. An `i32` is always 4 bytes. A `bool` is 1 byte. An array `[u8; 100]` is always 100 bytes. Those can all live on the stack.

## The heap: flexible

But what about a `String` that grows as the user types? Or a `Vec` that might hold 3 items or 3 million? Their size isn't known in advance, and it changes. These live on the **heap**: a big open area where the program can ask for a chunk of memory of any size, at any time.

When you create a `String`, Rust:

1. asks the heap for some memory (this is called **allocating**),
2. stores the text there,
3. keeps a small fixed-size "handle" on the stack: a **pointer** (the address of the heap memory), the **length**, and the **capacity**.

```text
   STACK (variable s)              HEAP
  ┌───────────────────┐         ┌───┬───┬───┬───┬───┐
  │ pointer ──────────────────► │ h │ e │ l │ l │ o │
  │ length:   5       │         └───┴───┴───┴───┴───┘
  │ capacity: 5       │
  └───────────────────┘
```

`Vec` works the same way.

## The big problem: who cleans up?

Heap memory must be **given back** (freed) when it's no longer needed, or the program slowly eats up all the memory (a **memory leak**). Languages have solved this in different ways:

| Approach | Languages | Downside |
|---|---|---|
| **Programmer frees memory by hand** | C, C++ | Forget → leaks. Free too early or twice → crashes and security holes. These bugs are *extremely* common. |
| **Garbage collector**: a background system finds unused memory and frees it | Java, C#, Python, JavaScript, Go | Uses extra memory and CPU, and can pause the program at unpredictable moments. |
| **Ownership**: the compiler works out exactly when memory can be freed | **Rust** | You have to learn the rules — which is what this module is for! |

Rust's approach gives you the speed of C with the safety of a garbage collector. The price is that the compiler is strict: it needs to *prove* your program handles memory correctly. When it complains, it's usually preventing a real bug.

## Scope and `drop`

The core idea is simple: **every value has an owner, and when the owner goes out of scope, the value is cleaned up.** A variable's **scope** is the block `{ }` it was created in:

```rust
fn main() {
    let outer = String::from("I live in main");
    {
        let inner = String::from("I live in the inner block");
        println!("{inner}");
    } // ← inner goes out of scope here: its heap memory is freed automatically

    println!("{outer}");
    // println!("{inner}"); // ❌ error: inner doesn't exist here
} // ← outer is freed here
```

At the closing `}`, Rust automatically inserts the cleanup code (it calls `drop`). You never forget to free memory, and you can never use a value after it's been freed — the compiler won't let you.

@@quiz
Where does the text of a `String` live?
- On the stack
* On the heap, with a small pointer/length/capacity handle on the stack
- Inside the compiler
? Its size can change, so the text goes on the heap. The fixed-size handle sits on the stack.

@@quiz
When is a value's memory freed in Rust?
- When the garbage collector runs
- When you call `free()`
* When its owner goes out of scope (at the closing `}`)
? Rust inserts the cleanup automatically at the end of the owner's scope.

@@exercise Fix the scope
This program tries to use a variable outside the block where it was created. Fix it **without removing any `println!`**, by moving the `let total` line so that `total` lives long enough. The output should be:

```text
Adding up...
Total: 60
```
@@starter
fn main() {
    let numbers = vec![10, 20, 30];
    {
        println!("Adding up...");
        let total: i32 = numbers.iter().sum();
    }
    println!("Total: {total}");
}
@@expected
Adding up...
Total: 60
@@hint
`total` is created inside the inner `{ }` block, so it's gone at the inner `}`. Create it outside the block instead (for example, right after the block, or before it).
@@solution
fn main() {
    let numbers = vec![10, 20, 30];
    {
        println!("Adding up...");
    }
    let total: i32 = numbers.iter().sum();
    println!("Total: {total}");
}

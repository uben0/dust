# DUST language interpreter

The DUST language is only a experimental laguage to train at creation of compileur. It does not have any specification, or a well defined grammar.

The repository contains an interpreter, and can run the following code:
```
fn fib n := cond {
    n == 0 then 0;
    n == 1 then 1;
    true then fib (n - 1) + fib (n - 2);
};

i := 0;
while i < 10 then {
    print (fib i);
    i = i + 1;
};
```
Which will print the 10 first elements of the fibonacci sequence.
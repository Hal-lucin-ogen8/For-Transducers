# For-Transducers Semantics

## Syntax

Let us provide an abstract syntax for the expressions arising in our programs.

```
int_const := 0,1,2,...
var := i,j,k, ...
bool_var := b,q,s,t,...
bool_expr := var < var
           | var == var
           | var == int_const
           | var == var + 1
           | bool_var
           | !bool_expr
           | True
           | False
           | bool_expr /\ bool_expr
           | bool_expr \/ bool_expr
str_expr := str
           | var.label
           | str_expr + str_expr
cmd := skip
     | if bool_expr then cmd else cmd end
     | for var in stdin (<-/->) do [cmd...] end
     | var = bool_expr
     | initialize var
     | print(str_expr)
```

An example of program is the `reverse` function, that one can write as follows

```
for i in stdin <- do
  print(i.label)
end
```

Another example is the `cyclicpermutation` function that brings the last letter upfront.

```
initialize b
for i in stdin <- do
  if b then
     print(i.label)
  else
     b = False
  end
end
for i in stdin -> do
  print(i.label)
end
```

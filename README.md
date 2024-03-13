# lox-interpreter

reference: https://craftinginterpreters.com/contents.html

## Grammar

```
program        → statement* EOF ;

statement      → exprStmt
               | printStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

expression     → unary ( ( "!=" | "==" | ">" | ">=" | "<" | "<=" | "-" | "+" | "/" | "*" | "?" expression ":") unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
```

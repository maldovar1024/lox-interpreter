# lox-interpreter

reference: https://craftinginterpreters.com/contents.html

## Grammar

```
program        → declaration* EOF ;

declaration    → varDecl
               | statement ;

statement      → exprStmt
               | printStmt ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

expression     → unary ( ( "=" | "!=" | "==" | ">" | ">=" | "<" | "<=" | "-" | "+" | "/" | "*" | "?" expression ":") unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" 
               | IDENTIFIER ;
```

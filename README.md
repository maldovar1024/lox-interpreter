# lox-interpreter

reference: https://craftinginterpreters.com/contents.html

## Grammar

```
program        → declaration* EOF ;

declaration    → funDecl
               | varDecl
               | statement ;

funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;

statement      → exprStmt
               | ifStmt
               | printStmt
               | whileStmt
               | forStmt
               | block ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;

whileStmt      → "while" "(" expression ")" statement ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;

block          → "{" declaration* "}" ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

exprStmt       → expression ";" ;
printStmt      → "print" expression ";" ;

expression     → unary ( ( "=" | "!=" | "==" | ">" | ">=" | "<" | "<=" | "-" | "+" | "/" | "*" | "?" expression ":" | "and" | "or") unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" 
               | IDENTIFIER ;
```

# R5RS Parser

A [nom](https://github.com/Geal/nom) parser
for the scheme synax descibed in chapter 7 of 
[R5RS Scheme](http://www.schemers.org/Documents/Standards/R5RS/r5rs.pdf).

## Current State

``` rust
$ cargo run
>> let
Parsed Done(
    [],
    Expression(
        Let
    )
)
>> letrec
Parsed Done(
    [],
    Expression(
        LetRec
    )
)
>> let*
Parsed Done(
    [],
    Expression(
        LetStar
    )
)
>> le
Parsed Error(
    Alt
)
>>
CTRL-C
```

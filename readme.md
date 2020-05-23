# YAIPR

YAIPR stands for *Y*et *A*nother *I*ni *P*arser in *R*ust.

I've alread created an Ini-Parser called [YAIP]. The basic structure of [YAIP]
does not allow multi-line values at the moment. Additional the C DLL interface
is missing. And there at least some convenience features like templates which
bloated the code.

*YAIPR* is designed to be the plain Ini file parser. Read the whole ini file at
once and pass the content to *YAIPR* and you will get the complete sections with
their containing key-/value entries.

## Motivation

Learn how to use [Rust] as programming language. Sting parsing in C/C++ is
from my point of view a potential memory risk. Trying to move this part to an
language/infrastructure which reduces this risks.

## How it works

The complete ini file is thrown in as one large string. Then this string is
parsed into a list of complete `sections`. This means that the large input
string will be sliced into `sections`-strings using a [regular expression](regex).
Each section will parse it's section name and the corresponding key/value-
entries.

- The [regex] for determining a full section:
```
(?ms)(?P<SEC_FULL>^\[[^\[]*)
 \-/  \----------/\-/\---/^
  |        |       |   |  |
  |        |       |   |  +--- 0 or many times
  |        |       |   +------ Match all until next [ is found or end of string
  |        |       +---------- Line starts with [
  |        +------------------ Name group SEC_FULL
  +--------------------------- Multi-line match
```
- The [regex] for determining a section name:
```
(^\[(?P<SEC_NAME>.*)\])
 \-/ \----------/\/ \/
  |        |      |  |
  |        |      |  +--- Pattern ends with ]
  |        |      +------ Match everything until ] appears
  |        +------------- Name group SEC_NAME
  +---------------------- Line starts with [
```

[regex]: https://en.wikipedia.org/wiki/Regular_expression
[Rust]: https://www.rust-lang.org/
[YAIP]: https://github.com/ThirtySomething/YAIP

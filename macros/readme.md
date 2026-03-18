1. There are two types of macros
   a) declarative
   b) procedural
2. Def: You define fucntions and rust expand code on them
3. where the inout is code and output is also code that is transformed
4. println and vec

5. Functions vs macros

   functions run at runtime and macros run at compile time

   functions:
   Fixed number & type of parameters ✅
   Must follow strict type rules ✅
   Executed at runtime ✅
   Cannot change structure of code ❌

   Macros can accept flexible syntax and generate different code
   Run before compilation finishes
   Can generate code dynamically
   Can take variable number of arguments
   Work with syntax, not just values


6) declartive: something like pattern matching is here like we match code and generate code
println and vec are example of declarative macros


 7) here we give some code and rust expands it and return an enhanced code
 where #derive is used, taht is called procedural macros
 there are 3 types:
 1) custom
 2) fucntional
 3) attribute
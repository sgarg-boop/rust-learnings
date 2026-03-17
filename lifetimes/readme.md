

1) lifetimes ensures that we dont have any dangling ref 
2) also ensures that each ref has some lifetime
3) it is mainly used to check at compile time that each ref has some lifetime and also it shouldnot have dangling ref
4) if we are passing two lifetimnes in func, then the min one is returned as lifetime
5) if we pass 2 var in func and and returning smething that has lifetime same as one of pram, then we dont need to declare lifetime for other param


3 rules for lifetime 
1) a func with one param gets one lifetime param
a func with multiple params gets same number of lifetime params

2) is there is one input param with lifetime, then all the other lifetime will have same lifetime

3) if we have multiple inputs lifetime params, and one of them is self or Mut self then other params lifetime will be same as self



//////
//// Imp : static lifetime
/////
1) all the string literals have static lifetime ->  means &str has static lifetime
2) because they are stored in binary and have whole program lifetime
3) their lifetimes cant be destroyed

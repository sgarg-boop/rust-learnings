

Rules for ownership
1) each variable has an owner
2) there can be only one owner at a time
3) when the owner goes out of scope, the variable is dropped
 
 the things for which sizes are known must be stored in stack
 and unknown are stored in heaps

4) rust uses one drop func internally which deletes the data auto
5) we cant call drop ourself


storing a string in rust have 3 things
1) ptr to heap
2) length
3) capacity

when we are copying one string to other

s1 = string.from("hello")
s2 = s1
print(s1) -> this condition creates double free error (suppose if s1 goes out of scope then s2 will also try to free the memory which is already freed by s1)

// when we are writing s2 = s1 it invalidates s1
// and s2 takes over the ownership
// and s1 is no longer valid
// and if we try to print s1 it will give error

4) if we do want to make a deep copy then we can use clone
// but if we want to create a copy we can do clone and both are diff things
s2 = s1.clone()


COPY traits
1) all int u32 implements copy trait
2) booleans, f64, chars.
3) tuples if they contains types that implmemet copy trait
4) string does not implement copy trait so it will be a move

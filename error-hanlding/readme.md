

1) there are two types of error
    a) recoverable
    b) unrecoverable

2) recoverable
    a) like file not found
    b) we can use Result enum for this
    c) Result<T, E> is an enum with two variants
        i) Ok(T)
        ii) Err(E)

3) unrecoverable
    a) like divide by zero
    b) we can use panic macro for this


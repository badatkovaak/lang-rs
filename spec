Types:
    int
    float
    bool
    unit - ()
    string
    function
    array ?
    table - hashmap ?
    gadt - product and sum

Syntax:
    
    if a:
        x
    else:
        y

    let x = if a then 1 else 2
    
    fn add(int x, (int -> bool -> float) y) -> float
        y x false

    or 
    
    fn add(int x, float y) -> bool:
        return x

    struct Foo:
        

    

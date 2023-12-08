fn main() {
    // I. what is ownership?
    what_is_ownership();

    // II. stack vs heap
    // 0. data in heap must be accessed via pointer
    // 1. for modern cpus in computers, the less pointers jumping, the faster the program runs

    // III.what is stack?
    what_is_stack();

    // IV.what is heap?
    what_is_heap();

    // V. how scope works in rust?
    // we've defined a function called how_scope_works_in_rust, and there is a variable s in it
    how_scope_works_in_rust();  // 👈s is valid inside.
    // 👇this will cause an error since s is out of scope 👇, meaning s is no longer valid
    // println!("caution, s has exceeded its scope: {}", s);

    // VI.how variables interact with a specific data?

    // 0. clone
    // related to copy trait, and it's mutually exclusive with drop trait
    // clone is a deep copy
    // which means the previous one will still be valid after clone
    // dedicated to data on stack / dedicated to known variables and simple types => i32, bool, etc.
    what_is_clone();

    // 1. move
    // related to drop trait, and it's mutually exclusive with copy trait
    // which is similar to shallow copy in other languages
    // but the previous one will be invalid after move
    // and the new one will be dedicated to heap data
    what_is_move();

    // VII. how ownership works in functions?
    let str = String::from("hello");
    // because str is a type of String, which has a drop trait
    takes_ownership(str);
    // println!("caution, str has been moved to function scope: {}", str); // 👈this will cause an error since str has been moved to function scope

    // beacause num is a type of i32, which has a copy trait
    let num = 5;
    makes_copy(num);
    println!("num is still valid: {}", num); // 👈this works perfectly since num is just copied into function scope

    // VIII. how to return ownership from functions?
    let _j = gives_ownership(); // 👈some_string in gives_ownership() has been moved to j
    let k = String::from("hello");
    // 👇 k has been moved to takes_and_gives_back(), and then moved back to l
    let _l = takes_and_gives_back(&k);
    // 👇 this will cause an error since k has been moved to takes_and_gives_back(), meaning k is no longer valid
    println!("caution, k has been moved to l: {}", k);

    // IVV. how to use values in functions without losing ownership?
    let m = String::from("hello");
    // 👇 m has been moved to use_values_without_losing_ownership(), but it will be moved back to n after the function has done
    let (n, len) = use_values_without_losing_ownership(m);
    // 👇 use n and len here, n is the exact same value with m, but m is no longer valid
    println!("The length of '{}' is {}.", n, len);
}

fn what_is_ownership() {
    // there is a key point in rust: ownership
    // rust does not need garbage collection since it defines ownership rules at compile time

    // ownership rules:
    // 0. each value in rust has a variable that is called its owner
    // 1. there can only be one owner at a time
    // 2. when the owner goes out of scope, the value will be dropped

    // the reason why rust has ownership:
    // 0. manage data in heap
    // 1. memory safety
    // 2. no need for garbage collection
    // 3. no need for reference counting
    // 4. no need for mutexes
    // 5. tracking what data is being used in heap at any point in time
    // 6. minimizing the amount of duplicate data on heap
    // 7. cleaning up unused data on heap to free up space
}

fn what_is_stack() {
    // 0. last in first out
    // 1. fixed size
    // 2. faster than heap
    // 3. data must be known at compile time
}

fn what_is_heap() {
    // 0. fist in first out
    // 1. allocate memory in runtime, which means data size may change
    // 2. slower than stack
    // 3. data size may not be known at compile time
}

fn how_scope_works_in_rust() {
    let mut s = String::from("hello"); // s is valid from this point forward
    s.push_str(",world") // do stuff with s
} // this scope is now over, any code after this function will not be able to access s anymore, means s is no longer valid

fn what_is_clone() {
    // here is a simple example of clone
    // 1. value will be pushed onto stack since the type and size of x and y are known at compile time
    // 2. any type that has the copy trait will be copied to stack, and the old one will be valid still after copy
    // 3. drop trait and copy trait are mutually exclusive, means that if a type has the copy trait, it will not have the drop trait
    let x = 5;
    // 👇this will not cause an error since y is a copy of x 👇
    let y = x;
    print!("x: {}, y: {}", x, y);

    let v = String::from("hello");
    // 👇this will not cause an error since c is a copy of v 👇
    let c = v.clone();
    println!("v: {}, c: {}", v, c);

    // how to recgonize whether a type has the copy trait?
    // any group of simple scalar values can be copy
    // any group of data that needs to be allocated in heap will not be able to copy
    
    // 1. all integer types => u32
    // 2. boolean type => bool
    // 3. floating point types => f64
    // 4. character type => char
    // 5. tuple, if all its elements in certain type have the copy trait => (i32, i32) has copy trait => (i32, String) does not have copy trait
}

fn what_is_move() {
    // here is a simple example of move
    // part of s1 will be pushed onto stack, including the pointer, length, and capacity
    // but the acutal value of s1 will be pushed onto heap
    let s1 = String::from("hello");

    // 👇 this will cause an error since pointer, length, and capacity of s1 has been moved to s2 👇
    // which means s1 on stack has been dropped by program and it's no longer valid
    // now, _s2 is the owner of acutal data of s1 on heap
    println!("s1 has been moved to s2!");
    let s2 = s1;
    println!("s2: {}",s2)
    // println!("s1: {}", s1);

    // how to recgonize whether a type has the drop trait?
    // 1. unknown size at compile time => String, Vec<T>, etc.
    // 2. any type that needs to be allocated in heap(memory) will have the drop trait
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // 👇this variable will be moved to the function that calls it, in this case, is main function above👇
    some_string
}

fn takes_and_gives_back(a_string: &String) -> String {
    a_string.replace("hello", "goodbye")
}

fn use_values_without_losing_ownership(a_string: String) -> (String, usize) {
    // caculate the length of a_string
    let length = a_string.len();
    // returns a tuple
    // a_string is the exact same value as what it passed in
    // length is the length of a_string
    (a_string, length)
}

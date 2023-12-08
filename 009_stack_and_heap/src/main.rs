fn main() {
    // why dose ownership matter?
    // to manage the data on the heap, we need to know three things:
    // who is using data on the heap
    // minimize the amount of duplicate data on the heap
    // clean up unused data on the heap in time to avoid memory leaks

    // 0. what is stack
    // stack receives data in LIFO order, last in first off
    // stack is fast than heap,
    // since it never has to search for a place to put new data,
    // but always pushes the data to the top of the stack
    // however, each data must have a known, fixed size to be pushed onto the stack

    // 1. what is heap
    // heap is less organized, when you put data on the heap, you request a certain amount of space
    // but you don't know where in the heap your data will be stored,
    // so it returns a pointer that points the location in the heap where your data is stored
    // this process is called allocating on the heap, or allocating memory, and sometimes just allocating

    // 2. pointer can also be stored on the stack if the pointer itself has a known, fixed size.

    // 3. ownership rules
    // each value in rust has a variable that is called its owner
    // there can only be one owner at a time
    // when the owner goes out of scope, the value will be dropped

    println!("hello, ownership!")
}

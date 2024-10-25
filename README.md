# VarSet

A container structure for storing Any types in a set.

The container is implemented with a ```HashMap<TypeId,Box<Any>>```, so data in the container is not adyacent(cache misses increase) and to reach a single item there is a two pointer indirection. because of the problems mentioned before performance isn't as fast as it could be.

## Goal
The goal is to create a type that can hold a compact and performant container, more specificaly, a kind of ```Vec<Any>```.

```

struct T {
    number: u32,
    list: Vec<u8>,
}

let mut set = VarSet::new();

set.insert(0u32);
set.insert(String::from("hiii"));
set.insert(T {
    number: 123,
    list: vec![1, 2, 3],
});

*set.get_mut::<u32>().unwrap() = 445;
(*set.get_mut::<String>().unwrap()).push_str(" everyone!!");
(*set.get_mut::<T>().unwrap()).list = vec![3,2,1];
(*set.get_mut::<T>().unwrap()).number = 4325;


```
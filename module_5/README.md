# [Module 5: Serialization/Deserialization (Serde)](#module-5)

- [The Problem and Motivation](#the-problem-and-motivation)
- [Serialization / Deserialization](#serialization--deserialization)
- [Integers, Strings, and Hybrid Data Structures](#integers-strings-and-hybrid-data-structures)
- [Databases](#databases)
- [Post-credits Scene](#post-credits-scene)

## The Problem and Motivation

We start this module by motivating the problem of serialization and deserialization. For that, suppose you have implemented a program that implements a function ```mysterious(a: u32, b:u32) -> u64```.

We use it in a program as follows:

```rust
let a = 5;
let b = 10;
let c = mysterious(a, b);
```

It does not matter what the function does, but we know that a call to mysterious(2, 3) will take about 3 hours to complete. After 3 hours we obtain a result. Because it is so expensive to run ```mysterious``` I may wish to store it (checkpoint) somewhere. That way, if my program crashes immediately after calling mysterious, or the lights go off, I do not need to repeat the computation again because I can just read the checkpoint. This is a common reason for one to *want to represent a value computed at runtime in durable storage*. There are other reasons. Perhaps, we would like to share that result with someone else. *Why* would we want to do that? and *How* do we do it?

### Why

There are many reasons. You may want to make c available to a process living in a different memory space (e.g., a different machine in the Internet), in which case you would like to store the content of c on disk, so they can access it from there. Or you may send the data directly to a different machine via the network. In general, you will work with programs that produce data that needs to be shared with someone else. The opposite is also true, you will work with programs that want to obtain data from an external source, such as disk or via the network, and work on that data. Databases are an example of such programs, but there are many many more.

Database management systems are in charge of, among many other things, storing and retrieving data efficiently. It follows that serializing and deserializing data is one of the most basic operations databases will perform. We will see this a lot and you will deal with this a lot in Crusty, so spend time ensuring you followed this module end to end.

### How

The following is a ***bad*** way of achieving the goal above of storing a value computed at runtime. If I want to make sure I keep the results of c after the program above finishes, I could print its value to the terminal and write it down on a piece of paper. Next time, if I want to use that value, I can make a program ask me for the value, and then I'll just copy it back from my piece of paper. This sounds awful. What if instead of one value I have multiple gigabytes of data? (you can roughly fit 678000 pages of text in one gigabyte -- that would be a lot of writing). And even if I write it, how do I send it to a different machine? What if I have many different variables that I would like to save instead of just one number? Doing this all manually sounds cumbersome and it is clearly not the way to go. The way to go is to use serialization and deserialization, which we explore next. 

## Serialization / Deserialization

Serialization consists of taking the contents of translating a data structure (or in its simplest form a variable, such as c) into a different format that can be interpreted by: i) the same program but during a different execution (e.g., after a crash or reboot); ii) a different program (e.g, to which we send the data via network, disk, or other); iii) a human. In addition, they may interpret this in the same machine or not.

### Human-readable serialization

We talk about human-readable serialization when we translate the contents of a program into a format that humans can interpret and understand. For example, I could obtain a string representation of the value c above and write that string into a file.txt that my operating system can open for me. Then, later, if a program wants to access the value of c *and it knows* that such a value is stored in file.txt, *and it knows* how to interpret the data stored inside file.txt, then it could read the content (as a string) and cast it into, e.g., a u32.

There are many other human-readable serialization formats. Take a look at JSON, YAML for some examples. The advantage of these formats is that they offer a bit of structure, so multiple different programs, written by different people, can read and write data and exchange it. This is exactly the goal we had initially.

### More efficient serialization

Human-readable serialization formats are easy to use by humans but less efficient to process by machines. The efficiency loss takes different dimensions. First, it often takes more space to store data in a human-readable way than not. Second, it often takes more time to translate data into human-readable formats and from human-readable formats, than not. If this is not clear, don't worry; we will briefly explore this at an intuitive level below.

Let's get hands-on and explore these ideas.

## Integers, Strings, and Hybrid Data Structures

### Part 1: The very basics

Take a look at the crate serde1 we include with this module. It is very simple, if you set the variable serialize to true, then it will take an integer, 33, and will serialize it. When serializing it, it will do it in two different ways. First, by casting 33 into a string, a human-readable representation. Second, by obtaining the bytes that represent the integer 33. After serializing the data, it will write it to disk. Instead, we could have chosen to send it over the network or to send it to another process, and many more: once data is represented as bits, *anyone who knows* how to interpret those bytes can translate them into variables again, in what is called deserialization.

This crate is extremely simple, feel free to play around, modify the code, and start honing your Rust programming skills. Also, these are the steps you should follow and questions you should answer (we indicate with to-submit answer you actually have to submit for the autograder to test your code):

* Implement the functions serialize_to_string, serialize_to_bytes, deserialize_from_bytes without changing the provided signature. (to-submit)
* After running the program once (for the first time), inspect the files that the program produces. What differences do you see between the files?
* Look at the comment on the serialize_to_string function. There is a question: what's the difference between casting into a string and serializing into a string? can you answer it?
* Investigate and try out alternative ways to serialize data in human-readable format, for example, in JSON. Instead of doing such a thing manually, you can look into existing libraries to achieve the goal.
* Above we explained some differences between human-readable serialization formats and more efficient ones (such as writing content in bytes), do you understand the difference now? can you explain it to someone else?
* What happens if you change the type of the integer? Try with u16, i32, u8... and observe its representation on disk. Make sure you understand the behavior.

The crux of serializing a data structure is to find a byte representation of the same. Different computer architectures, however, will interpret bits within a byte differently. Refresh your memory between the differences between *little endian* and *big endian*. You can find your computer architecture book or take a quick look at [the Wikipedia article on Endianness](https://en.wikipedia.org/wiki/Endianness).

### Part 2: The basics

Now let's move on to the serde2 crate, also included with this module. There is a function that creates a vector of an unknown size that is only known during execution. The contents of the vector are a monotonically increasing integer that starts in 0, so if the size is 5, then the vector will contain 0, 1, 2, 3, 4. After the vector is created, your goals are to:

* serialize the vector to bytes and write them to a file, as in serde1 (to-submit)
* deserialize the vector from the bytes in the file and verify that the vector is correct, i.e., by verifying the numbers it contains are monotonically increasing (to-submit)

What is different in serde2? What challenges did you find you did not in serde1? Think carefully about these questions. After you understand the answer, consider these others:

* Say you have to serialize 2 integers and one string, does serialization order matter? why?
* What if you have a collection of different variables, each with different a type. For example, a collection of objects that represent students, with attributes such as name, last name, birth date, description of interests, and more. How would you go about serializing and deserializing this data?

### Part 3: The third-party crate 

Now, we have understood how serialization and deserialization works. You can imagine, however, how quickly serializing and deserializing complicated data structures (think nested structs, multiple data types) may become. Let's design and implement a great serde crate in part 3!! ... No panic, we don't have to [reinvent the wheel](https://en.wikipedia.org/wiki/Reinventing_the_wheel). You can see how critical is to perform this serde task correctly (so we can recover the data), and efficiently (in space and time), and how complicated. For that reason, most programming languages will have their own libraries for performing these tasks (sometimes the PL provides it in its standard library, othertimes there are external libraries, and often you will find both). Rust is no exception.

We introduce the [serde crate](https://crates.io/crates/serde), a **framework** for serializing and deserializing Rust data structures. It supports various data formats like [JSON](https://github.com/serde-rs/json), [Bincode](https://github.com/bincode-org/bincode), [CBOR](https://github.com/enarx/ciborium), [etc](https://serde.rs/#data-formats). You will find that we use serde crate to serialize logical query plans as json and tuples as vector in CrustyDB.

In the serde3 crate, which is the final crate in this module, you need to import and use this external crate to serialize and deserialize JSON and CBOR data formats:

* Recalling the above questions, we have a collection of objects that represent universities, with attributes such as name, undergraduate_enrollment, schools (a list of school names), acceptance_rate, how would you serialize and deserialize this data to/from JSON format using serde crate? (to-submit)

* What if we need to do serialization and deserialization between CBOR and JSON?

After you finish the tasks, consider this open-end question:

* You probably have noticed the generality of serde crate, but what about efficiency? Find a way to compare the performance between serde create and your implementations in serde1&2.

[Read more about serde crate](https://serde.rs/)





### Part 4: Results/Options

Please review the notes from module 3 on result and option, and fix the code in result_option to get all tests to pass.

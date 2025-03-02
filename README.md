# HookEvent_System
Hook event System from GLua on Rust

## Installation & usage

Install the Rust language using the official language or another method.
[https://www.rust-lang.org/learn/get-started]

Next, create a test project using the ```cargo``` utility

```sh
cd ~/
cargo new test_hook_project
```

After creating the project, edit the ```Cargo.toml``` file to add the dependency as a repository git.

```toml
[package]
name = "test_hook_project"
version = "0.1.0"
edition = "2024"

[dependencies]
hookevent_lib = { git = "https://github.com/Neonpk/hookevent_lib.git" }
```
Edit the ```main.rs``` file and paste the following content to test:

```Rust
// Using the crate
use hookevent_lib::Hook;

fn main() {

    /*
        Initialize hook 
            Parameters: Unit (nil)
            Returns: Option<i32> (int32 OR None)
    */
    static HOOK: Hook<(), i32> = Hook::init();

    /*
        Add new event
            Event name: Think 
            Parameters: Unit (nil)
            Returns: Option<i32> (int32 OR None)
    */
    let mut x: i32 = 5;
    let _id = HOOK.add("Think".to_string(), move |_| {
        println!("tested {}", {x += 5; x});
        Some(1337)
    });

    /* 
        Remove event named "Think"
    */
    // Commented: let _r = HOOK.remove("Think".to_string(), _id);

    /*
        Call an event named "Think"
            Parameters: Unit (nil)
    */
    let result = HOOK.call("Think".to_string(), &());

    /*
        Get the result of the event execution 
            Returns: Some(1337)
    */
    match result {
        // This entry will be executed and 1337 will be output.
        Some(r) => println!("{}", r),
        // This entry will be executed if the event is returned to Some(None) when adding the event
        None => println!("No data")
    }
}
```

Afterwards try to build the project using the ```cargo build``` command and then ```cargo run```.
After a successful launch you should receive a message like this:

Standart output:
```
tested 10
1337
```

At this stage, the setup is considered complete.

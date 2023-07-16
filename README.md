# gaia-rs
A rust library to encapsulate boilerplate code for handling environment variables at startup

## sample usage
```
use envutil;

fn add_1() {
    let num: i64 = envutil::get_int("key"); // assume an environment "key" exists
    num + 1
}
```

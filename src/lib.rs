use std::env;

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
fn get_env(env_str: &str) -> Result<String, String> {
    match env::var(env_str) {
        Ok(e) => Ok(e),
        Err(_) => {
            let msg = format!("No such environment {e}", e=env_str);
            Err(msg.to_string())
        }
    }
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn get_env_or_die(env_str: &str) -> String {
    get_env(env_str).unwrap_or_else(
        |_| {
            panic!("Could not find required env: {key}", key=env_str)
        }
    )
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn has_env(env_str: &str) -> bool {
    env::var(env_str).is_ok()
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn get_env_with_default(env_str: &str, default: &str) -> String {
    get_env(env_str).unwrap_or_else(
        |msg| {
            println!("{m} ... using defaut {d}", m=msg, d=default);
            default.to_string()
        }
    )
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
fn get_parsed<T: std::str::FromStr>(env_str: &str) -> Result<T, String> {
    match get_env(env_str) {
        Err(m) => Err(m),
        Ok(s) => {
            match s.parse::<T>() {
                Ok(v) => Ok(v),
                Err(_) => Err(format!("Failed to parse {s}", s=s))
            }
        }
    }
}   

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn get_int(env_str: &str) -> Result<i64, String> {
    get_parsed::<i64>(env_str)
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn get_int_with_default(env_str: &str, default: i64) -> i64 {
    get_int(env_str).unwrap_or_else(
        |msg| {
            println!("{m} ... using defaut {d}", m=msg, d=default);
            default
        }
    )
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn get_int_in_range(env_str: &str, min: i64, max: i64) -> Result<i64, String> {
    match get_int(env_str) {
        Err(m) => Err(m),
        Ok(i) => {
            if min <= i && i <= max {
                return Ok(i)
            } else {
                let msg = format!("{i} is not in the range [{min}, {max}]", i=i, min=min, max=max);
                return Err(msg)
            }
        }
    }
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn get_port(env_str: &str) -> Result<u16, String> {
    match get_parsed::<u16>(env_str) {
        Ok(port) => Ok(port),
        Err(_) => {
            match self::has_env(env_str) {
                true => Err(format!("invalid port: {p}", p=get_env(env_str).unwrap())),
                false => Err(format!("invalid port: UNKNOWN"))
            }
        }
    }
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn get_port_with_default(env_str: &str, default: u16) -> u16 {
    get_port(env_str).unwrap_or(default)
}

//-----------------------------------------------------------------------------
//-----------------------------------------------------------------------------
pub fn get_port_or_die(env_str: &str) -> u16 {
    get_port(env_str).unwrap_or_else(
        |_| {
            panic!("Invalid or missing port for: {key}", key=env_str)
        }
    )
}


#[test]
pub fn test_has_env() {
    let key1 = "test_key::test_has_env::1";
    let key2 = "test_key::test_has_env::2";

    env::set_var(key1, "hello");
    assert_eq!(self::has_env(key1), true);
    assert_eq!(self::has_env(key2), false);
}

#[test]
pub fn test_get_env() {
    let key1 = "test_key::test_get_env::1";
    let key2 = "test_key::test_get_env::2";

    env::set_var(key1, "bar");
    env::set_var(key2, "world");
    let result: Result<String, String> = get_env(key1);
    match result {
        Ok(v) => assert_eq!(v, "bar"),
        Err(_) => assert_eq!(true, false)
    };
}

#[test]
pub fn test_get_env_with_default() {
    let key = "test_key::test_get_env_with_default::1";
    env::remove_var(key);
    let result1: String = self::get_env_with_default(key, "abc123");
    assert_eq!(result1, "abc123");
    env::set_var(key, "bar");
    let result2: String = self::get_env_with_default(key, "abc123");
    assert_eq!(result2, "bar");
}

#[test]
pub fn test_get_env_or_die() {
    let key1 = "test_key::test_get_env_or_die::1";
    env::set_var(key1, "foo");
    assert_eq!(get_env_or_die(key1), "foo");

    let key2 = "test_key::test_get_env_or_die::2";
    env::remove_var(key2);
    let result2 = std::panic::catch_unwind(
        || get_env_or_die(key2)
     );
     assert!(result2.is_err());
}

#[test]
pub fn test_get_port() {
    let key1 = "test_key::test_get_port::1";
    env::set_var(key1, "4567");
    let result1: Result<u16, String> = self::get_port(key1);
    assert_eq!(result1.unwrap(), 4567);

    let key2 = "test_key::test_get_port::2";
    env::set_var(key2, "-4567");
    let result2: Result<u16, String> = self::get_port(key2);
    match result2 {
        Ok(_) => assert_eq!(true, false),
        Err(msg) => assert_eq!(msg, "invalid port: -4567")
    };

    let key3 = "test_key::test_get_port::3";
    env::remove_var(key3);
    let result3: Result<u16, String> = self::get_port(key3);
    match result3 {
        Ok(_) => assert_eq!(true, false),
        Err(msg) => assert_eq!(msg, "invalid port: UNKNOWN")
    };
}

#[test]
pub fn test_get_parsed() {
    let key1 = "test_key::test_get_parsed::1";
    env::set_var(key1, "-1");
    let result1: Result<i64, String> = self::get_int(key1);
    assert_eq!(result1.unwrap(), -1);

    let key2 = "test_key::test_get_parsed::2";
    env::set_var(key2, "0");
    let result2: Result<i64, String> = self::get_int(key2);
    assert_eq!(result2.unwrap(), 0);

    let key3 = "test_key::test_get_parsed::3";
    env::set_var(key3, "1234567890");
    let result3: Result<i64, String> = self::get_int(key3);
    assert_eq!(result3.unwrap(), 1234567890);

    let key4 = "test_key::test_get_parsed::4";
    env::set_var(key4, "12.34");
    let result4: Result<i64, String> = self::get_int(key4);
    match result4 {
        Ok(_) => assert_eq!(true, false),
        Err(msg) => assert_eq!(msg, "Failed to parse 12.34")
    };

    let key5 = "test_key::test_get_parsed::5";
    env::set_var(key5, "trees");
    let result5: Result<i64, String> = self::get_int(key5);
    match result5 {
        Ok(_) => assert_eq!(true, false),
        Err(msg) => assert_eq!(msg, "Failed to parse trees")
    };
}

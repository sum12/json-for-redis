use crate::rejson::*;
use jsonpath_lib::replace_with;
use redis_module::{Context, NextArg, RedisResult, RedisString, RedisValue};
use serde_json::Value;

pub fn cmd(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);

    let key = args.next_arg()?;
    let path = match args.next_string() {
        Ok(v) => v,
        Err(_) => "$".to_owned(),
    };
    args.done()?;

    let key_ptr = ctx.open_key_writable(&key);
    let key_value = key_ptr.get_value::<Value>(&REDIS_JSON_TYPE)?;

    let val = match key_value {
        Some(v) => v,
        None => {
            return Ok(RedisValue::Integer(0));
        }
    };

    if path == "$" {
        key_ptr.delete()?;
        return Ok(RedisValue::Integer(1));
    }

    let mut i = 0;
    let res = replace_with(val.clone(), path.as_str(), &mut |_| {
        i += 1;
        None
    })?;
    key_ptr.set_value(&REDIS_JSON_TYPE, res)?;
    Ok(RedisValue::Integer(i))
}

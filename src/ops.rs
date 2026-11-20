use std::sync::Arc;

use handl_ops::{Op, OpContext, OpError, OpRegistry};

use crate::key_value_store::KeyValueStore;

/// The op names declared in `kv.hnl`.
pub const KV_OPS: &[&str] = &[
    "kv.get",
    "kv.set",
    "kv.remove",
    "kv.clear",
    "kv.has",
    "kv.size",
];

fn require_one_string(args: &[serde_json::Value]) -> Result<&str, OpError> {
    let arg = args.first().ok_or(OpError::TypeMismatch(format!(
        "expected {} args, got {}",
        1, 0,
    )))?;
    arg.as_str().ok_or(OpError::TypeMismatch(format!(
        "expected {}, got {}",
        "string", "non-string",
    )))
}

fn require_two_strings(args: &[serde_json::Value]) -> Result<(&str, &str), OpError> {
    if args.len() < 2 {
        return Err(OpError::TypeMismatch(format!(
            "expected {} args, got {}",
            2,
            args.len(),
        )));
    }
    let key = args[0].as_str().ok_or(OpError::TypeMismatch(format!(
        "expected {}, got {}",
        "string", "non-string",
    )))?;
    let value = args[1].as_str().ok_or(OpError::TypeMismatch(format!(
        "expected {}, got {}",
        "string", "non-string",
    )))?;
    Ok((key, value))
}

struct GetFn {
    store: Arc<dyn KeyValueStore>,
}

impl Op for GetFn {
    fn call(
        &self,
        args: &[serde_json::Value],
        _ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        let key = require_one_string(args)?;
        let value = self.store.get(key)?;
        Ok(match value {
            Some(v) => serde_json::Value::from(v),
            None => serde_json::Value::Null,
        })
    }
}

struct SetFn {
    store: Arc<dyn KeyValueStore>,
}

impl Op for SetFn {
    fn call(
        &self,
        args: &[serde_json::Value],
        _ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        let (key, value) = require_two_strings(args)?;
        self.store.set(key, value)?;
        Ok(serde_json::Value::Null)
    }
}

struct RemoveFn {
    store: Arc<dyn KeyValueStore>,
}

impl Op for RemoveFn {
    fn call(
        &self,
        args: &[serde_json::Value],
        _ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        let key = require_one_string(args)?;
        self.store.remove(key)?;
        Ok(serde_json::Value::Null)
    }
}

struct ClearFn {
    store: Arc<dyn KeyValueStore>,
}

impl Op for ClearFn {
    fn call(
        &self,
        _args: &[serde_json::Value],
        _ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        self.store.clear()?;
        Ok(serde_json::Value::Null)
    }
}

struct HasFn {
    store: Arc<dyn KeyValueStore>,
}

impl Op for HasFn {
    fn call(
        &self,
        args: &[serde_json::Value],
        _ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        let key = require_one_string(args)?;
        self.store
            .has(key)
            .map(serde_json::Value::from)
            .map_err(Into::into)
    }
}

struct SizeFn {
    store: Arc<dyn KeyValueStore>,
}

impl Op for SizeFn {
    fn call(
        &self,
        _args: &[serde_json::Value],
        _ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        self.store
            .size()
            .map(|n| serde_json::Value::from(serde_json::json!(n)))
            .map_err(Into::into)
    }
}

/// Build a [`handl_ops::OpRegistry`] for the kv ops, registering the Rust
/// ops that delegate to the given [`KeyValueStore`] under their op names.
///
/// The host imports this kernel registry via
/// [`handl_plugin_lib_handl::OpRegistry::import_kernel`].
pub fn kv_op_registry(store: impl KeyValueStore + 'static) -> OpRegistry {
    let store: Arc<dyn KeyValueStore> = Arc::new(store);
    let mut registry = OpRegistry::new();
    registry.register(
        "kv.get",
        Arc::new(GetFn {
            store: store.clone(),
        }),
    );
    registry.register(
        "kv.set",
        Arc::new(SetFn {
            store: store.clone(),
        }),
    );
    registry.register(
        "kv.remove",
        Arc::new(RemoveFn {
            store: store.clone(),
        }),
    );
    registry.register(
        "kv.clear",
        Arc::new(ClearFn {
            store: store.clone(),
        }),
    );
    registry.register(
        "kv.has",
        Arc::new(HasFn {
            store: store.clone(),
        }),
    );
    registry.register("kv.size", Arc::new(SizeFn { store }));
    registry
}

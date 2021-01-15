// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

use crate::resolve_addr::resolve_addr_sync;
use deno_core::error::generic_error;
use deno_core::error::AnyError;
use deno_core::serde_json;
use deno_core::serde_json::json;
use deno_core::serde_json::Value;
use deno_core::AsyncRefCell;
use deno_core::CancelHandle;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ZeroCopyBuf;
use serde::Deserialize;
use std::borrow::Cow;
use std::rc::Rc;

pub fn init(rt: &mut deno_core::JsRuntime) {
  super::reg_json_sync(rt, "op_bind_endpoint", op_bind_endpoint);
  // super::reg_json_async(rt, "op_connect_endpoint", op_connect_endpoint);
  // super::reg_json_async(rt, "op_listen_endpoint", op_listen_endpoint);
  // super::reg_json_sync(rt, "op_close_endpoint", op_close_endpoint);
}

struct QuicEndpointResource {
  _endpoint: AsyncRefCell<quinn::Endpoint>,
  cancel: CancelHandle,
}

impl Resource for QuicEndpointResource {
  fn name(&self) -> Cow<str> {
    "quicEndpoint".into()
  }

  fn close(self: Rc<Self>) {
    self.cancel.cancel();
  }
}

#[derive(Deserialize)]
pub(crate) struct BindArgs {
  pub hostname: String,
  pub port: u16,
}

fn op_bind_endpoint(
  state: &mut OpState,
  args: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let args: BindArgs = serde_json::from_value(args)?;
  let addr = resolve_addr_sync(&args.hostname, args.port)?
    .next()
    .ok_or_else(|| generic_error("No resolved address found"))?;

  let (endpoint, _) = quinn::Endpoint::builder().bind(&addr)?;
  let local_addr = endpoint.local_addr()?;
  let endpoint_resource = QuicEndpointResource {
    _endpoint: AsyncRefCell::new(endpoint),
    cancel: Default::default(),
  };
  let rid = state.resource_table.add(endpoint_resource);
  // dbg!(&rid);
  // dbg!(&local_addr);
  Ok(json!({
    "rid": rid,
    "hostname": local_addr.ip().to_string(),
    "port": local_addr.port(),
  }))
}


impl_attr!(I32Attr, "i32", "wasm");
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct I32Attr {
    value: i32,
}

impl_attr!(FuncrefAttr, "funcref", "wasm");
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FuncrefAttr {}


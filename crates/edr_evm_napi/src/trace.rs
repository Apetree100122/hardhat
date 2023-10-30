use std::mem;

use edr_eth::{Address, Bytes};
use edr_evm::{trace::BeforeMessage, Bytecode, OPCODE_JUMPMAP};
use napi::{
    bindgen_prelude::{BigInt, Buffer},
    Env, JsBuffer, JsBufferValue,
};
use napi_derive::napi;

use crate::{cast::TryCast, transaction::result::ExecutionResult};

#[napi(object)]
pub struct TracingMessage {
    /// Sender address
    #[napi(readonly)]
    pub caller: Buffer,

    /// Recipient address. None if it is a Create message.
    #[napi(readonly)]
    pub to: Option<Buffer>,

    /// Transaction gas limit
    #[napi(readonly)]
    pub gas_limit: BigInt,

    /// Depth of the message
    #[napi(readonly)]
    pub depth: u8,

    /// Input data of the message
    #[napi(readonly)]
    pub data: JsBuffer,

    /// Value sent in the message
    #[napi(readonly)]
    pub value: BigInt,

    /// Address of the code that is being executed. Can be different from `to`
    /// if a delegate call is being done.
    #[napi(readonly)]
    pub code_address: Option<Buffer>,

    /// Code of the contract that is being executed.
    #[napi(readonly)]
    pub code: Option<JsBuffer>,
}

impl TracingMessage {
    pub fn new(env: &Env, message: &BeforeMessage) -> napi::Result<Self> {
        let data = message.data.clone();
        let data = unsafe {
            env.create_buffer_with_borrowed_data(
                data.as_ptr(),
                data.len(),
                data,
                |data: edr_eth::Bytes, _env| {
                    mem::drop(data);
                },
            )
        }
        .map(JsBufferValue::into_raw)?;

        let code = message.code.as_ref().map_or(Ok(None), |code| {
            let code = code.original_bytes();

            unsafe {
                env.create_buffer_with_borrowed_data(
                    code.as_ptr(),
                    code.len(),
                    code,
                    |code: edr_eth::Bytes, _env| {
                        mem::drop(code);
                    },
                )
            }
            .map(JsBufferValue::into_raw)
            .map(Some)
        })?;

        Ok(TracingMessage {
            caller: Buffer::from(message.caller.as_bytes()),
            to: message.to.map(|to| Buffer::from(to.as_bytes())),
            gas_limit: BigInt::from(message.gas_limit),
            depth: message.depth as u8,
            data,
            value: BigInt {
                sign_bit: false,
                words: message.value.into_limbs().to_vec(),
            },
            code_address: message
                .code_address
                .map(|address| Buffer::from(address.to_vec())),
            code,
        })
    }
}

impl TryCast<BeforeMessage> for TracingMessage {
    type Error = napi::Error;

    fn try_cast(self) -> napi::Result<BeforeMessage> {
        let to = self.to.map(|to| Address::from_slice(to.as_ref()));
        let data = Bytes::copy_from_slice(self.data.into_value()?.as_ref());
        let value = BigInt::try_cast(self.value)?;
        let code_address = self
            .code_address
            .map(|code_address| Address::from_slice(code_address.as_ref()));
        let code = self
            .code
            .map::<napi::Result<_>, _>(|code| {
                Ok(Bytecode::new_raw(Bytes::copy_from_slice(
                    code.into_value()?.as_ref(),
                )))
            })
            .transpose()?;

        Ok(BeforeMessage {
            depth: self.depth as usize,
            caller: Address::from_slice(self.caller.as_ref()),
            to,
            gas_limit: BigInt::try_cast(self.gas_limit)?,
            data,
            value,
            code_address,
            code,
        })
    }
}

#[napi(object)]
pub struct TracingStep {
    /// Call depth
    #[napi(readonly)]
    pub depth: u8,
    /// The program counter
    #[napi(readonly)]
    pub pc: BigInt,
    /// The executed op code
    #[napi(readonly)]
    pub opcode: String,
    /// The top entry on the stack. None if the stack is empty.
    #[napi(readonly)]
    pub stack_top: Option<BigInt>,
    // /// The return value of the step
    // #[napi(readonly)]
    // pub return_value: u8,
    // /// The amount of gas that was used by the step
    // #[napi(readonly)]
    // pub gas_cost: BigInt,
    // /// The amount of gas that was refunded by the step
    // #[napi(readonly)]
    // pub gas_refunded: BigInt,
    // /// The amount of gas left
    // #[napi(readonly)]
    // pub gas_left: BigInt,
    // /// The stack
    // #[napi(readonly)]
    // pub stack: Vec<BigInt>,
    // /// The memory
    // #[napi(readonly)]
    // pub memory: Buffer,
    // /// The contract being executed
    // #[napi(readonly)]
    // pub contract: Account,
    // /// The address of the contract
    // #[napi(readonly)]
    // pub contract_address: Buffer,
    // /// The address of the code being executed
    // #[napi(readonly)]
    // pub code_address: Buffer,
}

impl TracingStep {
    pub fn new(step: &edr_evm::trace::Step) -> Self {
        Self {
            depth: step.depth as u8,
            pc: BigInt::from(step.pc),
            opcode: OPCODE_JUMPMAP[usize::from(step.opcode)]
                .unwrap_or("")
                .to_string(),
            stack_top: step.stack_top.map(|v| BigInt {
                sign_bit: false,
                words: v.into_limbs().to_vec(),
            }),
            // gas_cost: BigInt::from(0u64),
            // gas_refunded: BigInt::from(0u64),
            // gas_left: BigInt::from(0u64),
            // stack: Vec::new(),
            // memory: Buffer::from(Vec::new()),
            // contract: Account::from(step.contract),
            // contract_address: Buffer::from(step.contract_address.to_vec()),
        }
    }
}

#[napi(object)]
pub struct TracingMessageResult {
    /// Execution result
    #[napi(readonly)]
    pub execution_result: ExecutionResult,
}

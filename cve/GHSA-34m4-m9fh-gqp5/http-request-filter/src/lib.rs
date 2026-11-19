//! Ruby2Shell Case Conversion Overflow Guard.
//!
//! GHSA-34m4-m9fh-gqp5 (CVE pending; sharp-s payload rule).
//!
//! Ruby's `rb_str_casemap` (string.c) allocates its mapping buffer as
//! `offsetof(mapping_buffer, space) + capa` with `capa = len*n + 20`, and
//! the single-byte `case_map` (ISO-8859-x / Windows-125x) checks room for
//! only one byte before writing two for the sharp s (0xDF). A string with
//! 21 or more sharp-s bytes therefore writes one byte past the allocation
//! onto the next heap chunk's size field (`S` = 0x53 when upcasing or
//! swapcasing, `s` = 0x73 when folding), a one-byte heap overflow reachable
//! from `upcase`, `swapcase`, `downcase(:fold)` and `casecmp?`.
//!
//! The module refuses the request when any decoded request part (request
//! line, header block, or body) carries at least THRESHOLD sharp-s bytes
//! that are not valid UTF-8 lead bytes. Valid UTF-8 never contains a lone
//! 0xDF (every 0xDF lead is followed by one continuation byte), so UTF-8
//! traffic never trips the rule. The scan covers both the raw bytes and
//! the percent-decoded stream, so the trigger shape is caught whether it
//! rides the path, the query string, header values, a form body or a raw
//! (multipart/binary) body.
//!
//! Stated limits: the count is per request part; a payload split across
//! parts that an application later concatenates before case conversion is
//! not covered. Values already case-converted as ASCII-8BIT (binary) do not
//! overflow in Ruby, but the module cannot observe the application's
//! encoding choice, so it refuses those shapes as well.
//!
//! no_std; no allocator, storage, background work or state.

#![no_std]

use core::mem::size_of;
use core::panic::PanicInfo;
use eden_gateway_remediation::message::{Availability, MessageInput};
use eden_gateway_remediation::module::{
    ApiHeader, CONTINUE, Decision, EDIT, EditSink, Edits, Failure, ModuleOutput, NEED_MORE,
    REFUSE, RemediationModule,
};
use eden_gateway_remediation::native_http_v3::{self as http, Api, Input, Request};

const SHARP: u8 = 0xDF;
const THRESHOLD: u32 = 21;

fn is_hex(b: u8) -> bool {
    (b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F')
}

fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        _ => b - b'A' + 10,
    }
}

fn is_continuation(b: u8) -> bool {
    (0x80..=0xBF).contains(&b)
}

/// Count sharp-s bytes that are not valid UTF-8 lead pairs, in the raw bytes.
fn lone_sharp_raw(src: &[u8]) -> u32 {
    let mut count = 0u32;
    let mut i = 0usize;
    let n = src.len();
    while i < n {
        if src[i] == SHARP {
            if i + 1 < n && is_continuation(src[i + 1]) {
                // valid UTF-8 two-byte sequence: never a sharp s
                i += 2;
            } else {
                count += 1;
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    count
}

/// Pull the next percent-decoded byte from `src`, advancing `i`. Returns
/// None at end of input. A malformed escape yields the literal bytes.
fn next_decoded(src: &[u8], i: &mut usize) -> Option<u8> {
    let n = src.len();
    if *i >= n {
        return None;
    }
    let b = src[*i];
    if b == b'%' && *i + 2 < n && is_hex(src[*i + 1]) && is_hex(src[*i + 2]) {
        let byte = (hex_val(src[*i + 1]) << 4) | hex_val(src[*i + 2]);
        *i += 3;
        Some(byte)
    } else {
        *i += 1;
        Some(b)
    }
}

/// Count sharp-s bytes that are not valid UTF-8 lead pairs, in the
/// percent-decoded stream. Runs in O(n) with one byte of lookahead.
fn lone_sharp_decoded(src: &[u8]) -> u32 {
    let mut count = 0u32;
    let mut i = 0usize;
    let mut pending: Option<u8> = None;
    loop {
        let b = match pending.take() {
            Some(b) => b,
            None => match next_decoded(src, &mut i) {
                Some(b) => b,
                None => break,
            },
        };
        if b != SHARP {
            continue;
        }
        // one decoded byte of lookahead decides lead-pair vs lone sharp s
        let next = match pending.take().or_else(|| next_decoded(src, &mut i)) {
            Some(b) => b,
            None => {
                count += 1;
                break;
            }
        };
        if is_continuation(next) {
            // valid UTF-8 pair in the decoded stream: not a sharp s
        } else {
            count += 1;
            pending = Some(next);
        }
    }
    count
}

fn part_is_attack(src: &[u8]) -> bool {
    lone_sharp_raw(src) >= THRESHOLD || lone_sharp_decoded(src) >= THRESHOLD
}

struct Ruby2ShellCaseConversionOverflowGuard;

impl RemediationModule<Request<'_>> for Ruby2ShellCaseConversionOverflowGuard {
    fn evaluate(&self, request: &Request<'_>, edits: &mut Edits<'_>) -> Result<Decision, Failure> {
        let _ = edits;
        let _ = match request.header_count() {
            Availability::Ready(count) => count,
            Availability::NeedMore => return Ok(Decision::NeedMore),
            Availability::Unavailable => return Err(Failure),
        };
        let message = request.message();
        if !message.is_complete() {
            return Ok(Decision::NeedMore);
        }
        // The head covers the request line (method, path, query) and every
        // header value; the body covers form fields and raw/multipart parts.
        let head = message.prefix();
        if part_is_attack(head) {
            return Ok(Decision::Refuse);
        }
        let body_range = match request.body_range() {
            Availability::Ready(range) => range,
            Availability::NeedMore => return Ok(Decision::NeedMore),
            Availability::Unavailable => return Err(Failure),
        };
        let body = match message.bytes(body_range)? {
            Availability::Ready(bytes) => bytes,
            Availability::NeedMore => return Ok(Decision::NeedMore),
            Availability::Unavailable => return Err(Failure),
        };
        if part_is_attack(body) {
            return Ok(Decision::Refuse);
        }
        Ok(Decision::Continue)
    }
}

static API: Api = Api {
    header: ApiHeader { version: http::VERSION, table_size: size_of::<Api>() as u32 },
    input_size: size_of::<Input>() as u32,
    output_size: size_of::<ModuleOutput>() as u32,
    sink_size: size_of::<EditSink>() as u32,
    message_size: size_of::<MessageInput>() as u32,
    state_bytes: 0,
    capabilities: CONTINUE | REFUSE | NEED_MORE | EDIT,
    required_inputs: 0,
    evaluate: Some(evaluate),
};

unsafe extern "C" fn evaluate(input: *const Input, sink: *const EditSink, output: *mut ModuleOutput) -> i32 {
    unsafe { http::invoke(&Ruby2ShellCaseConversionOverflowGuard, input, sink, output) }
}

#[unsafe(no_mangle)]
pub extern "C" fn eden_http_remediation_v3() -> *const Api { &API }

#[panic_handler]
fn panic(_information: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
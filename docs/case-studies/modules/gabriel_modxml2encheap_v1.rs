//! Gabriel module: refuse SQL LIKE wildcards in the robot-list query
//! (CVE-2026-110697 shape)
//!
//! Gabriel module: CVE-2026-42536 (head_val_len rule, lab fixture delivery shape).
//!
//! Generated for the wave-12 Apache HTTP Server / AdonisJS sweep.
//! Refuses the request shape that triggers the advisory's primitive on
//! route /xml-conv; ordinary traffic passes unchanged.
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

const ROUTE: &[u8] = b"/xml-conv";
const LIMIT: usize = 48;
const PREFIX: &[u8] = &[99, 104, 97, 114, 115, 101, 116, 61];

fn find_ignore_case(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if haystack.len() < needle.len() || needle.is_empty() { return None; }
    let mut i = 0usize;
    while i + needle.len() <= haystack.len() {
        let mut j = 0usize; let mut same = true;
        while j < needle.len() {
            if haystack[i + j].to_ascii_lowercase() != needle[j].to_ascii_lowercase() { same = false; break; }
            j += 1;
        }
        if same { return Some(i); }
        i += 1;
    }
    None
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if haystack.len() < needle.len() || needle.is_empty() { return None; }
    let mut i = 0usize;
    while i + needle.len() <= haystack.len() {
        let mut j = 0usize; let mut same = true;
        while j < needle.len() {
            if haystack[i + j] != needle[j] { same = false; break; }
            j += 1;
        }
        if same { return Some(i); }
        i += 1;
    }
    None
}

fn is_space(b: u8) -> bool { b == b' ' || b == b'\t' }

struct gabriel_modxml2encheap_v1;

impl RemediationModule<Request<'_>> for gabriel_modxml2encheap_v1 {
    fn evaluate(&self, request: &Request<'_>, edits: &mut Edits<'_>) -> Result<Decision, Failure> {
        let _ = edits;
        let _ = match request.header_count() {
            Availability::Ready(count) => count,
            Availability::NeedMore => return Ok(Decision::NeedMore),
            Availability::Unavailable => return Err(Failure),
        };
        let message = request.message();
        if !message.is_complete() { return Ok(Decision::NeedMore); }
        let head = message.prefix();
        let line_end = head.iter().position(|&b| b == b'\r').unwrap_or(head.len());
        let request_line = &head[..line_end];
        let path_at = request_line.iter().position(|&b| b == b' ').unwrap_or(request_line.len());
        let target = &request_line[path_at + 1..];
        if !target.starts_with(ROUTE) { return Ok(Decision::Continue); }
        let Some(at) = find_ignore_case(head, PREFIX) else { return Ok(Decision::Continue); };
        let mut v = at + PREFIX.len();
        while v < head.len() && (head[v] == b' ' || head[v] == b'\t') { v += 1; }
        let mut end = v;
        while end + 1 < head.len() && !(head[end] == b'\r' && head[end + 1] == b'\n') { end += 1; }
        if end - v > LIMIT { return Ok(Decision::Refuse); }
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
    unsafe { http::invoke(&gabriel_modxml2encheap_v1, input, sink, output) }
}

#[unsafe(no_mangle)]
pub extern "C" fn eden_http_remediation_v3() -> *const Api { &API }

#[panic_handler]
fn panic(_information: &PanicInfo<'_>) -> ! {
    loop { core::hint::spin_loop(); }
}
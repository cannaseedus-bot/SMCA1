// cm1-verify.rs
// CM-1 Verifier CLI (reference)
// Version: v1.0 (frozen)

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Init,
    NullZone,
    Header,
    Body,
    Scope,
    Literal,
    End,
    Error,
}

#[derive(Clone, Copy, Debug)]
enum ReturnState {
    Header,
    Body,
}

#[derive(Debug)]
struct ScopeStack {
    depth: usize,
    return_state: Option<ReturnState>,
}

impl ScopeStack {
    fn new() -> Self {
        Self {
            depth: 0,
            return_state: None,
        }
    }

    fn push(&mut self, return_state: ReturnState) {
        if self.depth == 0 {
            self.return_state = Some(return_state);
        }
        self.depth = self.depth.saturating_add(1);
    }

    fn pop(&mut self) -> Option<ReturnState> {
        if self.depth == 0 {
            return None;
        }
        self.depth -= 1;
        if self.depth == 0 {
            self.return_state.take()
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.depth == 0
    }
}

const RESULT_PASS: &str = "{\"@cm1\":\"PASS\"}";
const RESULT_FAIL: &str = "{\"@cm1\":\"FAIL\"}";

fn usage_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    eprintln!("usage: cm1-verify <path>");
    std::process::exit(64);
}

fn parse_args() -> PathBuf {
    let mut args = env::args().skip(1);
    let Some(target) = args.next() else {
        usage_error("missing target");
    };

    if args.next().is_some() {
        usage_error("too many targets");
    }

    let target = PathBuf::from(target);
    let target_str = target.to_string_lossy();
    if target_str == "-" || target_str.contains("://") || target_str.starts_with("//") {
        usage_error("forbidden input target");
    }

    target
}

fn read_bytes(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).unwrap_or_else(|_| {
        eprintln!("cannot open target: {}", path.display());
        std::process::exit(66);
    });
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap_or_else(|_| {
        eprintln!("cannot read target: {}", path.display());
        std::process::exit(66);
    });
    bytes
}

fn is_allowed_symbol(byte: u8) -> bool {
    matches!(
        byte,
        0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x0E | 0x0F | 0x10 | 0x1C | 0x1D | 0x1E |
            0x1F | 0x20
    )
}

fn emit_illegal(offset: usize, byte: u8) -> ! {
    println!(
        "{{\"@cm1\":\"ILLEGAL\",\"offset\":{},\"symbol\":\"U+{:04X}\"}}",
        offset, byte
    );
    std::process::exit(65);
}

fn verify_stream(bytes: &[u8]) -> bool {
    let mut state = State::Init;
    let mut scope_stack = ScopeStack::new();
    let mut literal_mask = false;
    let mut last_control: Option<u8> = None;

    for (offset, byte) in bytes.iter().copied().enumerate() {
        if !is_allowed_symbol(byte) {
            emit_illegal(offset, byte);
        }

        last_control = Some(byte);

        state = match state {
            State::Init => match byte {
                0x00 => State::NullZone,
                0x01 => State::Header,
                0x20 => State::Init,
                _ => State::Error,
            },
            State::NullZone => match byte {
                0x00 => State::NullZone,
                0x01 => State::Header,
                _ => State::Error,
            },
            State::Header => match byte {
                0x02 => State::Body,
                0x0E => {
                    scope_stack.push(ReturnState::Header);
                    State::Scope
                }
                0x04 => State::End,
                0x20 => State::Header,
                _ => State::Error,
            },
            State::Body => match byte {
                0x03 => State::Header,
                0x0E => {
                    scope_stack.push(ReturnState::Body);
                    State::Scope
                }
                0x10 => {
                    literal_mask = true;
                    State::Literal
                }
                0x1C | 0x1D | 0x1E | 0x1F | 0x20 => State::Body,
                _ => State::Error,
            },
            State::Scope => match byte {
                0x0E => {
                    scope_stack.push(ReturnState::Body);
                    State::Scope
                }
                0x0F => {
                    if scope_stack.depth == 0 {
                        State::Error
                    } else {
                        let return_state = scope_stack.pop();
                        if let Some(return_state) = return_state {
                            match return_state {
                                ReturnState::Header => State::Header,
                                ReturnState::Body => State::Body,
                            }
                        } else {
                            State::Scope
                        }
                    }
                }
                _ => State::Error,
            },
            State::Literal => match byte {
                0x10 => {
                    literal_mask = false;
                    State::Body
                }
                _ => State::Literal,
            },
            State::End => match byte {
                0x20 => State::End,
                _ => State::Error,
            },
            State::Error => State::Error,
        };

        if state == State::Error {
            return false;
        }
    }

    if state != State::End {
        return false;
    }

    if !scope_stack.is_empty() || literal_mask {
        return false;
    }

    matches!(last_control, Some(0x04))
}

fn main() {
    let target = parse_args();
    let bytes = read_bytes(&target);

    let ok = verify_stream(&bytes);
    if ok {
        println!("{}", RESULT_PASS);
        std::process::exit(0);
    }

    println!("{}", RESULT_FAIL);
    std::process::exit(1);
}

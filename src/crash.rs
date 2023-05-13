use backtrace;
use chrono::prelude::*;
use log::error;
use winapi::um::errhandlingapi::SetUnhandledExceptionFilter;
use winapi::um::minwinbase as ErrorCode;
use winapi::um::winnt::{EXCEPTION_POINTERS, LONG};

use std::fs::File;
use std::io::Write;
use std::writeln;

use crate::config::CONFIG;
use crate::utils;

#[allow(dead_code)]
enum CrashResult {
  ExecuteHandler = 1,
  ContinueSearch = 0,
  ContinueExecution = -1,
}

pub unsafe fn install() {
  SetUnhandledExceptionFilter(Some(handler));
}

pub unsafe extern "system" fn handler(exception_info: *mut EXCEPTION_POINTERS) -> LONG {
  let record = *(*exception_info).ExceptionRecord;

  if record.ExceptionCode == ErrorCode::EXCEPTION_STACK_OVERFLOW {
    utils::message_box(
      "StackOverflow!",
      "dfint hook error",
      utils::MessageIconType::Error,
    );
    return CrashResult::ContinueExecution as LONG;
  }

  let stack = stacktrace(Some(record.ExceptionAddress as usize));
  let dt: DateTime<Utc> = Utc::now();
  let cr_filename = format!("cr_{}.txt", dt.format("%Y-%m-%d-%H-%M-%S"));
  let mut file = File::create(format!(
    "{}{}",
    CONFIG.settings.crash_report_dir, cr_filename
  ))
  .unwrap();
  writeln!(
    &mut file,
    "-----------------------------
Version: {}
Cheksum: {:x}
Error: {}
Address: {:?}
------------STACK------------
{}
------------STACK------------",
    CONFIG.offset.version,
    CONFIG.offset.checksum,
    code_to_str(record.ExceptionCode),
    record.ExceptionAddress,
    stack
  )
  .unwrap();

  error!(
    "crash occured, error {}, address {:?}, crashlog {}",
    code_to_str(record.ExceptionCode),
    record.ExceptionAddress,
    cr_filename
  );

  utils::message_box(
    format!(
      "Oops, it's a crash!\nError: {}\nAddress: {:?}\nCrashlog: {}\n",
      code_to_str(record.ExceptionCode),
      record.ExceptionAddress,
      cr_filename
    )
    .as_str(),
    "dfint hook error",
    utils::MessageIconType::Error,
  );

  CrashResult::ExecuteHandler as LONG
}

fn stacktrace(target: Option<usize>) -> String {
  let mut trace = Vec::<String>::new();
  let mut i = 1;
  backtrace::trace(|frame| {
    let ip = frame.ip();
    let symbol_address = frame.symbol_address();
    let ma = frame.module_base_address().unwrap();
    let offset = symbol_address as usize - ma as usize;

    let mut name = String::from("");
    backtrace::resolve_frame(frame, |symbol| {
      name += format!("{}", symbol.name().unwrap()).as_str()
    });
    if name == "" {
      name += "unknown";
    }
    let mut is_target = String::from("");
    if target.unwrap() == ip as usize {
      is_target += ">";
    }
    trace.push(format!("{}:{} {} + 0x{:X}", i, is_target, name, offset));
    i += 1;
    true
  });
  trace.join("\n")
}

fn code_to_str(code: u32) -> &'static str {
  match code {
    ErrorCode::EXCEPTION_ACCESS_VIOLATION => "EXCEPTION_ACCESS_VIOLATION",
    ErrorCode::EXCEPTION_ARRAY_BOUNDS_EXCEEDED => "EXCEPTION_ARRAY_BOUNDS_EXCEEDED",
    ErrorCode::EXCEPTION_BREAKPOINT => "EXCEPTION_BREAKPOINT",
    ErrorCode::EXCEPTION_DATATYPE_MISALIGNMENT => "EXCEPTION_DATATYPE_MISALIGNMENT",
    ErrorCode::EXCEPTION_FLT_DENORMAL_OPERAND => "EXCEPTION_FLT_DENORMAL_OPERAND",
    ErrorCode::EXCEPTION_FLT_DIVIDE_BY_ZERO => "EXCEPTION_FLT_DIVIDE_BY_ZERO",
    ErrorCode::EXCEPTION_FLT_INEXACT_RESULT => "EXCEPTION_FLT_INEXACT_RESULT",
    ErrorCode::EXCEPTION_FLT_INVALID_OPERATION => "EXCEPTION_FLT_INVALID_OPERATION",
    ErrorCode::EXCEPTION_FLT_OVERFLOW => "EXCEPTION_FLT_OVERFLOW",
    ErrorCode::EXCEPTION_FLT_STACK_CHECK => "EXCEPTION_FLT_STACK_CHECK",
    ErrorCode::EXCEPTION_FLT_UNDERFLOW => "EXCEPTION_FLT_UNDERFLOW",
    ErrorCode::EXCEPTION_ILLEGAL_INSTRUCTION => "EXCEPTION_ILLEGAL_INSTRUCTION",
    ErrorCode::EXCEPTION_IN_PAGE_ERROR => "EXCEPTION_IN_PAGE_ERROR",
    ErrorCode::EXCEPTION_INT_DIVIDE_BY_ZERO => "EXCEPTION_INT_DIVIDE_BY_ZERO",
    ErrorCode::EXCEPTION_INT_OVERFLOW => "EXCEPTION_INT_OVERFLOW",
    ErrorCode::EXCEPTION_INVALID_DISPOSITION => "EXCEPTION_INVALID_DISPOSITION",
    ErrorCode::EXCEPTION_NONCONTINUABLE_EXCEPTION => "EXCEPTION_NONCONTINUABLE_EXCEPTION",
    ErrorCode::EXCEPTION_PRIV_INSTRUCTION => "EXCEPTION_PRIV_INSTRUCTION",
    ErrorCode::EXCEPTION_SINGLE_STEP => "EXCEPTION_SINGLE_STEP",
    ErrorCode::EXCEPTION_STACK_OVERFLOW => "EXCEPTION_STACK_OVERFLOW",
    _ => "Unrecognized Exception",
  }
}

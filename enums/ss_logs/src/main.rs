/* 
Instructions
In this exercise you'll be generating semi-structured log messages.

1. Emit semi-structured messages
You'll start with some stubbed functions and the following enum:

#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
Your goal is to emit a log message as follows: "[<LEVEL>]: <MESSAGE>". You'll need to implement functions that correspond with log levels.

For example, the below snippet demonstrates an expected output for the log function.

log(LogLevel::Error, "Stack overflow")
// Returns: "[ERROR]: Stack overflow"
And for info:

info("Timezone changed")
// Returns: "[INFO]: Timezone changed"
*/

// SOLUTION

* Separating concerns - main.rs handles argument parsing and high-level flow, while lib.rs contains the core logic for searching.
* Error handling - instead of panicking on errors, try to use Result types and propagate errors up to the caller.
* Logging - use println! to log to standard output and eprintln! to log errors to standard error.
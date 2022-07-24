# Rs-Logger

A Rust logging library.

## Installation

Add rs-logger = "0.1.4" in you Cargo.toml file under depndencies

## Code Examples

There are five differrent types of logs.

* Debug
* Info
* Warning
* Error
* Critical
  
```
use rs_logger::*;

fn main() {
    let mut logger = Logger::default(); // New Logger object

    logger.set_level(LoggingLevel::LevelTwo); // LevelTwo ignores debug logs

    logger.error("date = %D, time = %T 'Error message' "); // %D = UTC date, %T = UTC time
    logger.debug("Debug"); // ignored
}

```

### Levels

1 => Logs everything

2 => Ignores Debug

3 => Ignores Debug + Info

4 => Ignores Debug + Info + Warning

5 => Ignores everything except critical

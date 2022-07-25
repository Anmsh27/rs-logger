# Rs-Logger

A Rust logging library.

## Installation

Add rs-logger = "0.1.5" in you Cargo.toml file under depndencies

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
    let mut logger = Logger::default(); // New Logger object and Config object is already instantiated

    logger.set_level(LoggingLevel::LevelTwo); // LevelTwo ignores debug logs

    logger.error("date = %D, time = %T 'Error message' "); // %D = UTC date, %T = UTC time
    logger.debug("Debug"); // ignored
}

```

or for more control

```
fn main() {
    let config = Config::new() // new config
        .filename("logs.json")
        .json(true)
    
    let mut logger = Logger::new(LoggingLevel::LevelOne, config).unwrap();

    logger.debug("It works");
    logger.info("Current date and time: %D %T ");
}
```

### Levels

1 => Logs everything

2 => Ignores Debug

3 => Ignores Debug + Info

4 => Ignores Debug + Info + Warning

5 => Ignores everything except critical


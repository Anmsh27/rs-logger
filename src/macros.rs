
/// String macro for making a new json string
/// Doesn't need to be a macro but I still wrote it as one
#[macro_export]
macro_rules! string {
    ( $x:expr ) => {{
        let string = json::JsonValue::String($x.to_string());
        string
    }};
}

/// Number macro for making a new json numbew
/// Doesn't need to be a macro but I still wrote it as one
#[macro_export]
macro_rules! number {
    ( $x:expr ) => {{
        let number = json::JsonValue::Number(json::number::Number::from($x));
        number
    }};
}

/// Bool macro for making a new json bool
/// Doesn't need to be a macro but I still wrote it as one
#[macro_export]
macro_rules! bool {
    ( $x:expr ) => {{
        let boolean = json::JsonValue::Boolean($x);
        boolean
    }};
}

/// Array macro for making a new json array
#[macro_export]
macro_rules! json_array {
    ( $( $x:expr ),* ) => {
        {
            let json_array = json::JsonValue::Array(json::Array::new());
            $(
                json_array.push($x).unwrap_or_else(|error| {
                    println!("{}", format!("Logger Error: {}", error).red());
                    panic!();
                });
            )*
            json_array
        }
    };
    ( ) => {
        let mut json_array = json::JsonValue::Array(json::Array::new());
        json_array 
    }
}

enum StringResult {
    StringOK(String),
    ErrorReason(String),
}

fn respond(greeting: &str) -> StringResult {
    if greeting == "Hello" {
        StringResult::StringOK("Good morning!" . to_string())
    } else {
        StringResult::ErrorReason("I didn't understand you!" . to_string())
    }
}

fn main() {
	match respond("Hello") {
        		StringResult::StringOK(n) => println!("{}", n),
        		StringResult::ErrorReason(n) => println!("{}",n),
    	}
	match respond("How are you") {
        		StringResult::StringOK(n) => println!("{}", n),
        		StringResult::ErrorReason(n) => println!("{}",n),
    	}    	
}
enum StringResult {
    StringOK(String),
    ErrorReason(String),
}

fn respond(greeting: &str) -> StringResult {
    if greeting == "Hello" {
        StringResult.StringOK("Good morning!".to_string())
    } else {
        StringResult.ErrorReason("I didn't understand you!".to_string())
    }
}

fn main() {
	let ret = respond("Hello");

}
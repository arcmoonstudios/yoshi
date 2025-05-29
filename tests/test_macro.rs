// Test file to debug macro expansion

fn main() {
    // Test the macro expansion
    println!("stringify!(severity) = {}", stringify!(severity));
    println!("stringify!(suggestion) = {}", stringify!(suggestion));
    
    // Test if the issue is with the match
    let key = "severity";
    let result = match key {
        "severity" => "matched severity",
        "suggestion" => "matched suggestion", 
        _ => "no match",
    };
    println!("Manual match result: {}", result);
}

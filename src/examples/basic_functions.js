// Basic function calls example for time travel debugger
function add(a, b) {
    console.log("Adding", a, "and", b);
    let result = a + b;
    console.log("Result:", result);
    return result;
}

function multiply(a, b) {
    console.log("Multiplying", a, "and", b);
    let result = a * b;
    console.log("Result:", result);
    return result;
}

function main() {
    console.log("🎯 Starting basic functions example");
    
    let x = 5;
    let y = 10;
    
    console.log("Initial values: x =", x, ", y =", y);
    
    let sum = add(x, y);
    let product = multiply(x, y);
    
    console.log("Final results:");
    console.log("  Sum:", sum);
    console.log("  Product:", product);
    
    console.log("✅ Example completed");
}

// Run the main function
main(); 
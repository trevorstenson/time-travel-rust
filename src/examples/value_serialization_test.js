// Test file for JavaScript Value Serialization
console.log("🧪 Testing JavaScript Value Serialization");

// Test basic value capture
function testBasicValues() {
    timeDebugger.functionEntry("testBasicValues");
    
    // Test primitive values
    let number = 42;
    let string = "Hello, Time Travel!";
    let boolean = true;
    let nullValue = null;
    let undefinedValue = undefined;
    
    // Capture primitive values
    timeDebugger.captureVariable("number", number);
    timeDebugger.captureVariable("string", string);
    timeDebugger.captureVariable("boolean", boolean);
    timeDebugger.captureVariable("nullValue", nullValue);
    timeDebugger.captureVariable("undefinedValue", undefinedValue);
    
    console.log("Basic values captured");
    
    timeDebugger.functionExit("testBasicValues", 0);
}

// Test complex object serialization
function testComplexObjects() {
    timeDebugger.functionEntry("testComplexObjects");
    
    // Test object
    let person = {
        name: "Alice",
        age: 30,
        hobbies: ["reading", "coding", "hiking"],
        address: {
            street: "123 Main St",
            city: "Techville",
            zipCode: 12345
        }
    };
    
    // Test array
    let numbers = [1, 2, 3, 4, 5];
    let mixedArray = [1, "two", true, null, {key: "value"}];
    
    // Test function
    function greet(name) {
        return `Hello, ${name}!`;
    }
    
    // Test Date
    let now = new Date();
    
    // Capture complex values
    timeDebugger.captureVariable("person", person);
    timeDebugger.captureVariable("numbers", numbers);
    timeDebugger.captureVariable("mixedArray", mixedArray);
    timeDebugger.captureVariable("greet", greet);
    timeDebugger.captureVariable("now", now);
    
    console.log("Complex objects captured");
    
    timeDebugger.functionExit("testComplexObjects", 0);
}

// Test scope capture
function testScopeCapture(x, y) {
    timeDebugger.functionEntry("testScopeCapture");
    
    let localVar = x + y;
    let result = localVar * 2;
    
    // Capture entire scope
    let scope = {
        x: x,
        y: y,
        localVar: localVar,
        result: result
    };
    
    let capturedCount = timeDebugger.captureScope("testScopeCapture", "local", scope);
    console.log(`Captured ${capturedCount} variables in scope`);
    
    timeDebugger.functionExit("testScopeCapture", 0);
    return result;
}

// Test function wrapper that auto-captures
function testAutoCapture() {
    console.log("\n🤖 Testing automatic function capture");
    
    // Original function
    function multiply(a, b) {
        let temp = a * 2;
        let result = temp * b;
        return result;
    }
    
    // Wrap function for auto-capture
    let capturedMultiply = timeDebugger.captureFunction(multiply, "multiply");
    
    // Call the wrapped function
    let result = capturedMultiply(5, 3);
    console.log(`Multiply result: ${result}`);
    
    return result;
}

// Test special values
function testSpecialValues() {
    timeDebugger.functionEntry("testSpecialValues");
    
    // Test special numbers
    let infinity = Infinity;
    let negInfinity = -Infinity;
    let notANumber = NaN;
    let bigInteger = 9007199254740991n; // BigInt
    
    // Test symbol
    let sym = Symbol("test");
    let symWithDescription = Symbol("described");
    
    // Capture special values
    timeDebugger.captureVariable("infinity", infinity);
    timeDebugger.captureVariable("negInfinity", negInfinity);
    timeDebugger.captureVariable("notANumber", notANumber);
    timeDebugger.captureVariable("bigInteger", bigInteger);
    timeDebugger.captureVariable("sym", sym);
    timeDebugger.captureVariable("symWithDescription", symWithDescription);
    
    console.log("Special values captured");
    
    timeDebugger.functionExit("testSpecialValues", 0);
}

// Test circular reference handling
function testCircularReferences() {
    timeDebugger.functionEntry("testCircularReferences");
    
    let obj1 = { name: "obj1" };
    let obj2 = { name: "obj2" };
    
    // Create circular reference
    obj1.ref = obj2;
    obj2.ref = obj1;
    
    // This should handle circular references gracefully
    timeDebugger.captureVariable("obj1", obj1);
    timeDebugger.captureVariable("obj2", obj2);
    
    console.log("Circular references captured");
    
    timeDebugger.functionExit("testCircularReferences", 0);
}

// Main test runner
function runSerializationTests() {
    console.log("🚀 Starting JavaScript Value Serialization Tests\n");
    
    testBasicValues();
    console.log(""); // spacing
    
    testComplexObjects();
    console.log(""); // spacing
    
    testScopeCapture(10, 5);
    console.log(""); // spacing
    
    testAutoCapture();
    console.log(""); // spacing
    
    testSpecialValues();
    console.log(""); // spacing
    
    testCircularReferences();
    console.log(""); // spacing
    
    // Get and display snapshot information
    let info = timeDebugger.getSnapshotInfo();
    if (info) {
        console.log("📊 Snapshot Summary:");
        console.log(`  Total snapshots: ${info.total_snapshots}`);
        console.log(`  Function calls: ${info.function_calls}`);
        console.log(`  Current function: ${info.current_function || 'none'}`);
        console.log(`  Call depth: ${info.call_depth}`);
        
        if (info.snapshots && info.snapshots.length > 0) {
            console.log("  Recent snapshots:");
            info.snapshots.forEach((snapshot, index) => {
                console.log(`    ${index + 1}. ${snapshot.function} [${snapshot.type}] - ${snapshot.variable_count} vars`);
            });
        }
    }
    
    console.log("\n✅ All serialization tests completed!");
}

// Run the tests
runSerializationTests(); 
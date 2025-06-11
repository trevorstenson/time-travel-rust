// Manual monitoring test for Phase 1.2
console.log("🚀 Manual Monitoring Test");

// Test manual function monitoring
function testFunction(a, b) {
    timeDebugger.functionEntry("testFunction");
    const startTime = timeDebugger.getTimestamp();
    
    console.log("Inside testFunction with:", a, b);
    
    // Simulate some work
    let result = 0;
    for (let i = 0; i < 1000; i++) {
        result += a + b;
    }
    
    const duration = (timeDebugger.getTimestamp() - startTime) * 1000;
    timeDebugger.functionExit("testFunction", duration);
    
    return result;
}

// Test nested function calls
function outerFunction() {
    timeDebugger.functionEntry("outerFunction");
    const startTime = timeDebugger.getTimestamp();
    
    console.log("In outer function");
    
    // Call inner function
    const result = innerFunction(5, 10);
    
    const duration = (timeDebugger.getTimestamp() - startTime) * 1000;
    timeDebugger.functionExit("outerFunction", duration);
    
    return result;
}

function innerFunction(x, y) {
    timeDebugger.functionEntry("innerFunction");
    const startTime = timeDebugger.getTimestamp();
    
    console.log("In inner function with:", x, y);
    
    const result = x * y;
    
    const duration = (timeDebugger.getTimestamp() - startTime) * 1000;
    timeDebugger.functionExit("innerFunction", duration);
    
    return result;
}

// Test context capture
function contextTest() {
    timeDebugger.functionEntry("contextTest");
    const startTime = timeDebugger.getTimestamp();
    
    console.log("Testing context capture");
    
    // Capture some execution context
    timeDebugger.captureContext("test_context", {
        message: "This is a test context",
        timestamp: timeDebugger.getTimestamp(),
        data: { a: 1, b: 2, c: "test" }
    });
    
    const duration = (timeDebugger.getTimestamp() - startTime) * 1000;
    timeDebugger.functionExit("contextTest", duration);
}

// Main execution
console.log("🎯 Starting manual monitoring tests");

console.log("\n📊 Test 1: Basic function monitoring");
const result1 = testFunction(3, 7);
console.log("Result:", result1);

console.log("\n📊 Test 2: Nested function calls");
const result2 = outerFunction();
console.log("Result:", result2);

console.log("\n📊 Test 3: Context capture");
contextTest();

console.log("\n✅ Manual monitoring tests completed"); 
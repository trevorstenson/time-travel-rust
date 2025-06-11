// Enhanced monitoring demo for Phase 1.2 - Basic Execution Monitoring
console.log("🚀 Starting Enhanced Execution Monitoring Demo");

// Demonstrate nested function calls with different patterns
function calculateFactorial(n) {
    console.log(`Calculating factorial of ${n}`);
    
    if (n <= 1) {
        return 1;
    }
    
    return n * calculateFactorial(n - 1);
}

// Demonstrate function with object manipulation
function processUserData(user) {
    console.log("Processing user data:", user.name);
    
    const profile = {
        id: user.id,
        displayName: user.name.toUpperCase(),
        email: user.email,
        timestamp: Date.now()
    };
    
    return validateProfile(profile);
}

function validateProfile(profile) {
    console.log("Validating profile for:", profile.displayName);
    
    if (!profile.email || !profile.email.includes('@')) {
        throw new Error('Invalid email address');
    }
    
    if (profile.displayName.length < 2) {
        throw new Error('Display name too short');
    }
    
    return profile;
}

// Demonstrate async operations
async function fetchUserPreferences(userId) {
    console.log(`Fetching preferences for user ${userId}`);
    
    // Simulate async operation
    await new Promise(resolve => setTimeout(resolve, 100));
    
    return {
        theme: 'dark',
        language: 'en',
        notifications: true
    };
}

async function processWithPreferences(user) {
    console.log("Processing user with preferences");
    
    const profile = processUserData(user);
    const preferences = await fetchUserPreferences(user.id);
    
    return {
        ...profile,
        preferences
    };
}

// Demonstrate recursive function with error handling
function fibonacci(n, memo = {}) {
    if (n in memo) {
        return memo[n];
    }
    
    if (n <= 1) {
        return n;
    }
    
    memo[n] = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    return memo[n];
}

// Main execution flow
async function main() {
    console.log("🎯 Demonstrating various execution patterns");
    
    // Test 1: Simple recursive function
    console.log("\n📊 Test 1: Factorial calculation");
    try {
        const result = calculateFactorial(5);
        console.log("Factorial result:", result);
    } catch (error) {
        console.error("Factorial error:", error.message);
    }
    
    // Test 2: Object processing with validation
    console.log("\n📊 Test 2: User data processing");
    try {
        const user = {
            id: 123,
            name: "Alice Johnson",
            email: "alice@example.com"
        };
        
        const processed = await processWithPreferences(user);
        console.log("Processed user:", processed.displayName);
    } catch (error) {
        console.error("Processing error:", error.message);
    }
    
    // Test 3: Error handling demonstration
    console.log("\n📊 Test 3: Error handling");
    try {
        const badUser = {
            id: 456,
            name: "X",
            email: "invalid-email"
        };
        
        processUserData(badUser);
    } catch (error) {
        console.error("Expected error:", error.message);
    }
    
    // Test 4: Optimized recursive function
    console.log("\n📊 Test 4: Fibonacci calculation");
    try {
        const fibResult = fibonacci(10);
        console.log("Fibonacci(10):", fibResult);
    } catch (error) {
        console.error("Fibonacci error:", error.message);
    }
    
    // Test 5: Multiple function calls to demonstrate statistics
    console.log("\n📊 Test 5: Multiple calls for statistics");
    for (let i = 1; i <= 3; i++) {
        calculateFactorial(i);
        fibonacci(i + 5);
    }
    
    console.log("✅ Monitoring demo completed");
}

// Execute main function
main().catch(error => {
    console.error("Demo failed:", error.message);
}); 
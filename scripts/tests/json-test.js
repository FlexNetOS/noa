#!/usr/bin/env node
// json-test.js - JSON validation and assertion helper
// Usage: node json-test.js FILE EXPRESSION

const fs = require('fs');
const path = require('path');

if (process.argv.length < 4) {
    console.error('Usage: json-test.js FILE EXPRESSION');
    process.exit(1);
}

const filePath = process.argv[2];
const expression = process.argv[3];

try {
    // Read and parse JSON
    const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));

    // Evaluate expression
    // The expression should be a JavaScript expression that returns true/false
    // It can reference 'data' variable
    const result = eval(expression);

    if (result === false) {
        console.error(`Assertion failed: ${expression}`);
        process.exit(1);
    }

    process.exit(0);
} catch (error) {
    console.error(`Error: ${error.message}`);
    process.exit(1);
}

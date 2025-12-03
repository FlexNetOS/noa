# Plan to Fix setup-noa-windows.ps1

## Problem
The setup script is not working. Based on the last error you showed:
- Missing closing braces
- Try statement missing catch/finally block
- Syntax errors preventing the script from running

## Root Cause Analysis
The issue is likely that the `StringBuilder` approach with string concatenation using `[char]34` is causing PowerShell to misinterpret the script structure. The parser sees unclosed braces because the string building code confuses it.

## Solution Plan

### Option 1: Simplify the Profile Generation (RECOMMENDED)
Instead of building complex strings with StringBuilder, write the profile file directly using simple, line-by-line Out-File commands.

### Option 2: Use a Template File
Create a template profile file and do simple variable replacement.

### Option 3: Rewrite with Here-Strings Properly
Use here-strings correctly with proper escaping (double `$` instead of backticks).

## Implementation Steps

1. ✅ Fix `.gitignore` to not use `/*` pattern (allows indexing)
2. Create a minimal working version of the script
3. Test it to ensure it runs
4. Gradually add features back
5. Verify each addition works

## Next Actions

1. Create `setup-noa-simple.ps1` - a minimal version that works
2. Test it
3. If it works, gradually enhance it
4. Replace the broken script with the working one

## Testing Approach

Since PowerShell output isn't showing in this environment, you'll need to:
1. Run the script manually: `.\setup-noa-simple.ps1`
2. Check if files are created: `Test-Path noa-profile.ps1`
3. Report back what you see

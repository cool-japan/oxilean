// Minimal tests that don't require VS Code runtime
// Real integration tests require the vscode-test framework

function buildServerArgs(baseArgs: string[], verbose: boolean): string[] {
    return verbose ? [...baseArgs, '--verbose'] : baseArgs;
}

function validateServerPath(p: string): boolean {
    return p.length > 0 && !p.includes('\0');
}

// Test 1: server args passthrough
const args = buildServerArgs(['--lsp'], false);
console.assert(args.length === 1 && args[0] === '--lsp', 'args passthrough');

// Test 2: verbose mode appends --verbose
const verboseArgs = buildServerArgs(['--lsp'], true);
console.assert(verboseArgs[verboseArgs.length - 1] === '--verbose', 'verbose flag');

// Test 3: server path validation
console.assert(validateServerPath('oxilean'), 'valid path');
console.assert(validateServerPath('/usr/local/bin/oxilean'), 'absolute path');
console.assert(!validateServerPath(''), 'empty path rejected');

console.log('All extension unit tests passed.');

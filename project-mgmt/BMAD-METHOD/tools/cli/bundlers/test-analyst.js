const { WebBundler } = require('./web-bundler');
const chalk = require('chalk');
const path = require('node:path');

async function testAnalystBundle() {
  console.log(chalk.cyan.bold('\n🧪 Testing Analyst Agent Bundle\n'));

  try {
    const bundler = new WebBundler();

    // Load web activation first
    await bundler.loadWebActivation();

    // Bundle just the analyst agent from bmm module
    // Only bundle the analyst for testing
    const agentPath = path.join(bundler.modulesPath, 'bmm', 'agents', 'analyst.md');
    await bundler.bundleAgent('bmm', 'analyst.md');

    console.log(chalk.green.bold('\n✅ Test completed successfully!\n'));
  } catch (error) {
    console.error(chalk.red('\n❌ Test failed:'), error.message);
    console.error(error.stack);
    process.exit(1);
  }
}

// Run test
testAnalystBundle();

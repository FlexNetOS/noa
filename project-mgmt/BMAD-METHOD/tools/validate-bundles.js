const fs = require('fs-extra');
const path = require('node:path');
const xml2js = require('xml2js');
const chalk = require('chalk');
const glob = require('glob');

async function validateXmlFile(filePath) {
  try {
    const content = await fs.readFile(filePath, 'utf8');
    await xml2js.parseStringPromise(content, {
      strict: true,
      explicitArray: false,
    });
    return { valid: true };
  } catch (error) {
    return { valid: false, error: error.message };
  }
}

async function validateAllBundles() {
  console.log(chalk.cyan.bold('\n═══════════════════════════════════════════════'));
  console.log(chalk.cyan.bold('        VALIDATING WEB BUNDLE XML FILES'));
  console.log(chalk.cyan.bold('═══════════════════════════════════════════════\n'));

  const bundlesDir = path.join(__dirname, '..', 'web-bundles');

  // Find all XML files in web-bundles
  const pattern = path.join(bundlesDir, '**/*.xml');
  const files = glob.sync(pattern);

  if (files.length === 0) {
    console.log(chalk.yellow('No XML files found in web-bundles directory'));
    return;
  }

  console.log(`Found ${chalk.bold(files.length)} XML files to validate\n`);

  let validCount = 0;
  let invalidCount = 0;
  const invalidFiles = [];

  for (const file of files) {
    const relativePath = path.relative(bundlesDir, file);
    const result = await validateXmlFile(file);

    if (result.valid) {
      console.log(`${chalk.green('✓')} ${relativePath}`);
      validCount++;
    } else {
      console.log(`${chalk.red('✗')} ${relativePath}`);
      console.log(`  ${chalk.red('→')} ${result.error}`);
      invalidCount++;
      invalidFiles.push({ path: relativePath, error: result.error });
    }
  }

  // Summary
  console.log(chalk.cyan.bold('\n═══════════════════════════════════════════════'));
  console.log(chalk.cyan.bold('                 SUMMARY'));
  console.log(chalk.cyan.bold('═══════════════════════════════════════════════\n'));

  console.log(`  Total files checked: ${chalk.bold(files.length)}`);
  console.log(`  Valid XML files:     ${chalk.green(validCount)}`);
  console.log(`  Invalid XML files:   ${invalidCount > 0 ? chalk.red(invalidCount) : chalk.green(invalidCount)}`);

  if (invalidFiles.length > 0) {
    console.log(chalk.red.bold('\n  Invalid Files:'));
    for (const { path, error } of invalidFiles) {
      console.log(`    ${chalk.red('•')} ${path}`);
      if (error.length > 100) {
        console.log(`      ${error.slice(0, 100)}...`);
      } else {
        console.log(`      ${error}`);
      }
    }
  }

  console.log(chalk.cyan.bold('\n═══════════════════════════════════════════════\n'));

  process.exit(invalidCount > 0 ? 1 : 0);
}

// Run validation
validateAllBundles().catch((error) => {
  console.error(chalk.red('Error running validation:'), error);
  process.exit(1);
});

const chalk = require('chalk');
const { Installer } = require('../installers/lib/core/installer');

const installer = new Installer();

module.exports = {
  command: 'list',
  description: 'List available modules',
  options: [],
  action: async () => {
    try {
      const modules = await installer.getAvailableModules();
      console.log(chalk.cyan('\n📦 Available BMAD Modules:\n'));

      for (const module of modules) {
        console.log(chalk.bold(`  ${module.id}`));
        console.log(chalk.dim(`    ${module.description}`));
        console.log(chalk.dim(`    Version: ${module.version}`));
        console.log();
      }

      process.exit(0);
    } catch (error) {
      console.error(chalk.red('Error:'), error.message);
      process.exit(1);
    }
  },
};

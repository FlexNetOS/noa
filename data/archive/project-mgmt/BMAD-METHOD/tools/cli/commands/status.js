const chalk = require('chalk');
const { Installer } = require('../installers/lib/core/installer');

const installer = new Installer();

module.exports = {
  command: 'status',
  description: 'Show installation status',
  options: [['-d, --directory <path>', 'Installation directory', '.']],
  action: async (options) => {
    try {
      const status = await installer.getStatus(options.directory);

      if (!status.installed) {
        console.log(chalk.yellow('\n⚠️  No BMAD installation found in:'), options.directory);
        console.log(chalk.dim('Run "bmad install" to set up BMAD Method'));
        process.exit(0);
      }

      console.log(chalk.cyan('\n📊 BMAD Installation Status\n'));
      console.log(chalk.bold('Location:'), status.path);
      console.log(chalk.bold('Version:'), status.version);
      console.log(chalk.bold('Core:'), status.hasCore ? chalk.green('✓ Installed') : chalk.red('✗ Not installed'));

      if (status.modules.length > 0) {
        console.log(chalk.bold('\nModules:'));
        for (const mod of status.modules) {
          console.log(`  ${chalk.green('✓')} ${mod.id} (v${mod.version})`);
        }
      } else {
        console.log(chalk.bold('\nModules:'), chalk.dim('None installed'));
      }

      if (status.ides.length > 0) {
        console.log(chalk.bold('\nConfigured IDEs:'));
        for (const ide of status.ides) {
          console.log(`  ${chalk.green('✓')} ${ide}`);
        }
      }

      process.exit(0);
    } catch (error) {
      console.error(chalk.red('Error:'), error.message);
      process.exit(1);
    }
  },
};

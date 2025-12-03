const chalk = require('chalk');
const { Installer } = require('../installers/lib/core/installer');

const installer = new Installer();

module.exports = {
  command: 'update',
  description: 'Update existing BMAD installation',
  options: [
    ['-d, --directory <path>', 'Installation directory', '.'],
    ['--force', 'Force update, overwriting modified files'],
    ['--dry-run', 'Show what would be updated without making changes'],
  ],
  action: async (options) => {
    try {
      await installer.update({
        directory: options.directory,
        force: options.force,
        dryRun: options.dryRun,
      });
      console.log(chalk.green('\n✨ Update complete!'));
      process.exit(0);
    } catch (error) {
      console.error(chalk.red('Update failed:'), error.message);
      process.exit(1);
    }
  },
};

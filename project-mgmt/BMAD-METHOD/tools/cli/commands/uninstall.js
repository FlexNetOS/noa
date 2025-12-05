const chalk = require('chalk');
const path = require('node:path');
const { Installer } = require('../installers/lib/core/installer');
const { UI } = require('../lib/ui');

const installer = new Installer();
const ui = new UI();

module.exports = {
  command: 'uninstall',
  description: 'Remove BMAD installation',
  options: [
    ['-d, --directory <path>', 'Installation directory', '.'],
    ['--force', 'Skip confirmation prompt'],
  ],
  action: async (options) => {
    try {
      const bmadPath = path.join(options.directory, 'bmad');

      if (!options.force) {
        const { confirm } = await ui.prompt([
          {
            type: 'confirm',
            name: 'confirm',
            message: `Are you sure you want to remove BMAD from ${bmadPath}?`,
            default: false,
          },
        ]);

        if (!confirm) {
          console.log('Uninstall cancelled.');
          process.exit(0);
        }
      }

      await installer.uninstall(options.directory);
      console.log(chalk.green('\n✨ BMAD Method has been uninstalled.'));
      process.exit(0);
    } catch (error) {
      console.error(chalk.red('Uninstall failed:'), error.message);
      process.exit(1);
    }
  },
};

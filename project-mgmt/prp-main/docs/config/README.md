# .prprc Configuration Reference

## Overview

The `.prprc` file is the main configuration file for PRP CLI. It defines project settings, development workflows, quality gates, CI/CD pipelines, and tool configurations. This comprehensive reference covers all available configuration options, their defaults, and usage examples.

## File Location and Hierarchy

PRP CLI looks for configuration files in the following order:

1. `.prprc` in the current directory
2. `.prprc.json` in the current directory
3. `.prprc.yaml` or `.prprc.yml` in the current directory
4. `prp.config.js` in the current directory
5. `~/.prp/config.json` (global configuration)
6. `/etc/prp/config.json` (system configuration)

## Configuration Schema

### Root Level Configuration

```json
{
  "name": "string",
  "version": "string",
  "description": "string",
  "type": "string",
  "author": "string",
  "license": "string",
  "repository": "string",
  "keywords": ["string"],
  "settings": {
    // Main configuration object
  },
  "scripts": {
    // Custom scripts
  },
  "dependencies": {
    // Project dependencies
  },
  "devDependencies": {
    // Development dependencies
  }
}
```

### Core Fields

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | string | Yes | - | Project name |
| `version` | string | Yes | "1.0.0" | Project version (semver) |
| `description` | string | No | "" | Project description |
| `type` | string | No | "auto" | Project type (auto, node, python, go, etc.) |
| `author` | string | No | "" | Project author |
| `license` | string | No | "MIT" | Project license |
| `repository` | string | No | "" | Git repository URL |
| `keywords` | array | No | [] | Project keywords |

## Settings Configuration

### Debug Settings

```json
{
  "settings": {
    "debug": {
      "enabled": true,
      "level": "info",
      "output": "console",
      "file": "debug.log",
      "maxFileSize": "10MB",
      "maxFiles": 5,
      "timestamp": true,
      "colors": true,
      "orchestrator": {
        "enabled": false,
        "url": "https://orchestrator.example.com",
        "apiKey": "${ORCHESTRATOR_API_KEY}",
        "timeout": 5000
      },
      "components": {
        "cli": true,
        "build": true,
        "test": true,
        "lint": true,
        "deploy": true
      }
    }
  }
}
```

#### Debug Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | false | Enable debug mode |
| `level` | string | "info" | Log level (error, warn, info, debug, verbose) |
| `output` | string | "console" | Output format (console, file, json) |
| `file` | string | "debug.log" | Log file path (when output=file) |
| `maxFileSize` | string | "10MB" | Maximum log file size |
| `maxFiles` | number | 5 | Maximum number of log files to keep |
| `timestamp` | boolean | true | Include timestamps in logs |
| `colors` | boolean | true | Use colored output |
| `orchestrator` | object | {} | Orchestrator integration settings |

### Quality Settings

```json
{
  "settings": {
    "quality": {
      "enabled": true,
      "strict": false,
      "gates": {
        "lint": {
          "enabled": true,
          "tools": ["eslint", "prettier"],
          "failOnWarnings": false,
          "maxWarnings": 0,
          "configFile": ".eslintrc.json"
        },
        "test": {
          "enabled": true,
          "coverage": {
            "enabled": true,
            "minimum": 80,
            "threshold": 5,
            "reporters": ["text", "lcov", "html"],
            "outputDirectory": "coverage"
          },
          "failures": {
            "maximum": 0,
            "retry": 1
          },
          "timeout": 5000
        },
        "security": {
          "enabled": true,
          "tools": ["npm-audit", "snyk"],
          "failOnHigh": true,
          "failOnMedium": false,
          "failOnLow": false,
          "excludeDevDependencies": false
        },
        "performance": {
          "enabled": false,
          "budget": {
            "size": "500KB",
            "loadTime": "3s",
            "firstContentfulPaint": "1.5s"
          },
          "lighthouse": {
            "enabled": false,
            "scores": {
              "performance": 90,
              "accessibility": 95,
              "bestPractices": 90,
              "seo": 80
            }
          }
        },
        "complexity": {
          "enabled": false,
          "maxComplexity": 10,
          "tools": ["complexity-report"]
        },
        "duplication": {
          "enabled": false,
          "threshold": 5,
          "tools": ["jscpd"]
        }
      },
      "reporting": {
        "formats": ["console", "json"],
        "outputDirectory": "quality-reports",
        "artifacts": true,
        "notifications": {
          "onFailure": true,
          "onThreshold": false
        }
      },
      "preCommitHooks": true,
      "prePushHooks": true
    }
  }
}
```

#### Quality Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | true | Enable quality gates |
| `strict` | boolean | false | Strict quality mode |
| `gates` | object | {} | Quality gate configurations |
| `reporting` | object | {} | Quality reporting settings |
| `preCommitHooks` | boolean | true | Enable pre-commit hooks |
| `prePushHooks` | boolean | true | Enable pre-push hooks |

### Build Settings

```json
{
  "settings": {
    "build": {
      "mode": "production",
      "target": "default",
      "output": "dist",
      "clean": true,
      "sourcemap": true,
      "minify": true,
      "compression": true,
      "analyze": false,
      "incremental": true,
      "parallel": true,
      "cache": {
        "enabled": true,
        "directory": ".cache",
        "strategy": "content"
      },
      "optimization": {
        "splitting": true,
        "treeShaking": true,
        "deadCodeElimination": true
      },
      "environment": {
        "variables": {
          "NODE_ENV": "production"
        },
        "files": [".env.production"]
      },
      "assets": {
        "inline": false,
        "limit": "8KB",
        "publicPath": "/assets/"
      },
      "externals": {
        "react": "React",
        "react-dom": "ReactDOM"
      }
    }
  }
}
```

#### Build Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `mode` | string | "production" | Build mode (development, production) |
| `target` | string | "default" | Build target |
| `output` | string | "dist" | Output directory |
| `clean` | boolean | true | Clean output directory before build |
| `sourcemap` | boolean | true | Generate source maps |
| `minify` | boolean | true | Minify output |
| `compression` | boolean | true | Compress output |
| `analyze` | boolean | false | Analyze bundle size |
| `incremental` | boolean | true | Enable incremental builds |
| `parallel` | boolean | true | Enable parallel builds |

### Test Settings

```json
{
  "settings": {
    "test": {
      "type": "all",
      "framework": "jest",
      "coverage": true,
      "watch": false,
      "parallel": true,
      "maxWorkers": "50%",
      "reporters": ["default"],
      "testEnvironment": "node",
      "testMatch": [
        "**/__tests__/**/*.+(ts|tsx|js)",
        "**/*.(test|spec).+(ts|tsx|js)"
      ],
      "collectCoverageFrom": [
        "src/**/*.{ts,tsx,js}",
        "!src/**/*.d.ts",
        "!src/**/*.stories.{ts,tsx}"
      ],
      "coverageThreshold": {
        "global": {
          "branches": 80,
          "functions": 80,
          "lines": 80,
          "statements": 80
        }
      },
      "setupFiles": ["<rootDir>/tests/setup.ts"],
      "snapshotSerializers": [],
      "transform": {
        "^.+\\.(ts|tsx)$": "ts-jest"
      },
      "moduleNameMapping": {
        "^@/(.*)$": "<rootDir>/src/$1"
      },
      "globals": {
        "ts-jest": {
          "tsconfig": "tsconfig.json"
        }
      }
    }
  }
}
```

#### Test Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `type` | string | "all" | Test type (unit, integration, e2e, all) |
| `framework` | string | "jest" | Test framework |
| `coverage` | boolean | true | Generate coverage report |
| `watch` | boolean | false | Watch mode |
| `parallel` | boolean | true | Parallel test execution |
| `maxWorkers` | string | "50%" | Maximum number of workers |
| `reporters` | array | ["default"] | Test reporters |
| `testEnvironment` | string | "node" | Test environment |

### CI/CD Settings

```json
{
  "settings": {
    "ci": {
      "provider": "github",
      "enabled": true,
      "workflows": {
        "test": {
          "enabled": true,
          "triggers": ["push", "pull_request"],
          "branches": ["main", "develop"],
          "nodeVersions": [16, 18, 20],
          "os": ["ubuntu-latest"],
          "cache": true,
          "artifacts": true
        },
        "build": {
          "enabled": true,
          "triggers": ["push"],
          "branches": ["main", "develop"],
          "dependsOn": ["test"],
          "artifacts": true
        },
        "deploy": {
          "enabled": true,
          "triggers": ["push", "release"],
          "branches": ["main"],
          "dependsOn": ["build"],
          "environments": ["staging", "production"],
          "approval": {
            "required": false,
            "approvers": ["@dev-team"]
          }
        }
      },
      "cache": {
        "enabled": true,
        "paths": ["node_modules", ".npm", "dist"],
        "key": "v1-${{ runner.os }}-${{ hashFiles('**/package-lock.json') }}"
      },
      "secrets": {
        "required": ["GITHUB_TOKEN"],
        "optional": ["NPM_TOKEN", "SLACK_WEBHOOK"]
      },
      "notifications": {
        "slack": {
          "enabled": false,
          "webhook": "${SLACK_WEBHOOK}",
          "channel": "#ci-cd",
          "onSuccess": true,
          "onFailure": true
        },
        "email": {
          "enabled": false,
          "recipients": ["team@example.com"],
          "onFailure": true
        }
      },
      "artifacts": {
        "retention": 30,
        "compression": true,
        "name": "build-artifacts"
      }
    }
  }
}
```

#### CI/CD Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `provider` | string | "github" | CI/CD provider |
| `enabled` | boolean | true | Enable CI/CD |
| `workflows` | object | {} | Workflow configurations |
| `cache` | object | {} | Caching settings |
| `secrets` | object | {} | Required/optional secrets |
| `notifications` | object | {} | Notification settings |
| `artifacts` | object | {} | Artifact settings |

### Development Settings

```json
{
  "settings": {
    "development": {
      "hotReload": true,
      "port": 3000,
      "host": "localhost",
      "proxy": {
        "/api": {
          "target": "http://localhost:8080",
          "changeOrigin": true,
          "secure": false
        }
      },
      "https": false,
      "open": true,
      "browser": "default",
      "devServer": {
        "compress": true,
        "historyApiFallback": true,
        "overlay": {
          "errors": true,
          "warnings": false
        }
      },
      "environment": {
        "variables": {
          "NODE_ENV": "development"
        },
        "files": [".env.development", ".env.local"]
      }
    }
  }
}
```

#### Development Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `hotReload` | boolean | true | Enable hot module replacement |
| `port` | number | 3000 | Development server port |
| `host` | string | "localhost" | Development server host |
| `proxy` | object | {} | Proxy configuration |
| `https` | boolean | false | Enable HTTPS |
| `open` | boolean | true | Open browser on start |
| `browser` | string | "default" | Browser to open |

### Package Management Settings

```json
{
  "settings": {
    "packageManager": {
      "type": "npm",
      "version": "latest",
      "registry": "https://registry.npmjs.org",
      "cache": true,
      "audit": true,
      "lockFile": true,
      "scripts": {
        "autoInstall": true,
        "updateCheck": true,
        "outdated": false
      },
      "engines": {
        "node": ">=16.0.0",
        "npm": ">=8.0.0"
      },
      "workspaces": {
        "enabled": false,
        "packages": ["packages/*"]
      }
    }
  }
}
```

#### Package Manager Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `type` | string | "npm" | Package manager (npm, yarn, pnpm) |
| `version` | string | "latest" | Package manager version |
| `registry` | string | "https://registry.npmjs.org" | Registry URL |
| `cache` | boolean | true | Enable package cache |
| `audit` | boolean | true | Run security audit |
| `lockFile` | boolean | true | Generate lock file |

### Debugging Settings

```json
{
  "settings": {
    "debugging": {
      "node": {
        "enabled": true,
        "port": 9229,
        "host": "localhost",
        "break": false,
        "inspect": true,
        "restart": true,
        "console": true,
        "sourceMaps": true,
        "timeout": 30000
      },
      "python": {
        "enabled": true,
        "port": 5678,
        "host": "localhost",
        "wait": true,
        "break": false,
        "console": true,
        "venv": true,
        "timeout": 30000
      },
      "browser": {
        "enabled": true,
        "port": 9222,
        "headless": false,
        "devtools": true,
        "slowMo": 0,
        "timeout": 30000
      }
    }
  }
}
```

#### Debugging Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `node` | object | {} | Node.js debugging settings |
| `python` | object | {} | Python debugging settings |
| `browser` | object | {} | Browser debugging settings |

### Token Accounting Settings

```json
{
  "settings": {
    "tokenAccounting": {
      "enabled": true,
      "provider": "openai",
      "tracking": {
        "inputTokens": true,
        "outputTokens": true,
        "totalTokens": true,
        "cost": true
      },
      "pricing": {
        "input": 0.001,
        "output": 0.002,
        "currency": "USD"
      },
      "limits": {
        "daily": 100000,
        "monthly": 1000000,
        "budget": 100
      },
      "reporting": {
        "frequency": "daily",
        "format": "json",
        "export": true
      },
      "alerts": {
        "threshold": 80,
        "notifications": true
      }
    }
  }
}
```

#### Token Accounting Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | true | Enable token accounting |
| `provider` | string | "openai" | Token provider |
| `tracking` | object | {} | Tracking settings |
| `pricing` | object | {} | Pricing configuration |
| `limits` | object | {} | Usage limits |
| `reporting` | object | {} | Reporting settings |
| `alerts` | object | {} | Alert settings |

## Scripts Configuration

### Custom Scripts

```json
{
  "scripts": {
    "dev": "prp build --mode development --watch",
    "build": "prp build",
    "test": "prp test",
    "test:watch": "prp test --watch",
    "test:coverage": "prp test --coverage",
    "lint": "prp lint",
    "lint:fix": "prp lint --fix",
    "quality": "prp quality",
    "quality:fix": "prp quality --fix",
    "debug": "prp debug",
    "deploy": "prp deploy",
    "ci": "prp ci run",
    "clean": "prp clean",
    "init": "prp init",
    "status": "prp status",
    "config": "prp config list",
    "version": "prp version"
  }
}
```

### Script Templates

```json
{
  "scripts": {
    "templates": {
      "start": "prp dev --port ${PORT:-3000}",
      "test:e2e": "prp test --type e2e",
      "build:analyze": "prp build --analyze",
      "deploy:staging": "prp deploy --env staging",
      "deploy:prod": "prp deploy --env production --confirm"
    }
  }
}
```

## Environment Variables

### Configuration Variables

Environment variables can be used in `.prprc` files using `${VARIABLE_NAME}` syntax:

```json
{
  "settings": {
    "ci": {
      "secrets": {
        "required": ["GITHUB_TOKEN"],
        "optional": ["${OPTIONAL_SECRET}"]
      },
      "notifications": {
        "slack": {
          "webhook": "${SLACK_WEBHOOK_URL}"
        }
      }
    },
    "debugging": {
      "node": {
        "port": "${DEBUG_PORT:-9229}"
      }
    }
  }
}
```

### Built-in Variables

| Variable | Description |
|----------|-------------|
| `PRP_PROJECT_NAME` | Current project name |
| `PRP_PROJECT_TYPE` | Current project type |
| `PRP_VERSION` | PRP CLI version |
| `PRP_ENV` | Current environment (development, production) |
| `PRP_CONFIG_PATH` | Path to current config file |
| `PRP_WORKSPACE` | Current workspace directory |

## Configuration Validation

### Schema Validation

PRP CLI validates configuration files against a JSON schema:

```bash
# Validate configuration
prp config validate

# Validate with detailed output
prp config validate --verbose

# Check specific section
prp config validate --section quality
```

### Common Validation Errors

1. **Missing Required Fields**: Ensure all required fields are present
2. **Invalid Types**: Check that values match expected types
3. **Invalid Values**: Verify that values are within allowed ranges
4. **Circular References**: Avoid circular references in configuration
5. **Syntax Errors**: Ensure JSON/YAML syntax is valid

## Project Type Templates

### Node.js Project

```json
{
  "name": "my-node-app",
  "type": "node",
  "settings": {
    "packageManager": {
      "type": "npm"
    },
    "build": {
      "target": "node",
      "output": "dist"
    },
    "test": {
      "framework": "jest",
      "testEnvironment": "node"
    },
    "debugging": {
      "node": {
        "enabled": true,
        "port": 9229
      }
    }
  }
}
```

### React Project

```json
{
  "name": "my-react-app",
  "type": "react",
  "settings": {
    "build": {
      "target": "browser",
      "output": "build"
    },
    "development": {
      "port": 3000,
      "hotReload": true
    },
    "test": {
      "framework": "jest",
      "testEnvironment": "jsdom"
    },
    "quality": {
      "gates": {
        "lint": {
          "tools": ["eslint", "prettier"]
        },
        "test": {
          "coverage": {
            "minimum": 80
          }
        }
      }
    }
  }
}
```

### Python Project

```json
{
  "name": "my-python-app",
  "type": "python",
  "settings": {
    "packageManager": {
      "type": "pip"
    },
    "build": {
      "target": "python",
      "output": "dist"
    },
    "test": {
      "framework": "pytest"
    },
    "debugging": {
      "python": {
        "enabled": true,
        "port": 5678
      }
    }
  }
}
```

## Best Practices

### Configuration Organization

1. **Keep it Simple**: Start with basic configuration and add complexity as needed
2. **Use Environment Variables**: Use environment variables for sensitive data
3. **Version Control**: Store configuration in version control
4. **Documentation**: Document configuration decisions
5. **Validation**: Always validate configuration before use

### Performance Optimization

1. **Enable Caching**: Use build and dependency caching
2. **Parallel Execution**: Enable parallel builds and tests
3. **Incremental Builds**: Use incremental builds when possible
4. **Optimize Dependencies**: Keep dependencies up to date
5. **Monitor Performance**: Track build and test performance

### Security

1. **Use Secrets**: Use environment variables for sensitive data
2. **Regular Audits**: Run regular security audits
3. **Dependency Scanning**: Enable dependency vulnerability scanning
4. **Access Control**: Limit access to configuration files
5. **Encryption**: Encrypt sensitive configuration data

## Troubleshooting

### Common Issues

1. **Configuration Not Found**: Check file location and naming
2. **Invalid JSON/YAML**: Validate syntax using online tools
3. **Missing Variables**: Ensure all environment variables are set
4. **Permission Errors**: Check file permissions
5. **Schema Validation Errors**: Review error messages and fix issues

### Debug Configuration

```bash
# Show current configuration
prp config list --verbose

# Check configuration file
prp config check

# Validate configuration
prp config validate --debug

# Show configuration path
prp config path
```

## Migration Guide

### Upgrading Configuration

```bash
# Check for configuration updates
prp config check-updates

# Migrate to new version
prp config migrate

# Validate migrated configuration
prp config validate
```

### Importing from Other Tools

```bash
# Import from package.json
prp config import --from package.json

# Import from existing CI configuration
prp config import --from .github/workflows

# Import from other build tools
prp config import --from webpack.config.js
```

## API Reference

### Configuration API

```typescript
interface PRPConfig {
  name: string;
  version: string;
  type?: string;
  description?: string;
  author?: string;
  license?: string;
  repository?: string;
  keywords?: string[];
  settings: {
    debug?: DebugSettings;
    quality?: QualitySettings;
    build?: BuildSettings;
    test?: TestSettings;
    ci?: CISettings;
    development?: DevelopmentSettings;
    packageManager?: PackageManagerSettings;
    debugging?: DebuggingSettings;
    tokenAccounting?: TokenAccountingSettings;
  };
  scripts?: Record<string, string>;
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
}
```

### Configuration Methods

```typescript
// Load configuration
const config = await loadConfig(configPath);

// Validate configuration
const validation = await validateConfig(config);

// Merge configurations
const merged = mergeConfigs(baseConfig, overrideConfig);

// Transform configuration
const transformed = transformConfig(config, transformer);

// Export configuration
await exportConfig(config, format, outputPath);
```

## Getting Help

- **Documentation**: Full documentation at `/docs/config`
- **Command Help**: `prp config --help`
- **Validation**: `prp config validate --help`
- **Examples**: Browse example configurations in `/examples/config`
- **Community**: Join our Discord community for support
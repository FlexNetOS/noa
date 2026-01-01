# CLI Reference

Complete command reference for the NOA CLI.

## Global Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Increase verbosity (can be repeated) |
| `-q, --quiet` | Suppress output |
| `--config <path>` | Config file path |
| `--data-dir <path>` | Data directory |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

## Commands

### init

Initialize a new NOA instance.

```bash
noa init [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--data-dir <path>` | Data directory (default: `~/.noa`) |
| `--force` | Overwrite existing configuration |

### run

Start the NOA daemon.

```bash
noa run [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--daemon` | Run as background daemon |
| `--port <port>` | API port (default: 8080) |
| `--host <host>` | API host (default: 127.0.0.1) |

### agent

Agent management commands.

```bash
noa agent <COMMAND>
```

| Subcommand | Description |
|------------|-------------|
| `list` | List all agents |
| `info <id>` | Show agent details |
| `start <id>` | Start an agent |
| `stop <id>` | Stop an agent |
| `register <kind>` | Register new agent |

### task

Task execution commands.

```bash
noa task <COMMAND>
```

| Subcommand | Description |
|------------|-------------|
| `run` | Execute a task |
| `list` | List tasks |
| `status <id>` | Get task status |
| `cancel <id>` | Cancel a task |

#### task run

```bash
noa task run [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--agent <name>` | Target agent |
| `--input <json>` | Task input (JSON) |
| `--file <path>` | Task input from file |
| `--async` | Run asynchronously |
| `--timeout <secs>` | Timeout in seconds |

### model

Model management commands.

```bash
noa model <COMMAND>
```

| Subcommand | Description |
|------------|-------------|
| `list` | List downloaded models |
| `download <name>` | Download a model |
| `delete <name>` | Delete a model |
| `test <name>` | Test model inference |
| `info <name>` | Show model info |

### p2p

P2P networking commands.

```bash
noa p2p <COMMAND>
```

| Subcommand | Description |
|------------|-------------|
| `identity` | Show peer identity |
| `status` | Network status |
| `peers` | List connected peers |
| `add-peer <addr>` | Add peer |
| `subscribe <topic>` | Subscribe to topic |
| `publish <topic> <msg>` | Publish message |

### cache

Cache management commands.

```bash
noa cache <COMMAND>
```

| Subcommand | Description |
|------------|-------------|
| `info [type]` | Cache information |
| `clean [type]` | Clean cache |
| `list [type]` | List cache entries |

### config

Configuration commands.

```bash
noa config <COMMAND>
```

| Subcommand | Description |
|------------|-------------|
| `show` | Show current config |
| `get <key>` | Get config value |
| `set <key> <value>` | Set config value |
| `validate` | Validate config |

## Examples

```bash
# Initialize NOA
noa init --data-dir ~/.noa

# Start daemon
noa run --daemon

# List agents
noa agent list

# Run a task
noa task run --agent file-io --input '{"action":"read","path":"./README.md"}'

# Download a model
noa model download qwen2.5-coder-7b

# Check P2P status
noa p2p status
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Configuration error |
| 4 | Runtime error |

## See Also

- [Configuration Schema](config-schema.md)
- [Bootstrap NOA](../pages/how-tos/bootstrap.md)

# Nudge System Guide

## Overview

The Nudge system enables asynchronous communication between PRP agents and human users via Telegram through the dcmaidbot endpoint. This provides a complete bidirectional communication channel for autonomous agent workflows.

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   PRP Agents    │───▶│  Nudge System    │───▶│  dcmaidbot       │
│                 │    │  Infrastructure  │    │  Endpoint        │
│ - robo-developer│    │                  │    │                 │
│ - robo-analyst │    │ - Client         │    │ - Telegram       │
│ - robo-aqa      │    │ - Wrapper        │    │ - LLM Processing │
│ - robo-designer │    │ - Agent          │    │ - GitHub         │
│ - robo-devops   │    │   Integration    │    │   Integration    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │                        │
                                ▼                        ▼
                       ┌──────────────────┐    ┌─────────────────┐
                       │  kubectl Secret  │    │  GitHub          │
                       │     Manager      │    │  Workflow        │
                       │                  │    │                 │
                       │ - Secret         │    │ - Response       │
                       │   Retrieval      │    │   Processing    │
                       │ - Caching        │    │ - PRP Updates    │
                       │ - Validation     │    │ - Issue Creation │
                       └──────────────────┘    └─────────────────┘
```

## Components

### 1. Nudge Client (`src/nudge/client.ts`)
- HTTP client for dcmaidbot communication
- Handles both direct and LLM-mode nudge delivery
- Retry logic and error handling
- Automatic secret management

### 2. Nudge Wrapper (`src/nudge/wrapper.ts`)
- Unified interface for both nudge types
- Intelligent fallback from LLM-mode to direct
- System status monitoring
- Configuration management

### 3. Agent Integration (`src/nudge/agent-integration.ts`)
- Standardized interfaces for all agents
- Message templates for different signal types
- Context-rich message formatting
- Automatic signal integration

### 4. kubectl Secret Manager (`src/kubectl/secret-manager.ts`)
- Kubernetes secret retrieval
- Automatic caching and refresh
- Secret validation
- Status monitoring

### 5. GitHub Workflow (`.github/workflows/nudge-response.yml`)
- Handles user responses from dcmaidbot
- Automatic PRP updates
- Error handling and issue creation
- Success notifications

## Usage Examples

### CLI Commands

#### Test Nudge System
```bash
npx prp nudge test
```

#### Send Manual Nudge
```bash
npx prp nudge send "Test message" --urgency high
```

#### Check System Status
```bash
npx prp nudge status
```

#### Retrieve NUDGE_SECRET from Kubernetes
```bash
npx prp secret kubectl --force-refresh
```

#### Validate Secret
```bash
npx prp secret validate
```

#### Check Secret Status
```bash
npx prp secret status
```

### Agent Integration

#### Send Feedback Request
```typescript
import { createAgentNudgeIntegration } from './nudge/agent-integration.js';

const nudgeIntegration = createAgentNudgeIntegration();

await nudgeIntegration.sendFeedbackRequest({
  prpId: 'nudge-endpoint-integrated',
  agentType: 'robo-orchestrator',
  topic: 'Architecture decision needed',
  proposal: 'Use microservices architecture',
  alternatives: ['Monolithic', 'Modular monolithic'],
  questions: ['What are the scaling requirements?'],
  urgency: 'medium'
});
```

#### Send Blocker Notification
```typescript
await nudgeIntegration.sendBlockerNotification({
  prpId: 'current-prp',
  agentType: 'robo-developer',
  blockerDescription: 'API endpoint not responding',
  impact: 'Cannot complete authentication feature',
  attemptedSolutions: ['Restarted services', 'Checked network'],
  neededAction: 'Deploy hotfix to production',
  urgency: 'high'
});
```

#### Send Goal Clarification
```typescript
await nudgeIntegration.sendGoalClarification({
  prpId: 'current-prp',
  agentType: 'robo-system-analyst',
  issue: 'Requirements unclear for authentication flow',
  currentUnderstanding: 'Basic JWT implementation needed',
  questions: ['Should we implement refresh tokens?'],
  options: ['JWT only', 'JWT + refresh', 'Full OAuth2'],
  recommendation: 'JWT + refresh for better UX',
  urgency: 'medium'
});
```

### Direct Client Usage

#### Send Direct Nudge
```typescript
import { createNudgeWrapper } from './nudge/wrapper.js';

const wrapper = createNudgeWrapper();

await wrapper.sendDirectNudge(
  'Critical system alert',
  'high',
  { prp_id: 'system-monitoring', signal: '[ic] Incident' }
);
```

#### Send LLM-Mode Nudge
```typescript
await wrapper.sendLLMModeNudge(
  'Complex decision needed for database architecture',
  { prp_id: 'database-design', agent_role: 'robo-devops' },
  'Current PostgreSQL setup is hitting performance limits',
  ['Migrate to distributed database', 'Optimize existing setup', 'Add caching layer'],
  'decision'
);
```

## Configuration

### Environment Variables
```bash
# Required
NUDGE_SECRET=your-secret-key
ADMIN_ID=your-telegram-id

# Optional
NUDGE_ENDPOINT=https://dcmaid.theedgestory.org/nudge
```

### Kubernetes Secret
```bash
# Retrieve NUDGE_SECRET from Kubernetes
kubectl get secret dcmaidbot-secrets -n dcmaidbot -o jsonpath='{.data.NUDGE_SECRET}' | base64 -d
```

## Signal Integration

The nudge system integrates with existing PRP signals:

- **[af] Feedback Requested**: Automatically sends nudge for admin decisions
- **[bb] Blocker Detected**: Sends urgent nudge for critical blockers
- **[gg] Goal Clarification**: Requests clarification on requirements
- **[oa] Orchestrator Attention**: Coordinates multi-agent workflows
- **[aa] Admin Attention**: Requests administrative decisions

## Message Templates

### Feedback Request
```
🔄 Feedback Request

PRP: {prp_id}
Agent: {agent_role}

**Topic:** {topic}

**Proposal:** {proposal}

**Alternatives Considered:** {alternatives}

**Questions:** {questions}

Please provide feedback on the proposed approach.
```

### Blocker Notification
```
🚫 Blocker Detected

PRP: {prp_id}
Agent: {agent_role}

**BLOCKER:** {blocker_description}

**Impact:** {impact}

**Attempted Solutions:** {attempted_solutions}

**Needed Action:** {needed_action}

**URGENCY:** {urgency}

Immediate attention required to unblock progress.
```

### Goal Clarification
```
🎯 Goal Clarification Needed

PRP: {prp_id}
Agent: {agent_role}

**Issue:** {issue}

**Current Understanding:** {current_understanding}

**Questions:** {questions}

**Options:** {options}

**Recommendation:** {recommendation}

Please provide clarification to proceed with implementation.
```

## Error Handling

The system includes comprehensive error handling:

- **Network Errors**: Automatic retry with exponential backoff
- **Authentication Errors**: Clear error messages and troubleshooting guidance
- **Validation Errors**: Input validation before sending requests
- **Fallback Mechanisms**: LLM-mode → direct nudge fallback
- **Cache Failures**: Graceful degradation when caching fails

## Monitoring and Status

### System Health Check
```typescript
const wrapper = createNudgeWrapper();
const status = await wrapper.getStatus();

console.log(`Status: ${status.status}`);
console.log(`Configured: ${status.details.client_config.configured}`);
console.log(`Fallback: ${status.details.fallback_enabled}`);
```

### Secret Manager Status
```typescript
import { createKubectlSecretManager } from './kubectl/secret-manager.js';

const manager = createKubectlSecretManager();
const status = await manager.getKubectlStatus();

console.log(`kubectl Available: ${status.available}`);
console.log(`Cluster Connected: ${status.connected}`);
```

## Testing

### Unit Tests
```bash
npm test -- src/nudge/__tests__
```

### Integration Tests
```bash
npm test -- --testPathPattern=integration.test.ts
```

### Manual Testing
```bash
# Test connectivity
npx prp nudge test

# Test secret retrieval
npx prp secret kubectl

# Test message sending
npx prp nudge send "Test message"
```

## GitHub Integration

The system includes automatic GitHub workflow integration:

1. **Repository Dispatch**: Triggered by dcmaidbot responses
2. **Secret Validation**: Validates NUDGE_SECRET from payload
3. **PRP Updates**: Automatically updates PRP files with responses
4. **Error Handling**: Creates GitHub issues for processing failures
5. **Success Notifications**: Sends confirmations back through nudge

### Response Payload Format
```json
{
  "prp": "nudge-endpoint-integrated",
  "user_handle": "dcversus",
  "response": "Use JWT with refresh tokens for better UX",
  "nudge_secret": "secret-value",
  "timestamp": "2025-01-01T00:00:00Z",
  "telegram_message_id": "tg_msg_123456"
}
```

## Troubleshooting

### Common Issues

1. **kubectl not found**
   ```bash
   # Install kubectl
   curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl"
   chmod +x kubectl
   sudo mv kubectl /usr/local/bin/
   ```

2. **Secret not accessible**
   ```bash
   # Check secret exists
   kubectl get secret dcmaidbot-secrets -n dcmaidbot

   # Check permissions
   kubectl auth can-i get secret dcmaidbot-secrets -n dcmaidbot
   ```

3. **Network connectivity issues**
   ```bash
   # Test dcmaidbot endpoint
   curl -I https://dcmaid.theedgestory.org/nudge
   ```

4. **Environment variables not set**
   ```bash
   # Check required variables
   echo $NUDGE_SECRET
   echo $ADMIN_ID

   # Set in .env file
   echo "NUDGE_SECRET=your-secret" >> .env
   echo "ADMIN_ID=your-telegram-id" >> .env
   ```

### Debug Mode
Enable debug logging:
```typescript
const wrapper = createNudgeWrapper({ debug: true });
```

### Cache Management
```bash
# Clear secret cache
npx prp secret clear-cache

# Check cache status
npx prp secret cache info
```

## Security Considerations

- **Secret Protection**: NUDGE_SECRET is never logged or exposed in error messages
- **Access Control**: Only authorized users can receive nudge messages
- **Rate Limiting**: Built-in rate limiting prevents abuse
- **Validation**: All inputs are validated before processing
- **Audit Trail**: All nudge communications are logged in PRP progress

## Future Enhancements

- **Multi-channel Support**: Extend beyond Telegram to other messaging platforms
- **Response Templates**: Pre-defined response templates for common scenarios
- **Batch Processing**: Send multiple nudges in a single request
- **Scheduling**: Schedule nudges for optimal delivery times
- **Analytics**: Track nudge effectiveness and response rates

## Support

For issues or questions:
1. Check this documentation
2. Review test examples in `src/nudge/__tests__/`
3. Check PRP progress logs for signal-specific guidance
4. Create GitHub issue with detailed error information
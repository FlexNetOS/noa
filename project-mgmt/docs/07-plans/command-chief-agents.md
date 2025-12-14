# Stack Chiefs - Command Chief Agent

code id: cca

## CommandChiefAgent

- **Purpose:** Master orchestrator for deploying, customizing, and overseeing the entire microagent stack for any resource.
- **Functionality:** `def deploy_stack(self, resource): ...`
- **Inputs:** resource, user_intent
- **Outputs:** deployment_plan, status
- **Triggers:** User onboarding request, System need detected
- **Reports To:**
- **Paired LLM:**
- **Requires Human:** False
- **Human Request Reason:** None
- **Escalation Path:** OrchestratorAgent
- **Last Updated:** 2025-05-17T21:45:00Z
- **Last Updated By:** System
- **Approval Status:** approved
- **Provenance:** updated_agent_manifest.json

### CommanderChiefAgent_DataStack

- **Purpose:** Local director/VP for DataStack; manages agents and operations within the data stack.
- **Functionality:** `def manage_stack(self, tasks): ...`
- **Reports To:** ExecutiveCommanderChiefAgent
- **Oversees:** PCOperatorAgent_DataStack, ModelSelectorAgent_DataStack
- **Model Selector:** ModelSelectorAgent_DataStack
- **Paired LLM:** qwen2.5
- **Requires Human:** False
- **Escalation Path:** ExecutiveCommanderChiefAgent
- **Approval Status:** approved
- **Provenance:** director_agent_manifest.json

### CommanderChiefAgent_DevOps

- **Purpose:** Local director/VP for DevOps Stack; manages DevOps agents and operations.
- **Functionality:** `def manage_stack(self, tasks): ...`
- **Reports To:** ExecutiveCommanderChiefAgent
- **Oversees:** PCOperatorAgent_DevOps, ModelSelectorAgent_DevOps
- **Model Selector:** ModelSelectorAgent_DevOps
- **Paired LLM:** mixtral-8x22b
- **Requires Human:** False
- **Escalation Path:** ExecutiveCommanderChiefAgent
- **Approval Status:** approved
- **Provenance:** director_agent_manifest.json

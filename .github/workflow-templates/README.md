# Workflow Templates

This directory contains workflow templates that can be used as references for setting up FlexNetOS automation in other repositories.

## Available Templates

### caller-template.yml
Template for calling reusable workflows from an organization's `.github` repository. 

**Note**: This template references external organization resources (`FlexNetOS/.github/workflows/flexnetos-reusable-resolver.yml@main`) that must exist in the organization's `.github` repository before use.

### flexnetos-reusable-resolver.yml
Reusable workflow template for organization-wide deployment of FlexNetOS automation.

**Note**: This template references an external action (`FlexNetOS/.github/actions/ai-resolver@main`) that must exist in the organization's `.github` repository before use.

## Usage

These are templates only and should not be placed in the `.github/workflows/` directory unless:
1. The required external organization resources exist
2. The templates have been customized for your specific use case
3. You have configured the necessary secrets (FLEXNETOS_BOT_TOKEN, AI_SESSION_TOKEN)

For active FlexNetOS automation in this repository, see `.github/workflows/flexnetos-auto-resolve.yml`.

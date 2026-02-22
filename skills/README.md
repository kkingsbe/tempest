# Skills Directory

This directory contains all available skills for the agent system. Skills are specialized capabilities that can be invoked by agents to perform specific tasks more effectively.

## Overview

Each skill is a self-contained module that provides:

- **Purpose**: A clearly defined use case or problem domain
- **Instructions**: Detailed guidelines for how the skill should be applied
- **Context**: Any necessary prerequisites or dependencies

## Skill Discovery

**Important**: Do not hardcode a list of available skills in documentation or code. The skills in this directory may change at any time as new skills are added, removed, or modified.

To discover available skills:

1. List all subdirectories in this directory (e.g., `ls -d */` or equivalent)
2. Each subdirectory represents a skill
3. Read the `SKILL.md` file within each skill directory to understand:
   - The skill's name and description
   - When and how to use it
   - Any specific requirements or parameters

## Skill Structure

Each skill follows a consistent structure:

```
skills/
└── [skill-name]/
    └── SKILL.md          # Main skill definition and instructions
    └── [optional files]  # Additional resources specific to the skill
```

### SKILL.md Format

The `SKILL.md` file is the authoritative source for understanding a skill. It contains:

- **Name**: The skill's identifier
- **Description**: What the skill does and when to use it
- **Instructions**: Step-by-step guidance for applying the skill
- **Constraints**: Any limitations or requirements

## For Agents

When operating as an agent:

1. Check this directory to discover available skills before attempting a task
2. Evaluate the user's request against skill descriptions to determine applicability
3. Load and follow the `SKILL.md` instructions for any applicable skill
4. Do not deviate from skill-defined workflows unless explicitly instructed

## Adding New Skills

To add a new skill:

1. Create a new subdirectory with a descriptive, kebab-case name
2. Create a `SKILL.md` file with the skill's definition
3. Add any supporting files or resources needed by the skill
4. Follow the established `SKILL.md` format for consistency

## Notes

- Skills are mode-specific and may have different implementations per mode
- Global skills are located at `C:\Users\Kyle\.kilocode\skills\`
- Project-level skills in this directory take precedence over global skills
- Always check the `SKILL.md` file for the most up-to-date information about a skill

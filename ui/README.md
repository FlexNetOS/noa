# User Interface (UI) Directory

This directory contains the user interface (UI) component of the system, which is responsible for providing an interactive platform for users to engage with the application. The UI handles user inputs, displays information, and ensures a seamless user experience across different devices.
It serves as the front-end layer that connects users to the underlying functionalities of the system.

## Key Features
- **Interactive Design**: Provides a user-friendly and visually appealing interface.
- **Responsive Layout**: Adapts to various screen sizes and devices for optimal usability.
- **User Input Handling**: Manages user interactions and inputs effectively.
- **Data Visualization**: Displays data and information in an easily understandable format.

## Directory Structure
- `components/`: Contains reusable UI components and widgets.
- `styles/`: Manages styling and theming for the UI.
- `views/`: Contains different views and pages of the application.
- `utils/`: Utility functions and helper modules for the UI.


ui/
├─ app/
│  ├─ shell/                       # main nav + layout
│  ├─ pages/
│  │  ├─ convo/                    # default home (chat + widgets)
│  │  ├─ tasks/                    # embedded task manager
│  │  ├─ hub/                      # unified “Tasks Hub” view (canonical)
│  │  │  ├─ app-A/                 # embedded view (optional)
│  │  │  ├─ app-B/
│  │  │  └─ app-C/
│  │  ├─ runs/                     # task execution runs (logs/artifacts)
│  │  └─ hive/                     # devices, compute, storage mesh view
│  └─ widgets/
│     ├─ task-summary/             # “My top tasks”, “Blocked”, “Agent running”
│     ├─ kanban-mini/
│     └─ dag-viewer/

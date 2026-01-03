# API, Connectors & Front‑End

## NOA Dynamic UI

Multi-Cross platform adaptive compatibility -
1. PC & Desktop - Windows 11, MacOS, Linux,
2. Mobile: Apple & Android
3. XR & MR & VR & AR: Glasses, Headset, Phone, Laptop, PC, Web
4. Web Search Engines: e.g. Chrome, Firefox, Edge

A Conversational Ai first UI/UX with Dynamic UI with real-time capabilities provides users with an interactive, adaptive, and highly responsive interface for monitoring and analysis. Its core features—real-time updates, dynamic component management, and interactive visualizations—enable users to gain actionable insights immediately. The "NOA interactive interface" would apply these principles to a system-specific, central dashboard, providing deeper customization, automation, and data interaction.

## Gateway API

The **Gateway API** is the central entry point for interacting with ark‑os‑noa’s backend services.  Implemented using FastAPI, it exposes endpoints for ingesting sources, spawning capsules, toggling CRM behaviours, ingesting models and administering the system.

### Key Endpoints

| Endpoint | Method | Description |
|---------|--------|-------------|
| `/digest` | POST | Submit a digest request.  The request includes sources (e.g. repo URL, API base URL), intent (integrate, analyse), and optional metadata.  It triggers the Intake Service and returns a job ID. |
| `/capsule/spawn` | POST | Spawn a new Capsule environment.  Returns Capsule identifiers and access tokens.  Used when custom stacks need to be run manually or via the front‑end. |
| `/crm/toggle` | POST | Toggle the CRM Strangler Proxy mode for a specific endpoint (e.g. enable write‑through for `/contacts`).  Allows incremental migration from external CRM to internal implementation. |
| `/models/ingest` | POST | Add a model to the local registry.  Accepts a model identifier (e.g. Hugging Face repo) and optional metadata.  The Model Serving service pulls the model and makes it available through the ModelSelector. |
| `/models/benchmark` | POST | Run evaluations on local or remote models.  Returns latency, cost and accuracy metrics that feed into the ModelSelector’s decision graph. |
| `/admin/*` | GET/POST | Administrative endpoints for tasks such as inspecting job statuses, viewing SBOMs, retrieving logs, enabling/disabling features (NATS, Supabase, vcluster) and rotating secrets.  Protected via authentication and authorisation. |

All endpoints accept and return JSON; error responses include descriptive messages and relevant codes.  The Gateway uses request identifiers and attaches trace IDs to facilitate debugging and correlation across services.

## Connectors & Integrations

Interacts with the outside world via **Adapters** and **Connectors**.  These modules encapsulate authentication, rate limiting, and protocol details, allowing the rest of the system to remain agnostic to third‑party specifics.

### Built‑in Connectors

- **GitHub Connector:** Uses the GitHub API to search, clone and pull repositories.  It supports scoping by organisation or repository and can read commit logs and PR metadata.
- **CRM Connector:** Provides read/write access to CRM systems (e.g. Salesforce, HubSpot).  Initially operates in shadow mode (read‑only) via the CRM Strangler Proxy; write‑through can be toggled per endpoint.  Handles pagination, rate limits and authentication.
- **Model Hub Connector:** Interfaces with external model repositories (e.g. Hugging Face).  Supports pulling models, downloading tokenizers and retrieving licences.  Works in conjunction with the Model Serving service.
- **Other API Connectors:** Additional connectors (e.g. for Slack, Notion, Jira) can be added by implementing the Adapter interface.  Each connector is packaged as its own microservice or plugin to preserve modularity.

### Internal Connectors

- **Registry & Object Store:** Adapters communicate with the private OCI registry and MinIO using signed URLs.  They ensure that images and artefacts are pushed/pulled securely and that content addressing is respected.
- **Database & Vector Store:** Adapters abstract database interactions.  They provide typed functions to query or insert metadata, run logs and embeddings without exposing SQL directly to the application logic.

## Front‑End (Admin Console)

The **Admin Console** is a web interface built with Next.js.  Its primary function is to give administrators and power users visibility and control over the system.  Major features include:

* **Jobs Dashboard:** Displays active and past digest jobs, their statuses, progress bars and any errors.  Users can drill down into individual jobs to view their `profile.json`, `system_card.md`, SBOMs and vulnerability reports.
* **Capacities & Capsules:** Shows currently running Capsules, their resource usage and health status.  Offers controls to spawn or destroy Capsules.
* **Artefacts Explorer:** Lists generated artefacts (zip files, PDFs, embeddings, SBOMs).  Allows downloading via signed URLs and cross‑referencing to their origins.
* **SBOM & Security:** Provides a dedicated section to review SBOMs, vulnerabilities, licences and risk scores.  Policies can be configured here (e.g. accepted licence list, vulnerability severity thresholds).
* **Model Registry & Selector:** Displays available models, their metadata, benchmarks and usage statistics.  Administrators can add models to the ingestion queue or deprecate existing ones.  The ModelSelector’s decisions and rationales are visible for transparency.
* **CRM Controls:** Allows toggling of CRM endpoint modes (shadow/write‑through), viewing recent calls, and measuring divergence between external CRM data and internal state.
* **Settings & Feature Flags:** Provides toggles for enabling/disabling optional services (NATS, Supabase, vcluster) and adjusting environment variables.  Also offers secret rotation and certificate management.

## Interaction Patterns

* **External Clients:** Use the Gateway API to submit work.  They receive job IDs and can query progress or results.  Authentication tokens limit access based on roles.
* **Internal Agents:** Call endpoints via Adapters.  For example, a CommandChiefAgent may call `/digest` to start digestion for a new source or `/models/ingest` to add an in‑house model.  Internal calls attach run IDs and context for traceability.
* **Front‑End Users:** Access the Admin Console to monitor and control the system.  When they trigger actions (e.g. toggling a CRM endpoint), the console issues calls to the Gateway API on their behalf.

By exposing a clear API and a rich front‑end, ark‑os‑noa ensures that humans and agents can seamlessly interact with the system, inspect its state and adapt its behaviour without compromising security or traceability.


## Expanded dynamic UI features

Real-time dashboard and widget modifications

Rather than static reports, a dynamic UI allows for immediate, on-the-fly adjustments to the dashboard layout and its components.

• Live configuration: Users can add, remove, and re-arrange widgets through a drag-and-drop interface. Changes are reflected instantly without a page refresh.

• Contextual UI: The system can dynamically change the dashboard or widget based on real-time events. For example, during a system alert, a red banner might appear, or a specific metric widget might be automatically enlarged and moved to the top of the screen.

• User-specific presets: Users can save and switch between multiple dashboard layouts tailored for different tasks, such as monitoring, reporting, or troubleshooting.
Spinning up and spinning down
This feature is about the dynamic provisioning and de-provisioning of UI components, like new dashboards or processing resources, to manage system load or user needs.

• Dynamic resource allocation: When an engineer starts a performance analysis, the system "spins up" a dedicated, compute-intensive dashboard. When the task is complete, the dashboard and its resources are "spun down" to save processing power.

• On-demand environments: A user could spin up a new, temporary dashboard for a specific event, complete with a unique set of monitoring widgets. After the event, the dashboard can be archived or deleted.

• Scaling based on need: If monitoring an application requires more data streams, the system could spin up additional widgets and adjust the dashboard layout automatically to accommodate the new information.
Interactive dashboards, widgets, and graphs
The "interactive" element is about enabling users to manipulate and explore the data directly within the dashboard's visualizations.

• Drill-down functionality: Clicking on a data point in a graph can reveal more granular detail. For example, selecting a spike on a CPU usage graph could open a table of the processes running at that time.

• Real-time filtering: Users can filter dashboard data by time, dimension, or other metrics, with all relevant widgets updating instantly.

• Interactive graphs: Users can zoom, pan, and hover over data points for tooltips that provide additional context. This includes features like animated transitions and dynamic axis scaling.

• AI-assisted insights: Widgets with built-in AI could automatically highlight anomalies, predict trends, or suggest related data points for further analysis.
Expansion of the "Main NOA interactive interface"
Assuming "NOA" refers to a centralized control system (like a "Network Orchestration and Automation" or similar platform), the expansion would focus on making the core interface a more powerful, all-encompassing control center.
Personalized command and control center

• Customizable workspace: Users can create personal dashboards by dragging and dropping widgets that matter most to their roles, such as network status, service health, or resource utilization.

• "My Favorites" shortcuts: Frequently used actions, reports, or deep-dive views can be saved as bookmarks or quick-access buttons on the main interface.

• User role-based views: The main dashboard can automatically adapt its content and capabilities based on the user's permissions, ensuring they see relevant information while maintaining security.
Advanced contextual awareness

• Unified data streams: The main interface integrates data from all connected systems into a single, cohesive view, eliminating the need for users to switch between different applications.

• Cross-dashboard interaction: Filters and selections made on one widget or dashboard automatically apply to other linked areas of the interface, providing a consistent and contextual view across the system.

• Predictive visualization: The interface uses predictive analytics to highlight potential issues or upcoming bottlenecks on the main dashboard, allowing for proactive intervention.
Streamlined operational capabilities

• Real-time status monitor: A central "system health" widget provides an at-a-glance overview of the entire NOA environment with color-coded indicators for quick assessment.

• Task automation widgets: The interface can include widgets that trigger pre-defined automation scripts with a single click, such as "Restart Service" or "Provision New Resources."

• Collaborative features: Team members can work together in real-time within the interface, sharing views, commenting on data, and receiving updates on shared tasks.
Enhanced data exploration and analysis

• Embedded analytics: Instead of exporting data to another tool, the interface includes powerful embedded analytics features, such as the ability to perform complex queries directly on the visualized data.

• Timeline analysis: A global timeline control allows users to go back in time, replaying events and seeing how the dashboard changed during a specific period.

• Interactive topology maps: A map of the network or system topology could be a primary widget, allowing users to click on a device to drill down into its real-time health and performance metrics.

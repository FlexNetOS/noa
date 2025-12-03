import { KPIMetrics } from "@/components/charts/workspace/kpi-metrics";
import { ProjectStatusChart } from "@/components/charts/workspace/project-status-chart";
import { TaskPriorityChart } from "@/components/charts/workspace/task-priority-chart";
import { TaskTypeChart } from "@/components/charts/workspace/task-type-chart";
import { SprintStatusChart } from "@/components/charts/workspace/sprint-status-chart";
import { MonthlyTaskCompletionChart } from "@/components/charts/workspace/monthly-task-completion-chart";
import { Widget } from "@/types/analytics";

export const workspaceWidgets: Widget[] = [
  {
    id: "kpi-metrics",
    title: "KPI Metrics",
    component: KPIMetrics,
    dataKey: "kpiMetrics",
    visible: true,
    gridCols: "col-span-full",
    priority: 1,
  },
  {
    id: "project-status",
    title: "Project Status Distribution",
    component: ProjectStatusChart,
    dataKey: "projectStatus",
    visible: true,
    gridCols: "col-span-1 md:col-span-1",
    priority: 2,
  },
  {
    id: "task-priority",
    title: "Task Priority Breakdown",
    component: TaskPriorityChart,
    dataKey: "taskPriority",
    visible: true,
    gridCols: "col-span-1 md:col-span-1",
    priority: 3,
  },
  {
    id: "task-type",
    title: "Task Type Distribution",
    component: TaskTypeChart,
    dataKey: "taskType",
    visible: true,
    gridCols: "col-span-1 md:col-span-1",
    priority: 4,
  },
  {
    id: "sprint-status",
    title: "Sprint Status Overview",
    component: SprintStatusChart,
    dataKey: "sprintStatus",
    visible: true,
    gridCols: "col-span-1 md:col-span-1",
    priority: 5,
  },
  {
    id: "monthly-completion",
    title: "Monthly Task Completion",
    component: MonthlyTaskCompletionChart,
    dataKey: "monthlyCompletion",
    visible: false,
    gridCols: "col-span-full",
    priority: 6,
  },
];

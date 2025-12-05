import { Task } from "./tasks";

export interface TimeRange {
  start: Date;
  end: Date;
  days: Date[];
}

// Types
export interface TaskGanttViewProps {
  tasks: Task[];
  workspaceSlug: string;
  projectSlug?: string;
  onTaskUpdate?: (taskId: string, updates: Partial<Task>) => Promise<void>;
  viewMode?: ViewMode;
  onViewModeChange?: (mode: ViewMode) => void;
}

export interface TimelineHeaderProps {
  timeRange: TimeRange;
  viewMode: ViewMode;
  isCompact: boolean;
  scrollToToday: () => void;
}

export type ViewMode = "days" | "weeks" | "months";

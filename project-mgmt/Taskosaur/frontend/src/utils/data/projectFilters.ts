import { CheckSquare, Flame, Clock, CheckCircle, PauseCircle, Calendar } from "lucide-react";

export const availableStatuses = [
  {
    id: "ACTIVE",
    name: "Active",
    value: "ACTIVE",
    color: "#10b981",
    icon: CheckCircle,
  },
  {
    id: "PLANNING",
    name: "Planning",
    value: "PLANNING",
    color: "#f59e0b",
    icon: Calendar,
  },
  {
    id: "ON_HOLD",
    name: "On Hold",
    value: "ON_HOLD",
    color: "#6b7280",
    icon: PauseCircle,
  },
  {
    id: "COMPLETED",
    name: "Completed",
    value: "COMPLETED",
    color: "#3b82f6",
    icon: CheckSquare,
  },
];

export const availablePriorities = [
  { id: "LOW", name: "Low", value: "LOW", color: "#6b7280", icon: Clock },
  {
    id: "MEDIUM",
    name: "Medium",
    value: "MEDIUM",
    color: "#f59e0b",
    icon: Flame,
  },
  { id: "HIGH", name: "High", value: "HIGH", color: "#ef4444", icon: Flame },
  {
    id: "URGENT",
    name: "Urgent",
    value: "URGENT",
    color: "#dc2626",
    icon: Flame,
  },
];

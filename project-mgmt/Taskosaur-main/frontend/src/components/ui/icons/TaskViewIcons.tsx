import React from "react";
import { HiViewList, HiChartBar } from "react-icons/hi";

export const taskViewTypes = [
  {
    name: "List",
    key: "list",
    icon: <HiViewList size={16} />,
  },
  {
    name: "Gantt",
    key: "gantt",
    icon: <HiChartBar size={16} />,
  },
];

export default taskViewTypes;

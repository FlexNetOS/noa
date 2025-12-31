export function normalizeAssignee(task: { assignee?: string | string[] }): void {
	if (typeof task.assignee === "string") {
		task.assignee = [task.assignee];
	} else if (!Array.isArray(task.assignee)) {
		task.assignee = [];
	}
}

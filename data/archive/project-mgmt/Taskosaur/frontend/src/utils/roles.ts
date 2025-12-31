// roles.ts
export type Role = "SUPER_ADMIN" | "OWNER" | "MANAGER" | "MEMBER" | "VIEWER";

export const rolePermissions = {
  organization: {
    create: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    update: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    archive: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    delete: ["SUPER_ADMIN", "OWNER"],
    read: ["SUPER_ADMIN", "OWNER", "MANAGER", "MEMBER", "VIEWER"],
  },
  workspace: {
    create: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    update: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    archive: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    delete: ["SUPER_ADMIN", "OWNER"],
    read: ["SUPER_ADMIN", "OWNER", "MANAGER", "MEMBER", "VIEWER"],
  },
  project: {
    create: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    update: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    archive: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    delete: ["SUPER_ADMIN", "OWNER"],
    read: ["SUPER_ADMIN", "OWNER", "MANAGER", "MEMBER", "VIEWER"],
  },
  task: {
    create: ["SUPER_ADMIN", "OWNER", "MANAGER", "MEMBER"],
    update: ["SUPER_ADMIN", "OWNER", "MANAGER", "MEMBER"],
    delete: ["SUPER_ADMIN", "OWNER", "MANAGER"],
    read: ["SUPER_ADMIN", "OWNER", "MANAGER", "MEMBER", "VIEWER"],
  },
  visibility_rules: {
    SUPER_ADMIN: "all",
    OWNER: "all",
    MANAGER: "all",
    MEMBER: "self_related_only",
    VIEWER: "self_related_only",
  },
} as const;

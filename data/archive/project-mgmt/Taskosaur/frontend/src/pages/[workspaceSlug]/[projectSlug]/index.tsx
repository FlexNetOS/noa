// In your project page

import { ProjectAnalytics } from "@/components/projects/ProjectAnalytics";
import { useRouter } from "next/router";

export default function ProjectPage() {
  const router = useRouter();
  const { projectSlug } = router.query;

  return (
    <div className="dashboard-container">
      <ProjectAnalytics projectSlug={projectSlug as string} />
    </div>
  );
}

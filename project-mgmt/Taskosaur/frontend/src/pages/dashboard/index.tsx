import { OrganizationAnalytics } from "@/components/organizations/OrganizationAnalytics";
import { TokenManager } from "@/lib/api";

export default function DashboarPage() {
  const orgId = TokenManager.getCurrentOrgId();
  return (
    <div className="dashboard-container">
      <OrganizationAnalytics organizationId={orgId} />
    </div>
  );
}

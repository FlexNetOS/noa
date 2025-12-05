# Commands

---

## NOA Quick Commands (inside WSL):

  cda  # cd $NOA_ROOT (NOA root)
  cdr  # cd $NOA_ROOT/repos
  cdc  # cd $NOA_ROOT/containers
  cdw  # cd $NOA_ROOT/workspace
  cdp  # cd $NOA_ROOT/p2p

---

## NOA P2P Server Commands:

  noa start      # Start P2P server
  noa stop       # Stop P2P server
  noa status     # Check server status
  noa nodes      # List connected nodes
  noa storage    # Check storage
  noa compute    # Check compute resources

---

## NOA Multi-Provider AI Commands:

  noa ai providers          # List AI providers
  noa ai devices            # List registered devices
  noa ai shared             # Show shared resources
  noa ai switch <provider>  # Switch AI provider

---

## NOA Device Orchestration:

  noa device register <name>  # Register device
  noa device list             # List devices
  noa device capabilities     # Show capabilities

---

## NOA Self-Containment Commands:

  bundle-libraries <binary>    # Bundle libraries for a binary
  bundle-all-libs            # Bundle libraries for all NOA binaries
  download-static-binaries   # Download static binaries (no deps)
  noa-kmod check             # Check kernel module availability
  noa-kmod load <module>     # Load kernel module
  noa-kernel-params set <p> <v>  # Set kernel parameter

---

## NOA Local Git CI/CD Commands:

  git-pr create <branch>     # Create local PR
  git-pr list                 # List local PRs
  git-pr merge <pr-id>        # Merge PR (AI conflict resolution)
  git-pr sync <pr-id>         # Sync PR to GitHub
  git-conflict detect         # Detect conflicts
  git-conflict resolve <id>   # Resolve with AI/MLOps
  git-ci run <pipeline>       # Run local CI/CD
  git-ci status              # Check CI/CD status

---

## Verification commands (run inside WSL):

  df -h          # Check disk space
  free -h        # Check memory and swap
  nproc          # Check CPU cores
  docker ps      # Check Docker containers
  node --version # Check Node.js version
  nvidia-smi     # Check GPU (if NVIDIA)

---

## Self-Containment Status:

  Bootstrap Phase: Uses apt-get for initial setup (one-time)
  Self-Contained: All binaries and configs in $NOA_ROOT
  Library Bundling: System ready - run 'bundle-all-libs' to complete
  Kernel Isolation: Namespace and cgroup support ready

---

## To complete self-containment after first boot:

  cd $NOA_ROOT
  ./scripts/bundle-all-libs           # Bundle all libraries
  ./scripts/download-static-binaries # Get static binaries
  ./scripts/noa-kmod check            # Verify kernel modules
  ./init/noa-init start               # Start NOA services

---

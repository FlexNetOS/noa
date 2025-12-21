# NOA_HOME vs NOA_ROOT Environment Variables

---

## The Conceptual Difference

When building a cross-platform, hardware-agnostic application (often referred to as a "portable" or "self-contained" app), the distinction between `_HOME` and `_ROOT` variables is primarily about **hierarchy** and **scope**.

While different frameworks might use these terms slightly differently, here is the industry-standard logic for defining these paths in a portable ecosystem.

### 1. NOA_ROOT (The Ecosystem Base)

Think of `NOA_ROOT` as the absolute **anchor point** for the entire installation. It defines where the "world" of your application begins.

* **Scope:** Global.
* **Purpose:** To locate shared resources that don't change regardless of which specific version of the app you are running.
* **Contents:** Global libraries, shared assets, plugin folders, and potentially multiple versions of the application.
* **Hardware Impact:** On a PC, this might be `C:\Games\NOA`. On XR glasses, it might be `/storage/emulated/0/Android/data/com.noa.platform/`.

### 2. NOA_HOME (The Active Instance)

`NOA_HOME` refers to the **active directory** where the specific binaries and configurations for the current execution reside.

* **Scope:** Instance-specific.
* **Purpose:** To tell the OS where the "heart" of the currently running app is.
* **Contents:** The executable (`.exe`, `.bin`), the specific version's config files, and the local `bin/` or `lib/` folders.
* **Hardware Impact:** This is where the specific hardware-optimized binary for that device (e.g., the ARM binary for XR glasses vs. the x86 binary for a PC) is located.

---

## Comparison Table

| Feature | NOA_ROOT | NOA_HOME |
| --- | --- | --- |
| **Logic** | "Where is the platform installed?" | "Where is the active app running?" |
| **Hierarchy** | Usually the Parent directory. | Usually a Child of the Root. |
| **Portability** | Hard-coded to the drive/mount point. | Relative to the Root. |
| **Example Path** | `/opt/noa/` | `/opt/noa/v1.2.0/` |
| **XR Context** | The persistent data storage area. | The specific runtime/app bundle folder. |

---

## Why the distinction matters for "Install Anywhere"

If you want your app to be truly "hardware agnostic" and stay contained within its own folder (avoiding system folders like `AppData` or `/usr/local`), you use these variables to resolve paths dynamically:

1. **Version Switching:** You can have one `NOA_ROOT` but multiple `NOA_HOME` directories (e.g., `NOA_HOME_V1` and `NOA_HOME_V2`). This allows for seamless updates without breaking global links.
2. **Hardware Abstraction:** On a **Server**, your script might look for logs in `$NOA_ROOT/logs`. On **XR Glasses**, the same script works because it doesn't care about the absolute path, only the relationship to the Root.
3. **Dependency Resolution:** If the app needs a shared library, it looks in `$NOA_ROOT/shared/lib`. If it needs its own private config, it looks in `$NOA_HOME/config`.

### The "Single Folder" Exception

If your application is very simple and you don't plan on having multiple versions or shared global assets, **NOA_HOME and NOA_ROOT are often set to the same path.** However, defining both from the start is a best practice for future-proofing your architecture.

> **Pro Tip:** For a "Global installs stay in the app" approach, ensure your code never uses absolute paths (like `C:\...`). Always derive paths using `os.path.join(NOA_ROOT, 'data')`.

---

Would you like me to help you draft a configuration script or a **Mandatory** `.sh/.bat` file that automatically detects and sets these paths based on the device it's running on.

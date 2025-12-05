from pathlib import Path
import sys
import os

def find_directories(project_dir, target_dirs):
    """Find directories using multiple search strategies."""
    project_dir = Path(project_dir).resolve()
    print(f"Looking for directories in: {project_dir}")
    print(f"Target directories: {target_dirs}")
    
    found_dirs = []
    
    # Strategy 1: Direct lookup at the top level
    print("\nStrategy 1: Direct top-level lookup")
    for target in target_dirs:
        path = project_dir / target
        if path.exists() and path.is_dir():
            print(f"✅ Found at top level: {path}")
            found_dirs.append(path)
        else:
            print(f"❌ Not found at top level: {target}")
    
    # Strategy 2: Search one level down
    print("\nStrategy 2: One level down search")
    for item in project_dir.iterdir():
        if item.is_dir() and not item.name.startswith('.'):
            for target in target_dirs:
                path = item / target
                if path.exists() and path.is_dir():
                    print(f"✅ Found one level down: {path}")
                    found_dirs.append(path)
    
    # Strategy 3: Case-insensitive search
    print("\nStrategy 3: Case-insensitive search")
    for target in target_dirs:
        target_lower = target.lower()
        for item in project_dir.rglob("*"):
            if item.is_dir() and item.name.lower() == target_lower:
                print(f"✅ Found with case-insensitive search: {item}")
                found_dirs.append(item)
    
    # Strategy 4: Partial name match
    print("\nStrategy 4: Partial name match")
    for target in target_dirs:
        for item in project_dir.rglob("*"):
            if item.is_dir() and target.lower() in item.name.lower():
                print(f"✅ Found with partial name match: {item}")
                found_dirs.append(item)
    
    print(f"\nTotal found directories: {len(set(found_dirs))}")
    for dir_path in sorted(set(found_dirs)):
        print(f"  - {dir_path}")
    
    return found_dirs

if __name__ == "__main__":
    # Get target directory and focus dirs from command line
    project_dir = sys.argv[1] if len(sys.argv) > 1 else "../cursor-chat-browser"
    target_dirs = sys.argv[2:] if len(sys.argv) > 2 else ["api", "app", "src"]
    
    find_directories(project_dir, target_dirs)
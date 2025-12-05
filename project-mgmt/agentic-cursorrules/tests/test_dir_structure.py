from pathlib import Path
import sys
import os

def explore_directory(target_dir, max_depth=2):
    """List all directories in the target project up to a specified depth."""
    target_dir = Path(target_dir).resolve()
    print(f"Exploring directory structure of: {target_dir}\n")
    
    def _explore(current_dir, depth=0):
        if depth > max_depth:
            return
            
        if depth == 0:
            print(f"Root: {current_dir}")
        
        # List immediate subdirectories
        subdirs = [d for d in current_dir.iterdir() if d.is_dir() and not d.name.startswith('.')]
        for subdir in sorted(subdirs):
            indent = "  " * depth
            print(f"{indent}├── {subdir.name}/")
            _explore(subdir, depth + 1)
    
    _explore(target_dir)
    
if __name__ == "__main__":
    # Get target directory from command line or use default
    target_dir = sys.argv[1] if len(sys.argv) > 1 else "../cursor-chat-browser"
    explore_directory(target_dir)
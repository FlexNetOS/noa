from pathlib import Path
import sys
import os
from collections import Counter
import yaml

def analyze_project_structure(project_dir):
    """Analyze project structure to find directories with code files."""
    project_dir = Path(project_dir).resolve()
    print(f"Analyzing project structure of: {project_dir}")
    
    # Common code file extensions
    code_extensions = {
        '.py', '.js', '.jsx', '.ts', '.tsx', '.html', '.css', '.scss',
        '.java', '.c', '.cpp', '.h', '.cs', '.go', '.rb', '.php',
        '.vue', '.svelte', '.json', '.yaml', '.yml', '.md'
    }
    
    # Directories to exclude
    exclude_dirs = {
        'node_modules', 'dist', 'build', '.git', '__pycache__',
        'venv', 'env', '.next', 'out'
    }
    
    code_files = []
    dir_counts = Counter()
    
    # Walk the directory tree
    for root, dirs, files in os.walk(str(project_dir)):
        # Skip excluded directories
        dirs[:] = [d for d in dirs if d not in exclude_dirs and not d.startswith('.')]
        
        # Count code files
        for file in files:
            file_ext = os.path.splitext(file)[1].lower()
            if file_ext in code_extensions:
                rel_path = os.path.relpath(root, str(project_dir))
                if rel_path == '.':
                    rel_path = ''
                
                # Count each directory
                if rel_path:
                    dir_counts[rel_path] += 1
                    
                    # Also count parent directories
                    parts = rel_path.split(os.sep)
                    for i in range(1, len(parts)):
                        parent = os.sep.join(parts[:i])
                        dir_counts[parent] += 1
    
    # Get top-level directories
    top_dirs = [d for d in dir_counts.keys() if not os.sep in d]
    top_dirs = sorted(top_dirs, key=lambda d: -dir_counts[d])
    
    # Get important subdirectories
    important_subdirs = []
    for dir_path, count in dir_counts.items():
        if os.sep in dir_path and count >= 5:  # Only significant subdirs
            important_subdirs.append(dir_path)
    
    important_subdirs = sorted(important_subdirs, key=lambda d: -dir_counts[d])[:10]  # Top 10
    
    # Combine for focus directories
    focus_dirs = top_dirs[:5] + important_subdirs  # Top 5 top-level + important subdirs
    
    print("\nMost important directories (by code file count):")
    for dir_path in focus_dirs:
        print(f"  - {dir_path} ({dir_counts[dir_path]} code files)")
    
    # Generate config.yaml
    config = {
        'project_title': project_dir.name,
        'tree_focus': focus_dirs,
        'exclude_dirs': list(exclude_dirs)
    }
    
    config_path = Path('detected_config.yaml')
    with open(config_path, 'w') as f:
        yaml.dump(config, f, default_flow_style=False, sort_keys=False)
    
    print(f"\nGenerated config.yaml at {config_path.resolve()}")
    print(f"Found {len(focus_dirs)} important directories")
    
    return focus_dirs

if __name__ == "__main__":
    # Get target directory from command line
    project_dir = sys.argv[1] if len(sys.argv) > 1 else "../cursor-chat-browser"
    analyze_project_structure(project_dir)
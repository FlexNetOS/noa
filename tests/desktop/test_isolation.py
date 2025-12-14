"""Desktop containment smoke tests (Phase 19 T877).

These tests validate that desktop apps are redirected into NOA-controlled paths
and that launcher wrappers enforce the redirection in both Windows (.cmd) and
Unix scripts.
"""
import json
import os
from pathlib import Path

NOA_ROOT = Path(__file__).resolve().parents[2]
DATA_DIR = NOA_ROOT / "data" / "apps"
CONFIG_FILE = NOA_ROOT / "config" / "desktop-apps.json"


def _assert_contains(path: Path, needle: str) -> None:
    text = path.read_text(encoding="utf-8", errors="ignore")
    assert needle in text, f"{needle!r} not found in {path}"


def test_config_points_to_data_apps():
    assert CONFIG_FILE.exists(), "desktop-apps.json missing"
    config = json.loads(CONFIG_FILE.read_text())
    assert "apps" in config, "apps key missing"
    for app in config["apps"]:
        data_path = app.get("dataPath", "")
        assert "/data/apps/" in data_path or "\\data\\apps\\" in data_path, (
            f"dataPath for {app.get('id')} does not target NOA data: {data_path}"
        )


def test_data_directories_exist():
    required = {"chatgpt", "claude", "github-desktop", "cursor", "vscode"}
    for name in required:
        path = DATA_DIR / name
        assert path.exists(), f"{path} should exist for data redirection"


def test_windows_wrappers_redirect_appdata():
    wrappers = [
        "bin/chatgpt.cmd",
        "bin/claude-desktop.cmd",
        "bin/github-desktop.cmd",
        "bin/cursor.cmd",
        "bin/code.cmd",
    ]
    for rel in wrappers:
        _assert_contains(NOA_ROOT / rel, "data\\apps\\")


def test_unix_wrappers_redirect_xdg():
    wrappers = [
        "bin/chatgpt",
        "bin/claude-desktop",
        "bin/github-desktop",
        "bin/cursor",
        "bin/code",
    ]
    for rel in wrappers:
        _assert_contains(NOA_ROOT / rel, "/data/apps/")

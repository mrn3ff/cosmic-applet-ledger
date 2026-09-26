#!/usr/bin/env python3
"""Crop Ledger panels from full-screen screenshots."""
import argparse
import os
import subprocess
import tempfile
from pathlib import Path

SRC_DIR = Path.home() / "Pictures/Screenshots"
DST_DIR = Path(__file__).parent.parent / "resources/screenshots"

CROP_LEFT = 2891
CROP_TOP = 37
CROP_WIDTH = 396

HERO = (
    "Screenshot_2026-09-03_12-04-04.png",
    "screenshot-hero.png",
    "1190x888+2250+0",
)
DETAIL = (
    "Screenshot_2026-09-03_12-04-04.png",
    "screenshot-detail.png",
    "396x687+2891+37",
)

PROVIDERS = [
    ("Screenshot_2026-09-03_12-04-30.png", "codex", 687),
    ("Screenshot_2026-09-03_12-04-42.png", "claude", 785),
    ("Screenshot_2026-09-03_12-04-55.png", "cursor", 656),
    ("Screenshot_2026-09-03_12-05-06.png", "antigravity", 853),
    ("Screenshot_2026-09-03_12-05-17.png", "gemini", 656),
    ("Screenshot_2026-09-03_12-05-31.png", "copilot", 627),
    ("Screenshot_2026-09-03_12-05-40.png", "minimax", 572),
    ("Screenshot_2026-09-03_12-05-56.png", "kimi", 588),
    ("Screenshot_2026-09-03_12-06-10.png", "opencode-go", 656),
]

SETTINGS = [
    ("Screenshot_2026-09-03_12-06-21.png", "screenshot-settings.png", 528),
    ("Screenshot_2026-09-03_12-06-50.png", "screenshot-accounts.png", 479),
]

THEMES = [
    ("Screenshot_2026-09-03_12-22-40.png", "screenshot-theme-dark-orange.png", 785),
    ("Screenshot_2026-09-03_12-22-53.png", "screenshot-theme-dark-blue.png", 785),
    ("Screenshot_2026-09-03_12-23-06.png", "screenshot-theme-light-blue.png", 785),
    ("Screenshot_2026-09-03_12-23-21.png", "screenshot-theme-light-red.png", 785),
]


def crop_image(src: Path, dst: Path, crop: str) -> None:
    if not src.exists():
        print(f"skip: missing {src}")
        return
    with tempfile.NamedTemporaryFile(suffix=dst.suffix, dir=dst.parent, delete=False) as file:
        candidate = Path(file.name)
    try:
        subprocess.run(
            ["convert", str(src), "-crop", crop, "+repage", str(candidate)],
            check=True,
        )
        if dst.exists():
            comparison = subprocess.run(
                ["compare", "-metric", "AE", str(dst), str(candidate), "null:"],
                capture_output=True,
            )
            if comparison.returncode == 0:
                print(f"Unchanged {dst}")
                return
            if comparison.returncode != 1:
                comparison.check_returncode()
        os.replace(candidate, dst)
        print(f"Saved {dst}")
    finally:
        candidate.unlink(missing_ok=True)


def crop_panel(src: Path, dst: Path, height: int) -> None:
    crop_image(src, dst, f"{CROP_WIDTH}x{height}+{CROP_LEFT}+{CROP_TOP}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-dir", type=Path, default=SRC_DIR)
    parser.add_argument("--destination-dir", type=Path, default=DST_DIR)
    args = parser.parse_args()
    args.destination_dir.mkdir(parents=True, exist_ok=True)
    for src_name, dst_name, crop in (HERO, DETAIL):
        crop_image(
            args.source_dir / src_name,
            args.destination_dir / dst_name,
            crop,
        )
    for src_name, provider, height in PROVIDERS:
        src = args.source_dir / src_name
        dst = args.destination_dir / f"screenshot-zoom-{provider}.png"
        crop_panel(src, dst, height)
    for src_name, dst_name, height in SETTINGS:
        crop_panel(args.source_dir / src_name, args.destination_dir / dst_name, height)
    for src_name, dst_name, height in THEMES:
        crop_panel(args.source_dir / src_name, args.destination_dir / dst_name, height)


if __name__ == "__main__":
    main()

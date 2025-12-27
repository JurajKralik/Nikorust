#!/usr/bin/env python3
import re
from pathlib import Path
from datetime import date

CARGO_TOML = Path("nikolaj/Cargo.toml")
text = CARGO_TOML.read_text(encoding="utf-8")

match = re.search(r"(?m)^\[package\](.*?)(?=^\[|\Z)", text, re.S)
if not match:
	raise SystemExit("No [package] section found")

package_block = match.group(1)

version_match = re.search(r'version\s*=\s*"(\d+)\.(\d+)\.(\d+)"', package_block)
if not version_match:
	raise SystemExit("No version line found in [package] section")

major, minor, patch = map(int, version_match.groups())
patch += 1
if patch >= 10:
	patch = 0
	minor += 1

new_version = f"{major}.{minor}.{patch}"
print(f"Bumping version → {new_version}")

package_block = re.sub(
	r'version\s*=\s*"\d+\.\d+\.\d+"',
	f'version = \"{new_version}\"',
	package_block,
)

today = date.today()
new_description = f'A StarCraft II Terran bot. Last build: {today.day:02d}/{today.month:02d}/{today.year}'

if re.search(r'description\s*=', package_block):
	package_block = re.sub(
		r'description\s*=\s*".*?"',
		f'description = "{new_description}"',
		package_block,
	)
else:
	insert_pos = re.search(r'(name\s*=\s*".*?")', package_block)
	if insert_pos:
		idx = insert_pos.end()
		package_block = package_block[:idx] + f'\n' + f'description = "{new_description}"' + package_block[idx:]
	else:
		package_block = f'description = "{new_description}"\n' + package_block

new_text = text.replace(match.group(1), package_block, 1)
CARGO_TOML.write_text(new_text, encoding="utf-8")
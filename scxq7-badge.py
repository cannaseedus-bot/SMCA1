#!/usr/bin/env python3
import hashlib
import json
import sys
from pathlib import Path


def fatal(message: str) -> None:
    print(f"error: {message}", file=sys.stderr)
    sys.exit(1)


def read_badge(path: Path) -> dict:
    try:
        data = path.read_text(encoding="utf-8")
    except OSError as exc:
        fatal(f"cannot read {path}: {exc}")
    try:
        return json.loads(data)
    except json.JSONDecodeError as exc:
        fatal(f"invalid JSON in {path}: {exc}")


def require_string(obj: dict, keys: list[str]) -> str:
    current = obj
    for key in keys[:-1]:
        if key not in current or not isinstance(current[key], dict):
            fatal(f"missing required field: {'.'.join(keys)}")
        current = current[key]
    leaf = keys[-1]
    if leaf not in current or not isinstance(current[leaf], str) or not current[leaf]:
        fatal(f"missing required field: {'.'.join(keys)}")
    return current[leaf]


def short_manifest_hash(manifest_hash: str) -> str:
    if ":" in manifest_hash:
        _, payload = manifest_hash.split(":", 1)
    else:
        payload = manifest_hash
    if len(payload) < 12:
        fatal("manifest_hash too short for 12-hex short hash")
    return payload[:12]


def hue_from_badge_id(badge_id: str) -> int:
    digest = hashlib.sha3_256(badge_id.encode("utf-8")).digest()
    value = (digest[0] << 8) | digest[1]
    return value % 360


def render_svg(badge_id: str, short_hash: str) -> str:
    hue = hue_from_badge_id(badge_id)
    fill = f"hsl({hue},60%,45%)"
    return (
        f'<svg height="64" viewBox="0 0 320 64" width="320" xmlns="http://www.w3.org/2000/svg">'
        f'<rect fill="#0B0E14" height="64" rx="8" ry="8" width="320" x="0" y="0"/>'
        f'<circle cx="32" cy="32" fill="{fill}" r="18"/>'
        f'<text fill="#E6EDF3" font-family="monospace" font-size="12" x="80" y="26">{badge_id}</text>'
        f'<text fill="#E6EDF3" font-family="monospace" font-size="12" x="80" y="44">{short_hash}</text>'
        f"</svg>"
    )


def main() -> None:
    if len(sys.argv) < 2 or len(sys.argv) > 3:
        print("usage: scxq7-badge.py <compliance.badge.xjson> [output.svg]", file=sys.stderr)
        sys.exit(2)

    badge_path = Path(sys.argv[1])
    data = read_badge(badge_path)
    badge_id = require_string(data, ["badge", "id"])
    require_string(data, ["badge", "level"])
    manifest_hash = require_string(data, ["derivation", "manifest_hash"])

    short_hash = short_manifest_hash(manifest_hash)
    svg = render_svg(badge_id, short_hash)

    if len(sys.argv) == 3:
        output_path = Path(sys.argv[2])
        try:
            output_path.write_text(svg, encoding="utf-8")
        except OSError as exc:
            fatal(f"cannot write {output_path}: {exc}")
    else:
        sys.stdout.write(svg)


if __name__ == "__main__":
    main()

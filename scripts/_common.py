from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
COMMONLIB_INCLUDE_RE = REPO_ROOT / "CommonLibVR" / "include" / "RE"
COMMONLIB_SRC_RE = REPO_ROOT / "CommonLibVR" / "src" / "RE"
LIBSKYRIM_RE = REPO_ROOT / "libskyrim" / "src" / "re"
OFFSETS_HEADERS = REPO_ROOT / "scripts" / "offsets"
OFFSETS_RS = REPO_ROOT / "libskyrim" / "src" / "offsets"


@dataclass(frozen=True)
class TypePaths:
    type_name: str
    rust: Path | None
    header: Path | None
    cpp: Path | None


def read_text(path: Path | None) -> str:
    if path is None or not path.exists():
        return ""
    return path.read_text(encoding="utf-8")


def camel_to_snake(name: str) -> str:
    name = name.strip()
    name = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    return name.replace("__", "_").lower()


def type_name_from_target(target: str) -> str:
    value = target.replace("\\", "/").strip()
    if value.endswith(".rs"):
        return snake_to_pascal(Path(value).stem)
    return Path(value).stem


def snake_to_pascal(name: str) -> str:
    return "".join(part.capitalize() for part in name.split("_") if part)


def find_rust_file(type_name: str) -> Path | None:
    snake = camel_to_snake(type_name)
    exact = LIBSKYRIM_RE / f"{snake}.rs"
    if exact.exists():
        return exact
    for path in LIBSKYRIM_RE.glob("*.rs"):
        if path.stem.lower() == snake.lower():
            return path
    return None


def find_header(type_name: str) -> Path | None:
    matches = sorted(COMMONLIB_INCLUDE_RE.rglob(f"{type_name}.h"))
    return matches[0] if matches else None


def find_cpp(type_name: str) -> Path | None:
    matches = sorted(COMMONLIB_SRC_RE.rglob(f"{type_name}.cpp"))
    return matches[0] if matches else None


def resolve_type_paths(target: str) -> TypePaths:
    type_name = type_name_from_target(target)
    return TypePaths(
        type_name=type_name,
        rust=find_rust_file(type_name),
        header=find_header(type_name),
        cpp=find_cpp(type_name),
    )


def extract_direct_bases(header_text: str, type_name: str) -> list[str]:
    if not header_text:
        return []
    pattern = re.compile(
        rf"(?:class|struct)\s+{re.escape(type_name)}\s*(?::\s*([^\{{]+))?\{{",
        re.MULTILINE,
    )
    match = pattern.search(header_text)
    if not match:
        return []
    clause = match.group(1)
    if not clause:
        return []
    clause = re.sub(r"//.*", "", clause)
    clause = "\n".join(
        line for line in clause.splitlines() if not line.strip().startswith("#")
    )
    bases = []
    for raw_base in clause.split(","):
        raw_base = raw_base.strip()
        raw_base = re.sub(r"\b(public|protected|private|virtual)\b", "", raw_base)
        raw_base = re.sub(r"\s+", " ", raw_base).strip()
        if raw_base:
            bases.append(raw_base)
    return bases


def extract_nested_types(header_text: str, type_name: str) -> list[str]:
    if not header_text:
        return []
    body = extract_type_body(header_text, type_name)
    if not body:
        return []
    pattern = re.compile(r"\b(?:struct|class)\s+([A-Za-z_]\w*)\b")
    return sorted(set(pattern.findall(body)))


def extract_type_body(header_text: str, type_name: str) -> str:
    start_pattern = re.compile(
        rf"(?:class|struct)\s+{re.escape(type_name)}\b[^\{{]*\{{", re.MULTILINE
    )
    match = start_pattern.search(header_text)
    if not match:
        return ""
    start = match.end()
    depth = 1
    index = start
    while index < len(header_text):
        char = header_text[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return header_text[start:index]
        index += 1
    return ""


def type_has_runtime_layout_signals(header_text: str, cpp_text: str) -> bool:
    haystack = "\n".join(part for part in (header_text, cpp_text) if part)
    if not haystack:
        return False
    markers = (
        "RuntimeDataAccessors.h",
        "RelocateMember(",
        "RelocateMemberIfNewer(",
        "RUNTIME_DATA",
        "VR_RUNTIME_DATA",
        "EXCLUSIVE_SKYRIM_",
        "ENABLE_SKYRIM_AE",
    )
    if any(marker in haystack for marker in markers):
        return True
    sizes = set(re.findall(r"static_assert\(sizeof\([^)]+\)\s*==\s*([^)]+)\)", haystack))
    return len(sizes) > 1


def has_generated_rtti(type_name: str) -> bool:
    offsets = read_text(OFFSETS_RS / "offsets_rtti.rs")
    return f"RTTI_{type_name}" in offsets


def has_generated_vtable(type_name: str) -> bool:
    offsets = read_text(OFFSETS_RS / "offsets_vtable.rs")
    return f"VTABLE_{type_name}" in offsets


def find_mixin_owners(type_name: str) -> list[Path]:
    pattern = re.compile(rf"inherit!\(\s*\w+\s*=>\s*{re.escape(type_name)}\s*,")
    owners: list[Path] = []
    for path in LIBSKYRIM_RE.glob("*.rs"):
        if path.name == "mod.rs":
            continue
        if pattern.search(read_text(path)):
            owners.append(path)
    return sorted(owners)


def repo_relative(path: Path | None) -> str:
    if path is None:
        return "<missing>"
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return str(path)

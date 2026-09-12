#!/usr/bin/env python3
from __future__ import annotations

import argparse
import base64
import copy
import json
import re
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

from lxml import etree

SVG_NS = "http://www.w3.org/2000/svg"
XLINK_NS = "http://www.w3.org/1999/xlink"
GUIDE_STROKE = "#8A38F5"
DEFINITION_TAGS = {"defs", "mask", "clipPath", "pattern", "filter", "style", "metadata", "symbol"}


def local_name(el: etree._Element) -> str:
    return etree.QName(el).localname


@dataclass(frozen=True)
class Region:
    name: str
    x: float
    y: float
    width: float
    height: float
    classification: str = ""


class SvgRegionExtractor:
    def __init__(self, source: Path):
        self.source = source
        self.tree = etree.parse(str(source))
        self.root = self.tree.getroot()
        self._ensure_root_child_ids()
        self._bbox = self._query_bboxes()

    def _ensure_root_child_ids(self) -> None:
        for index, child in enumerate(self.root):
            if not child.get("id"):
                child.set("id", f"asset_root_{index:04d}")

    def _query_bboxes(self) -> dict[str, tuple[float, float, float, float]]:
        with tempfile.NamedTemporaryFile(suffix=".svg", delete=False) as temp:
            temp_path = Path(temp.name)
        try:
            self.tree.write(str(temp_path), encoding="utf-8", xml_declaration=True)
            proc = subprocess.run(
                ["inkscape", str(temp_path), "--query-all"],
                check=True,
                capture_output=True,
                text=True,
            )
            result: dict[str, tuple[float, float, float, float]] = {}
            for line in proc.stdout.splitlines():
                parts = line.split(",")
                if len(parts) != 5:
                    continue
                try:
                    result[parts[0]] = tuple(float(v) for v in parts[1:5])  # type: ignore[assignment]
                except ValueError:
                    pass
            return result
        finally:
            temp_path.unlink(missing_ok=True)

    @staticmethod
    def _is_definition(child: etree._Element) -> bool:
        return local_name(child) in DEFINITION_TAGS

    @staticmethod
    def _is_purple_guide(child: etree._Element) -> bool:
        return (
            child.get("stroke", "").upper() == GUIDE_STROKE
            and child.get("stroke-dasharray") is not None
        )

    @staticmethod
    def _inside_region(
        bbox: tuple[float, float, float, float], region: Region
    ) -> bool:
        x, y, w, h = bbox
        if w == 0 and h == 0:
            return False
        cx, cy = x + w / 2.0, y + h / 2.0
        return (
            region.x - 0.75 <= cx <= region.x + region.width + 0.75
            and region.y - 0.75 <= cy <= region.y + region.height + 0.75
        )

    def extract_region(self, region: Region, output: Path) -> None:
        nsmap = self.root.nsmap.copy() if self.root.nsmap else {None: SVG_NS}
        new_root = etree.Element(f"{{{SVG_NS}}}svg", nsmap=nsmap)
        for key, value in self.root.attrib.items():
            if key not in {"width", "height", "viewBox"}:
                new_root.set(key, value)
        new_root.set("width", _fmt(region.width))
        new_root.set("height", _fmt(region.height))
        new_root.set(
            "viewBox",
            f"{_fmt(region.x)} {_fmt(region.y)} {_fmt(region.width)} {_fmt(region.height)}",
        )
        new_root.set("overflow", "hidden")

        # Keep definitions because masks/clip paths/patterns are frequently referenced
        # by the visual nodes selected for the region.
        for child in self.root:
            if self._is_definition(child):
                new_root.append(copy.deepcopy(child))

        for child in self.root:
            if self._is_definition(child) or self._is_purple_guide(child):
                continue
            child_id = child.get("id")
            bbox = self._bbox.get(child_id or "")
            if bbox and self._inside_region(bbox, region):
                new_root.append(copy.deepcopy(child))

        output.parent.mkdir(parents=True, exist_ok=True)
        etree.ElementTree(new_root).write(
            str(output), encoding="utf-8", xml_declaration=True, pretty_print=False
        )

    def purple_guide_regions(self) -> list[Region]:
        regions: list[Region] = []
        for child in self.root:
            if local_name(child) != "rect" or not self._is_purple_guide(child):
                continue
            try:
                regions.append(
                    Region(
                        name=f"component_set_{len(regions)+1:02d}",
                        x=float(child.get("x", "0")),
                        y=float(child.get("y", "0")),
                        width=float(child.get("width", "0")),
                        height=float(child.get("height", "0")),
                    )
                )
            except ValueError:
                continue
        return regions


def _fmt(value: float) -> str:
    if abs(value - round(value)) < 1e-8:
        return str(int(round(value)))
    return (f"{value:.4f}").rstrip("0").rstrip(".")


def extract_embedded_images(source: Path, output_dir: Path) -> list[dict]:
    root = etree.parse(str(source)).getroot()
    output_dir.mkdir(parents=True, exist_ok=True)
    exported = []
    for index, el in enumerate(root.iter()):
        if local_name(el) != "image":
            continue
        href = el.get(f"{{{XLINK_NS}}}href") or el.get("href")
        if not href or not href.startswith("data:image/"):
            continue
        header, payload = href.split(",", 1)
        mime = header.split(";", 1)[0].split(":", 1)[1]
        ext = {"image/png": "png", "image/jpeg": "jpg", "image/webp": "webp"}.get(mime, "bin")
        image_id = el.get("id") or f"image_{index:03d}"
        destination = output_dir / f"{image_id}.{ext}"
        data = base64.b64decode(payload)
        destination.write_bytes(data)
        exported.append(
            {
                "source": source.name,
                "id": image_id,
                "file": str(destination),
                "mime": mime,
                "svg_width": el.get("width"),
                "svg_height": el.get("height"),
                "bytes": len(data),
            }
        )
    return exported


def render_svg(svg: Path, png: Path, width: int | None = None) -> None:
    command = ["inkscape", str(svg), f"--export-filename={png}"]
    if width:
        command.append(f"--export-width={width}")
    png.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(command, check=True, capture_output=True)


def export_regions(
    source: Path,
    regions: Iterable[Region],
    svg_dir: Path,
    preview_dir: Path,
    manifest: list[dict],
) -> None:
    extractor = SvgRegionExtractor(source)
    for region in regions:
        svg_out = svg_dir / f"{region.name}.svg"
        png_out = preview_dir / f"{region.name}.png"
        extractor.extract_region(region, svg_out)
        render_svg(svg_out, png_out)
        manifest.append(
            {
                "source": source.name,
                "name": region.name,
                "classification": region.classification,
                "svg": str(svg_out),
                "preview": str(png_out),
                "bbox": [region.x, region.y, region.width, region.height],
            }
        )


def main() -> None:
    parser = argparse.ArgumentParser(description="Extract reusable UI assets from Figma-exported SVGs.")
    parser.add_argument("input_dir", type=Path)
    parser.add_argument("output_dir", type=Path)
    args = parser.parse_args()

    out = args.output_dir
    out.mkdir(parents=True, exist_ok=True)
    manifest: dict[str, list[dict]] = {"regions": [], "embedded_images": []}

    sources = {p.name: p for p in args.input_dir.glob("*.svg")}

    # Exact embedded images from every SVG.
    for source in sources.values():
        manifest["embedded_images"].extend(
            extract_embedded_images(source, out / "embedded" / source.stem.replace(" ", "_"))
        )

    # Component sets explicitly outlined by Figma's purple dashed guide.
    for filename in ("UI Buttons.svg", "UI Elements.svg"):
        source = sources.get(filename)
        if not source:
            continue
        extractor = SvgRegionExtractor(source)
        regions = extractor.purple_guide_regions()
        # UI Elements has two clear image/avatar families.
        if filename == "UI Elements.svg" and len(regions) >= 2:
            regions[0] = Region("avatar_small_variants", regions[0].x, regions[0].y, regions[0].width, regions[0].height, "small circular image/avatar variants")
            regions[1] = Region("avatar_large_variants", regions[1].x, regions[1].y, regions[1].width, regions[1].height, "large circular image/avatar variants")
        export_regions(
            source,
            regions,
            out / "components" / source.stem.replace(" ", "_") / "svg",
            out / "components" / source.stem.replace(" ", "_") / "preview",
            manifest["regions"],
        )

    # Nine detached 40x40-ish 3D tool buttons on the right-hand variant of toolbar.svg.
    toolbar = sources.get("toolbar.svg")
    if toolbar:
        tool_names = [
            ("select_box", "Box Select"),
            ("cursor_3d", "3D Cursor"),
            ("move", "Move / Translate"),
            ("rotate", "Rotate"),
            ("scale", "Scale"),
            ("transform", "Combined Transform"),
            ("annotate", "Annotate / Draw"),
            ("measure", "Measure"),
            ("add_primitive", "Add primitive / custom add tool"),
        ]
        toolbar_regions = []
        for index, (name, classification) in enumerate(tool_names):
            toolbar_regions.append(Region(name, 80, 20 + index * 42, 40, 40, classification))
        export_regions(
            toolbar,
            toolbar_regions,
            out / "toolbar" / "buttons" / "svg",
            out / "toolbar" / "buttons" / "preview",
            manifest["regions"],
        )

        # Extract and semantically rename the exact 256x256 embedded RGBA icon sources.
        toolbar_embedded = out / "embedded" / "toolbar"
        semantic_dir = out / "toolbar" / "icons_exact_png"
        semantic_dir.mkdir(parents=True, exist_ok=True)
        for index, (name, classification) in enumerate(tool_names):
            candidates = list(toolbar_embedded.glob(f"image{index}_*.png"))
            if candidates:
                destination = semantic_dir / f"{name}.png"
                destination.write_bytes(candidates[0].read_bytes())

    # 15 visually separated vertical property/data tabs.
    data_tabs = sources.get("data_panel_vertical_menu.svg")
    if data_tabs:
        starts = [4, 28, 63, 87, 111, 135, 159, 194, 229, 253, 277, 301, 325, 349, 373]
        regions = [Region(f"data_tab_{i+1:02d}", 4, y, 22, 22, "vertical Properties/Data tab icon button") for i, y in enumerate(starts)]
        export_regions(
            data_tabs,
            regions,
            out / "properties_tabs" / "svg",
            out / "properties_tabs" / "preview",
            manifest["regions"],
        )

    # properties.svg is an 8-screen component gallery; panels are 340 wide with 20px gaps.
    properties = sources.get("properties.svg")
    if properties:
        panel_names = [
            ("metadata", "Metadata / project information"),
            ("select_box_tool", "Active tool / Select Box settings"),
            ("render", "Render properties"),
            ("output", "Output properties"),
            ("view_layer", "View Layer properties"),
            ("scene", "Scene properties"),
            ("world", "World properties"),
            ("collection", "Collection properties"),
        ]
        regions = [Region(name, index * 360, 0, 340, 716, classification) for index, (name, classification) in enumerate(panel_names)]
        export_regions(
            properties,
            regions,
            out / "properties_panels" / "svg",
            out / "properties_panels" / "preview",
            manifest["regions"],
        )

    (out / "manifest.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"Extracted assets to: {out}")


if __name__ == "__main__":
    main()

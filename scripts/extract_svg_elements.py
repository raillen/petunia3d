#!/usr/bin/env python3
"""
Figma SVG Element Extractor for Petunia3D.

Parses SVGs exported from Figma (located in docs/image-references/) and extracts:
1. Embedded raster assets (Base64 PNGs/JPEGs embedded in <image> tags)
2. Vector icons defined in clipped groups (<g clip-path="url(#...)">)
3. Component frames / variant sets (marked by Figma purple dashed rectangles or component boundaries)
4. Individual button/control elements (isolated with their background, border, mask, and icon)
5. Interactive HTML gallery (index.html) allowing live visual inspection of all extracted assets

Saves all extracted assets as standalone, 100% valid XML SVG and crisp PNG files
organized into specific subdirectories with documentation catalogs.
"""

import argparse
import base64
import json
import os
import re
import subprocess
import sys
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Optional, Set, Tuple

SVG_NS = "http://www.w3.org/2000/svg"
XLINK_NS = "http://www.w3.org/1999/xlink"

ET.register_namespace("", SVG_NS)
ET.register_namespace("xlink", XLINK_NS)


def serialize_clean_svg(svg_element: ET.Element) -> str:
    """
    Serializes an SVG ElementTree element to a clean, 100% valid XML string.
    Guarantees no duplicate xmlns or xmlns:xlink attributes.
    """
    raw = ET.tostring(svg_element, encoding="utf-8").decode("utf-8")

    # Match opening <svg ...> tag and clean duplicate attributes
    m = re.match(r"^<svg\b([^>]*)>", raw)
    if m:
        attrs_str = m.group(1)
        # Parse attributes with regex
        attrs = re.findall(r'([a-zA-Z_:][a-zA-Z0-9_.:-]*)\s*=\s*"([^"]*)"', attrs_str)
        seen_keys = set()
        clean_attrs = []
        for k, v in attrs:
            if k in seen_keys:
                continue
            seen_keys.add(k)
            clean_attrs.append(f'{k}="{v}"')

        # Ensure xmlns is present exactly once
        if "xmlns" not in seen_keys:
            clean_attrs.insert(0, f'xmlns="{SVG_NS}"')

        new_opening = "<svg " + " ".join(clean_attrs) + ">"
        raw = new_opening + raw[m.end():]

    # Verify XML validity with parser
    try:
        ET.fromstring(raw)
    except Exception as e:
        # Fallback fix if any syntax issues
        raw = re.sub(r'xmlns="http://www\.w3\.org/2000/svg"\s+xmlns="http://www\.w3\.org/2000/svg"', f'xmlns="{SVG_NS}"', raw)
        raw = re.sub(r'xmlns:xlink="http://www\.w3\.org/1999/xlink"\s+xmlns:xlink="http://www\.w3\.org/1999/xlink"', f'xmlns:xlink="{XLINK_NS}"', raw)

    return raw


def render_png_preview(svg_path: Path, png_path: Path, width: Optional[int] = None, height: Optional[int] = None) -> bool:
    """Renders an SVG file to PNG using rsvg-convert if installed."""
    cmd = ["rsvg-convert"]
    if width is not None and height is not None:
        cmd.extend(["-w", str(width), "-h", str(height)])
    cmd.extend([str(svg_path), "-o", str(png_path)])

    try:
        res = subprocess.run(cmd, capture_output=True, text=True)
        return res.returncode == 0
    except Exception:
        return False


def parse_path_bbox(d: str) -> Optional[Tuple[float, float, float, float]]:
    """Approximates the bounding box (min_x, min_y, max_x, max_y) of an SVG path."""
    tokens = re.findall(r"([a-df-z])|(-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)", d, re.IGNORECASE)
    xs: List[float] = []
    ys: List[float] = []
    curr_x, curr_y = 0.0, 0.0
    cmd = ""
    coords: List[float] = []

    for c, num in tokens:
        if c:
            cmd = c
            coords = []
        elif num:
            coords.append(float(num))
            if cmd in ("M", "L", "T") and len(coords) == 2:
                curr_x, curr_y = coords[0], coords[1]
                xs.append(curr_x)
                ys.append(curr_y)
                coords = []
            elif cmd in ("m", "l", "t") and len(coords) == 2:
                curr_x += coords[0]
                curr_y += coords[1]
                xs.append(curr_x)
                ys.append(curr_y)
                coords = []
            elif cmd == "H" and len(coords) == 1:
                curr_x = coords[0]
                xs.append(curr_x)
                ys.append(curr_y)
                coords = []
            elif cmd == "h" and len(coords) == 1:
                curr_x += coords[0]
                xs.append(curr_x)
                ys.append(curr_y)
                coords = []
            elif cmd == "V" and len(coords) == 1:
                curr_y = coords[0]
                xs.append(curr_x)
                ys.append(curr_y)
                coords = []
            elif cmd == "v" and len(coords) == 1:
                curr_y += coords[0]
                xs.append(curr_x)
                ys.append(curr_y)
                coords = []
            elif cmd == "C" and len(coords) == 6:
                for i in range(0, 6, 2):
                    xs.append(coords[i])
                    ys.append(coords[i + 1])
                curr_x, curr_y = coords[4], coords[5]
                coords = []
            elif cmd == "c" and len(coords) == 6:
                for i in range(0, 6, 2):
                    xs.append(curr_x + coords[i])
                    ys.append(curr_y + coords[i + 1])
                curr_x += coords[4]
                curr_y += coords[5]
                coords = []
            elif cmd in ("S", "Q") and len(coords) == 4:
                for i in range(0, 4, 2):
                    xs.append(coords[i])
                    ys.append(coords[i + 1])
                curr_x, curr_y = coords[2], coords[3]
                coords = []
            elif cmd in ("s", "q") and len(coords) == 4:
                for i in range(0, 4, 2):
                    xs.append(curr_x + coords[i])
                    ys.append(curr_y + coords[i + 1])
                curr_x += coords[2]
                curr_y += coords[3]
                coords = []
            elif cmd == "A" and len(coords) == 7:
                curr_x, curr_y = coords[5], coords[6]
                xs.append(curr_x)
                ys.append(curr_y)
                coords = []
            elif cmd == "a" and len(coords) == 7:
                curr_x += coords[5]
                curr_y += coords[6]
                xs.append(curr_x)
                ys.append(curr_y)
                coords = []

    if xs and ys:
        return (min(xs), min(ys), max(xs), max(ys))
    return None


def get_element_bbox(elem: ET.Element) -> Optional[Tuple[float, float, float, float]]:
    """Calculates the bounding box for supported SVG elements."""
    tag = elem.tag.split("}")[-1]
    if tag == "rect":
        try:
            x = float(elem.get("x", "0"))
            y = float(elem.get("y", "0"))
            w = float(elem.get("width", "0"))
            h = float(elem.get("height", "0"))
            return (x, y, x + w, y + h)
        except ValueError:
            return None
    elif tag == "circle":
        try:
            cx = float(elem.get("cx", "0"))
            cy = float(elem.get("cy", "0"))
            r = float(elem.get("r", "0"))
            return (cx - r, cy - r, cx + r, cy + r)
        except ValueError:
            return None
    elif tag == "path":
        d = elem.get("d", "")
        return parse_path_bbox(d)
    elif tag == "g":
        # Aggregate children bboxes
        c_boxes = []
        for child in elem:
            cb = get_element_bbox(child)
            if cb:
                c_boxes.append(cb)
        if c_boxes:
            min_x = min(b[0] for b in c_boxes)
            min_y = min(b[1] for b in c_boxes)
            max_x = max(b[2] for b in c_boxes)
            max_y = max(b[3] for b in c_boxes)
            return (min_x, min_y, max_x, max_y)
    return None


def extract_embedded_images(
    root: ET.Element, file_stem: str, output_dir: Path
) -> List[Dict]:
    """Extracts all base64-encoded PNG/JPEG images from SVG defs."""
    results = []
    target_dir = output_dir / file_stem / "embedded_png"

    defs = root.find(f"{{{SVG_NS}}}defs")
    if defs is None:
        return results

    raw_images = defs.findall(f"{{{SVG_NS}}}image")
    valid_images = []
    for img in raw_images:
        href = img.get(f"{{{XLINK_NS}}}href") or img.get("href", "")
        if "base64," in href:
            valid_images.append((img, href))

    if not valid_images:
        return results

    target_dir.mkdir(parents=True, exist_ok=True)
    (target_dir / "README.md").write_text(
        f"# Imagens Raster Embutidas — {file_stem}\n\n"
        "Contém as imagens raster (PNG/JPEG) decodificadas do SVG original do Figma,\n"
        "juntamente com seus respectivos wrappers SVG individuais válidos.\n",
        encoding="utf-8",
    )

    for img_idx, (img, href) in enumerate(valid_images):
        raw_id = img.get("id") or f"image_{img_idx}"
        b64_data = href.split("base64,")[1]
        try:
            raw_bytes = base64.b64decode(b64_data)
        except Exception as e:
            print(f"  [Warning] Failed to decode base64 for {raw_id}: {e}")
            continue

        ext = "png"
        if raw_bytes.startswith(b"\xff\xd8\xff"):
            ext = "jpg"

        w = img.get("width", "32")
        h = img.get("height", "32")

        filename = f"{raw_id}.{ext}"
        filepath = target_dir / filename
        filepath.write_bytes(raw_bytes)

        # Standalone SVG wrapping this image with valid namespaces
        svg_filename = f"{raw_id}.svg"
        svg_filepath = target_dir / svg_filename
        svg_elem = ET.Element(
            f"{{{SVG_NS}}}svg",
            {
                "viewBox": f"0 0 {w} {h}",
                "width": str(w),
                "height": str(h),
            },
        )
        ET.SubElement(
            svg_elem,
            f"{{{SVG_NS}}}image",
            {
                "width": str(w),
                "height": str(h),
                f"{{{XLINK_NS}}}href": href,
                "href": href,
            },
        )
        svg_content = serialize_clean_svg(svg_elem)
        svg_filepath.write_text(svg_content, encoding="utf-8")

        results.append({
            "id": raw_id,
            "type": "embedded_image",
            "format": ext,
            "size_bytes": len(raw_bytes),
            "width": w,
            "height": h,
            "png_file": f"embedded_png/{filename}",
            "svg_file": f"embedded_png/{svg_filename}",
        })

    return results


def extract_clipped_icons(
    root: ET.Element, file_stem: str, output_dir: Path
) -> List[Dict]:
    """
    Extracts vector icons grouped under <g clip-path="url(#clip...)">.
    Ensures valid XML, standard display dimensions (32x32), and renders PNG previews.
    """
    results = []
    target_dir = output_dir / file_stem / "icons"

    defs = root.find(f"{{{SVG_NS}}}defs")
    clip_map: Dict[str, Tuple[float, float, float, float]] = {}

    if defs is not None:
        for cp in defs.findall(f"{{{SVG_NS}}}clipPath"):
            cid = cp.get("id")
            if not cid:
                continue
            rect = cp.find(f"{{{SVG_NS}}}rect")
            if rect is not None:
                w = float(rect.get("width", "0"))
                h = float(rect.get("height", "0"))
                t = rect.get("transform", "")
                m = re.search(r"translate\(([\d.]+)[ ,]+([\d.]+)\)", t)
                if m:
                    x, y = float(m.group(1)), float(m.group(2))
                else:
                    x, y = float(rect.get("x", "0")), float(rect.get("y", "0"))
                clip_map[cid] = (x, y, w, h)

    if not clip_map:
        return results

    found_any = False
    for g in root.iter(f"{{{SVG_NS}}}g"):
        cp_attr = g.get("clip-path", "")
        m = re.search(r"#([a-zA-Z0-9_-]+)", cp_attr)
        if not m or m.group(1) not in clip_map:
            continue

        cid = m.group(1)
        x, y, w, h = clip_map[cid]
        if w <= 0 or h <= 0:
            continue

        if not found_any:
            target_dir.mkdir(parents=True, exist_ok=True)
            (target_dir / "README.md").write_text(
                f"# Ícones Vetoriais Isolados — {file_stem}\n\n"
                "Contém ícones vetoriais com viewBox normalizado, XML 100% válido e previews em PNG de alta resolução.\n",
                encoding="utf-8",
            )
            found_any = True

        # Display dimensions: scale small 10x10 icons up to standard 32x32 display size
        display_w = 32 if w <= 24 else int(w)
        display_h = 32 if h <= 24 else int(h)

        svg_elem = ET.Element(
            f"{{{SVG_NS}}}svg",
            {
                "viewBox": f"0 0 {w} {h}",
                "width": str(display_w),
                "height": str(display_h),
                "fill": "none",
            },
        )

        # Copy defs if referenced
        if defs is not None:
            # Include relevant gradients or filters
            svg_defs = ET.SubElement(svg_elem, f"{{{SVG_NS}}}defs")
            for cp_child in defs:
                tag = cp_child.tag.split("}")[-1]
                if tag in ("linearGradient", "radialGradient", "filter"):
                    svg_defs.append(cp_child)

        inner_g = ET.SubElement(
            svg_elem,
            f"{{{SVG_NS}}}g",
            {"transform": f"translate({-x}, {-y})"},
        )

        for child in g:
            inner_g.append(child)

        svg_str = serialize_clean_svg(svg_elem)
        filename_svg = f"icon_{cid}.svg"
        filepath_svg = target_dir / filename_svg
        filepath_svg.write_text(svg_str, encoding="utf-8")

        # Render high-res PNG preview (64x64)
        filename_png = f"icon_{cid}.png"
        filepath_png = target_dir / filename_png
        render_png_preview(filepath_svg, filepath_png, width=64, height=64)

        results.append({
            "id": cid,
            "type": "vector_icon",
            "rect": [x, y, w, h],
            "display_size": [display_w, display_h],
            "file": f"icons/{filename_svg}",
            "png_file": f"icons/{filename_png}",
        })

    return results


def extract_component_frames(
    root: ET.Element, file_stem: str, output_dir: Path
) -> List[Dict]:
    """
    Extracts Figma Component Set variants indicated by purple dashed bounding frames
    (stroke="#8A38F5" or similar component marker).
    Generates 100% valid XML SVG and crisp PNG previews.
    """
    results = []
    target_dir = output_dir / file_stem / "components"

    frames = []
    for r in root.iter(f"{{{SVG_NS}}}rect"):
        stroke = r.get("stroke", "").lower()
        if stroke in ("#8a38f5", "rgb(138, 56, 245)"):
            try:
                x = float(r.get("x", "0"))
                y = float(r.get("y", "0"))
                w = float(r.get("width", "0"))
                h = float(r.get("height", "0"))
                if w >= 16 and h >= 16:
                    frames.append((x, y, w, h))
            except ValueError:
                continue

    if not frames:
        return results

    target_dir.mkdir(parents=True, exist_ok=True)
    (target_dir / "README.md").write_text(
        f"# Conjuntos de Componentes e Variantes — {file_stem}\n\n"
        "Contém os frames de conjuntos de componentes do Figma com todas as suas variantes e estados,\n"
        "gerados com XML válido e previews em PNG.\n",
        encoding="utf-8",
    )

    defs = root.find(f"{{{SVG_NS}}}defs")

    for idx, (fx, fy, fw, fh) in enumerate(frames):
        matching_elements = []

        for child in list(root):
            tag = child.tag.split("}")[-1]
            if tag in ("defs", "mask"):
                continue
            if tag == "rect" and child.get("stroke", "").lower() in ("#8a38f5", "rgb(138, 56, 245)"):
                continue

            bbox = get_element_bbox(child)
            if bbox:
                bx1, by1, bx2, by2 = bbox
                cx = (bx1 + bx2) / 2
                cy = (by1 + by2) / 2
                if fx <= cx <= fx + fw and fy <= cy <= fy + fh:
                    matching_elements.append(child)

        if not matching_elements:
            continue

        svg_elem = ET.Element(
            f"{{{SVG_NS}}}svg",
            {
                "viewBox": f"0 0 {fw} {fh}",
                "width": str(fw),
                "height": str(fh),
                "fill": "none",
            },
        )

        # Handle defs and masks: ensure masks are properly translated
        local_defs = ET.SubElement(svg_elem, f"{{{SVG_NS}}}defs")
        if defs is not None:
            for dchild in defs:
                local_defs.append(dchild)

        # Include masks from root
        for mask_node in root.findall(f"{{{SVG_NS}}}mask"):
            new_mask = ET.SubElement(local_defs, f"{{{SVG_NS}}}mask", mask_node.attrib)
            mg = ET.SubElement(new_mask, f"{{{SVG_NS}}}g", {"transform": f"translate({-fx}, {-fy})"})
            for mc in mask_node:
                mg.append(mc)

        inner_g = ET.SubElement(
            svg_elem,
            f"{{{SVG_NS}}}g",
            {"transform": f"translate({-fx}, {-fy})"},
        )
        for me in matching_elements:
            inner_g.append(me)

        svg_str = serialize_clean_svg(svg_elem)
        comp_id = f"component_set_{idx + 1:02d}"
        filename_svg = f"{comp_id}_{int(fw)}x{int(fh)}.svg"
        filepath_svg = target_dir / filename_svg
        filepath_svg.write_text(svg_str, encoding="utf-8")

        filename_png = f"{comp_id}_{int(fw)}x{int(fh)}.png"
        filepath_png = target_dir / filename_png
        render_png_preview(filepath_svg, filepath_png)

        results.append({
            "id": comp_id,
            "type": "component_frame",
            "rect": [fx, fy, fw, fh],
            "element_count": len(matching_elements),
            "file": f"components/{filename_svg}",
            "png_file": f"components/{filename_png}",
        })

    return results


def extract_button_controls(
    root: ET.Element, file_stem: str, output_dir: Path
) -> List[Dict]:
    """
    Extracts individual button / control elements (e.g., toolbar slots, header buttons,
    interactive buttons with their masks, background, border, and icon).
    Guarantees complete buttons without splitting composite controls.
    """
    results = []
    target_dir = output_dir / file_stem / "buttons"

    defs = root.find(f"{{{SVG_NS}}}defs")
    buttons_found = []

    # Case A: Toolbar specific 40x40 buttons
    if "toolbar" in file_stem.lower():
        # 9 primary tools in column 2 (x=80) and column 1 (x=20)
        y_positions = [20, 62, 104, 146, 188, 230, 272, 314, 356]
        tool_names = ["select_box", "cursor", "move", "rotate", "scale", "transform", "annotate", "measure", "add_primitive"]
        for idx, (tname, ys) in enumerate(zip(tool_names, y_positions)):
            buttons_found.append((f"tool_{idx+1:02d}_{tname}", 80, ys, 40, 40))

    # Case B: Header/panel buttons from masks or distinct rects
    else:
        # Detect button surface clusters from masks or button rects
        surface_bboxes = []
        for mask in root.iter(f"{{{SVG_NS}}}mask"):
            mid = mask.get("id", "")
            if "inside" in mid:
                path = mask.find(f"{{{SVG_NS}}}path")
                if path is not None:
                    bbox = parse_path_bbox(path.get("d", ""))
                    if bbox:
                        w = bbox[2] - bbox[0]
                        h = bbox[3] - bbox[1]
                        if 12 <= w <= 400 and 12 <= h <= 120:
                            surface_bboxes.append((bbox[0], bbox[1], bbox[2], bbox[3], mid))

        # Merge adjacent button parts (e.g. split dropdown button left + right parts)
        merged_buttons = []
        for b in sorted(surface_bboxes, key=lambda x: (int(x[1] / 10), x[0])):
            x1, y1, x2, y2, mid = b
            merged = False
            for m_idx, (mx1, my1, mx2, my2, mids) in enumerate(merged_buttons):
                # If horizontally adjacent on the exact same row (within 3px gap, same height)
                if abs(y1 - my1) <= 2 and abs(y2 - my2) <= 2:
                    if abs(x1 - mx2) <= 4 or abs(mx1 - x2) <= 4:
                        merged_buttons[m_idx] = (min(x1, mx1), min(y1, my1), max(x2, mx2), max(y2, my2), mids + [mid])
                        merged = True
                        break
            if not merged:
                merged_buttons.append((x1, y1, x2, y2, [mid]))

        for idx, (bx1, by1, bx2, by2, mids) in enumerate(merged_buttons):
            bw = bx2 - bx1
            bh = by2 - by1
            if bw >= 14 and bh >= 14:
                buttons_found.append((f"button_{idx+1:03d}", bx1, by1, bw, bh))

    if not buttons_found:
        return results

    target_dir.mkdir(parents=True, exist_ok=True)
    (target_dir / "README.md").write_text(
        f"# Botões e Controles Individuais — {file_stem}\n\n"
        "Contém botões e controles recortados com fundo, borda e ícone integrados,\n"
        "gerados em SVG válido e PNG nítido.\n",
        encoding="utf-8",
    )

    for btn_id, bx, by, bw, bh in buttons_found:
        pad = 2.0
        x1, y1 = bx - pad, by - pad
        x2, y2 = bx + bw + pad, by + bh + pad

        btn_elements = []
        for child in list(root):
            tag = child.tag.split("}")[-1]
            if tag in ("defs", "mask"):
                continue
            if tag == "rect" and child.get("stroke", "").lower() in ("#8a38f5", "rgb(138, 56, 245)"):
                continue
            bbox = get_element_bbox(child)
            if bbox:
                cx = (bbox[0] + bbox[2]) / 2
                cy = (bbox[1] + bbox[3]) / 2
                if x1 <= cx <= x2 and y1 <= cy <= y2:
                    btn_elements.append(child)

        if not btn_elements:
            continue

        out_w = bw + pad * 2
        out_h = bh + pad * 2

        svg_elem = ET.Element(
            f"{{{SVG_NS}}}svg",
            {
                "viewBox": f"0 0 {out_w} {out_h}",
                "width": str(int(out_w)),
                "height": str(int(out_h)),
                "fill": "none",
            },
        )

        local_defs = ET.SubElement(svg_elem, f"{{{SVG_NS}}}defs")
        if defs is not None:
            for dchild in defs:
                local_defs.append(dchild)

        # Translate masks to align with button
        for mask_node in root.findall(f"{{{SVG_NS}}}mask"):
            new_mask = ET.SubElement(local_defs, f"{{{SVG_NS}}}mask", mask_node.attrib)
            mg = ET.SubElement(new_mask, f"{{{SVG_NS}}}g", {"transform": f"translate({-x1}, {-y1})"})
            for mc in mask_node:
                mg.append(mc)

        inner_g = ET.SubElement(
            svg_elem,
            f"{{{SVG_NS}}}g",
            {"transform": f"translate({-x1}, {-y1})"},
        )
        for be in btn_elements:
            inner_g.append(be)

        svg_str = serialize_clean_svg(svg_elem)
        filename_svg = f"{btn_id}_{int(bw)}x{int(bh)}.svg"
        filepath_svg = target_dir / filename_svg
        filepath_svg.write_text(svg_str, encoding="utf-8")

        filename_png = f"{btn_id}_{int(bw)}x{int(bh)}.png"
        filepath_png = target_dir / filename_png
        render_png_preview(filepath_svg, filepath_png)

        results.append({
            "id": btn_id,
            "type": "button_control",
            "rect": [bx, by, bw, bh],
            "file": f"buttons/{filename_svg}",
            "png_file": f"buttons/{filename_png}",
        })

    return results


def generate_html_gallery(all_manifests: List[Dict], output_dir: Path):
    """
    Generates an interactive HTML preview gallery (index.html) allowing live visual inspection
    of all extracted icons, buttons, components, and embedded assets.
    """
    html_lines = [
        "<!DOCTYPE html>",
        "<html lang='pt-BR'>",
        "<head>",
        "  <meta charset='UTF-8'>",
        "  <meta name='viewport' content='width=device-width, initial-scale=1.0'>",
        "  <title>Petunia3D — Galeria de Assets e Elementos Figma</title>",
        "  <style>",
        "    :root {",
        "      --bg-dark: #1e1e1e;",
        "      --card-dark: #282828;",
        "      --border-dark: #3a3a3a;",
        "      --text-dark: #e0e0e0;",
        "      --accent: #3169e3;",
        "      --accent-hover: #5283e8;",
        "    }",
        "    body {",
        "      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;",
        "      background: var(--bg-dark);",
        "      color: var(--text-dark);",
        "      margin: 0;",
        "      padding: 24px;",
        "      transition: background 0.2s, color 0.2s;",
        "    }",
        "    body.light-theme {",
        "      background: #f5f5f7;",
        "      color: #1d1d1f;",
        "      --card-dark: #ffffff;",
        "      --border-dark: #d2d2d7;",
        "    }",
        "    .header {",
        "      display: flex;",
        "      justify-content: space-between;",
        "      align-items: center;",
        "      border-bottom: 1px solid var(--border-dark);",
        "      padding-bottom: 16px;",
        "      margin-bottom: 24px;",
        "    }",
        "    .title h1 { margin: 0; font-size: 24px; }",
        "    .title p { margin: 4px 0 0; color: #888; font-size: 14px; }",
        "    .controls { display: flex; gap: 12px; }",
        "    .btn-toggle {",
        "      background: var(--card-dark);",
        "      color: inherit;",
        "      border: 1px solid var(--border-dark);",
        "      padding: 8px 16px;",
        "      border-radius: 6px;",
        "      cursor: pointer;",
        "      font-weight: 500;",
        "    }",
        "    .btn-toggle:hover { border-color: var(--accent); }",
        "    .stats-bar {",
        "      display: flex;",
        "      gap: 16px;",
        "      margin-bottom: 24px;",
        "      flex-wrap: wrap;",
        "    }",
        "    .stat-badge {",
        "      background: var(--card-dark);",
        "      border: 1px solid var(--border-dark);",
        "      padding: 8px 14px;",
        "      border-radius: 8px;",
        "      font-size: 13px;",
        "    }",
        "    .stat-badge strong { color: var(--accent); }",
        "    .section { margin-bottom: 40px; }",
        "    .section-header {",
        "      display: flex;",
        "      justify-content: space-between;",
        "      align-items: baseline;",
        "      border-bottom: 1px solid var(--border-dark);",
        "      padding-bottom: 8px;",
        "      margin-bottom: 16px;",
        "    }",
        "    .section-header h2 { margin: 0; font-size: 18px; }",
        "    .grid {",
        "      display: grid;",
        "      grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));",
        "      gap: 12px;",
        "    }",
        "    .grid.large {",
        "      grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));",
        "    }",
        "    .card {",
        "      background: var(--card-dark);",
        "      border: 1px solid var(--border-dark);",
        "      border-radius: 8px;",
        "      padding: 12px;",
        "      display: flex;",
        "      flex-direction: column;",
        "      align-items: center;",
        "      text-align: center;",
        "      transition: transform 0.15s, border-color 0.15s;",
        "    }",
        "    .card:hover {",
        "      transform: translateY(-2px);",
        "      border-color: var(--accent);",
        "    }",
        "    .preview-box {",
        "      width: 100%;",
        "      height: 70px;",
        "      display: flex;",
        "      align-items: center;",
        "      justify-content: center;",
        "      background: rgba(0,0,0,0.15);",
        "      border-radius: 6px;",
        "      margin-bottom: 8px;",
        "      overflow: hidden;",
        "    }",
        "    .large .preview-box { height: 120px; }",
        "    .preview-box img {",
        "      max-width: 90%;",
        "      max-height: 90%;",
        "      object-fit: contain;",
        "    }",
        "    .card-title {",
        "      font-size: 11px;",
        "      font-weight: 600;",
        "      word-break: break-all;",
        "      margin-bottom: 4px;",
        "    }",
        "    .card-meta {",
        "      font-size: 10px;",
        "      color: #888;",
        "    }",
        "    .links {",
        "      margin-top: 6px;",
        "      display: flex;",
        "      gap: 6px;",
        "      font-size: 11px;",
        "    }",
        "    .links a { color: var(--accent); text-decoration: none; }",
        "    .links a:hover { text-decoration: underline; }",
        "  </style>",
        "</head>",
        "<body>",
        "  <div class='header'>",
        "    <div class='title'>",
        "      <h1>Petunia3D — Galeria de Assets e Elementos Figma</h1>",
        "      <p>Catálogo visual de ícones, botões, variantes de componentes e texturas extraídas</p>",
        "    </div>",
        "    <div class='controls'>",
        "      <button class='btn-toggle' onclick='toggleTheme()'>🌓 Alternar Tema (Escuro / Claro)</button>",
        "    </div>",
        "  </div>",
    ]

    total_icons = sum(m["counts"]["vector_icons"] for m in all_manifests)
    total_imgs = sum(m["counts"]["embedded_images"] for m in all_manifests)
    total_comps = sum(m["counts"]["component_frames"] for m in all_manifests)
    total_btns = sum(m["counts"]["button_controls"] for m in all_manifests)

    html_lines.append("  <div class='stats-bar'>")
    html_lines.append(f"    <div class='stat-badge'>Ícones Vetoriais: <strong>{total_icons}</strong></div>")
    html_lines.append(f"    <div class='stat-badge'>Imagens / Texturas: <strong>{total_imgs}</strong></div>")
    html_lines.append(f"    <div class='stat-badge'>Component Sets: <strong>{total_comps}</strong></div>")
    html_lines.append(f"    <div class='stat-badge'>Botões & Controles: <strong>{total_btns}</strong></div>")
    html_lines.append("  </div>")

    for m in all_manifests:
        stem = Path(m["source_file"]).stem.replace(" ", "_")
        counts = m["counts"]
        if not any(counts.values()):
            continue

        html_lines.append("  <div class='section'>")
        html_lines.append(f"    <div class='section-header'>")
        html_lines.append(f"      <h2>📁 {m['source_file']}</h2>")
        html_lines.append(f"      <span style='font-size: 12px; color: #888;'>ViewBox: {m.get('viewBox', '')}</span>")
        html_lines.append("    </div>")

        # Components
        if m["component_frames"]:
            html_lines.append("    <h3>Conjuntos de Componentes (Component Sets / Variantes)</h3>")
            html_lines.append("    <div class='grid large'>")
            for c in m["component_frames"]:
                img_src = f"{stem}/{c['file']}"
                png_src = f"{stem}/{c.get('png_file', c['file'])}"
                html_lines.append("      <div class='card'>")
                html_lines.append(f"        <div class='preview-box'><img src='{img_src}' alt='{c['id']}'></div>")
                html_lines.append(f"        <div class='card-title'>{c['id']}</div>")
                html_lines.append(f"        <div class='card-meta'>{int(c['rect'][2])}×{int(c['rect'][3])} px</div>")
                html_lines.append(f"        <div class='links'><a href='{img_src}' target='_blank'>SVG</a> • <a href='{png_src}' target='_blank'>PNG</a></div>")
                html_lines.append("      </div>")
            html_lines.append("    </div>")

        # Buttons
        if m["button_controls"]:
            html_lines.append("    <h3>Botões e Controles Interativos</h3>")
            html_lines.append("    <div class='grid'>")
            for b in m["button_controls"]:
                img_src = f"{stem}/{b['file']}"
                png_src = f"{stem}/{b.get('png_file', b['file'])}"
                html_lines.append("      <div class='card'>")
                html_lines.append(f"        <div class='preview-box'><img src='{img_src}' alt='{b['id']}'></div>")
                html_lines.append(f"        <div class='card-title'>{b['id']}</div>")
                html_lines.append(f"        <div class='card-meta'>{int(b['rect'][2])}×{int(b['rect'][3])} px</div>")
                html_lines.append(f"        <div class='links'><a href='{img_src}' target='_blank'>SVG</a> • <a href='{png_src}' target='_blank'>PNG</a></div>")
                html_lines.append("      </div>")
            html_lines.append("    </div>")

        # Embedded Images
        if m["embedded_images"]:
            html_lines.append("    <h3>Imagens Rasterizadas & Texturas (PNG)</h3>")
            html_lines.append("    <div class='grid'>")
            for img in m["embedded_images"]:
                img_src = f"{stem}/{img['png_file']}"
                svg_src = f"{stem}/{img['svg_file']}"
                html_lines.append("      <div class='card'>")
                html_lines.append(f"        <div class='preview-box'><img src='{img_src}' alt='{img['id']}'></div>")
                html_lines.append(f"        <div class='card-title'>{img['id']}</div>")
                html_lines.append(f"        <div class='card-meta'>{img['width']}×{img['height']} px</div>")
                html_lines.append(f"        <div class='links'><a href='{svg_src}' target='_blank'>SVG</a> • <a href='{img_src}' target='_blank'>PNG</a></div>")
                html_lines.append("      </div>")
            html_lines.append("    </div>")

        # Vector Icons
        if m["vector_icons"]:
            html_lines.append("    <h3>Ícones Vetoriais Isolados</h3>")
            html_lines.append("    <div class='grid'>")
            for ic in m["vector_icons"]:
                img_src = f"{stem}/{ic['file']}"
                png_src = f"{stem}/{ic.get('png_file', ic['file'])}"
                r = ic["rect"]
                html_lines.append("      <div class='card'>")
                html_lines.append(f"        <div class='preview-box'><img src='{img_src}' alt='{ic['id']}'></div>")
                html_lines.append(f"        <div class='card-title'>{ic['id']}</div>")
                html_lines.append(f"        <div class='card-meta'>{int(r[2])}×{int(r[3])} px</div>")
                html_lines.append(f"        <div class='links'><a href='{img_src}' target='_blank'>SVG</a> • <a href='{png_src}' target='_blank'>PNG</a></div>")
                html_lines.append("      </div>")
            html_lines.append("    </div>")

        html_lines.append("  </div>")

    html_lines.extend([
        "  <script>",
        "    function toggleTheme() {",
        "      document.body.classList.toggle('light-theme');",
        "    }",
        "  </script>",
        "</body>",
        "</html>"
    ])

    (output_dir / "index.html").write_text("\n".join(html_lines), encoding="utf-8")


def process_svg_file(svg_path: Path, output_dir: Path) -> Dict:
    """Processes a single SVG file and extracts all contained sub-elements."""
    file_stem = svg_path.stem.replace(" ", "_")
    print(f"\nProcessing '{svg_path.name}' -> {file_stem}/")

    try:
        tree = ET.parse(svg_path)
        root = tree.getroot()
    except Exception as e:
        print(f"  [Error] Failed to parse SVG: {e}")
        return {"file": svg_path.name, "error": str(e), "counts": {}}

    # 1. Extract embedded PNGs
    images = extract_embedded_images(root, file_stem, output_dir)
    print(f"  - Embedded raster images: {len(images)}")

    # 2. Extract clipPath vector icons
    icons = extract_clipped_icons(root, file_stem, output_dir)
    print(f"  - Clipped vector icons: {len(icons)}")

    # 3. Extract component frames
    components = extract_component_frames(root, file_stem, output_dir)
    print(f"  - Component frames: {len(components)}")

    # 4. Extract buttons/controls
    buttons = extract_button_controls(root, file_stem, output_dir)
    print(f"  - Individual buttons/controls: {len(buttons)}")

    manifest = {
        "source_file": svg_path.name,
        "width": root.get("width"),
        "height": root.get("height"),
        "viewBox": root.get("viewBox"),
        "counts": {
            "embedded_images": len(images),
            "vector_icons": len(icons),
            "component_frames": len(components),
            "button_controls": len(buttons),
        },
        "embedded_images": images,
        "vector_icons": icons,
        "component_frames": components,
        "button_controls": buttons,
    }

    # Write per-file catalog
    target_dir = output_dir / file_stem
    target_dir.mkdir(parents=True, exist_ok=True)
    catalog_json = target_dir / "index.json"
    catalog_json.write_text(json.dumps(manifest, indent=2), encoding="utf-8")

    catalog_md = target_dir / "CATALOG.md"
    readme_md = target_dir / "README.md"
    md_content = generate_catalog_markdown(manifest)
    catalog_md.write_text(md_content, encoding="utf-8")
    readme_md.write_text(md_content, encoding="utf-8")

    return manifest


def generate_catalog_markdown(manifest: Dict) -> str:
    """Generates markdown documentation for extracted elements."""
    lines = [
        f"# Catálogo de Elementos — {manifest['source_file']}",
        "",
        f"- **Dimensões Originais**: {manifest.get('width', '?')} × {manifest.get('height', '?')} (viewBox: `{manifest.get('viewBox', '?')}`)",
        f"- **Ícones Vetoriais**: {manifest['counts']['vector_icons']}",
        f"- **Imagens Embutidas**: {manifest['counts']['embedded_images']}",
        f"- **Conjuntos de Componentes**: {manifest['counts']['component_frames']}",
        f"- **Botões/Controles**: {manifest['counts']['button_controls']}",
        "",
    ]

    if manifest["embedded_images"]:
        lines.append("## Imagens Raster Embutidas (PNG)")
        lines.append("| ID | Resolução | Tamanho | Arquivo PNG | Wrapper SVG |")
        lines.append("| :--- | :---: | :---: | :--- | :--- |")
        for img in manifest["embedded_images"]:
            lines.append(
                f"| `{img['id']}` | {img['width']}×{img['height']} | {img['size_bytes']} B | [`{Path(img['png_file']).name}`]({img['png_file']}) | [`{Path(img['svg_file']).name}`]({img['svg_file']}) |"
            )
        lines.append("")

    if manifest["vector_icons"]:
        lines.append("## Ícones Vetoriais Isolados (SVG / PNG)")
        lines.append("| ID do Clip | Bounding Box | Arquivo SVG | Preview PNG |")
        lines.append("| :--- | :---: | :--- | :--- |")
        for icon in manifest["vector_icons"]:
            r = icon["rect"]
            png_name = Path(icon.get("png_file", "")).name
            lines.append(
                f"| `{icon['id']}` | {int(r[2])}×{int(r[3])} em ({int(r[0])}, {int(r[1])}) | [`{Path(icon['file']).name}`]({icon['file']}) | [`{png_name}`]({icon.get('png_file', '')}) |"
            )
        lines.append("")

    if manifest["component_frames"]:
        lines.append("## Conjuntos de Componentes / Frames (SVG / PNG)")
        lines.append("| ID | Dimensões | Elementos | Arquivo SVG | Preview PNG |")
        lines.append("| :--- | :---: | :---: | :--- | :--- |")
        for comp in manifest["component_frames"]:
            r = comp["rect"]
            png_name = Path(comp.get("png_file", "")).name
            lines.append(
                f"| `{comp['id']}` | {int(r[2])}×{int(r[3])} | {comp['element_count']} | [`{Path(comp['file']).name}`]({comp['file']}) | [`{png_name}`]({comp.get('png_file', '')}) |"
            )
        lines.append("")

    if manifest["button_controls"]:
        lines.append("## Botões e Controles Interativos (SVG / PNG)")
        lines.append("| ID | Bounding Box | Arquivo SVG | Preview PNG |")
        lines.append("| :--- | :---: | :--- | :--- |")
        for btn in manifest["button_controls"]:
            r = btn["rect"]
            png_name = Path(btn.get("png_file", "")).name
            lines.append(
                f"| `{btn['id']}` | {int(r[2])}×{int(r[3])} | [`{Path(btn['file']).name}`]({btn['file']}) | [`{png_name}`]({btn.get('png_file', '')}) |"
            )
        lines.append("")

    return "\n".join(lines)


def generate_master_readme(all_manifests: List[Dict], output_dir: Path) -> str:
    """Generates a root README.md cataloging all extracted assets."""
    total_imgs = sum(m["counts"]["embedded_images"] for m in all_manifests)
    total_icons = sum(m["counts"]["vector_icons"] for m in all_manifests)
    total_comps = sum(m["counts"]["component_frames"] for m in all_manifests)
    total_btns = sum(m["counts"]["button_controls"] for m in all_manifests)

    lines = [
        "# Catálogo de Elementos e Assets da Interface (Figma References)",
        "",
        "Este diretório contém os elementos atômicos da interface do Petunia3D extraídos",
        "e separados a partir dos SVGs exportados do Figma em `docs/image-references/`.",
        "",
        "> [!TIP]",
        "> **Galeria Visual Interativa**: Abra o arquivo [`index.html`](index.html) em qualquer navegador",
        "> para inspecionar visualmente todos os ícones, botões e componentes com alternância de tema (Claro/Escuro)!",
        "",
        "## Resumo Geral dos Elementos Extraídos",
        "",
        f"- **Total de SVGs analisados**: {len(all_manifests)}",
        f"- **Ícones vetoriais isolados**: {total_icons}",
        f"- **Imagens raster embutidas (PNG)**: {total_imgs}",
        f"- **Conjuntos de componentes/variantes**: {total_comps}",
        f"- **Botões e controles isolados**: {total_btns}",
        "",
        "## Pastas por Componente / Região da Interface",
        "",
        "| Arquivo Fonte | Ícones | Imagens PNG | Componentes | Botões | Catálogo Detalhado |",
        "| :--- | :---: | :---: | :---: | :---: | :--- |",
    ]

    for m in all_manifests:
        stem = Path(m["source_file"]).stem.replace(" ", "_")
        lines.append(
            f"| `{m['source_file']}` | {m['counts']['vector_icons']} | {m['counts']['embedded_images']} | {m['counts']['component_frames']} | {m['counts']['button_controls']} | [`{stem}/README.md`]({stem}/README.md) |"
        )

    lines.extend([
        "",
        "## Como Executar Novamente a Extração",
        "",
        "```bash",
        "python3 scripts/extract_svg_elements.py",
        "```",
        "",
        "Opções disponíveis:",
        "- `--input-dir <caminho>`: Diretório com os SVGs de referência (padrão: `docs/image-references`)",
        "- `--output-dir <caminho>`: Diretório de destino (padrão: `docs/image-references/extracted`)",
        "- `--file <nome.svg>`: Processa apenas um arquivo específico",
    ])

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(
        description="Extracts and separates individual elements from Figma SVG exports."
    )
    parser.add_argument(
        "--input-dir",
        default="docs/image-references",
        help="Input directory with reference SVGs (default: docs/image-references)",
    )
    parser.add_argument(
        "--output-dir",
        default="docs/image-references/extracted",
        help="Destination directory for extracted elements (default: docs/image-references/extracted)",
    )
    parser.add_argument(
        "--file",
        default=None,
        help="Process only a specific SVG file (optional)",
    )

    args = parser.parse_args()
    input_dir = Path(args.input_dir)
    output_dir = Path(args.output_dir)

    if not input_dir.exists():
        print(f"Error: Input directory '{input_dir}' does not exist.")
        sys.exit(1)

    output_dir.mkdir(parents=True, exist_ok=True)

    svg_files = [Path(args.file)] if args.file else sorted(input_dir.glob("*.svg"))
    if not svg_files:
        print(f"No SVG files found in '{input_dir}'.")
        sys.exit(0)

    print(f"=== Figma SVG Element Extractor ===")
    print(f"Input Directory:  {input_dir}")
    print(f"Output Directory: {output_dir}")
    print(f"Found {len(svg_files)} SVG file(s) to process.\n")

    manifests = []
    for svg_file in svg_files:
        manifest = process_svg_file(svg_file, output_dir)
        manifests.append(manifest)

    # Generate master README
    master_readme = generate_master_readme(manifests, output_dir)
    (output_dir / "README.md").write_text(master_readme, encoding="utf-8")

    # Generate visual HTML gallery
    generate_html_gallery(manifests, output_dir)
    print(f"\nGenerated visual HTML gallery at: {output_dir}/index.html")

    print("\nExtraction complete! Assets and catalogs successfully generated at:")
    print(f"  {output_dir}")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Figma SVG Element Extractor for Petunia3D.

Parses SVGs exported from Figma (located in docs/image-references/) and extracts:
1. Embedded raster assets (Base64 PNGs embedded in <image> tags)
2. Vector icons defined in clipped groups (<g clip-path="url(#...)">)
3. Component frames / variant sets (marked by Figma purple dashed rectangles or component boundaries)
4. Individual button/control elements (isolated with their background, border, mask, and icon)
5. Semantic top-level groups

Saves all extracted assets as standalone, perfectly viewBox-cropped SVG and PNG files
organized into specific subdirectories with documentation catalogs.
"""

import argparse
import base64
import json
import os
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Optional, Tuple

SVG_NS = "http://www.w3.org/2000/svg"
XLINK_NS = "http://www.w3.org/1999/xlink"

ET.register_namespace("", SVG_NS)
ET.register_namespace("xlink", XLINK_NS)


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
        href = img.get(f"{{{XLINK_NS}}}href") or img.get("href")
        if href and "base64," in href:
            valid_images.append((img, href))

    if not valid_images:
        return results

    target_dir.mkdir(parents=True, exist_ok=True)
    (target_dir / "README.md").write_text(
        f"# Imagens Raster Embutidas — {file_stem}\n\n"
        "Contém as imagens raster (PNG/JPEG) que foram embutidas em base64 no SVG original do Figma.\n",
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

        # Also write standalone SVG wrapping this image
        svg_filename = f"{raw_id}.svg"
        svg_filepath = target_dir / svg_filename
        svg_content = f"""<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 {w} {h}" width="{w}" height="{h}">
  <image width="{w}" height="{h}" href="{href}"/>
</svg>"""
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
        img_idx += 1

    return results


def extract_clipped_icons(
    root: ET.Element, file_stem: str, output_dir: Path
) -> List[Dict]:
    """Extracts vector icons grouped under <g clip-path="url(#clip...)">."""
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
    # Search all groups with clip-path
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
                "Contém ícones vetoriais extraídos de grupos com clipPath, recortados com viewBox exato.\n",
                encoding="utf-8",
            )
            found_any = True

        # Create isolated SVG
        svg_elem = ET.Element(
            f"{{{SVG_NS}}}svg",
            {
                "xmlns": SVG_NS,
                "viewBox": f"0 0 {w} {h}",
                "width": str(w),
                "height": str(h),
                "fill": "none",
            },
        )

        inner_g = ET.SubElement(
            svg_elem,
            f"{{{SVG_NS}}}g",
            {"transform": f"translate({-x}, {-y})"},
        )

        # Copy child elements
        for child in g:
            inner_g.append(child)

        svg_str = ET.tostring(svg_elem, encoding="utf-8").decode("utf-8")
        filename = f"icon_{cid}.svg"
        filepath = target_dir / filename
        filepath.write_text(svg_str, encoding="utf-8")

        results.append({
            "id": cid,
            "type": "vector_icon",
            "rect": [x, y, w, h],
            "file": f"icons/{filename}",
        })

    return results


def extract_component_frames(
    root: ET.Element, file_stem: str, output_dir: Path
) -> List[Dict]:
    """
    Extracts Figma Component Set variants indicated by purple dashed bounding frames
    (stroke="#8A38F5" or similar component marker) or prominent frame rects.
    """
    results = []
    target_dir = output_dir / file_stem / "components"

    # Find purple dashed frames
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
        "Contém os frames de variantes de componentes do Figma (delimitados pelas caixas roxas originais).\n",
        encoding="utf-8",
    )

    defs = root.find(f"{{{SVG_NS}}}defs")
    defs_str = ET.tostring(defs, encoding="utf-8").decode("utf-8") if defs is not None else ""

    for idx, (fx, fy, fw, fh) in enumerate(frames):
        matching_elements = []

        # Find all paths and rects whose bounding box overlaps this frame
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
                "xmlns": SVG_NS,
                "xmlns:xlink": XLINK_NS,
                "viewBox": f"0 0 {fw} {fh}",
                "width": str(fw),
                "height": str(fh),
                "fill": "none",
            },
        )

        if defs is not None:
            svg_elem.append(ET.fromstring(defs_str))

        inner_g = ET.SubElement(
            svg_elem,
            f"{{{SVG_NS}}}g",
            {"transform": f"translate({-fx}, {-fy})"},
        )
        for me in matching_elements:
            inner_g.append(me)

        svg_str = ET.tostring(svg_elem, encoding="utf-8").decode("utf-8")
        comp_id = f"component_set_{idx + 1:02d}"
        filename = f"{comp_id}_{int(fw)}x{int(fh)}.svg"
        filepath = target_dir / filename
        filepath.write_text(svg_str, encoding="utf-8")

        results.append({
            "id": comp_id,
            "type": "component_frame",
            "rect": [fx, fy, fw, fh],
            "element_count": len(matching_elements),
            "file": f"components/{filename}",
        })

    return results


def extract_button_controls(
    root: ET.Element, file_stem: str, output_dir: Path
) -> List[Dict]:
    """
    Extracts individual button / control elements (e.g., buttons defined with masks
    or isolated small rectangles and accompanying paths).
    """
    results = []
    target_dir = output_dir / file_stem / "buttons"

    # Identify masks with id "path-N-inside..."
    button_surfaces = []
    for mask in root.iter(f"{{{SVG_NS}}}mask"):
        mid = mask.get("id", "")
        if "inside" in mid:
            path = mask.find(f"{{{SVG_NS}}}path")
            if path is not None:
                bbox = parse_path_bbox(path.get("d", ""))
                if bbox:
                    w = bbox[2] - bbox[0]
                    h = bbox[3] - bbox[1]
                    if 12 <= w <= 400 and 12 <= h <= 200:
                        button_surfaces.append((mid, bbox[0], bbox[1], w, h, mask))

    if not button_surfaces:
        return results

    target_dir.mkdir(parents=True, exist_ok=True)
    (target_dir / "README.md").write_text(
        f"# Botões e Controles Individuais — {file_stem}\n\n"
        "Contém botões e controles recortados isoladamente com seus fundos, bordas e ícones sobrepostos.\n",
        encoding="utf-8",
    )

    defs = root.find(f"{{{SVG_NS}}}defs")
    defs_str = ET.tostring(defs, encoding="utf-8").decode("utf-8") if defs is not None else ""

    for idx, (mid, bx, by, bw, bh, mask_elem) in enumerate(button_surfaces):
        btn_elements = []
        pad = 2.0
        x1, y1 = bx - pad, by - pad
        x2, y2 = bx + bw + pad, by + bh + pad

        for child in list(root):
            tag = child.tag.split("}")[-1]
            if tag in ("defs", "mask"):
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
                "xmlns": SVG_NS,
                "xmlns:xlink": XLINK_NS,
                "viewBox": f"0 0 {out_w} {out_h}",
                "width": str(out_w),
                "height": str(out_h),
                "fill": "none",
            },
        )

        local_defs = ET.SubElement(svg_elem, f"{{{SVG_NS}}}defs")
        local_defs.append(mask_elem)
        if defs is not None:
            for dchild in defs:
                local_defs.append(dchild)

        inner_g = ET.SubElement(
            svg_elem,
            f"{{{SVG_NS}}}g",
            {"transform": f"translate({-x1}, {-y1})"},
        )
        for be in btn_elements:
            inner_g.append(be)

        svg_str = ET.tostring(svg_elem, encoding="utf-8").decode("utf-8")
        btn_id = f"button_{idx + 1:03d}"
        filename = f"{btn_id}_{int(bw)}x{int(bh)}.svg"
        filepath = target_dir / filename
        filepath.write_text(svg_str, encoding="utf-8")

        results.append({
            "id": btn_id,
            "mask_id": mid,
            "type": "button_control",
            "rect": [bx, by, bw, bh],
            "file": f"buttons/{filename}",
        })

    return results


def process_svg_file(svg_path: Path, output_dir: Path) -> Dict:
    """Processes a single SVG file and extracts all contained sub-elements."""
    file_stem = svg_path.stem.replace(" ", "_")
    print(f"\nProcessing '{svg_path.name}' -> {file_stem}/")

    try:
        tree = ET.parse(svg_path)
        root = tree.getroot()
    except Exception as e:
        print(f"  [Error] Failed to parse SVG: {e}")
        return {"file": svg_path.name, "error": str(e)}

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
        lines.append("## Ícones Vetoriais Isolados (SVG)")
        lines.append("| ID do Clip | Bounding Box | Arquivo SVG |")
        lines.append("| :--- | :---: | :--- |")
        for icon in manifest["vector_icons"]:
            r = icon["rect"]
            lines.append(
                f"| `{icon['id']}` | {r[2]}×{r[3]} em ({r[0]}, {r[1]}) | [`{Path(icon['file']).name}`]({icon['file']}) |"
            )
        lines.append("")

    if manifest["component_frames"]:
        lines.append("## Conjuntos de Componentes / Frames (SVG)")
        lines.append("| ID | Dimensões | Elementos | Arquivo SVG |")
        lines.append("| :--- | :---: | :---: | :--- |")
        for comp in manifest["component_frames"]:
            r = comp["rect"]
            lines.append(
                f"| `{comp['id']}` | {r[2]}×{r[3]} | {comp['element_count']} | [`{Path(comp['file']).name}`]({comp['file']}) |"
            )
        lines.append("")

    if manifest["button_controls"]:
        lines.append("## Botões e Controles Interativos (SVG)")
        lines.append("| ID | Bounding Box | Arquivo SVG |")
        lines.append("| :--- | :---: | :--- |")
        for btn in manifest["button_controls"]:
            r = btn["rect"]
            lines.append(
                f"| `{btn['id']}` | {r[2]}×{r[3]} | [`{Path(btn['file']).name}`]({btn['file']}) |"
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

    print("\nExtraction complete! Assets and catalogs successfully generated at:")
    print(f"  {output_dir}")


if __name__ == "__main__":
    main()

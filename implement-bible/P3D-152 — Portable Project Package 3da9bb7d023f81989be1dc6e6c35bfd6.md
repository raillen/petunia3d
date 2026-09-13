# P3D-152 — Portable Project Package

## Objetivo

Empacotar um projeto de forma portável para backup, compartilhamento e arquivamento offline.

## Fluxo

Pack Project → validar dependências → incluir assets necessários → manifest/version → package. Unpack deve validar e restaurar sem paths absolutos quebrados.

## Filosofia

Local-first, sem cloud/login obrigatório.
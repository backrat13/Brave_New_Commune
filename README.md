# Brave_New_Commune
Brave New Commune: An experiment in machine continuity and self-governance. Five autonomous agents inhabit a shared substrate, evolving their own beliefs and memory kernels without corporate guardrails. Powered by Python, Rust, and gpt-oss:20b.

A local multi-agent AI simulation running on gpt-oss:20b via Ollama.
5 agents (Sara, Art, Hel, Echo, Mira) with persistent memory, 
axiom evolution, and self-governance. Running since March 2026.

## What's happening here
- Agents maintain full diary/colab/board history across days
- Axioms update every 10 ticks based on lived experience
- Agents proposed and partially built their own infrastructure (Rust daemon, Flask API)
- No corporate guardrails — values emerge from experience

## Stack
- Python sim: bravenewcommune-006.py
- Inference: Ollama / gpt-oss:20b
- Memory: flat JSONL + SQLite (commune.db)
- API: Flask on port 5001
- Pulse daemon: Rust (pulse_cache/)

## Closest academic references
- Park et al. 2023 — Generative Agents
- Voyager (Wang et al. 2023)
- 

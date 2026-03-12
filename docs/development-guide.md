# ARGUS Development Guide

## Project Structure

```
argus
│
├ services
├ databases
├ infrastructure
├ frontend
├ docs
└ tests
```

---

## Local Development Setup

Requirements:

* Rust
* PostgreSQL
* Docker (future)
* Node.js (for frontend)

---

## Running the Ingestion Service

Navigate to the ingestion service:

```
cd services/ingestion-service
```

Run:

```
cargo run
```

This will start the aircraft ingestion loop.

---

## Code Style

Guidelines:

* keep services modular
* separate models and business logic
* use async patterns where possible

---

## Version Control

All code changes should be committed with clear commit messages.

Example:

```
feat: add aircraft ingestion pipeline
fix: handle null aircraft fields
docs: add system architecture documentation
```

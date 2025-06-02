# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- `cargo build` - Compile the project
- `cargo run` - Run the main executable (demonstrates media catalog functionality)
- `cargo test` - Run tests (currently no tests exist)
- `cargo check` - Check code for errors without building

## Architecture

This is a Rust media catalog system with a modular structure:

### Core Components

- **Media enum** (`src/content/media.rs`): Defines 5 media types (Book, Movie, AudioBook, Podcast, Placeholder) with a `describe()` method for display
- **Catalog struct** (`src/content/catalog.rs`): Manages collections of media items with `add_item()` and `get_by_index()` methods
- **Content module** (`src/content/mod.rs`): Organizes media-related functionality

### Key Design Patterns

- Uses Rust enums with associated data for type-safe media representation
- Implements Option<&Media> return type for safe index-based access
- Follows Rust module system with clear separation of concerns
- Private `items` field in Catalog ensures controlled access through public methods

The main.rs demonstrates usage by creating various media items, adding them to a catalog, and retrieving them by index with proper error handling.
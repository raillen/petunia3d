//! Petunia MCP boundary (P1-04 `rmcp` + P1-05 `tokio`).
//!
//! Flow enforced here:
//!
//! ```text
//! MCP transport (stdio)
//!     ↓
//! Petunia MCP adapter (this crate)
//!     ↓
//! capability/validation (allowlisted tools, bounded args)
//!     ↓
//! Petunia domain (Project + UndoStack + DTOs)
//! ```
//!
//! The server never touches UI, renderer or filesystem on its own: tools
//! mutate an in-memory [`Project`](petunia_project::Project) through
//! checkpoints (undo-safe) and return DTOs. Tokio lives only inside this
//! crate (`serve_stdio`); no Tokio type crosses into Core.

pub mod server;

pub use rmcp::ErrorData as McpError;
pub use server::{PetuniaMcp, serve_stdio, serve_stdio_blocking};

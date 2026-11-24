//! Opts the process into the D3D12 Agility SDK.
//!
//! The D3D12 runtime built into Windows 10 predates shader model 6.6, so it
//! refuses root signatures that index the descriptor heaps directly (flags
//! `0x400`/`0x800`) even when the GPU supports them. When the executable
//! exports these two symbols, the system loader instead uses the newer
//! `D3D12Core.dll` found at `D3D12SDKPath`, relative to the executable.
//! Windows 11 accepts the same root signatures without it.
//!
//! The runtime is not part of the repository: copy `D3D12Core.dll` (and,
//! for the debug layer, `d3d12SDKLayers.dll`) from the
//! `Microsoft.Direct3D.D3D12` NuGet package, version 1.620.x, into a `D3D12`
//! folder beside `victory.exe`. Without that folder the system runtime is
//! used, as before.
//!
//! `build.rs` adds the linker exports; Rust does not export symbols from an
//! executable on its own.

// The runtime looks the symbols up by these exact names.
#![allow(non_upper_case_globals)]

/// The Agility SDK version the runtime in `D3D12\` must match
/// (`D3D12_SDK_VERSION` of Microsoft.Direct3D.D3D12 1.620.x).
#[unsafe(no_mangle)]
#[used]
pub static D3D12SDKVersion: u32 = 620;

/// Where the runtime looks for `D3D12Core.dll`, relative to the executable.
#[unsafe(no_mangle)]
#[used]
pub static D3D12SDKPath: &[u8; 9] = b".\\D3D12\\\0";

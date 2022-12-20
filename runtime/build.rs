#[cfg(not(feature = "parachain-gen"))]
use substrate_wasm_builder::WasmBuilder;

fn main() {
	#[cfg(not(feature = "parachain-gen"))]
	WasmBuilder::new()
		.with_current_project()
		.export_heap_base()
		.import_memory()
		.build()
}

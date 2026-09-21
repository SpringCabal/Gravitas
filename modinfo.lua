local modinfo = {
	name = "Gravitas WASM",
	shortname = "GRRW",
	game = "Gravitas WASM",
	shortgame = "GRRW",
	version = "0.1.0-wasm",
	description = "Gravitas gameplay ported to Rust Core-WASM and RmlUi",
	modtype = "1",
	-- Single-player only: without this the engine binds a UDP socket and warns
	-- that nobody else can reach it. Nothing here is ever hosted or joined.
	onlyLocal = true,
	depend = {
		"cursors.sdz",
	}
}

return modinfo

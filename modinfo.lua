local modinfo = {
	name = "Gravitas",
	shortname = "GRR",
	game = "Gravitas",
	shortgame = "GRR",
	description = "Spring Cabals Ludum Dare entry: Gravitas",
	url = "https://github.com/SpringCabal/Gravitas",
	version = "",
	modtype = "1",
	-- Single-player only: without this the engine binds a UDP socket and warns
	-- that nobody else can reach it. Nothing here is ever hosted or joined.
	onlyLocal = true,
	depend = {
		"cursors.sdz"
	}
}

return modinfo

if addon.InGetInfo then
	return {
		name      = "Remove Engine Menu";
		desc      = "Removes the engines build/command menu";
		author    = "Bluestone";
		date      = "2008-2013";
		license   = "GNU GPL, v2 or later";

		layer     = math.huge;
		hidden    = true; -- don't show in the widget selector
		api       = true; -- load before all others?
		enabled   = true; -- loaded by default?
	}
end

local function DummyHandler(xIcons, yIcons, cmdCount, commands)
	handler.commands   = commands
	handler.commands.n = cmdCount
	handler:CommandsChanged()
	return "", xIcons, yIcons, {}, {}, {}, {}, {}, {}, {}, {}
end

function addon.Initialize()
	RegisterGlobal("LayoutButtons", DummyHandler)
end


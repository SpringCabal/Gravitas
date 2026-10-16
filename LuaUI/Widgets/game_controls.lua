local controls = VFS.Include("LuaHandler/Utilities/game_controls.lua")

widget.GetInfo = controls.GetInfo
widget.Initialize = controls.Initialize
widget.KeyPress = controls.KeyPress
widget.Shutdown = controls.Shutdown

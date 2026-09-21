function widget:GetInfo()
  return {
    name = "Screenshot Taker",
    desc = "Take screenshot for automated testing when configured",
    author = "gajop",
    date = "2026",
    license = "MIT",
    layer = 1000,
    enabled = false,
  }
end

local targetFrame = nil
local quitFrame = nil
local taken = false

function widget:Initialize()
  local modOptions = Spring.GetModOptions() or {}
  targetFrame = tonumber(modOptions.screenshot_frame)
  quitFrame = tonumber(modOptions.quit_frame)
end

function widget:GameFrame(n)
  if targetFrame and n >= targetFrame and not taken then
    taken = true
    Spring.SendCommands("screenshot")
  end
  if quitFrame and n >= quitFrame then
    Spring.SendCommands("quit")
  end
end

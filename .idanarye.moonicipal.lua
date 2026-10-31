local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

local P, cfg = moonicipal.import(require'idan.project.rust.bevy')
cfg.crate_name = 'trolls_gold'
--cfg.setup_level_editor()

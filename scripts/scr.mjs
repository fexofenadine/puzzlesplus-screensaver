import { copyFileSync, mkdirSync } from 'node:fs'

//a screensaver is the program renamed .scr; windows lists it by its string 1 (screensaver.rc)
//no space in the name, which github would turn into a dot
mkdirSync('release', { recursive: true })
copyFileSync('src-tauri/target/release/puzzlesplus-screensaver.exe', 'release/PuzzlesPlus.scr')

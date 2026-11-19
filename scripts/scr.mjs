import { copyFileSync, mkdirSync } from 'node:fs'

//a screensaver is the program renamed .scr, which windows lists by its file name
mkdirSync('release', { recursive: true })
copyFileSync('src-tauri/target/release/puzzlesplus-screensaver.exe', 'release/Puzzles Plus.scr')

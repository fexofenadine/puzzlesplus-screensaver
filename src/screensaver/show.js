import './seed'
import '../app.css'
import './screensaver.css'
import { startBackgrounds } from '../shell/backgrounds'
import { loadOptions, randomTheme } from './options'

//the screensaver itself, fullscreen (/s) or in the screen saver settings' little monitor (?preview)
const root = document.documentElement
const preview = new URLSearchParams(location.search).has('preview')
if (preview) root.dataset.preview = ''

const { random, minutes } = loadOptions()
if (random) {
  root.dataset.theme = randomTheme(root.dataset.theme)
  setInterval(() => (root.dataset.theme = randomTheme(root.dataset.theme)), minutes * 60000)
}

startBackgrounds()

//a key, a click, a turned wheel, a mouse moved past a nudge or another window ends it
const quit = () => {
  if (window.__TAURI__) window.__TAURI__.core.invoke('quit')
  else window.close()
}

const NUDGE = 8
if (!preview) {
  let from = null
  window.addEventListener('pointermove', (event) => {
    if (!from) from = [event.screenX, event.screenY]
    else if (Math.hypot(event.screenX - from[0], event.screenY - from[1]) > NUDGE) quit()
  })
  for (const name of ['pointerdown', 'keydown', 'wheel', 'blur'])
    window.addEventListener(name, quit)
}

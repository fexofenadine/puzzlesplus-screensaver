import { load, save } from '../shell/storage'
import { themes } from '../shell/themes'

//the screensaver's own options: a theme picked at random, changed every so many minutes
export const MINUTES = { min: 1, max: 60, step: 1 }

export const loadOptions = () => {
  const options = { random: false, minutes: 5 }
  try {
    const saved = JSON.parse(load('screensaver')) || {}
    if (typeof saved.random === 'boolean') options.random = saved.random
    const minutes = Number(saved.minutes)
    if (Number.isInteger(minutes) && minutes >= MINUTES.min && minutes <= MINUTES.max)
      options.minutes = minutes
  } catch {
    //nothing saved, or not readable
  }
  return options
}

export const saveOptions = (options) => save('screensaver', JSON.stringify(options))

//any theme but the one showing
export const randomTheme = (current, rng = Math.random) => {
  const others = themes.map(([id]) => id).filter((id) => id !== current)
  return others[Math.floor(rng() * others.length)]
}

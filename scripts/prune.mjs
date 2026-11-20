//after merging upstream (the website), removes again whatever the screensaver does not use:
//node scripts/prune.mjs, then commit; it starts from the screensaver's pages and its tests
import { execFileSync } from 'node:child_process'
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs'
import { dirname, join, normalize, resolve, sep } from 'node:path'

const roots = [
  'src/screensaver/show.js',
  'src/screensaver/settings.jsx',
  'src/screensaver/options.test.js',
  'src/shell/backgrounds/backgrounds.test.js',
  'src/shell/glitches.test.js',
  'src/shell/motion.test.js',
  'src/shell/moveWindow.test.js',
  'src/shell/prepaint.test.js',
  'src/shell/zoom.test.js',
  'src/Overlay.test.jsx',
  //read by vite.config.js, not imported
  'src/shell/prepaint.js',
]
//the website's pages and files, none of them part of this repo
const websiteOnly = [
  '2048',
  'futoshiki',
  'how-to-play',
  'index.html',
  'lightup',
  'minesweeper',
  'nonograms',
  'public',
  'sudoku',
  'takuzu',
  'docs/banner',
  'docs/screenshots',
  'docs/screenshots.py',
  'docs/puzzlesplus.png',
]

const seen = new Set()
const tries = ['', '.js', '.jsx', '.css', '/index.js', '/index.jsx']
const find = (from, spec) => {
  const base = resolve(dirname(from), spec.split('?')[0])
  for (const end of tries) {
    const file = base + end
    if (existsSync(file) && statSync(file).isFile()) return normalize(file)
  }
  return null
}
const walk = (file) => {
  if (!file || seen.has(file)) return
  seen.add(file)
  if (!/\.(js|jsx|css)$/.test(file)) return
  const text = readFileSync(file, 'utf8')
  const specs = [...text.matchAll(/(?:from|import)\s*\(?\s*['"](\.[^'"]+)['"]/g)].map((m) => m[1])
  for (const m of text.matchAll(/url\(\s*['"]?(\.[^'")]+)/g)) specs.push(m[1])
  for (const spec of specs) walk(find(file, spec))
}
roots.forEach((file) => walk(resolve(file)))

const unused = []
const list = (dir) => {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name)
    if (statSync(path).isDirectory()) list(path)
    else if (!seen.has(path)) unused.push(path)
  }
}
list(resolve('src'))

const remove = [...unused, ...websiteOnly.map((path) => resolve(path))].filter(existsSync)
const names = remove.map((path) =>
  path
    .slice(resolve('.').length + 1)
    .split(sep)
    .join('/'),
)
for (let i = 0; i < names.length; i += 50) {
  execFileSync('git', ['rm', '-q', '-r', '-f', '--ignore-unmatch', ...names.slice(i, i + 50)])
}
process.stdout.write(`removed ${names.length} paths
`)

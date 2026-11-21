#retakes the readme's pictures (docs/screensaver) from `npx vite --mode screensaver --port 5175`
#waits follow the project's rule: synthwave and matrix a minute, terafox 22 seconds (a lot of hexagons are on screen)
#in playwright's firefox with webgl2 switched on; py -3 -I scripts/shots.py from the repo's root
#the settings stand in for the program's bridge (__TAURI__) so they show its preview button
import json, os, subprocess
from playwright.sync_api import sync_playwright

BASE = 'http://localhost:5175/screensaver/'
OUT = os.path.join('docs', 'screensaver')

#a draw costs nothing in a software renderer, so the shader keeps its full size
INIT = 'if (window.WebGL2RenderingContext) WebGL2RenderingContext.prototype.readPixels = function () {}'


def take(b, name, page, store, wait=6000, size=(1280, 720), init=''):
    c = b.new_context(viewport={'width': size[0], 'height': size[1]})
    sets = ''.join(f'localStorage.setItem({json.dumps(k)}, {json.dumps(v)});' for k, v in store.items())
    c.add_init_script(sets + INIT + ';\n' + init)
    pg = c.new_page()
    pg.goto(BASE + page)
    pg.wait_for_timeout(wait)
    png = os.path.join(OUT, name + '.png')
    pg.screenshot(path=png)
    subprocess.run(['ffmpeg', '-v', 'error', '-y', '-i', png, '-q:v', '3', os.path.join(OUT, name + '.jpg')], check=True)
    os.remove(png)
    c.close()
    print('wrote', name)


neon = json.dumps({'version': 2, 'neon': {'resolution': 100}})
os.makedirs(OUT, exist_ok=True)
with sync_playwright() as p:
    b = p.firefox.launch(firefox_user_prefs={'webgl.force-enabled': True, 'webgl.enable-webgl2': True})
    take(b, 'synthwave', 'index.html', {'theme': 'synthwave', 'backgrounds': neon}, wait=60000)
    take(b, 'vaporwave', 'index.html', {'theme': 'vaporwave'})
    take(b, 'matrix', 'index.html', {'theme': 'matrix'}, wait=60000)
    take(b, 'terafox', 'index.html', {'theme': 'terafox'}, wait=22000)
    take(b, 'settings', 'settings.html', {'theme': 'vaporwave', 'screensaver': json.dumps({'random': True, 'minutes': 5})},
         size=(900, 1000), init='window.__TAURI__ = { core: { invoke: () => {} } }')
    b.close()

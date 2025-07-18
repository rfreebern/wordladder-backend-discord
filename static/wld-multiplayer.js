(async () => {
    const today = new Date().toISOString().slice(0, 10);
    const url = new URL(location.href);
    const guild = url.searchParams.get('guild_id') || 'solo';
    const uid = crypto.randomUUID();

    // override storage
    const kv = (k, v) => fetch(`/kv/${guild}:${uid}`, {
        method: v ? 'PUT' : 'GET',
        headers: { Authorization: `Discord ${window.DISCORD_TOKEN}` },
        body: v ? JSON.stringify(v) : undefined
    }).then(r => r.json());

    // IndexedDB shim
    const idb = indexedDB.open;
    indexedDB.open = () => Promise.reject('disabled');
    window.localStorage = {
        getItem: k => kv(k),
        setItem: (k, v) => kv(k, v)
    };

    // WebSocket presence
    const ws = new WebSocket(`wss://${location.host}/ws`);
    ws.onopen = () => ws.send(window.DISCORD_TOKEN);
    ws.onmessage = e => renderPresence(JSON.parse(e.data));

    // rich activity override
    if (window.parent !== window) {
        window.parent.postMessage({
            cmd: 'SET_ACTIVITY',
            args: {
                activity: {
                    state: 'Solving today’s ladder',
                    details: 'WordLadder Daily',
                    timestamps: { start: Date.now() },
                    assets: { large_image: 'wordladder_banner' }
                }
            }
        }, '*');
    }

    function renderPresence(list) {
        // draw floating avatar bar
        const bar = document.getElementById('wld-presence') || createPresenceBar();
        bar.innerHTML = '';
        
        list.forEach(p => {
            const avatar = document.createElement('div');
            avatar.className = 'wld-avatar';
            avatar.innerHTML = `
                <img src="${p.icon_url || '/default-avatar.png'}" alt="${p.user_id}">
                <span class="wld-len">${p.ladder_len || 0}</span>
            `;
            bar.appendChild(avatar);
        });
    }

    function createPresenceBar() {
        const bar = document.createElement('div');
        bar.id = 'wld-presence';
        bar.style.cssText = `
            position: fixed;
            top: 10px;
            right: 10px;
            display: flex;
            gap: 5px;
            z-index: 1000;
            background: rgba(0,0,0,0.8);
            padding: 10px;
            border-radius: 20px;
        `;
        document.body.appendChild(bar);
        return bar;
    }

    // inject CSS
    const style = document.createElement('style');
    style.textContent = `
        .wld-avatar {
            position: relative;
            width: 40px;
            height: 40px;
            border-radius: 50%;
            overflow: hidden;
            border: 2px solid #7289da;
        }
        .wld-avatar img {
            width: 100%;
            height: 100%;
            object-fit: cover;
        }
        .wld-len {
            position: absolute;
            bottom: -5px;
            right: -5px;
            background: #43b581;
            color: white;
            border-radius: 50%;
            width: 20px;
            height: 20px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 12px;
            font-weight: bold;
        }
    `;
    document.head.appendChild(style);
})();

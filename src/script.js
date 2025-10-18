//console.log("script loaded");

const tauri = window.__TAURI__;
const invoke = (...args) => (tauri?.core?.invoke ?? tauri?.invoke)(...args);
//const listen = tauri?.event?.listen;

// const tauri = window.__TAURI__;
const tauriInvoke = (...args) => {
    const inv = window.__TAURI__?.core?.invoke;
    if (!inv) throw new Error("Tauri not ready yet");
    return inv(...args);
};

const gatherConfig = () => ({
    interval_ms: Number(document.getElementById("interval").value || 50),
    random_minus: Number(document.getElementById("random-minus").value || 0),
    random_plus: Number(document.getElementById("random-plus").value || 0),
    follow_cursor: document.getElementById("follow-cursor").checked,
    //TODO: fix this
    // fixed_x: document.getElementById("fixed-x").value
    //     ? Number(document.getElementById("fixed-x").value)
    //     : null,
    // fixed_y: document.getElementById("fixed-y").value
    //     ? Number(document.getElementById("fixed-y").value)
    //     : null,
    fixed_x: null,
    fixed_y: null,
    button: document.getElementById("button").value || "left",
});

function hook() {
    //console.log("hook attaching listeners");

    const startBtn = document.getElementById("start-btn");
    const stopBtn    = document.getElementById("stop-btn");
    const statusEl = document.getElementById("status");
    const clickEl = document.getElementById("total-clicks");
    const clearClicksBtn = document.getElementById("clear-clicks");

    // console.log("hook found elements:", {
    //     startBtn: !!startBtn,
    //     stopBtn: !!stopBtn,
    //     statusEl: !!statusEl,
    // });

    if (!startBtn || !stopBtn || !statusEl) return; 

    // async function pushConfig() {
    //     const cfg = gatherConfig();
    //     // NOTE: arg name must match Rust command signature: fn update_click_config(new_cfg: ClickConfig, ...)
    //     await tauriInvoke("update_click_config", { newCfg: cfg });
    // }

    // const gatherConfig = () => ({
    //     interval_ms: Number(document.getElementById("interval").value || 50),
    //     random_minus: Number(document.getElementById("random-minus").value || 0),
    //     random_plus: Number(document.getElementById("random-plus").value || 0),
    //     follow_cursor: document.getElementById("follow-cursor").checked,
    //     //TODO: fix this
    //     // fixed_x: document.getElementById("fixed-x").value
    //     //     ? Number(document.getElementById("fixed-x").value)
    //     //     : null,
    //     // fixed_y: document.getElementById("fixed-y").value
    //     //     ? Number(document.getElementById("fixed-y").value)
    //     //     : null,
    //     fixed_x: null,
    //     fixed_y: null,
    //     button: document.getElementById("button").value || "left",
    // });

    startBtn.addEventListener("click", async () => {
        //console.log("start-btn clicked - ", gatherConfig());
        startBtn.disabled = true;
        try {
            await tauriInvoke("start_clicking", { newCfg: gatherConfig() });
            //await tauriInvoke("start_clicking", gatherConfig());
            statusEl.textContent = "Running…";
        } catch (e) {
            console.error("start_clicking failed", e);
            statusEl.textContent = "Error";
        } finally {
            startBtn.disabled = false;
        }
    });

    stopBtn.addEventListener("click", async () => {
        //console.log("stop-btn clicked");
        try {
            await tauriInvoke("stop_clicking");
            statusEl.textContent = "Stopped";
        } catch (e) {
            //console.error("stop_clicking failed", e);
        }
    });

    clearClicksBtn?.addEventListener("click", async () => {
        try {
            await tauriInvoke("clear_clicks");
            clickEl.textContent = `Total Clicks: 0`;
        } catch (e) {
            //console.error("clear_clicks failed", e);
        }
    });

    f6Listener = async (e) => {
        if (e.key === "F6") {
            try {
                const running = await tauriInvoke("is_running");
                if (running) {
                    await tauriInvoke("stop_clicking");
                    statusEl.textContent = "Stopped";
                } else {
                    await tauriInvoke("start_clicking", { newCfg: gatherConfig() });
                    statusEl.textContent = "Running…";
                }
            } catch (e) {
                console.error("F6 handler error:", e);
            }
        }
    };
    window.addEventListener("keydown", f6Listener);

    setInterval(async () => {
        try {
            const running = await tauriInvoke("is_running");
            statusEl.textContent = running ? "Running…" : "Stopped";

            const total = await invoke("get_clicks");
            console.log("total clicks:", total);
            clickEl.textContent = `Total Clicks: ${total}`;   
        } catch {
            // Saul Goodman
        }
    }, 500);
}

document.addEventListener("DOMContentLoaded", () => {
    console.log("DOMContentLoaded");
    invoke("update_click_config", { newCfg: gatherConfig() }).catch(()=>{});
    hook();
});

const debounce = (fn, ms=150) => { let t; return (...a)=>{ clearTimeout(t); t=setTimeout(()=>fn(...a),ms); }; };
const pushConfig = debounce(() => invoke("update_click_config", { newCfg: gatherConfig() }), 150);

["interval","random-minus","random-plus","button","follow-cursor"].forEach(id => {
  const el = document.getElementById(id);
  if (!el) return;
  el.addEventListener(el.type === "checkbox" ? "change" : "input", pushConfig);
});

// listen?.("request-cfg", async () => {
//   console.log("request-cfg received")
//   await invoke("update_click_config", { newCfg: gatherConfig() });
// });

// window.addEventListener("tauri://ready", () => {
//     console.log("tauri://ready");
//     hook();
// });

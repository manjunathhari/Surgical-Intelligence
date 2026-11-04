import { useEffect, useState } from "react";

type Summary = { format: string; version: number; nodes: number; edges: number; learning_records: number; publications: number; evidence_claims: number; source_artifacts: number; events: number; embedding_profiles: number };
type PortableState = Record<string, unknown> & { graph?: { nodes?: unknown[]; edges?: unknown[] }; learning_records?: unknown[]; publications?: unknown[]; evidence_claims?: unknown[]; source_artifacts?: unknown[]; events?: unknown[] };
const empty: Summary = { format: "—", version: 0, nodes: 0, edges: 0, learning_records: 0, publications: 0, evidence_claims: 0, source_artifacts: 0, events: 0, embedding_profiles: 0 };

export default function App() {
  const [summary, setSummary] = useState<Summary>(empty);
  const [state, setState] = useState<PortableState | null>(null);
  const [health, setHealth] = useState<"connecting" | "online" | "offline">("connecting");
  const [notice, setNotice] = useState("Connecting to the local Rust engine…");
  const [tab, setTab] = useState("Overview");
  const [jsonText, setJsonText] = useState("");
  const [busy, setBusy] = useState(false);

  async function refresh() {
    try {
      const [h, s, d] = await Promise.all([
        fetch("/api/health").then(r => { if (!r.ok) throw new Error("API unavailable"); return r.json(); }),
        fetch("/api/summary").then(r => r.json()),
        fetch("/api/state").then(r => r.json())
      ]);
      setHealth(h.status === "ok" ? "online" : "offline"); setSummary(s); setState(d);
      setJsonText(JSON.stringify(d, null, 2)); setNotice("Connected directly to si-core through the local Rust API.");
    } catch { setHealth("offline"); setNotice("Rust API is not running. Start it using the instructions in README."); }
  }
  useEffect(() => { void refresh(); }, []);

  async function validate() {
    setBusy(true);
    try { const r = await fetch("/api/validate"); const d = await r.json(); setNotice(d.valid ? "✓ " + d.message : "Validation failed: " + d.message); }
    catch { setNotice("Could not reach the local API."); }
    finally { setBusy(false); }
  }
  async function saveState() {
    setBusy(true);
    try {
      const parsed = JSON.parse(jsonText);
      const r = await fetch("/api/state", { method: "PUT", headers: { "Content-Type": "application/json" }, body: JSON.stringify(parsed) });
      const d = await r.json(); setNotice(d.message); if (r.ok) await refresh();
    } catch (e) { setNotice(e instanceof Error ? e.message : "Save failed"); }
    finally { setBusy(false); }
  }
  const metrics = [
    ["Graph nodes", summary.nodes, "network"], ["Graph edges", summary.edges, "git-merge"],
    ["Learning records", summary.learning_records, "brain"], ["Publications", summary.publications, "book-open"],
    ["Evidence claims", summary.evidence_claims, "microscope"], ["Source artifacts", summary.source_artifacts, "paperclip"],
    ["Events", summary.events, "history"], ["Embedding profiles", summary.embedding_profiles, "layers"]
  ];
  const tabs = ["Overview", "Knowledge graph", "Learning", "Evidence & papers", "Media assets", "State inspector"];
  const collection = tab === "Knowledge graph" ? state?.graph?.nodes : tab === "Learning" ? state?.learning_records : tab === "Evidence & papers" ? [...(state?.publications ?? []), ...(state?.evidence_claims ?? [])] : tab === "Media assets" ? state?.source_artifacts : null;

  return <div className="shell">
    <aside className="sidebar">
      <div className="brand"><div className="brand-mark">SI</div><div><strong>SURGICAL</strong><span>INTELLIGENCE</span></div></div>
      <div className="side-label">WORKSPACE</div>
      {tabs.map(t => <button key={t} className={tab === t ? "nav active" : "nav"} onClick={() => setTab(t)}>{t}</button>)}
      <div className="side-bottom"><span className={"status-dot " + health}></span><span>{health === "online" ? "Rust engine connected" : health === "offline" ? "Engine offline" : "Connecting…"}</span><small>LOCAL DEVELOPMENT</small></div>
    </aside>
    <main className="main">
      <header><div><div className="eyebrow">SURGICAL INTELLIGENCE / LOCAL</div><h1>{tab}</h1></div><div className="header-actions"><span className={"pill " + health}>{health === "online" ? "● ENGINE ONLINE" : health === "offline" ? "● OFFLINE" : "◌ CONNECTING"}</span><button className="secondary" onClick={() => void refresh()}>↻ Refresh</button></div></header>
      <div className="notice">{notice}</div>
      {tab === "Overview" && <>
        <section className="hero"><div><div className="eyebrow">PORTABLE KNOWLEDGE ENGINE</div><h2>Knowledge that<br/><em>outlives the model.</em></h2><p>A model-independent surgical knowledge substrate. Explore the actual portable state managed by the Rust core.</p><div className="hero-actions"><button onClick={() => setTab("State inspector")}>Inspect knowledge state →</button><button className="secondary" onClick={() => void validate()} disabled={busy}>Validate state</button></div></div><div className="hero-orbit"><div className="orbit orbit-a"></div><div className="orbit orbit-b"></div><div className="core-node">SI<span>CORE</span></div><i className="node n1"></i><i className="node n2"></i><i className="node n3"></i><i className="node n4"></i></div></section>
        <div className="section-head"><div><div className="eyebrow">LIVE STATE</div><h3>Knowledge inventory</h3></div><span className="muted">Format {summary.version || "—"} · {summary.format}</span></div>
        <div className="metrics">{metrics.map(([label, value]) => <div className="metric" key={String(label)}><span>{label}</span><strong>{value}</strong></div>)}</div>
        <div className="bottom-grid"><div className="panel"><div className="eyebrow">ENGINE STATUS</div><h3>Runtime connection</h3><p>Browser → Rust API → si-core → portable JSON state.</p><div className="runtime"><span className="status-dot online"></span> Local API <b>{health === "online" ? "Connected" : "Not connected"}</b></div><div className="runtime"><span className="status-dot online"></span> State validation <b>On demand</b></div></div><div className="panel"><div className="eyebrow">DESIGN PRINCIPLE</div><h3>Structured, traceable, portable</h3><p>Assertions, graph relationships, events, learning records, publications and media manifests remain inspectable data—not hidden inside model weights.</p></div></div>
      </>}
      {tab !== "Overview" && tab !== "State inspector" && <section className="panel data-panel"><div className="section-head"><div><div className="eyebrow">PORTABLE STATE EXPLORER</div><h3>{tab}</h3></div><span className="muted">{collection?.length ?? 0} records</span></div>{collection?.length ? <pre>{JSON.stringify(collection, null, 2)}</pre> : <div className="empty"><span>◇</span><strong>No records in this collection yet</strong><p>This view reads the current portable state. It does not fabricate sample surgical knowledge.</p></div>}</section>}
      {tab === "State inspector" && <section className="panel data-panel"><div className="section-head"><div><div className="eyebrow">VERSIONED JSON</div><h3>Portable state inspector</h3></div><div className="header-actions"><button className="secondary" onClick={() => void validate()} disabled={busy}>Validate</button><button onClick={() => void saveState()} disabled={busy || health !== "online"}>Save validated state</button></div></div><p className="muted">Edit the portable JSON directly. Saving replaces the local state only after Rust validation succeeds. Keep a backup before editing.</p><textarea spellCheck={false} value={jsonText} onChange={e => setJsonText(e.target.value)} /><div className="editor-foot"><span>Source: local state file · API bound to 127.0.0.1</span><button className="secondary" onClick={() => { if (state) setJsonText(JSON.stringify(state, null, 2)); }}>Discard edits</button></div></section>}
      <footer><span>SURGICAL INTELLIGENCE</span><span>RUST CORE · PORTABLE STATE · LOCAL FIRST</span></footer>
    </main>
  </div>;
}

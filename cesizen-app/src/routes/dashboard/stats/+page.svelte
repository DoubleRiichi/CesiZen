<script lang="ts">
	import { onMount, onDestroy, tick } from 'svelte';
	import { goto } from '$app/navigation';
	import { user as userStore, isAuthenticated } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import { toasts } from '$lib/stores/toasts';
	import { feelingTrackerApi, feelingsApi, ApiError } from '$lib/api';
	import type { FeelingTrackerGet, FeelingGet } from '$lib/types';
	import {
		Chart,
		ArcElement,
		BarElement,
		LineElement,
		PointElement,
		BarController,
		DoughnutController,
		LineController,
		CategoryScale,
		LinearScale,
		Filler,
		Legend,
		Tooltip
	} from 'chart.js';

	// Register Chart.js components
	Chart.register(
		ArcElement, BarElement, LineElement, PointElement,
		BarController, DoughnutController, LineController,
		CategoryScale, LinearScale, Filler, Legend, Tooltip
	);

	// ── Auth guard ──────────────────────────────────────────
	let currentUser = $derived($userStore);

	// ── State ───────────────────────────────────────────────
	let entries = $state<FeelingTrackerGet[]>([]);
	let feelings = $state<FeelingGet[]>([]);
	let loading = $state(true);
	let period = $state<'semaine' | 'mois' | 'trimestre' | 'année'>('mois');
	let activeTab = $state<'overview' | 'emotions' | 'tendances' | 'lieux'>('overview');

	// Canvas refs
	let pieCanvas = $state<HTMLCanvasElement | null>(null);
	let lineCanvas = $state<HTMLCanvasElement | null>(null);
	let areaCanvas = $state<HTMLCanvasElement | null>(null);
	let barTrendCanvas = $state<HTMLCanvasElement | null>(null);
	let stackedBarCanvas = $state<HTMLCanvasElement | null>(null);
	let intensityDistCanvas = $state<HTMLCanvasElement | null>(null);
	let locationBarCanvas = $state<HTMLCanvasElement | null>(null);
	let locationIntensityCanvas = $state<HTMLCanvasElement | null>(null);

	let chartInstances: Chart[] = [];

	// ── Emotion category colors ─────────────────────────────
	const CATEGORY_COLORS: Record<string, string> = {
		'Positive': '#27ae60', 'Negative': '#c0392b', 
		'Neutral': '#2980b9'
	};

	function getCategoryColor(cat: string): string {
		return CATEGORY_COLORS[cat] || '#6b6b6b';
	}

	// ── Period range ────────────────────────────────────────
	function getPeriodRange(p: string): { start: Date; end: Date } {
		const now = new Date();
		const end = new Date(now);
		const start = new Date(now);
		switch (p) {
			case 'semaine': start.setDate(now.getDate() - 7); break;
			case 'mois': start.setMonth(now.getMonth() - 1); break;
			case 'trimestre': start.setMonth(now.getMonth() - 3); break;
			case 'année': start.setFullYear(now.getFullYear() - 1); break;
		}
		return { start, end };
	}

	// ── Derived data ────────────────────────────────────────
	let filteredEntries = $derived.by(() => {
		const { start, end } = getPeriodRange(period);
		return entries.filter(e => { const d = new Date(e.timestamp_start); return d >= start && d <= end; });
	});

	let totalEntries = $derived(filteredEntries.length);
	let avgIntensity = $derived(
		filteredEntries.length > 0 ? (filteredEntries.reduce((s, e) => s + e.intensity, 0) / filteredEntries.length).toFixed(1) : '—'
	);

	let topCategory = $derived.by(() => {
		if (filteredEntries.length === 0) return '—';
		const c = new Map<string, number>();
		for (const e of filteredEntries) c.set(e.feeling_category, (c.get(e.feeling_category) ?? 0) + 1);
		let max = 0, name = '—';
		for (const [k, v] of c) { if (v > max) { max = v; name = k; } }
		return name;
	});

	let topFeeling = $derived.by(() => {
		if (filteredEntries.length === 0) return '—';
		const c = new Map<string, number>();
		for (const e of filteredEntries) c.set(e.feeling, (c.get(e.feeling) ?? 0) + 1);
		let max = 0, name = '—';
		for (const [k, v] of c) { if (v > max) { max = v; name = k; } }
		return name;
	});

	let categoryBreakdown = $derived.by(() => {
		const counts: Record<string, number> = {};
		for (const e of filteredEntries) counts[e.feeling_category] = (counts[e.feeling_category] ?? 0) + 1;
		return Object.entries(counts).map(([name, value]) => ({ name, value, color: getCategoryColor(name) })).sort((a, b) => b.value - a.value);
	});

	let feelingBreakdown = $derived.by(() => {
		const counts: Record<string, { name: string; count: number; category: string }> = {};
		for (const e of filteredEntries) {
			if (!counts[e.feeling]) counts[e.feeling] = { name: e.feeling, count: 0, category: e.feeling_category };
			counts[e.feeling].count++;
		}
		return Object.values(counts).sort((a, b) => b.count - a.count).slice(0, 10);
	});

	let categoryDetails = $derived.by(() => {
		return Object.keys(CATEGORY_COLORS).map(cat => {
			const ce = filteredEntries.filter(e => e.feeling_category === cat);
			return { name: cat, count: ce.length, avgIntensity: ce.length > 0 ? (ce.reduce((s, e) => s + e.intensity, 0) / ce.length).toFixed(1) : '—', color: getCategoryColor(cat) };
		});
	});

	let locationStats = $derived.by(() => {
		const locs: Record<string, { name: string; count: number; totalIntensity: number }> = {};
		for (const e of filteredEntries) {
			const loc = e.location || 'Non renseigné';
			if (!locs[loc]) locs[loc] = { name: loc, count: 0, totalIntensity: 0 };
			locs[loc].count++; locs[loc].totalIntensity += e.intensity;
		}
		return Object.values(locs).map(l => ({ ...l, avgIntensity: +(l.totalIntensity / l.count).toFixed(1) })).sort((a, b) => b.count - a.count);
	});

	let intensityDistribution = $derived.by(() => {
		const b = Array.from({ length: 10 }, (_, i) => ({ level: i + 1, count: 0 }));
		for (const e of filteredEntries) { if (e.intensity >= 1 && e.intensity <= 10) b[e.intensity - 1].count++; }
		return b;
	});

	// ── Time grouping ───────────────────────────────────────
	function groupByTime(data: FeelingTrackerGet[], p: string): Map<string, FeelingTrackerGet[]> {
		const groups = new Map<string, FeelingTrackerGet[]>();
		for (const e of data) {
			const d = new Date(e.timestamp_start);
			let key: string;
			if (p === 'semaine') key = d.toLocaleDateString('fr-FR', { weekday: 'short', day: 'numeric' });
			else if (p === 'mois') key = d.toLocaleDateString('fr-FR', { day: '2-digit', month: 'short' });
			else if (p === 'trimestre') { const w = Math.ceil((d.getDate() + new Date(d.getFullYear(), d.getMonth(), 1).getDay()) / 7); key = `S${w} ${d.toLocaleDateString('fr-FR', { month: 'short' })}`; }
			else key = d.toLocaleDateString('fr-FR', { month: 'short' });
			if (!groups.has(key)) groups.set(key, []);
			groups.get(key)!.push(e);
		}
		return groups;
	}

	let timeGroups = $derived(groupByTime(filteredEntries, period));
	let timeLabels = $derived([...timeGroups.keys()]);
	let intensityOverTime = $derived(timeLabels.map(l => { const it = timeGroups.get(l) ?? []; return it.length > 0 ? +(it.reduce((s, e) => s + e.intensity, 0) / it.length).toFixed(1) : 0; }));
	let entriesOverTime = $derived(timeLabels.map(l => (timeGroups.get(l) ?? []).length));
	let categoryOverTime = $derived.by(() => {
		const ds: Record<string, number[]> = {};
		for (const cat of Object.keys(CATEGORY_COLORS)) ds[cat] = timeLabels.map(l => (timeGroups.get(l) ?? []).filter(e => e.feeling_category === cat).length);
		return ds;
	});

	// ── Lifecycle ───────────────────────────────────────────
	onMount(() => {
		if (!get(isAuthenticated)) { goto('/login'); return; }
		loadData();
	});

	onDestroy(() => destroyCharts());

	async function loadData() {
		loading = true;
		try {
			const [eRes, fRes] = await Promise.all([
				feelingTrackerApi.search({ page_size: 500 }),
				feelingsApi.search({ page_size: 100 })
			]);
			entries = eRes; feelings = fRes;
		} catch (err) { console.error(err); toasts.error('Erreur lors du chargement.'); }
		finally { loading = false; await tick(); renderCharts(); }
	}

	// ── Chart management ────────────────────────────────────
	function destroyCharts() { for (const c of chartInstances) { try { c.destroy(); } catch {} } chartInstances = []; }

	async function handlePeriodChange(p: typeof period) { period = p; await tick(); destroyCharts(); renderCharts(); }
	async function handleTabChange(t: typeof activeTab) { activeTab = t; await tick(); destroyCharts(); renderCharts(); }

	function renderCharts() {
		const ff = "'Nunito', 'Segoe UI', sans-serif";
		const tc = '#6b6b6b';
		const gc = 'rgba(0,0,0,0.06)';

		if (activeTab === 'overview') {
			if (pieCanvas) chartInstances.push(new Chart(pieCanvas, {
				type: 'doughnut', data: { labels: categoryBreakdown.map(c => c.name), datasets: [{ data: categoryBreakdown.map(c => c.value), backgroundColor: categoryBreakdown.map(c => c.color), borderWidth: 2, borderColor: '#fff' }] },
				options: { responsive: true, maintainAspectRatio: false, cutout: '55%', plugins: { legend: { position: 'bottom', labels: { font: { family: ff, size: 12 }, color: tc, padding: 16, usePointStyle: true, pointStyleWidth: 12 } } } }
			}));
			if (lineCanvas) chartInstances.push(new Chart(lineCanvas, {
				type: 'line', data: { labels: timeLabels, datasets: [{ label: 'Intensité moyenne', data: intensityOverTime, borderColor: '#2a7d6e', backgroundColor: 'rgba(42,125,110,0.1)', fill: true, tension: 0.35, pointRadius: 3, pointBackgroundColor: '#2a7d6e', pointBorderWidth: 0, borderWidth: 2.5 }] },
				options: { responsive: true, maintainAspectRatio: false, scales: { y: { min: 0, max: 10, grid: { color: gc }, ticks: { font: { family: ff, size: 11 }, color: tc } }, x: { grid: { display: false }, ticks: { font: { family: ff, size: 10 }, color: tc, maxRotation: 45 } } }, plugins: { legend: { display: false } } }
			}));
			if (areaCanvas) chartInstances.push(new Chart(areaCanvas, {
				type: 'line', data: { labels: timeLabels, datasets: Object.keys(CATEGORY_COLORS).map(cat => ({ label: cat, data: categoryOverTime[cat] ?? [], borderColor: getCategoryColor(cat), backgroundColor: getCategoryColor(cat) + '40', fill: true, tension: 0.3, pointRadius: 0, borderWidth: 2 })) },
				options: { responsive: true, maintainAspectRatio: false, scales: { y: { stacked: true, grid: { color: gc }, ticks: { font: { family: ff, size: 11 }, color: tc } }, x: { grid: { display: false }, ticks: { font: { family: ff, size: 10 }, color: tc, maxRotation: 45 } } }, plugins: { legend: { position: 'bottom', labels: { font: { family: ff, size: 11 }, color: tc, padding: 12, usePointStyle: true, pointStyleWidth: 10 } } } }
			}));
		}

		if (activeTab === 'emotions' && intensityDistCanvas) {
			const grad = intensityDistribution.map((_, i) => { const t = i / 9; return `rgb(${Math.round(42 + t * 150)},${Math.round(125 - t * 68)},${Math.round(110 - t * 67)})`; });
			chartInstances.push(new Chart(intensityDistCanvas, {
				type: 'bar', data: { labels: intensityDistribution.map(b => b.level.toString()), datasets: [{ label: 'Occurrences', data: intensityDistribution.map(b => b.count), backgroundColor: grad, borderRadius: 6, borderSkipped: false }] },
				options: { responsive: true, maintainAspectRatio: false, scales: { y: { grid: { color: gc }, ticks: { font: { family: ff, size: 11 }, color: tc } }, x: { grid: { display: false }, ticks: { font: { family: ff, size: 11 }, color: tc } } }, plugins: { legend: { display: false } } }
			}));
		}

		if (activeTab === 'tendances') {
			if (barTrendCanvas) chartInstances.push(new Chart(barTrendCanvas, {
				type: 'bar', data: { labels: timeLabels, datasets: [{ label: 'Entrées', data: entriesOverTime, backgroundColor: 'rgba(42,125,110,0.7)', borderRadius: 6, borderSkipped: false }] },
				options: { responsive: true, maintainAspectRatio: false, scales: { y: { grid: { color: gc }, ticks: { font: { family: ff, size: 11 }, color: tc } }, x: { grid: { display: false }, ticks: { font: { family: ff, size: 10 }, color: tc, maxRotation: 45 } } }, plugins: { legend: { display: false } } }
			}));
			if (stackedBarCanvas) chartInstances.push(new Chart(stackedBarCanvas, {
				type: 'bar', data: { labels: timeLabels, datasets: Object.keys(CATEGORY_COLORS).map(cat => ({ label: cat, data: categoryOverTime[cat] ?? [], backgroundColor: getCategoryColor(cat) + 'CC', borderRadius: 2, borderSkipped: false })) },
				options: { responsive: true, maintainAspectRatio: false, scales: { y: { stacked: true, grid: { color: gc }, ticks: { font: { family: ff, size: 11 }, color: tc } }, x: { stacked: true, grid: { display: false }, ticks: { font: { family: ff, size: 10 }, color: tc, maxRotation: 45 } } }, plugins: { legend: { position: 'bottom', labels: { font: { family: ff, size: 11 }, color: tc, padding: 12, usePointStyle: true } } } }
			}));
		}

		if (activeTab === 'lieux') {
			if (locationBarCanvas) chartInstances.push(new Chart(locationBarCanvas, {
				type: 'bar', data: { labels: locationStats.map(l => l.name), datasets: [{ label: 'Entrées', data: locationStats.map(l => l.count), backgroundColor: 'rgba(142,68,173,0.7)', borderRadius: 6, borderSkipped: false }] },
				options: { responsive: true, maintainAspectRatio: false, indexAxis: 'y', scales: { x: { grid: { color: gc }, ticks: { font: { family: ff, size: 11 }, color: tc } }, y: { grid: { display: false }, ticks: { font: { family: ff, size: 12 }, color: '#2c2c2c' } } }, plugins: { legend: { display: false } } }
			}));
			if (locationIntensityCanvas) chartInstances.push(new Chart(locationIntensityCanvas, {
				type: 'bar', data: { labels: locationStats.map(l => l.name), datasets: [{ label: 'Intensité moy.', data: locationStats.map(l => l.avgIntensity), backgroundColor: 'rgba(230,126,34,0.7)', borderRadius: 6, borderSkipped: false }] },
				options: { responsive: true, maintainAspectRatio: false, indexAxis: 'y', scales: { x: { min: 0, max: 10, grid: { color: gc }, ticks: { font: { family: ff, size: 11 }, color: tc } }, y: { grid: { display: false }, ticks: { font: { family: ff, size: 12 }, color: '#2c2c2c' } } }, plugins: { legend: { display: false } } }
			}));
		}
	}

	const periods: Array<{ key: typeof period; label: string }> = [
		{ key: 'semaine', label: 'Semaine' }, { key: 'mois', label: 'Mois' },
		{ key: 'trimestre', label: 'Trimestre' }, { key: 'année', label: 'Année' },
	];
	const tabs: Array<{ key: typeof activeTab; label: string }> = [
		{ key: 'overview', label: "Vue d'ensemble" }, { key: 'emotions', label: 'Émotions' },
		{ key: 'tendances', label: 'Tendances' }, { key: 'lieux', label: 'Lieux' },
	];
</script>

<svelte:head><title>Statistiques — CESIZen</title></svelte:head>

{#if loading}
	<div class="state-center"><div class="spinner"></div><p>Chargement des statistiques…</p></div>
{:else}
	<div class="stats-page">
		<header class="stats-header">
			<div>
				<h1 class="stats-title">Rapport Émotionnel</h1>
				<p class="stats-subtitle">Statistiques et consolidations de votre tracker d'émotions</p>
			</div>
			<div class="period-selector">
				{#each periods as p}
					<button class="period-btn" class:active={period === p.key} onclick={() => handlePeriodChange(p.key)}>{p.label}</button>
				{/each}
			</div>
		</header>

		<div class="indicators">
			<div class="indicator-card"><span class="indicator-icon"></span><span class="indicator-value">{totalEntries}</span><span class="indicator-label">Total entrées</span><span class="indicator-sub">sur la période : {period}</span></div>
			<div class="indicator-card"><span class="indicator-icon"></span><span class="indicator-value">{avgIntensity}</span><span class="indicator-label">Intensité moyenne</span><span class="indicator-sub">sur 10</span></div>
			<div class="indicator-card"><span class="indicator-icon"></span><span class="indicator-value">{topCategory}</span><span class="indicator-label">Émotion dominante</span></div>
			<div class="indicator-card"><span class="indicator-icon"></span><span class="indicator-value indicator-value--sm">{topFeeling}</span><span class="indicator-label">Sentiment le + fréquent</span></div>
		</div>

		<nav class="tabs-nav">
			{#each tabs as t}
				<button class="tab-btn" class:active={activeTab === t.key} onclick={() => handleTabChange(t.key)}>{t.label}</button>
			{/each}
		</nav>

		{#if activeTab === 'overview'}
			<div class="charts-grid">
				<section class="chart-card"><h3 class="chart-title">Répartition par catégorie</h3><div class="chart-container chart-container--pie"><canvas bind:this={pieCanvas}></canvas></div></section>
				<section class="chart-card"><h3 class="chart-title">Intensité moyenne dans le temps</h3><div class="chart-container"><canvas bind:this={lineCanvas}></canvas></div></section>
				<section class="chart-card chart-card--full"><h3 class="chart-title">Évolution des émotions dans le temps</h3><div class="chart-container chart-container--wide"><canvas bind:this={areaCanvas}></canvas></div></section>
			</div>
		{:else if activeTab === 'emotions'}
			<div class="charts-grid">
				<section class="chart-card">
					<h3 class="chart-title">Top 10 des sentiments</h3>
					<div class="feeling-list">
						{#each feelingBreakdown as f, i}
							{@const maxCount = feelingBreakdown[0]?.count ?? 1}
							{@const pct = (f.count / maxCount) * 100}
							<div class="feeling-row">
								<span class="feeling-rank">#{i + 1}</span>
								<div class="feeling-bar-wrapper"><div class="feeling-bar-fill" style="width:{pct}%;background-color:{getCategoryColor(f.category)}25"></div><div class="feeling-bar-content"><span class="feeling-name">{f.name}</span><span class="feeling-count">{f.count}×</span></div></div>
								<span class="feeling-dot" style="background-color:{getCategoryColor(f.category)}"></span>
							</div>
						{/each}
						{#if feelingBreakdown.length === 0}<p class="empty-state">Aucune donnée pour cette période.</p>{/if}
					</div>
				</section>
				<section class="chart-card"><h3 class="chart-title">Distribution des intensités</h3><div class="chart-container"><canvas bind:this={intensityDistCanvas}></canvas></div></section>
				<section class="chart-card chart-card--full">
					<h3 class="chart-title">Détail par catégorie d'émotion</h3>
					<div class="category-grid">
						{#each categoryDetails as cat}
							<div class="category-card" style="border-color:{cat.color}40;background:{cat.color}08"><span class="category-count" style="color:{cat.color}">{cat.count}</span><span class="category-name">{cat.name}</span><span class="category-avg">Intensité moy : {cat.avgIntensity}</span></div>
						{/each}
					</div>
				</section>
			</div>
		{:else if activeTab === 'tendances'}
			<div class="charts-grid charts-grid--single">
				<section class="chart-card"><h3 class="chart-title">Nombre d'entrées dans le temps</h3><div class="chart-container chart-container--wide"><canvas bind:this={barTrendCanvas}></canvas></div></section>
				<section class="chart-card"><h3 class="chart-title">Ratio émotionnel par période</h3><div class="chart-container chart-container--wide"><canvas bind:this={stackedBarCanvas}></canvas></div></section>
			</div>
		{:else if activeTab === 'lieux'}
			<div class="charts-grid">
				<section class="chart-card"><h3 class="chart-title">Entrées par lieu</h3><div class="chart-container"><canvas bind:this={locationBarCanvas}></canvas></div></section>
				<section class="chart-card"><h3 class="chart-title">Intensité moyenne par lieu</h3><div class="chart-container"><canvas bind:this={locationIntensityCanvas}></canvas></div></section>
				<section class="chart-card chart-card--full">
					<h3 class="chart-title">Synthèse par lieu</h3>
					<div class="table-wrapper"><table class="data-table"><thead><tr><th>Lieu</th><th>Entrées</th><th>Intensité moy.</th><th>% du total</th></tr></thead><tbody>
						{#each locationStats as l}<tr><td class="td-name">{l.name}</td><td>{l.count}</td><td>{l.avgIntensity}/10</td><td>{totalEntries > 0 ? ((l.count / totalEntries) * 100).toFixed(1) : 0}%</td></tr>{/each}
						{#if locationStats.length === 0}<tr><td colspan="4" class="empty-state">Aucune donnée.</td></tr>{/if}
					</tbody></table></div>
				</section>
			</div>
		{/if}

		<div class="back-row"><a href="/dashboard" class="back-link">← Retour au Dashboard</a></div>
	</div>
{/if}

<style>
	.stats-page { max-width: 1000px; margin: 0 auto; }
	.stats-header { display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 1rem; margin-bottom: 1.5rem; }
	.stats-title { font-family: var(--font-heading); font-size: 2rem; font-weight: 800; color: var(--color-primary); }
	.stats-subtitle { color: var(--color-text-muted); font-size: 0.9rem; }
	.period-selector { display: flex; gap: 2px; background: var(--color-bg); border: 1px solid var(--color-border); border-radius: var(--radius); padding: 3px; }
	.period-btn { padding: 0.45rem 1rem; border-radius: 6px; border: none; cursor: pointer; font-size: 0.825rem; font-weight: 600; font-family: var(--font-body); transition: all var(--transition); background: transparent; color: var(--color-text-muted); }
	.period-btn.active { background: var(--color-primary); color: white; }
	.period-btn:not(.active):hover { background: var(--color-surface); }
	.indicators { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 1rem; margin-bottom: 1.5rem; }
	.indicator-card { background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius); padding: 1.25rem; display: flex; flex-direction: column; align-items: center; text-align: center; gap: 0.15rem; }
	.indicator-icon { font-size: 1.1rem; margin-bottom: 0.25rem; }
	.indicator-value { display: block; font-family: var(--font-heading); font-size: 1.75rem; font-weight: 800; color: var(--color-primary); }
	.indicator-value--sm { font-size: 1.25rem; }
	.indicator-label { font-size: 0.825rem; color: var(--color-text-muted); }
	.indicator-sub { font-size: 0.7rem; color: var(--color-border); }
	.tabs-nav { display: flex; gap: 0; border-bottom: 2px solid var(--color-border); margin-bottom: 1.5rem; }
	.tab-btn { padding: 0.75rem 1.25rem; border: none; border-bottom: 2px solid transparent; margin-bottom: -2px; background: transparent; color: var(--color-text-muted); font-size: 0.9rem; font-weight: 500; font-family: var(--font-body); cursor: pointer; transition: all var(--transition); }
	.tab-btn.active { color: var(--color-primary); border-bottom-color: var(--color-primary); font-weight: 700; }
	.tab-btn:not(.active):hover { color: var(--color-text); }
	.charts-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem; margin-bottom: 2rem; }
	.charts-grid--single { grid-template-columns: 1fr; }
	@media (max-width: 768px) { .charts-grid { grid-template-columns: 1fr; } }
	.chart-card { background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius); padding: 1.5rem; }
	.chart-card--full { grid-column: 1 / -1; }
	.chart-title { font-family: var(--font-heading); font-size: 1rem; font-weight: 700; color: var(--color-text); margin: 0 0 1rem; }
	.chart-container { position: relative; height: 280px; }
	.chart-container--pie { height: 300px; }
	.chart-container--wide { height: 300px; }
	.feeling-list { display: flex; flex-direction: column; gap: 0.5rem; }
	.feeling-row { display: flex; align-items: center; gap: 0.65rem; }
	.feeling-rank { width: 24px; font-size: 0.8rem; color: var(--color-text-muted); text-align: right; font-weight: 600; }
	.feeling-bar-wrapper { flex: 1; position: relative; height: 34px; background: var(--color-bg); border-radius: 6px; overflow: hidden; }
	.feeling-bar-fill { position: absolute; inset: 0; border-radius: 6px; transition: width 0.4s ease; }
	.feeling-bar-content { position: relative; padding: 0 0.75rem; line-height: 34px; font-size: 0.85rem; font-weight: 500; display: flex; justify-content: space-between; }
	.feeling-name { color: var(--color-text); }
	.feeling-count { color: var(--color-text-muted); font-size: 0.8rem; }
	.feeling-dot { width: 10px; height: 10px; border-radius: 3px; flex-shrink: 0; }
	.category-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 0.75rem; }
	.category-card { border: 1px solid; border-radius: var(--radius); padding: 1rem; text-align: center; display: flex; flex-direction: column; gap: 0.2rem; }
	.category-count { font-family: var(--font-heading); font-size: 1.5rem; font-weight: 800; }
	.category-name { font-size: 0.875rem; font-weight: 600; color: var(--color-text); }
	.category-avg { font-size: 0.75rem; color: var(--color-text-muted); }
	.table-wrapper { overflow-x: auto; }
	.data-table { width: 100%; border-collapse: collapse; }
	.data-table th { background: var(--color-bg); font-size: 0.8rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.03em; color: var(--color-text-muted); padding: 0.75rem 1rem; text-align: left; }
	.data-table td { padding: 0.75rem 1rem; font-size: 0.9rem; border-top: 1px solid var(--color-border); color: var(--color-text); }
	.td-name { font-weight: 600; }
	.empty-state { text-align: center; color: var(--color-text-muted); padding: 2rem 0; font-style: italic; }
	.back-row { margin-top: 1rem; margin-bottom: 2rem; }
	.back-link { color: var(--color-primary); font-weight: 600; font-size: 0.9rem; }
	.back-link:hover { text-decoration: underline; }
	.state-center { text-align: center; padding: 3rem; }
	.spinner { width: 32px; height: 32px; border: 3px solid var(--color-border); border-top-color: var(--color-primary); border-radius: 50%; animation: spin 0.6s linear infinite; margin: 0 auto; }
	@keyframes spin { to { transform: rotate(360deg); } }
</style>
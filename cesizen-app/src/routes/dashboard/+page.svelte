<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { user as userStore, isAuthenticated } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import { toasts } from '$lib/stores/toasts';
	import { feelingTrackerApi, feelingsApi, ApiError } from '$lib/api';
	import type { FeelingTrackerGet, FeelingGet } from '$lib/types';

	// ── Auth guard ──────────────────────────────────────────
	let currentUser = $derived($userStore);

	// ── State ───────────────────────────────────────────────
	let entries = $state<FeelingTrackerGet[]>([]);
	let feelings = $state<FeelingGet[]>([]);
	let loading = $state(true);
	let showForm = $state(false);

	// Calendrier
	let currentDate = $state(new Date());
	let selectedDate = $state<string | null>(null);

	// Formulaire
	let formFeelingId = $state<number>(0);
	let formIntensity = $state(5);
	let formNotes = $state('');
	let formLocation = $state('');
	let formSubmitting = $state(false);
	let formStart = $state('')
	let formEnd = $state('')

	// ── Computed ─────────────────────────────────────────────
	let currentYear = $derived(currentDate.getFullYear());
	let currentMonth = $derived(currentDate.getMonth());
	let monthLabel = $derived(
		currentDate.toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' })
	);

	let calendarDays = $derived.by(() => {
		const firstDay = new Date(currentYear, currentMonth, 1);
		const lastDay = new Date(currentYear, currentMonth + 1, 0);
		const startPad = (firstDay.getDay() + 6) % 7; // Lundi = 0
		const days: Array<{ date: string; day: number; inMonth: boolean }> = [];

		// Jours du mois précédent
		for (let i = startPad - 1; i >= 0; i--) {
			const d = new Date(currentYear, currentMonth, -i);
			days.push({ date: toDateKey(d), day: d.getDate(), inMonth: false });
		}

		// Jours du mois courant
		for (let d = 1; d <= lastDay.getDate(); d++) {
			const date = new Date(currentYear, currentMonth, d);
			days.push({ date: toDateKey(date), day: d, inMonth: true });
		}

		return days;
	});

	// Map date → entrées pour coloration du calendrier
	let entriesByDate = $derived.by(() => {
		const map = new Map<string, FeelingTrackerGet[]>();
		for (const entry of entries) {
			const key = entry.timestamp_start.slice(0, 10);
			if (!map.has(key)) map.set(key, []);
			map.get(key)!.push(entry);
		}
		return map;
	});

	let selectedEntries = $derived(
		selectedDate ? entriesByDate.get(selectedDate) ?? [] : []
	);

	// Indicateurs
	let totalEntries = $derived(entries.length);
	let avgIntensity = $derived(
		entries.length > 0
			? (entries.reduce((s, e) => s + e.intensity, 0) / entries.length).toFixed(1)
			: '—'
	);
	let topFeeling = $derived.by(() => {
		if (entries.length === 0) return '—';
		const counts = new Map<string, number>();
		for (const e of entries) {
			counts.set(e.feeling, (counts.get(e.feeling) ?? 0) + 1);
		}
		let max = 0;
		let name = '—';
		for (const [k, v] of counts) {
			if (v > max) { max = v; name = k; }
		}
		return name;
	});

	// ── Lifecycle ────────────────────────────────────────────
	onMount(() => {
		if (!get(isAuthenticated)) {
			goto('/login');
			return;
		}
		loadData();
	});

	async function loadData() {
		loading = true;
		try {
			const [entriesRes, feelingsRes] = await Promise.all([
				feelingTrackerApi.search({ page_size: 200 }),
				feelingsApi.search({ page_size: 100 })
			]);
			entries = entriesRes;
			feelings = feelingsRes;
			if (feelings.length > 0 && formFeelingId === 0) {
				formFeelingId = feelings[0].id;
			}
		} catch (err) {
			console.error(err);
			toasts.error('Erreur lors du chargement des données.');
		} finally {
			loading = false;
		}
	}

	// ── Helpers ──────────────────────────────────────────────
	function toDateKey(d: Date): string {
		return d.toISOString().slice(0, 10);
	}

	function prevMonth() {
		currentDate = new Date(currentYear, currentMonth - 1, 1);
	}

	function nextMonth() {
		currentDate = new Date(currentYear, currentMonth + 1, 1);
	}

	function selectDay(dateKey: string) {
		selectedDate = selectedDate === dateKey ? null : dateKey;
	}

	function emotionColor(category: string): string {
		switch (category.toLowerCase()) {
			case 'positive': return '#27ae60';
			case 'negative': return '#c0392b';
			case 'neutral': return '#2980b9';
			default: return '#6b6b6b';
		}
	}

	function openAddForm() {
		showForm = true;
	}

	async function submitEntry(e: Event) {
		e.preventDefault();
		if (!currentUser) return;
		formSubmitting = true;

		const now = new Date().toISOString();
		try {
			await feelingTrackerApi.create({
				user_id: currentUser.id,
				feeling_id: formFeelingId,
				timestamp_start: new Date(formStart).toISOString(),
				timestamp_end: new Date(formEnd).toISOString(),
				intensity: formIntensity,
				notes: formNotes,
				location: formLocation
			});
			toasts.success('Émotion ajoutée !');
			showForm = false;
			formNotes = '';
			formLocation = '';
			formIntensity = 5;
			formStart = now;
			formEnd = now;
			await loadData();
		} catch (err) {
			if (err instanceof ApiError) {
				toasts.error('Erreur lors de l\'ajout.');
			}
		} finally {
			formSubmitting = false;
		}
	}

	function formatTime(iso: string): string {
		return new Date(iso).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });
	}
</script>

<svelte:head>
	<title>Dashboard — CESIZen</title>
</svelte:head>

{#if loading}
	<div class="state-center">
		<div class="spinner"></div>
		<p>Chargement du dashboard…</p>
	</div>
{:else}
	<div class="dashboard">
		<header class="dash-header">
			<div>
				<h1 class="dash-title">Mon Tracker d'émotions</h1>
				<p class="dash-subtitle">Dashboard</p>
			</div>
			<button class="btn btn--primary" onclick={openAddForm}>
				+ Ajouter une émotion
			</button>
		</header>

		<!-- Indicateurs -->
		<div class="indicators">
			<div class="indicator-card">
				<span class="indicator-value">{totalEntries}</span>
				<span class="indicator-label">Entrées ce mois</span>
			</div>
			<div class="indicator-card">
				<span class="indicator-value">{avgIntensity}</span>
				<span class="indicator-label">Intensité moyenne</span>
			</div>
			<div class="indicator-card">
				<span class="indicator-value">{topFeeling}</span>
				<span class="indicator-label">Émotion dominante</span>
			</div>
		</div>

		<div class="dash-grid">
			<!-- Calendrier -->
			<section class="calendar-section">
				<div class="calendar-nav">
					<button class="cal-nav-btn" onclick={prevMonth}>←</button>
					<h2 class="cal-month">{monthLabel}</h2>
					<button class="cal-nav-btn" onclick={nextMonth}>→</button>
				</div>

				<div class="calendar">
					<div class="cal-header">
						{#each ['Lu', 'Ma', 'Me', 'Je', 'Ve', 'Sa', 'Di'] as day}
							<span class="cal-day-label">{day}</span>
						{/each}
					</div>
					<div class="cal-grid">
						{#each calendarDays as d}
							<button
								class="cal-day"
								class:outside={!d.inMonth}
								class:has-entries={entriesByDate.has(d.date)}
								class:selected={selectedDate === d.date}
								onclick={() => selectDay(d.date)}
							>
								{d.day}
								{#if entriesByDate.has(d.date)}
									<span class="cal-dot"></span>
								{/if}
							</button>
						{/each}
					</div>
				</div>
			</section>

			<!-- Entrées du jour sélectionné -->
			<section class="entries-section">
				{#if selectedDate}
					<h3 class="entries-title">
						{new Date(selectedDate + 'T00:00:00').toLocaleDateString('fr-FR', {
							weekday: 'long',
							day: 'numeric',
							month: 'long'
						})}
					</h3>

					{#if selectedEntries.length === 0}
						<p class="entries-empty">Aucune entrée ce jour.</p>
					{:else}
						<div class="entries-list">
							{#each selectedEntries as entry (entry.id)}
								<div class="entry-card">
									<div class="entry-header">
										<span
											class="entry-feeling"
											style="color: {emotionColor(entry.feeling_category)}"
										>
											{entry.feeling}
										</span>
										<span class="entry-category">{entry.feeling_category}</span>
									</div>
									<div class="entry-time">{formatTime(entry.timestamp_start)}</div>
									<div class="entry-intensity">
										<span>Intensité</span>
										<div class="intensity-bar">
											<div
												class="intensity-fill"
												style="width: {entry.intensity * 10}%"
											></div>
										</div>
										<span class="intensity-value">{entry.intensity}/10</span>
									</div>
									{#if entry.notes}
										<p class="entry-notes">{entry.notes}</p>
									{/if}
								</div>
							{/each}
						</div>
					{/if}
				{:else}
					<div class="entries-prompt">
						<p>Sélectionnez un jour dans le calendrier pour voir vos émotions.</p>
					</div>
				{/if}
			</section>
		</div>
	</div>

	<!-- Modal ajout émotion -->
	{#if showForm}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="modal-overlay" onclick={() => (showForm = false)}>
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="modal" onclick={(e) => e.stopPropagation()}>
				<h2 class="modal__title">Ajouter une émotion</h2>

				<form onsubmit={submitEntry} class="form">
					<label class="form-group">
						<span class="form-label">Émotion</span>
						<select class="form-input" bind:value={formFeelingId}>
							{#each feelings as feeling}
								<option value={feeling.id}>
									{feeling.name} ({feeling.feeling_category.name})
								</option>
							{/each}
						</select>
					</label>

					<label class="form-group">
						<span class="form-label">Début :</span>
						<input
							type="datetime-local"
							bind:value={formStart}
						/>
					</label>


					<label class="form-group">
						<span class="form-label">Fin :</span>
						<input
							type="datetime-local"
							bind:value={formEnd}
						/>
					</label>

					<label class="form-group">
						<span class="form-label">Intensité : {formIntensity}/10</span>
						<input
							type="range"
							min="1"
							max="10"
							bind:value={formIntensity}
							class="range-input"
						/>
					</label>

					<label class="form-group">
						<span class="form-label">Notes (optionnel)</span>
						<textarea
							class="form-input"
							rows="3"
							maxlength="2000"
							bind:value={formNotes}
							placeholder="Comment vous sentez-vous ?"
						></textarea>
					</label>

					<label class="form-group">
						<span class="form-label">Lieu (optionnel)</span>
						<input
							type="text"
							class="form-input"
							bind:value={formLocation}
							placeholder="Maison, bureau…"
						/>
					</label>

					<div class="modal__actions">
						<button type="button" class="btn btn--outline" onclick={() => (showForm = false)}>
							Annuler
						</button>
						<button type="submit" class="btn btn--primary" disabled={formSubmitting}>
							{formSubmitting ? 'Enregistrement…' : 'Ajouter'}
						</button>
					</div>
				</form>
			</div>
		</div>
	{/if}
{/if}

<style>
	.dashboard {
		max-width: 1000px;
		margin: 0 auto;
	}

	.dash-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		flex-wrap: wrap;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.dash-title {
		font-family: var(--font-heading);
		font-size: 2rem;
		font-weight: 800;
	}

	.dash-subtitle {
		color: var(--color-text-muted);
	}

	/* ── Indicateurs ──────────────────────────────────────── */
	.indicators {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.indicator-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 1.25rem;
		text-align: center;
	}

	.indicator-value {
		display: block;
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 800;
		color: var(--color-primary);
	}

	.indicator-label {
		font-size: 0.825rem;
		color: var(--color-text-muted);
	}

	/* ── Grille principale ────────────────────────────────── */
	.dash-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}

	@media (max-width: 768px) {
		.dash-grid { grid-template-columns: 1fr; }
	}

	/* ── Calendrier ───────────────────────────────────────── */
	.calendar-section {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 1.5rem;
	}

	.calendar-nav {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.cal-month {
		font-family: var(--font-heading);
		font-size: 1.1rem;
		font-weight: 700;
		text-transform: capitalize;
	}

	.cal-nav-btn {
		background: none;
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 0.35rem 0.75rem;
		cursor: pointer;
		font-size: 1rem;
		transition: all var(--transition);
	}

	.cal-nav-btn:hover { background: var(--color-bg); }

	.cal-header {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		text-align: center;
		margin-bottom: 0.5rem;
	}

	.cal-day-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--color-text-muted);
		text-transform: uppercase;
	}

	.cal-grid {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		gap: 2px;
	}

	.cal-day {
		aspect-ratio: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		border: none;
		background: none;
		border-radius: var(--radius);
		font-size: 0.875rem;
		cursor: pointer;
		position: relative;
		transition: all var(--transition);
		font-family: var(--font-body);
	}

	.cal-day:hover { background: var(--color-bg); }
	.cal-day.outside { color: var(--color-border); }
	.cal-day.selected { background: var(--color-primary); color: white; }
	.cal-day.has-entries { font-weight: 700; }

	.cal-dot {
		width: 5px;
		height: 5px;
		border-radius: 50%;
		background: var(--color-accent);
		position: absolute;
		bottom: 4px;
	}

	.cal-day.selected .cal-dot { background: white; }

	/* ── Entrées ──────────────────────────────────────────── */
	.entries-section {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 1.5rem;
		max-height: 500px;
		overflow-y: auto;
	}

	.entries-title {
		font-family: var(--font-heading);
		font-weight: 700;
		text-transform: capitalize;
		margin-bottom: 1rem;
	}

	.entries-prompt,
	.entries-empty {
		text-align: center;
		color: var(--color-text-muted);
		padding: 2rem 0;
	}

	.entries-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.entry-card {
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 1rem;
	}

	.entry-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.25rem;
	}

	.entry-feeling {
		font-weight: 700;
		font-size: 1rem;
	}

	.entry-category {
		font-size: 0.75rem;
		padding: 0.15rem 0.5rem;
		background: var(--color-bg);
		border-radius: 999px;
		color: var(--color-text-muted);
	}

	.entry-time {
		font-size: 0.8rem;
		color: var(--color-text-muted);
		margin-bottom: 0.5rem;
	}

	.entry-intensity {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.825rem;
		color: var(--color-text-muted);
		margin-bottom: 0.5rem;
	}

	.intensity-bar {
		flex: 1;
		height: 6px;
		background: var(--color-bg);
		border-radius: 3px;
		overflow: hidden;
	}

	.intensity-fill {
		height: 100%;
		background: var(--color-primary);
		border-radius: 3px;
		transition: width 0.3s ease;
	}

	.intensity-value { font-weight: 600; min-width: 3ch; text-align: right; }

	.entry-notes {
		font-size: 0.9rem;
		color: var(--color-text-muted);
		font-style: italic;
		border-top: 1px solid var(--color-border);
		padding-top: 0.5rem;
	}

	/* ── Modal ────────────────────────────────────────────── */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 200;
		padding: 1rem;
	}

	.modal {
		background: var(--color-surface);
		border-radius: var(--radius);
		padding: 2rem;
		width: 100%;
		max-width: 480px;
		box-shadow: var(--shadow-md);
	}

	.modal__title {
		font-family: var(--font-heading);
		font-size: 1.5rem;
		font-weight: 800;
		margin-bottom: 1.5rem;
	}

	.modal__actions {
		display: flex;
		gap: 0.75rem;
		justify-content: flex-end;
		margin-top: 0.5rem;
	}

	.form { display: flex; flex-direction: column; gap: 1.1rem; }
	.form-group { display: flex; flex-direction: column; gap: 0.35rem; }
	.form-label { font-weight: 600; font-size: 0.875rem; }

	.range-input {
		width: 100%;
		accent-color: var(--color-primary);
	}

	/* ── State ────────────────────────────────────────────── */
	.state-center {
		text-align: center;
		padding: 4rem;
		color: var(--color-text-muted);
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--color-border);
		border-top-color: var(--color-primary);
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
		margin: 0 auto 1rem;
	}

	@keyframes spin { to { transform: rotate(360deg); } }
</style>

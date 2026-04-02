<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAdmin } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import { toasts } from '$lib/stores/toasts';
	import { feelingsApi, feelingCategoriesApi, ApiError } from '$lib/api';
	import { api } from '$lib/api/client';
	import type { FeelingGet, FeelingCategoryGet } from '$lib/types';

	let categories = $state<FeelingCategoryGet[]>([]);
	let feelings = $state<FeelingGet[]>([]);
	let loading = $state(true);

	// Formulaires
	let newCategoryName = $state('');
	let newFeelingName = $state('');
	let newFeelingCategoryId = $state<number>(0);
	let addingCategory = $state(false);
	let addingFeeling = $state(false);

	// Regroupement
	let feelingsByCategory = $derived.by(() => {
		const map = new Map<number, { category: FeelingCategoryGet; feelings: FeelingGet[] }>();
		for (const cat of categories) {
			map.set(cat.id, { category: cat, feelings: [] });
		}
		for (const f of feelings) {
			const entry = map.get(f.feeling_category.id);
			if (entry) entry.feelings.push(f);
		}
		return [...map.values()];
	});

	onMount(() => {
		if (!get(isAdmin)) {
			goto('/');
			return;
		}
		loadData();
	});

	async function loadData() {
		loading = true;
		try {
			const [cats, feels] = await Promise.all([
				feelingCategoriesApi.search({ page_size: 100 }),
				feelingsApi.search({ page_size: 200 })
			]);
			categories = cats;
			feelings = feels;
			if (categories.length > 0 && newFeelingCategoryId === 0) {
				newFeelingCategoryId = categories[0].id;
			}
		} catch {
			toasts.error('Erreur de chargement.');
		} finally {
			loading = false;
		}
	}

	async function createCategory() {
		if (!newCategoryName.trim()) return;
		addingCategory = true;
		try {
			await api.post('/feeling_category', { name: newCategoryName.trim() });
			toasts.success('Catégorie créée !');
			newCategoryName = '';
			await loadData();
		} catch (err) {
			if (err instanceof ApiError) toasts.error('Erreur lors de la création.');
		} finally {
			addingCategory = false;
		}
	}

	async function deleteCategory(id: number) {
		if (!confirm('Supprimer cette catégorie et toutes ses émotions ?')) return;
		try {
			await api.delete(`/feeling_category/${id}`);
			toasts.success('Catégorie supprimée.');
			await loadData();
		} catch {
			toasts.error('Erreur lors de la suppression.');
		}
	}

	async function createFeeling() {
		if (!newFeelingName.trim() || newFeelingCategoryId === 0) return;
		addingFeeling = true;
		try {
			await api.post('/feeling', {
				feeling_category_id: newFeelingCategoryId,
				name: newFeelingName.trim()
			});
			toasts.success('Émotion créée !');
			newFeelingName = '';
			await loadData();
		} catch (err) {
			if (err instanceof ApiError) toasts.error('Erreur lors de la création.');
		} finally {
			addingFeeling = false;
		}
	}

	async function deleteFeeling(id: number) {
		if (!confirm('Supprimer cette émotion ?')) return;
		try {
			await api.delete(`/feeling/${id}`);
			toasts.success('Émotion supprimée.');
			feelings = feelings.filter((f) => f.id !== id);
		} catch {
			toasts.error('Erreur lors de la suppression.');
		}
	}
</script>

<svelte:head>
	<title>Admin — Émotions — CESIZen</title>
</svelte:head>

<div class="admin-page">
	<h1 class="page-title">Référentiel d'émotions</h1>

	{#if loading}
		<div class="state-center"><div class="spinner"></div></div>
	{:else}
		<!-- Ajout catégorie -->
		<div class="add-section">
			<h2 class="section-title">Ajouter une catégorie</h2>
			<div class="inline-form">
				<input
					type="text"
					class="form-input"
					placeholder="Nom de la catégorie…"
					bind:value={newCategoryName}
				/>
				<button class="btn btn--primary btn--sm" onclick={createCategory} disabled={addingCategory}>
					{addingCategory ? '…' : 'Ajouter'}
				</button>
			</div>
		</div>

		<!-- Ajout émotion -->
		<div class="add-section">
			<h2 class="section-title">Ajouter une émotion</h2>
			<div class="inline-form">
				<select class="form-input" bind:value={newFeelingCategoryId}>
					{#each categories as cat}
						<option value={cat.id}>{cat.name}</option>
					{/each}
				</select>
				<input
					type="text"
					class="form-input"
					placeholder="Nom de l'émotion…"
					bind:value={newFeelingName}
					minlength="4"
				/>
				<button class="btn btn--primary btn--sm" onclick={createFeeling} disabled={addingFeeling}>
					{addingFeeling ? '…' : 'Ajouter'}
				</button>
			</div>
		</div>

		<!-- Liste par catégorie -->
		<div class="feelings-grid">
			{#each feelingsByCategory as group (group.category.id)}
				<div class="category-card">
					<div class="category-header">
						<h3 class="category-name">{group.category.name}</h3>
						<button
							class="btn btn--danger btn--sm"
							onclick={() => deleteCategory(group.category.id)}
						>
							Suppr. catégorie
						</button>
					</div>

					{#if group.feelings.length === 0}
						<p class="empty-text">Aucune émotion dans cette catégorie.</p>
					{:else}
						<div class="feelings-list">
							{#each group.feelings as feeling (feeling.id)}
								<div class="feeling-item">
									<span class="feeling-name">{feeling.name}</span>
									<button
										class="delete-btn"
										onclick={() => deleteFeeling(feeling.id)}
										title="Supprimer"
									>
										×
									</button>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.admin-page { max-width: 900px; margin: 0 auto; }

	.page-title {
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 800;
		margin-bottom: 1.5rem;
	}

	.add-section {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 1.25rem;
		margin-bottom: 1rem;
	}

	.section-title {
		font-family: var(--font-heading);
		font-size: 1rem;
		font-weight: 700;
		margin-bottom: 0.75rem;
	}

	.inline-form {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.inline-form .form-input { flex: 1; min-width: 160px; }

	/* ── Grille catégories ────────────────────────────────── */
	.feelings-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
		margin-top: 1.5rem;
	}

	.category-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 1.25rem;
	}

	.category-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.category-name {
		font-family: var(--font-heading);
		font-weight: 700;
		font-size: 1.1rem;
	}

	.empty-text {
		font-size: 0.85rem;
		color: var(--color-text-muted);
		font-style: italic;
	}

	.feelings-list {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
	}

	.feeling-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0.75rem;
		background: var(--color-bg);
		border-radius: var(--radius);
	}

	.feeling-name {
		font-weight: 500;
		font-size: 0.9rem;
	}

	.delete-btn {
		background: none;
		border: none;
		color: var(--color-danger);
		font-size: 1.25rem;
		cursor: pointer;
		opacity: 0.6;
		transition: opacity var(--transition);
		line-height: 1;
	}
	.delete-btn:hover { opacity: 1; }

	.state-center { text-align: center; padding: 3rem; }
	.spinner {
		width: 32px; height: 32px;
		border: 3px solid var(--color-border);
		border-top-color: var(--color-primary);
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
		margin: 0 auto;
	}
	@keyframes spin { to { transform: rotate(360deg); } }
</style>

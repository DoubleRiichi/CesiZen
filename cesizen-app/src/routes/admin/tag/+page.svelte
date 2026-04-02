<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAdmin } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import { toasts } from '$lib/stores/toasts';
	import { ApiError, tagsApi, articlesApi } from '$lib/api';
	import { api } from '$lib/api/client';
	import {  type TagGet } from '$lib/types';

	let all_tags = $state<TagGet[]>([]);
    let loading = $state(true);


	// Formulaires
	let newTagName = $state('');
	let addingTag = $state(false);


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
			all_tags = await tagsApi.getAll();
			
		} catch {
			toasts.error('Erreur de chargement.');
		} finally {
			loading = false;
		}
	}

	async function createTag() {
		if (!newTagName.trim()) return;
		addingTag = true;
		try {
			await api.post('/tag', { name: newTagName.trim() });
			toasts.success('Catégorie créée !');
			newTagName = '';
			await loadData();
		} catch (err) {
			if (err instanceof ApiError) toasts.error('Erreur lors de la création.');
		} finally {
			addingTag = false;
		}
	}

	async function deleteCategory(id: number) {
		if (!confirm('Supprimer cette catégorie ? (peut impacter les articles existants')) return;
		try {
			await api.delete(`/tag/${id}`);
			toasts.success('Catégorie supprimée.');
			await loadData();
		} catch {
			toasts.error('Erreur lors de la suppression.');
		}
	}

 /*   async function loadArticlesWithCategory(tag: TagGet) {
        try {
            
            articles = await articlesApi.search({tag_ids: [tag.id]})
        } catch (err) {
            if (err instanceof ApiError) toasts.error(`Erreur lors de la récupération des articles avec la catégorie ${tag.name}`)
        }
    }*/

</script>

<svelte:head>
	<title>Admin — Catégories d'Articles — CESIZen</title>
</svelte:head>

<div class="admin-page">
	<h1 class="page-title">Référentiel de catégories d'articles</h1>

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
					bind:value={newTagName}
				/>
				<button class="btn btn--primary btn--sm" onclick={createTag} disabled={addingTag}>
					{addingTag ? '…' : 'Ajouter'}
				</button>
			</div>
		</div>

		

		<!-- Liste par catégorie -->
				<div class="category-card">	
						<div class="feelings-list">
							{#each all_tags as tag (tag.id)}

                            {#if all_tags.length == 0}
                                <p class="empty-text">Aucune catégorie d'articles.</p>
                            {:else}
								<div class="feeling-item">
									<span class="feeling-name">{tag.name}</span>
									<button
										class="delete-btn"
										onclick={() => deleteCategory(tag.id)}
										title="Supprimer"
									>
										×
									</button>
								</div>
                            {/if}
							{/each}
						</div>
				</div>

        <!-- Articles avec ce tag -->


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

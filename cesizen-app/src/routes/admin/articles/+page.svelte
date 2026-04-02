<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAdmin, user as userStore } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import { toasts } from '$lib/stores/toasts';
	import { articlesApi, tagsApi, ApiError } from '$lib/api';
	import type { ArticleGet, TagGet } from '$lib/types';

	let articles = $state<ArticleGet[]>([]);
	let tags = $state<TagGet[]>([]);
	let loading = $state(true);
	let showCreateForm = $state(false);

	// Formulaire création
	let formTitle = $state('');
	let formContent = $state('');
	let formVisibility = $state('Public');
	let formTags = $state<number[]>([]);
	let formSubmitting = $state(false);

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
			const [arts, allTags] = await Promise.all([
				articlesApi.search({ page_size: 100 }),
				tagsApi.getAll().catch(() => [])
			]);
			articles = arts;
			tags = allTags as TagGet[];
		} catch {
			toasts.error('Erreur de chargement.');
		} finally {
			loading = false;
		}
	}

	async function handleCreate(e: Event) {
		e.preventDefault();
		if (!get(userStore)) return;
		formSubmitting = true;

		try {
			await articlesApi.create({
				author_id: get(userStore)!.id,
				title: formTitle,
				content: formContent,
				visibility: formVisibility,
				tags: formTags
			});
			toasts.success('Article créé !');
			showCreateForm = false;
			formTitle = '';
			formContent = '';
			formTags = [];
			await loadData();
		} catch (err) {
			if (err instanceof ApiError) {
				const body = err.body as any;
				toasts.error(typeof body === 'string' ? body : 'Erreur lors de la création.');
			}
		} finally {
			formSubmitting = false;
		}
	}

	async function deleteArticle(id: number) {
		if (!confirm('Supprimer cet article ?')) return;
		try {
			await articlesApi.delete(id);
			toasts.success('Article supprimé.');
			articles = articles.filter((a) => a.id !== id);
		} catch {
			toasts.error('Erreur lors de la suppression.');
		}
	}

	function toggleTag(tagId: number) {
		if (formTags.includes(tagId)) {
			formTags = formTags.filter((t) => t !== tagId);
		} else {
			formTags = [...formTags, tagId];
		}
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric' });
	}
</script>

<svelte:head>
	<title>Admin — Articles — CESIZen</title>
</svelte:head>

<div class="admin-page">
	<header class="admin-header">
		<h1 class="page-title">Gestion des articles</h1>
		<button class="btn btn--primary" onclick={() => (showCreateForm = !showCreateForm)}>
			{showCreateForm ? 'Annuler' : '+ Nouvel article'}
		</button>
	</header>

	<!-- Formulaire de création -->
	{#if showCreateForm}
		<div class="create-form-card">
			<h2 class="form-card-title">Créer un article</h2>
			<form onsubmit={handleCreate} class="form">
				<label class="form-group">
					<span class="form-label">Titre (min. 10 caractères)</span>
					<input type="text" class="form-input" bind:value={formTitle} minlength="10" maxlength="256" required />
				</label>

				<label class="form-group">
					<span class="form-label">Contenu (min. 300 caractères)</span>
					<textarea class="form-input" rows="8" bind:value={formContent} minlength="300" required></textarea>
					<span class="form-hint">{formContent.length}/300 min</span>
				</label>

				<label class="form-group">
					<span class="form-label">Visibilité</span>
					<select class="form-input" bind:value={formVisibility}>
						<option value="Public">Public</option>
						<option value="Private">Privé</option>
						<option value="Unlisted">Non listé</option>
					</select>
				</label>

				<div class="form-group">
					<span class="form-label">Tags</span>
					<div class="tag-selector">
						{#each tags as tag}
							<button
								type="button"
								class="tag-chip"
								class:active={formTags.includes(tag.id)}
								onclick={() => toggleTag(tag.id)}
							>
								{tag.name}
							</button>
						{/each}
					</div>
				</div>

				<button type="submit" class="btn btn--primary" disabled={formSubmitting}>
					{formSubmitting ? 'Création…' : 'Publier'}
				</button>
			</form>
		</div>
	{/if}

	<!-- Liste des articles -->
	{#if loading}
		<div class="state-center"><div class="spinner"></div></div>
	{:else}
		<div class="table-wrapper">
			<table class="data-table">
				<thead>
					<tr>
						<th>ID</th>
						<th>Titre</th>
						<th>Auteur</th>
						<th>Visibilité</th>
						<th>Date</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each articles as article (article.id)}
						<tr class:deleted={article.is_deleted}>
							<td>{article.id}</td>
							<td class="title-cell">
								<a href="/articles/{article.id}" class="table-link">{article.title}</a>
							</td>
							<td>{article.author.username}</td>
							<td>
								<span class="visibility-badge visibility--{article.visibility.toLowerCase()}">
									{article.visibility}
								</span>
							</td>
							<td>{formatDate(article.created_at)}</td>
							<td class="actions-cell">
								<a href="/admin/articles/{article.id}" class="btn btn--outline btn--sm">Éditer</a>
								<button class="btn btn--danger btn--sm" onclick={() => deleteArticle(article.id)}>
									Suppr.
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style>
	.admin-page { max-width: 1100px; margin: 0 auto; }

	.admin-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.page-title {
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 800;
	}

	.create-form-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 2rem;
		margin-bottom: 2rem;
	}

	.form-card-title {
		font-family: var(--font-heading);
		font-weight: 700;
		margin-bottom: 1.25rem;
	}

	.form { display: flex; flex-direction: column; gap: 1.1rem; }
	.form-group { display: flex; flex-direction: column; gap: 0.35rem; }
	.form-label { font-weight: 600; font-size: 0.875rem; }
	.form-hint { font-size: 0.75rem; color: var(--color-text-muted); }

	.tag-selector { display: flex; gap: 0.35rem; flex-wrap: wrap; }

	.tag-chip {
		padding: 0.3rem 0.75rem;
		border-radius: 999px;
		border: 1px solid var(--color-border);
		background: var(--color-surface);
		font-size: 0.8rem;
		font-family: var(--font-body);
		cursor: pointer;
		transition: all var(--transition);
	}
	.tag-chip.active { background: var(--color-primary); color: white; border-color: var(--color-primary); }

	/* ── Table ────────────────────────────────────────────── */
	.table-wrapper {
		overflow-x: auto;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		overflow: hidden;
	}

	.data-table th {
		background: var(--color-bg);
		font-size: 0.8rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.03em;
		color: var(--color-text-muted);
		padding: 0.75rem 1rem;
		text-align: left;
	}

	.data-table td {
		padding: 0.75rem 1rem;
		font-size: 0.9rem;
		border-top: 1px solid var(--color-border);
		vertical-align: middle;
	}

	.data-table tr.deleted { opacity: 0.4; }

	.title-cell { max-width: 260px; }

	.table-link {
		color: var(--color-primary);
		font-weight: 600;
	}
	.table-link:hover { text-decoration: underline; }

	.visibility-badge {
		font-size: 0.7rem;
		padding: 0.15rem 0.5rem;
		border-radius: 999px;
		font-weight: 600;
		text-transform: uppercase;
	}
	.visibility--public { background: rgba(39, 174, 96, 0.1); color: var(--color-success); }
	.visibility--private { background: rgba(192, 57, 43, 0.1); color: var(--color-danger); }
	.visibility--unlisted { background: rgba(243, 156, 18, 0.1); color: var(--color-warning); }

	.actions-cell {
		display: flex;
		gap: 0.35rem;
		flex-wrap: nowrap;
	}

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

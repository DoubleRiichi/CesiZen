<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAdmin } from '$lib/stores/auth';
	import { get } from 'svelte/store';

	import { toasts } from '$lib/stores/toasts';
	import { articlesApi, tagsApi, ApiError } from '$lib/api';
	import type { ArticleGet, TagGet } from '$lib/types';

	let article = $state<ArticleGet | null>(null);
	let tags = $state<TagGet[]>([]);
	let loading = $state(true);

	let formTitle = $state('');
	let formContent = $state('');
	let formVisibility = $state('Public');
	let formTags = $state<number[]>([]);
	let saving = $state(false);

	onMount(async () => {
		if (!get(isAdmin)) {
			goto('/');
			return;
		}

		const id = Number($page.params.id);
		if (isNaN(id)) {
			toasts.error('ID invalide.');
			goto('/admin/articles');
			return;
		}

		try {
			const [art, allTags] = await Promise.all([
				articlesApi.getById(id),
				tagsApi.getAll().catch(() => [])
			]);
			article = art;
			tags = allTags as TagGet[];
			formTitle = art.title;
			formContent = art.content;
			formVisibility = art.visibility;
			// Résoudre les tag IDs à partir des noms
			formTags = (allTags as TagGet[])
				.filter((t) => art.tags.includes(t.name))
				.map((t) => t.id);
		} catch {
			toasts.error('Article introuvable.');
			goto('/admin/articles');
		} finally {
			loading = false;
		}
	});

	async function handleSave(e: Event) {
		e.preventDefault();
		if (!article) return;
		saving = true;

		try {
			await articlesApi.update(article.id, {
				title: formTitle,
				content: formContent,
				visibility: formVisibility,
				tags: formTags
			});
			toasts.success('Article mis à jour !');
			goto('/admin/articles');
		} catch (err) {
			if (err instanceof ApiError) {
				toasts.error('Erreur lors de la mise à jour.');
			}
		} finally {
			saving = false;
		}
	}

	function toggleTag(tagId: number) {
		if (formTags.includes(tagId)) {
			formTags = formTags.filter((t) => t !== tagId);
		} else {
			formTags = [...formTags, tagId];
		}
	}
</script>

<svelte:head>
	<title>Éditer article — CESIZen Admin</title>
</svelte:head>

<div class="admin-page">
	<a href="/admin/articles" class="back-link">← Retour à la liste</a>

	{#if loading}
		<div class="state-center"><div class="spinner"></div></div>
	{:else if article}
		<h1 class="page-title">Éditer l'article #{article.id}</h1>

		<div class="edit-card">
			<form onsubmit={handleSave} class="form">
				<label class="form-group">
					<span class="form-label">Titre</span>
					<input type="text" class="form-input" bind:value={formTitle} minlength="10" maxlength="256" required />
				</label>

				<label class="form-group">
					<span class="form-label">Contenu</span>
					<textarea class="form-input" rows="12" bind:value={formContent} minlength="300" required></textarea>
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

				<div class="form-actions">
					<a href="/admin/articles" class="btn btn--outline">Annuler</a>
					<button type="submit" class="btn btn--primary" disabled={saving}>
						{saving ? 'Sauvegarde…' : 'Sauvegarder'}
					</button>
				</div>
			</form>
		</div>
	{/if}
</div>

<style>
	.admin-page { max-width: 800px; margin: 0 auto; }

	.back-link {
		display: inline-block;
		color: var(--color-primary);
		font-weight: 600;
		font-size: 0.9rem;
		margin-bottom: 1rem;
	}
	.back-link:hover { text-decoration: underline; }

	.page-title {
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 800;
		margin-bottom: 1.5rem;
	}

	.edit-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 2rem;
	}

	.form { display: flex; flex-direction: column; gap: 1.1rem; }
	.form-group { display: flex; flex-direction: column; gap: 0.35rem; }
	.form-label { font-weight: 600; font-size: 0.875rem; }

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

	.form-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: flex-end;
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

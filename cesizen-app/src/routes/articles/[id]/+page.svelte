<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { articlesApi } from '$lib/api';
	import { isAdmin } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import type { ArticleGet } from '$lib/types';

	let article = $state<ArticleGet | null>(null);
	let loading = $state(true);
	let error = $state('');

	onMount(async () => {
		const id = Number($page.params.id);
		if (isNaN(id)) {
			error = 'ID d\'article invalide.';
			loading = false;
			return;
		}

		try {
			article = await articlesApi.getById(id);
		} catch {
			error = 'Article introuvable.';
		} finally {
			loading = false;
		}
	});

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString('fr-FR', {
			day: 'numeric',
			month: 'long',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
</script>

<svelte:head>
	<title>{article?.title ?? 'Article'} — CESIZen</title>
</svelte:head>

{#if loading}
	<div class="state-container">
		<div class="spinner"></div>
		<p>Chargement…</p>
	</div>
{:else if error}
	<div class="state-container">
		<p class="error-text">{error}</p>
		<a href="/articles" class="btn btn--outline">Retour aux articles</a>
	</div>
{:else if article}
	<article class="article-detail">
		<a href="/articles" class="back-link">← Retour aux articles</a>

		<header class="article-header">
			<div class="article-tags">
				{#each article.tags as tag}
					<span class="tag">{tag}</span>
				{/each}
			</div>
			<div>
				{#if get(isAdmin)}
				<a class="btn btn--primary" href="/admin/articles/{article.id}">Editer</a>
				{/if}

			</div>
			<h1 class="article-title">{article.title}</h1>
			<div class="article-meta">
				<span class="article-author">Par {article.author.username}</span>
				<span class="article-date">Publié le {formatDate(article.created_at)}</span>
				{#if article.updated_at !== article.created_at}
					<span class="article-date">Mis à jour le {formatDate(article.updated_at)}</span>
				{/if}
			</div>
		</header>

		<div class="article-body">
			{@html article.content.replace(/\n/g, '<br />')}
		</div>
	</article>
{/if}

<style>
	.article-detail {
		max-width: 720px;
		margin: 0 auto;
	}

	.back-link {
		display: inline-block;
		color: var(--color-primary);
		font-weight: 600;
		font-size: 0.9rem;
		margin-bottom: 1.5rem;
	}
	.back-link:hover { text-decoration: underline; }

	.article-header {
		margin-bottom: 2rem;
	}

	.article-tags {
		display: flex;
		gap: 0.35rem;
		margin-bottom: 0.75rem;
		flex-wrap: wrap;
	}

	.tag {
		font-size: 0.7rem;
		padding: 0.2rem 0.6rem;
		background: rgba(42, 125, 110, 0.1);
		color: var(--color-primary);
		border-radius: 999px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.article-title {
		font-family: var(--font-heading);
		font-size: clamp(1.5rem, 4vw, 2.25rem);
		font-weight: 800;
		line-height: 1.2;
		margin-bottom: 0.75rem;
	}

	.article-meta {
		display: flex;
		gap: 1.5rem;
		flex-wrap: wrap;
		font-size: 0.875rem;
		color: var(--color-text-muted);
	}

	.article-author { font-weight: 600; }

	.article-body {
		line-height: 1.8;
		font-size: 1.05rem;
		color: var(--color-text);
	}

	.state-container {
		text-align: center;
		padding: 3rem;
		color: var(--color-text-muted);
	}

	.error-text {
		color: var(--color-danger);
		margin-bottom: 1rem;
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

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>

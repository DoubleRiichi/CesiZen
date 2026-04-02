<script lang="ts">
	import { onMount } from 'svelte';
	import { articlesApi, tagsApi, ApiError } from '$lib/api';
	import type { ArticleGet, TagGet } from '$lib/types';

	let articles = $state<ArticleGet[]>([]);
	let tags = $state<TagGet[]>([]);
	let loading = $state(true);

	// Filtres
	let searchTitle = $state('');
	let selectedTagId = $state<number | null>(null);
	let sortNewest = $state(true);

	// Pagination curseur
	let hasMore = $state(true);
	let loadingMore = $state(false);

	const PAGE_SIZE = 12;

	async function fetchArticles(append = false) {
		if (append) loadingMore = true;
		else loading = true;

		try {
			const params: Record<string, any> = { page_size: PAGE_SIZE, visibility: "Public" };
			if (searchTitle.trim()) params.title = searchTitle.trim();
			if (selectedTagId) params.tag_ids = [selectedTagId];
			if (append && articles.length > 0) {
				params.cursor = articles[articles.length - 1].created_at;
			}

			const result = await articlesApi.search(params);

			if (append) {
				articles = [...articles, ...result];
			} else {
				articles = result;
			}
			hasMore = result.length === PAGE_SIZE;
		} catch (err) {
			console.error('Erreur chargement articles:', err);
		} finally {
			loading = false;
			loadingMore = false;
		}
	}

	onMount(async () => {
		const [_, allTags] = await Promise.all([fetchArticles(), tagsApi.getAll().catch(() => [])]);
		tags = allTags as TagGet[];
	});

	function handleSearch() {
		fetchArticles();
	}

	function handleTagFilter(tagId: number | null) {
		selectedTagId = tagId;
		fetchArticles();
	}

	function loadMore() {
		fetchArticles(true);
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString('fr-FR', {
			day: 'numeric',
			month: 'long',
			year: 'numeric'
		});
	}

	let displayedArticles = $derived(
		sortNewest
			? [...articles]
			: [...articles].reverse()
	);
</script>

<svelte:head>
	<title>Informations — CESIZen</title>
</svelte:head>

<div class="articles-page">
	<header class="page-header">
		<h1 class="page-title">Informations & Articles</h1>
		<p class="page-subtitle">Contenus validés sur la santé mentale et la prévention du stress</p>
	</header>

	<!-- Barre de recherche & filtres -->
	<div class="toolbar">
		<div class="search-bar">
			<input
				type="search"
				class="form-input search-input"
				placeholder="Rechercher un article…"
				bind:value={searchTitle}
				onkeydown={(e) => e.key === 'Enter' && handleSearch()}
			/>
			<button class="btn btn--primary btn--sm" onclick={handleSearch}>Rechercher</button>
		</div>

		<div class="filters">
			<div class="tag-filters">
				<button
					class="tag-chip"
					class:active={selectedTagId === null}
					onclick={() => handleTagFilter(null)}
				>
					Tous
				</button>
				{#each tags as tag}
					<button
						class="tag-chip"
						class:active={selectedTagId === tag.id}
						onclick={() => handleTagFilter(tag.id)}
					>
						{tag.name}
					</button>
				{/each}
			</div>

			<div class="sort-toggle">
				<button
					class="sort-btn"
					class:active={sortNewest}
					onclick={() => (sortNewest = true)}
				>
					Récents
				</button>
				<button
					class="sort-btn"
					class:active={!sortNewest}
					onclick={() => (sortNewest = false)}
				>
					Anciens
				</button>
			</div>
		</div>
	</div>

	<!-- Liste d'articles -->
	{#if loading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Chargement des articles…</p>
		</div>
	{:else if displayedArticles.length === 0}
		<div class="empty-state">
			<p>Aucun article trouvé.</p>
		</div>
	{:else}
		<div class="articles-grid">
			{#each displayedArticles as article (article.id)}
				<a href="/articles/{article.id}" class="article-card">
					<div class="article-card__tags">
						{#each article.tags as tag}
							<span class="article-tag">{tag}</span>
						{/each}
					</div>
					<h2 class="article-card__title">{article.title}</h2>
					<p class="article-card__excerpt">
						{article.content.slice(0, 160)}…
					</p>
					<div class="article-card__meta">
						<span class="article-card__author">{article.author.username}</span>
						<span class="article-card__date">{formatDate(article.created_at)}</span>
					</div>
				</a>
			{/each}
		</div>

		{#if hasMore}
			<div class="load-more">
				<button class="btn btn--outline" onclick={loadMore} disabled={loadingMore}>
					{loadingMore ? 'Chargement…' : 'Charger plus'}
				</button>
			</div>
		{/if}
	{/if}
</div>

<style>
	.articles-page {
		max-width: 960px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.page-title {
		font-family: var(--font-heading);
		font-size: 2rem;
		font-weight: 800;
	}

	.page-subtitle {
		color: var(--color-text-muted);
		margin-top: 0.25rem;
	}

	/* ── Toolbar ──────────────────────────────────────────── */
	.toolbar {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.search-bar {
		display: flex;
		gap: 0.5rem;
	}

	.search-input { flex: 1; }

	.filters {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 0.75rem;
	}

	.tag-filters {
		display: flex;
		gap: 0.35rem;
		flex-wrap: wrap;
	}

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

	.tag-chip:hover {
		border-color: var(--color-primary);
	}

	.tag-chip.active {
		background: var(--color-primary);
		color: white;
		border-color: var(--color-primary);
	}

	.sort-toggle {
		display: flex;
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		overflow: hidden;
	}

	.sort-btn {
		padding: 0.35rem 0.75rem;
		border: none;
		background: var(--color-surface);
		font-family: var(--font-body);
		font-size: 0.825rem;
		cursor: pointer;
		transition: all var(--transition);
	}

	.sort-btn.active {
		background: var(--color-primary);
		color: white;
	}

	/* ── Grid ─────────────────────────────────────────────── */
	.articles-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1.25rem;
	}

	.article-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		transition: box-shadow var(--transition), transform var(--transition);
	}

	.article-card:hover {
		box-shadow: var(--shadow-md);
		transform: translateY(-2px);
	}

	.article-card__tags {
		display: flex;
		gap: 0.35rem;
		flex-wrap: wrap;
	}

	.article-tag {
		font-size: 0.7rem;
		padding: 0.15rem 0.5rem;
		background: rgba(42, 125, 110, 0.1);
		color: var(--color-primary);
		border-radius: 999px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.article-card__title {
		font-family: var(--font-heading);
		font-size: 1.15rem;
		font-weight: 700;
		line-height: 1.3;
	}

	.article-card__excerpt {
		color: var(--color-text-muted);
		font-size: 0.9rem;
		line-height: 1.5;
		flex: 1;
	}

	.article-card__meta {
		display: flex;
		justify-content: space-between;
		font-size: 0.8rem;
		color: var(--color-text-muted);
		border-top: 1px solid var(--color-border);
		padding-top: 0.75rem;
	}

	.article-card__author { font-weight: 600; }

	/* ── States ───────────────────────────────────────────── */
	.loading-state,
	.empty-state {
		text-align: center;
		padding: 3rem 1rem;
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

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.load-more {
		text-align: center;
		margin-top: 2rem;
	}

	@media (max-width: 640px) {
		.filters { flex-direction: column; align-items: flex-start; }
	}
</style>

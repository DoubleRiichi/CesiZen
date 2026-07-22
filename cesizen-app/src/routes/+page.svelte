<script lang="ts">
	import { isAuthenticated } from '$lib/stores/auth';
</script>

<svelte:head>
	<title>CESIZen : Santé mentale et bien-être</title>
</svelte:head>

<section class="hero">
	<div class="hero__content">
		<h1 class="hero__title">
			Prenez soin de votre <span class="highlight">santé mentale</span>
		</h1>
		<p class="hero__subtitle">
			CESIZen est un service proposé par le Ministère pour vous accompagner au quotidien avec des articles, un suivi émotionnel
			personnalisé et des outils pour mieux gérer votre stress. 
		</p>
		<div class="hero__actions">
			{#if $isAuthenticated}
				<a href="/dashboard" class="btn btn--primary">Mon Dashboard</a>
				<a href="/articles" class="btn btn--outline">Parcourir les articles</a>
			{:else}
				<a href="/register" class="btn btn--primary">Commencer gratuitement</a>
				<a href="/articles" class="btn btn--outline">Découvrir</a>
			{/if}
		</div>
	</div>
</section>

<section class="features">
	<div class="feature-card">
		<h3 class="feature-card__title">Articles & Informations</h3>
		<p class="feature-card__text">
			Accédez à des contenus validés sur la santé mentale, le stress et la prévention.
		</p>
	</div>

	<div class="feature-card">
		<h3 class="feature-card__title">Tracker d'émotions</h3>
		<p class="feature-card__text">
			Suivez vos émotions au quotidien et visualisez votre évolution sur la durée.
		</p>
	</div>

	<div class="feature-card">
		<h3 class="feature-card__title">Données sécurisées</h3>
		<p class="feature-card__text">
			Vos données sont protégées selon les normes RGPD. Rien ne sort de l'UE.
		</p>
	</div>
</section>

<style>
	.hero {
		text-align: center;
		padding: 4rem 1rem;
	}

	.hero__title {
		font-family: var(--font-heading);
		font-size: clamp(2rem, 5vw, 3.25rem);
		font-weight: 800;
		line-height: 1.15;
		margin-bottom: 1.25rem;
	}

	.highlight {
		color: var(--color-primary);
	}

	.hero__subtitle {
		max-width: 600px;
		margin: 0 auto 2rem;
		color: var(--color-text-muted);
		font-size: 1.1rem;
	}

	.hero__actions {
		display: flex;
		gap: 0.75rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	/* ── Features ─────────────────────────────────────────── */
	.features {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.feature-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 2rem;
		text-align: center;
		transition: box-shadow var(--transition);
	}

	.feature-card:hover {
		box-shadow: var(--shadow-md);
	}

	.feature-card__icon {
		font-size: 2.5rem;
		margin-bottom: 1rem;
	}

	.feature-card__title {
		font-family: var(--font-heading);
		font-size: 1.2rem;
		font-weight: 700;
		margin-bottom: 0.5rem;
	}

	.feature-card__text {
		color: var(--color-text-muted);
		font-size: 0.95rem;
	}
</style>

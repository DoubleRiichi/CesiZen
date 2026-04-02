<script lang="ts">
	import { goto } from '$app/navigation';
	import { authApi, ApiError } from '$lib/api';
	import { login as authLogin } from '$lib/stores/auth';
	import { toasts } from '$lib/stores/toasts';

	let email = $state('');
	let password = $state('');
	let loading = $state(false);
	let errorMsg = $state('');

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errorMsg = '';
		loading = true;

		try {
			const res = await authApi.login({ email, password });
			authLogin(res.token, res.user);
			toasts.success(`Bienvenue, ${res.user.username} !`);
			goto('/');
		} catch (err) {
			if (err instanceof ApiError) {
				errorMsg = err.status === 400 || err.status === 422
					? 'Email ou mot de passe invalide.'
					: 'Erreur serveur, veuillez réessayer.';
			} else {
				errorMsg = 'Impossible de contacter le serveur.';
			}
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Connexion — CESIZen</title>
</svelte:head>

<div class="auth-page">
	<div class="auth-card">
		<h1 class="auth-card__title">Se connecter</h1>
		<p class="auth-card__subtitle">Accédez à votre espace CESIZen</p>

		{#if errorMsg}
			<div class="alert alert--error">{errorMsg}</div>
		{/if}

		<form onsubmit={handleSubmit} class="form">
			<label class="form-group">
				<span class="form-label">Email</span>
				<input
					type="email"
					class="form-input"
					bind:value={email}
					placeholder="vous@exemple.com"
					required
					autocomplete="email"
				/>
			</label>

			<label class="form-group">
				<span class="form-label">Mot de passe</span>
				<input
					type="password"
					class="form-input"
					bind:value={password}
					placeholder="••••••••"
					required
					autocomplete="current-password"
				/>
			</label>

			<button type="submit" class="btn btn--primary btn--full" disabled={loading}>
				{loading ? 'Connexion…' : 'Se connecter'}
			</button>
		</form>

		<p class="auth-card__footer">
			Pas encore de compte ?
			<a href="/register" class="link">S'inscrire</a>
		</p>
	</div>
</div>

<style>
	.auth-page {
		display: flex;
		justify-content: center;
		align-items: flex-start;
		padding-top: 4rem;
	}

	.auth-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 2.5rem;
		width: 100%;
		max-width: 440px;
		box-shadow: var(--shadow-sm);
	}

	.auth-card__title {
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 800;
		margin-bottom: 0.25rem;
	}

	.auth-card__subtitle {
		color: var(--color-text-muted);
		margin-bottom: 1.5rem;
	}

	.auth-card__footer {
		margin-top: 1.5rem;
		text-align: center;
		font-size: 0.9rem;
		color: var(--color-text-muted);
	}

	.link {
		color: var(--color-primary);
		font-weight: 600;
	}
	.link:hover { text-decoration: underline; }

	/* ── Form elements (réutilisables) ───────────────────── */
	.form { display: flex; flex-direction: column; gap: 1.25rem; }

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
	}

	.form-label {
		font-weight: 600;
		font-size: 0.875rem;
	}

	:global(.form-input) {
		padding: 0.65rem 0.875rem;
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		font-size: 0.95rem;
		font-family: var(--font-body);
		transition: border-color var(--transition);
		background: var(--color-bg);
	}
	:global(.form-input:focus) {
		outline: none;
		border-color: var(--color-primary);
		box-shadow: 0 0 0 3px rgba(42, 125, 110, 0.15);
	}

	:global(.btn--full) { width: 100%; }

	.alert {
		padding: 0.75rem 1rem;
		border-radius: var(--radius);
		font-size: 0.9rem;
		margin-bottom: 0.5rem;
	}

	.alert--error {
		background: rgba(192, 57, 43, 0.08);
		color: var(--color-danger);
		border: 1px solid rgba(192, 57, 43, 0.2);
	}
</style>

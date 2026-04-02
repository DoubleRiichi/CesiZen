<script lang="ts">
	import { goto } from '$app/navigation';
	import { authApi, ApiError } from '$lib/api';
	import { toasts } from '$lib/stores/toasts';

	let username = $state('');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let age = $state(18);
	let avatar = $state('default');
	let loading = $state(false);
	let errorMsg = $state('');

	// Validations côté front (miroir des contraintes Rust)
	let errors = $derived.by(() => {
		const e: Record<string, string> = {};
		if (username.length > 0 && (username.length < 4 || username.length > 20))
			e.username = 'Entre 4 et 20 caractères';
		if (password.length > 0 && (password.length < 8 || password.length > 32))
			e.password = 'Entre 8 et 32 caractères';
		if (confirmPassword.length > 0 && password !== confirmPassword)
			e.confirmPassword = 'Les mots de passe ne correspondent pas';
		if (age < 13 || age > 120)
			e.age = 'Âge entre 13 et 120 ans';
		return e;
	});

	let canSubmit = $derived(
		username.length >= 4 &&
		email.length > 0 &&
		password.length >= 8 &&
		password === confirmPassword &&
		age >= 13 &&
		Object.keys(errors).length === 0
	);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!canSubmit) return;
		errorMsg = '';
		loading = true;

		try {
			await authApi.register({ username, password, email, avatar, age });
			toasts.success('Compte créé avec succès ! Connectez-vous.');
			goto('/login');
		} catch (err) {
			if (err instanceof ApiError) {
				const body = err.body as any;
				errorMsg = typeof body === 'string' ? body : body?.message ?? 'Erreur lors de l\'inscription.';
			} else {
				errorMsg = 'Impossible de contacter le serveur.';
			}
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Inscription — CESIZen</title>
</svelte:head>

<div class="auth-page">
	<div class="auth-card">
		<h1 class="auth-card__title">Créer un compte</h1>
		<p class="auth-card__subtitle">Rejoignez la communauté CESIZen</p>

		{#if errorMsg}
			<div class="alert alert--error">{errorMsg}</div>
		{/if}

		<form onsubmit={handleSubmit} class="form">
			<label class="form-group">
				<span class="form-label">Nom d'utilisateur</span>
				<input
					type="text"
					class="form-input"
					bind:value={username}
					placeholder="MonPseudo"
					required
					minlength="4"
					maxlength="20"
				/>
				{#if errors.username}
					<span class="form-error">{errors.username}</span>
				{/if}
			</label>

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
				<span class="form-label">Âge</span>
				<input
					type="number"
					class="form-input"
					bind:value={age}
					min="13"
					max="120"
					required
				/>
				{#if errors.age}
					<span class="form-error">{errors.age}</span>
				{/if}
			</label>

			<label class="form-group">
				<span class="form-label">Mot de passe</span>
				<input
					type="password"
					class="form-input"
					bind:value={password}
					placeholder="••••••••"
					required
					minlength="8"
					maxlength="32"
					autocomplete="new-password"
				/>
				{#if errors.password}
					<span class="form-error">{errors.password}</span>
				{/if}
			</label>

			<label class="form-group">
				<span class="form-label">Confirmer le mot de passe</span>
				<input
					type="password"
					class="form-input"
					bind:value={confirmPassword}
					placeholder="••••••••"
					required
					autocomplete="new-password"
				/>
				{#if errors.confirmPassword}
					<span class="form-error">{errors.confirmPassword}</span>
				{/if}
			</label>

			<button type="submit" class="btn btn--primary btn--full" disabled={!canSubmit || loading}>
				{loading ? 'Inscription…' : 'S\'inscrire'}
			</button>
		</form>

		<p class="auth-card__footer">
			Déjà un compte ?
			<a href="/login" class="link">Se connecter</a>
		</p>
	</div>
</div>

<style>
	.auth-page {
		display: flex;
		justify-content: center;
		align-items: flex-start;
		padding-top: 3rem;
	}

	.auth-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 2.5rem;
		width: 100%;
		max-width: 480px;
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

	.link { color: var(--color-primary); font-weight: 600; }
	.link:hover { text-decoration: underline; }

	.form { display: flex; flex-direction: column; gap: 1.1rem; }

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
	}

	.form-label { font-weight: 600; font-size: 0.875rem; }

	.form-error {
		font-size: 0.8rem;
		color: var(--color-danger);
	}

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

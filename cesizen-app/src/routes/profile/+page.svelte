<script lang="ts">
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { user as userStore, isAuthenticated, updateUser } from '$lib/stores/auth';
	import { toasts } from '$lib/stores/toasts';
	import { usersApi, ApiError } from '$lib/api';

	let currentUser = $derived($userStore);

	let email = $state('');
	let password = $state('');
	let avatar = $state('');
	let loading = $state(false);
	let showDeleteConfirm = $state(false);

	onMount(() => {
		const u = get(userStore);
		if (!get(isAuthenticated) || !u) {
			goto('/login');
			return;
		}
		email = u.email;
		avatar = u.avatar;
	});

	async function handleUpdate(e: Event) {
		e.preventDefault();
		if (!currentUser) return;
		loading = true;

		try {
			const updated = await usersApi.update(currentUser.id, {
				email,
				password,
				avatar,
				is_active: true
			});
			updateUser(updated);
			toasts.success('Profil mis à jour !');
			password = '';
		} catch (err) {
			if (err instanceof ApiError) {
				toasts.error('Erreur lors de la mise à jour.');
			}
		} finally {
			loading = false;
		}
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString('fr-FR', {
			day: 'numeric',
			month: 'long',
			year: 'numeric'
		});
	}
</script>

<svelte:head>
	<title>Mon profil — CESIZen</title>
</svelte:head>

{#if currentUser}
	<div class="profile-page">
		<h1 class="page-title">Mon profil</h1>

		<div class="profile-grid">
			<!-- Info card -->
			<div class="info-card">
				<div class="info-card__avatar">{currentUser.username.charAt(0).toUpperCase()}</div>
				<h2 class="info-card__name">{currentUser.username}</h2>
				<span class="info-card__role">{currentUser.role}</span>
				<div class="info-card__details">
					<p><strong>Email :</strong> {currentUser.email}</p>
					<p><strong>Âge :</strong> {currentUser.age} ans</p>
					<p><strong>Inscrit le :</strong> {formatDate(currentUser.created_at)}</p>
				</div>
			</div>

			<!-- Edit form -->
			<div class="edit-card">
				<h2 class="edit-card__title">Modifier mes informations</h2>

				<form onsubmit={handleUpdate} class="form">
					<label class="form-group">
						<span class="form-label">Email</span>
						<input type="email" class="form-input" bind:value={email} required />
					</label>

					<label class="form-group">
						<span class="form-label">Nouveau mot de passe</span>
						<input
							type="password"
							class="form-input"
							bind:value={password}
							placeholder="Laisser vide pour ne pas changer"
							minlength="8"
							maxlength="32"
							autocomplete="new-password"
						/>
					</label>

					<label class="form-group">
						<span class="form-label">Avatar (URL)</span>
						<input type="text" class="form-input" bind:value={avatar} />
					</label>

					<button type="submit" class="btn btn--primary" disabled={loading}>
						{loading ? 'Enregistrement…' : 'Sauvegarder'}
					</button>
				</form>

				<hr class="divider" />

				<div class="danger-zone">
					<h3 class="danger-zone__title">Zone de danger</h3>
					<p class="danger-zone__text">
						La suppression de votre compte est irréversible. Toutes vos données seront effacées.
					</p>
					{#if !showDeleteConfirm}
						<button class="btn btn--danger btn--sm" onclick={() => (showDeleteConfirm = true)}>
							Supprimer mon compte
						</button>
					{:else}
						<div class="confirm-row">
							<span>Êtes-vous sûr(e) ?</span>
							<button class="btn btn--danger btn--sm" onclick={() => { /* TODO: delete endpoint ownership */ }}>
								Oui, supprimer
							</button>
							<button class="btn btn--outline btn--sm" onclick={() => (showDeleteConfirm = false)}>
								Annuler
							</button>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.profile-page {
		max-width: 900px;
		margin: 0 auto;
	}

	.page-title {
		font-family: var(--font-heading);
		font-size: 2rem;
		font-weight: 800;
		margin-bottom: 1.5rem;
	}

	.profile-grid {
		display: grid;
		grid-template-columns: 280px 1fr;
		gap: 1.5rem;
	}

	@media (max-width: 768px) {
		.profile-grid { grid-template-columns: 1fr; }
	}

	/* Info card */
	.info-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 2rem;
		text-align: center;
		height: fit-content;
	}

	.info-card__avatar {
		width: 72px;
		height: 72px;
		border-radius: 50%;
		background: var(--color-primary);
		color: white;
		font-family: var(--font-heading);
		font-size: 2rem;
		font-weight: 800;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 1rem;
	}

	.info-card__name {
		font-family: var(--font-heading);
		font-size: 1.25rem;
		font-weight: 700;
	}

	.info-card__role {
		display: inline-block;
		font-size: 0.75rem;
		padding: 0.15rem 0.5rem;
		background: rgba(42, 125, 110, 0.1);
		color: var(--color-primary);
		border-radius: 999px;
		font-weight: 600;
		text-transform: uppercase;
		margin: 0.5rem 0 1rem;
	}

	.info-card__details {
		text-align: left;
		font-size: 0.9rem;
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
		color: var(--color-text-muted);
	}

	/* Edit card */
	.edit-card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius);
		padding: 2rem;
	}

	.edit-card__title {
		font-family: var(--font-heading);
		font-size: 1.25rem;
		font-weight: 700;
		margin-bottom: 1.5rem;
	}

	.form { display: flex; flex-direction: column; gap: 1.1rem; }
	.form-group { display: flex; flex-direction: column; gap: 0.35rem; }
	.form-label { font-weight: 600; font-size: 0.875rem; }

	.divider {
		border: none;
		border-top: 1px solid var(--color-border);
		margin: 2rem 0;
	}

	.danger-zone__title {
		font-size: 1rem;
		font-weight: 700;
		color: var(--color-danger);
		margin-bottom: 0.35rem;
	}

	.danger-zone__text {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin-bottom: 0.75rem;
	}

	.confirm-row {
		display: flex;
		gap: 0.5rem;
		align-items: center;
		font-size: 0.9rem;
	}
</style>

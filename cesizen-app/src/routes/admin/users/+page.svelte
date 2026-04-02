<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAdmin, user as userStore } from '$lib/stores/auth';
	import { get } from 'svelte/store';
	import { toasts } from '$lib/stores/toasts';
	import { usersApi } from '$lib/api';
	import type { UserGet } from '$lib/types';

	let users = $state<UserGet[]>([]);
	let loading = $state(true);
	let searchEmail = $state('');
	let searchUsername = $state('');

	onMount(() => {
		if (!get(isAdmin)) {
			goto('/');
			return;
		}
		loadUsers();
	});

	async function loadUsers() {
		loading = true;
		try {
			users = await usersApi.search({
				email: searchEmail || undefined,
				username: searchUsername || undefined,
				page_size: 100
			});
		} catch {
			toasts.error('Erreur de chargement des utilisateurs.');
		} finally {
			loading = false;
		}
	}

	async function deleteUser(id: number) {
		if (!confirm('Supprimer définitivement cet utilisateur ?')) return;
		try {
			await usersApi.delete(id);
			toasts.success('Utilisateur supprimé.');
			users = users.filter((u) => u.id !== id);
		} catch {
			toasts.error('Erreur lors de la suppression.');
		}
	}

	function handleSearch() {
		loadUsers();
	}

	function formatDate(iso: string): string {
		return new Date(iso).toLocaleDateString('fr-FR', { day: '2-digit', month: '2-digit', year: 'numeric' });
	}
</script>

<svelte:head>
	<title>Admin — Utilisateurs — CESIZen</title>
</svelte:head>

<div class="admin-page">
	<h1 class="page-title">Gestion des utilisateurs</h1>

	<!-- Barre de recherche -->
	<div class="search-bar">
		<input type="text" class="form-input" placeholder="Nom d'utilisateur…" bind:value={searchUsername} />
		<input type="email" class="form-input" placeholder="Email…" bind:value={searchEmail} />
		<button class="btn btn--primary btn--sm" onclick={handleSearch}>Rechercher</button>
	</div>

	{#if loading}
		<div class="state-center"><div class="spinner"></div></div>
	{:else}
		<div class="table-wrapper">
			<table class="data-table">
				<thead>
					<tr>
						<th>ID</th>
						<th>Utilisateur</th>
						<th>Email</th>
						<th>Rôle</th>
						<th>Âge</th>
						<th>Actif</th>
						<th>Inscrit le</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each users as user (user.id)}
						<tr class:inactive={!user.is_active}>
							<td>{user.id}</td>
							<td class="user-cell">
								<span class="user-avatar">{user.username.charAt(0).toUpperCase()}</span>
								{user.username}
							</td>
							<td>{user.email}</td>
							<td>
								<span class="role-badge role--{user.role.toLowerCase()}">{user.role}</span>
							</td>
							<td>{user.age}</td>
							<td>
								<span class="status-dot" class:active={user.is_active}></span>
								{user.is_active ? 'Oui' : 'Non'}
							</td>
							<td>{formatDate(user.created_at)}</td>
							<td class="actions-cell">
								<button
									class="btn btn--danger btn--sm"
									onclick={() => deleteUser(user.id)}
									disabled={user.id === get(userStore)?.id}
								>
									Suppr.
								</button>
							</td>
						</tr>
					{/each}

					{#if users.length === 0}
						<tr>
							<td colspan="8" class="empty-row">Aucun utilisateur trouvé.</td>
						</tr>
					{/if}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style>
	.admin-page { max-width: 1100px; margin: 0 auto; }

	.page-title {
		font-family: var(--font-heading);
		font-size: 1.75rem;
		font-weight: 800;
		margin-bottom: 1.5rem;
	}

	.search-bar {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-bar .form-input { flex: 1; min-width: 180px; }

	.table-wrapper { overflow-x: auto; }

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

	.data-table tr.inactive { opacity: 0.5; }

	.user-cell {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.user-avatar {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		background: var(--color-primary);
		color: white;
		font-size: 0.75rem;
		font-weight: 700;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.role-badge {
		font-size: 0.7rem;
		padding: 0.15rem 0.5rem;
		border-radius: 999px;
		font-weight: 600;
		text-transform: uppercase;
	}
	.role--admin { background: rgba(232, 168, 56, 0.15); color: var(--color-accent); }
	.role--mod { background: rgba(41, 128, 185, 0.15); color: var(--color-info); }
	.role--user { background: var(--color-bg); color: var(--color-text-muted); }

	.status-dot {
		display: inline-block;
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--color-danger);
		margin-right: 0.25rem;
	}
	.status-dot.active { background: var(--color-success); }

	.actions-cell { display: flex; gap: 0.35rem; }

	.empty-row {
		text-align: center;
		color: var(--color-text-muted);
		padding: 2rem !important;
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

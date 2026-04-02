<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { user, isAuthenticated, isAdmin, logout as doLogout } from '$lib/stores/auth';
	import { toasts } from '$lib/stores/toasts';
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();

	const navLinks = [
		{ href: '/', label: 'Accueil' },
		{ href: '/articles', label: 'Informations' },
		{ href: '/dashboard', label: 'Dashboard', auth: true },
	];

	const adminLinks = [
		{ href: '/admin/articles', label: 'Articles' },
		{ href: '/admin/tag', label: 'Catégories Articles' },
		{ href: '/admin/users', label: 'Utilisateurs' },
		{ href: '/admin/feelings', label: 'Émotions' },

	];

	let mobileMenuOpen = $state(false);

	function handleLogout() {
		doLogout();
		goto('/login');
	}
</script>

<div class="app">
	<!-- HEADER / NAV -->
	<header class="header">
		<div class="header__inner">
			<a href="/" class="header__logo">
				<span class="logo-cesi">CESI</span><span class="logo-zen">Zen</span>
			</a>

			<!-- Desktop nav -->
			<nav class="header__nav desktop-only">
				{#each navLinks as link}
					{#if !link.auth || $isAuthenticated}
						<a
							href={link.href}
							class="nav-link"
							class:active={$page.url.pathname === link.href}
						>
							{link.label}
						</a>
					{/if}
				{/each}

				{#if $isAdmin}
					<span class="nav-separator">|</span>
					{#each adminLinks as link}
						<a
							href={link.href}
							class="nav-link nav-link--admin"
							class:active={$page.url.pathname.startsWith(link.href)}
						>
							{link.label}
						</a>
					{/each}
				{/if}
			</nav>

			<div class="header__actions desktop-only">
				{#if $isAuthenticated}
					<a href="/profile" class="nav-link">
						{$user?.username}
					</a>
					<button class="btn btn--outline btn--sm" onclick={handleLogout}>
						Déconnexion
					</button>
				{:else}
					<a href="/login" class="btn btn--primary btn--sm">Connexion</a>
					<a href="/register" class="btn btn--outline btn--sm">Inscription</a>
				{/if}
			</div>

			<!-- Mobile hamburger -->
			<button
				class="hamburger mobile-only"
				onclick={() => (mobileMenuOpen = !mobileMenuOpen)}
				aria-label="Menu"
			>
				<span class="hamburger__line" class:open={mobileMenuOpen}></span>
				<span class="hamburger__line" class:open={mobileMenuOpen}></span>
				<span class="hamburger__line" class:open={mobileMenuOpen}></span>
			</button>
		</div>

		<!-- Mobile menu -->
		{#if mobileMenuOpen}
			<nav class="mobile-nav mobile-only">
				{#each navLinks as link}
					{#if !link.auth || $isAuthenticated}
						<a href={link.href} class="mobile-nav__link" onclick={() => (mobileMenuOpen = false)}>
							{link.label}
						</a>
					{/if}
				{/each}

				{#if $isAdmin}
					<hr class="mobile-nav__divider" />
					<span class="mobile-nav__label">Administration</span>
					{#each adminLinks as link}
						<a href={link.href} class="mobile-nav__link" onclick={() => (mobileMenuOpen = false)}>
							{link.label}
						</a>
					{/each}
				{/if}

				<hr class="mobile-nav__divider" />
				{#if $isAuthenticated}
					<a href="/profile" class="mobile-nav__link" onclick={() => (mobileMenuOpen = false)}>
						Mon profil
					</a>
					<button class="mobile-nav__link" onclick={() => { handleLogout(); mobileMenuOpen = false; }}>
						Déconnexion
					</button>
				{:else}
					<a href="/login" class="mobile-nav__link" onclick={() => (mobileMenuOpen = false)}>Connexion</a>
					<a href="/register" class="mobile-nav__link" onclick={() => (mobileMenuOpen = false)}>Inscription</a>
				{/if}
			</nav>
		{/if}
	</header>

	<!-- MAIN CONTENT -->
	<main class="main">
		{@render children()}
	</main>

	<!-- FOOTER -->
	<footer class="footer">
		<p>&copy; {new Date().getFullYear()} CESIZen — Ministère de la Santé et de la Prévention</p>
		<a href="/about" class="footer__link">À propos</a>
	</footer>

	<!-- TOAST CONTAINER -->
	<div class="toast-container" aria-live="polite">
		{#each $toasts as toast (toast.id)}
			<div class="toast toast--{toast.type}" role="alert">
				<span>{toast.message}</span>
				<button class="toast__close" onclick={() => toasts.dismiss(toast.id)}>&times;</button>
			</div>
		{/each}
	</div>
</div>

<style>
	/* ── Reset & Variables ─────────────────────────────────── */
	:global(*) {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
	}

	:global(:root) {
		--color-primary: #2a7d6e;
		--color-primary-light: #3da894;
		--color-primary-dark: #1d5c51;
		--color-accent: #e8a838;
		--color-bg: #f7f5f2;
		--color-surface: #ffffff;
		--color-text: #2c2c2c;
		--color-text-muted: #6b6b6b;
		--color-border: #e0ddd8;
		--color-danger: #c0392b;
		--color-success: #27ae60;
		--color-warning: #f39c12;
		--color-info: #2980b9;
		--radius: 8px;
		--shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.08);
		--shadow-md: 0 4px 12px rgba(0, 0, 0, 0.1);
		--transition: 0.2s ease;
		--font-body: 'Nunito', 'Segoe UI', sans-serif;
		--font-heading: 'Outfit', 'Segoe UI', sans-serif;
		--max-width: 1200px;
	}

	:global(body) {
		font-family: var(--font-body);
		background: var(--color-bg);
		color: var(--color-text);
		line-height: 1.6;
	}

	:global(a) {
		text-decoration: none;
		color: inherit;
	}

	/* ── Layout ───────────────────────────────────────────── */
	.app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	.main {
		flex: 1;
		width: 100%;
		max-width: var(--max-width);
		margin: 0 auto;
		padding: 2rem 1.5rem;
	}

	/* ── Header ──────────────────────────────────────────── */
	.header {
		background: var(--color-surface);
		border-bottom: 1px solid var(--color-border);
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.header__inner {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: 0 1.5rem;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.header__logo {
		font-family: var(--font-heading);
		font-size: 1.5rem;
		font-weight: 700;
	}

	.logo-cesi { color: var(--color-primary); }
	.logo-zen { color: var(--color-accent); }

	.header__nav {
		display: flex;
		gap: 0.25rem;
	}

	.nav-link {
		padding: 0.5rem 0.75rem;
		border-radius: var(--radius);
		font-weight: 500;
		font-size: 0.9rem;
		transition: background var(--transition), color var(--transition);
	}

	.nav-link:hover { background: rgba(42, 125, 110, 0.08); }
	.nav-link.active { color: var(--color-primary); background: rgba(42, 125, 110, 0.12); }
	.nav-link--admin { color: var(--color-accent); }
	.nav-link--admin.active { background: rgba(232, 168, 56, 0.12); }

	.nav-separator {
		color: var(--color-border);
		display: flex;
		align-items: center;
		margin: 0 0.25rem;
	}

	.header__actions {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	/* ── Buttons (global) ────────────────────────────────── */
	:global(.btn) {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		border: 2px solid transparent;
		border-radius: var(--radius);
		font-family: var(--font-body);
		font-weight: 600;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all var(--transition);
		text-decoration: none;
	}

	:global(.btn--primary) {
		background: var(--color-primary);
		color: white;
	}
	:global(.btn--primary:hover) { background: var(--color-primary-dark); }

	:global(.btn--outline) {
		background: transparent;
		border-color: var(--color-primary);
		color: var(--color-primary);
	}
	:global(.btn--outline:hover) {
		background: var(--color-primary);
		color: white;
	}

	:global(.btn--danger) { background: var(--color-danger); color: white; }
	:global(.btn--danger:hover) { opacity: 0.9; }

	:global(.btn--sm) { padding: 0.4rem 0.9rem; font-size: 0.825rem; }
	:global(.btn:disabled) { opacity: 0.5; cursor: not-allowed; }

	/* ── Footer ──────────────────────────────────────────── */
	.footer {
		background: var(--color-surface);
		border-top: 1px solid var(--color-border);
		padding: 1.5rem;
		text-align: center;
		font-size: 0.85rem;
		color: var(--color-text-muted);
		display: flex;
		justify-content: center;
		gap: 1.5rem;
		flex-wrap: wrap;
	}

	.footer__link { color: var(--color-primary); }
	.footer__link:hover { text-decoration: underline; }

	/* ── Toast ────────────────────────────────────────────── */
	.toast-container {
		position: fixed;
		bottom: 1.5rem;
		right: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		z-index: 9999;
		max-width: 400px;
	}

	.toast {
		padding: 0.75rem 1rem;
		border-radius: var(--radius);
		color: white;
		font-weight: 500;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
		box-shadow: var(--shadow-md);
		animation: slideIn 0.25s ease-out;
	}

	.toast--success { background: var(--color-success); }
	.toast--error { background: var(--color-danger); }
	.toast--info { background: var(--color-info); }
	.toast--warning { background: var(--color-warning); }

	.toast__close {
		background: none;
		border: none;
		color: white;
		font-size: 1.25rem;
		cursor: pointer;
		opacity: 0.8;
	}
	.toast__close:hover { opacity: 1; }

	@keyframes slideIn {
		from { transform: translateX(100%); opacity: 0; }
		to { transform: translateX(0); opacity: 1; }
	}

	/* ── Hamburger ───────────────────────────────────────── */
	.hamburger {
		background: none;
		border: none;
		cursor: pointer;
		display: flex;
		flex-direction: column;
		gap: 5px;
		padding: 0.5rem;
	}

	.hamburger__line {
		display: block;
		width: 24px;
		height: 2px;
		background: var(--color-text);
		transition: all 0.3s;
	}

	.hamburger__line.open:nth-child(1) { transform: rotate(45deg) translate(5px, 5px); }
	.hamburger__line.open:nth-child(2) { opacity: 0; }
	.hamburger__line.open:nth-child(3) { transform: rotate(-45deg) translate(5px, -5px); }

	/* ── Mobile nav ──────────────────────────────────────── */
	.mobile-nav {
		background: var(--color-surface);
		border-bottom: 1px solid var(--color-border);
		padding: 1rem 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.mobile-nav__link {
		display: block;
		padding: 0.75rem;
		border-radius: var(--radius);
		font-weight: 500;
		background: none;
		border: none;
		font-size: 1rem;
		cursor: pointer;
		text-align: left;
		font-family: var(--font-body);
		color: var(--color-text);
	}
	.mobile-nav__link:hover { background: rgba(42, 125, 110, 0.08); }

	.mobile-nav__divider {
		border: none;
		border-top: 1px solid var(--color-border);
		margin: 0.5rem 0;
	}

	.mobile-nav__label {
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		padding: 0.25rem 0.75rem;
	}

	/* ── Responsive ──────────────────────────────────────── */
	.desktop-only { display: flex; }
	.mobile-only { display: none; }

	@media (max-width: 768px) {
		.desktop-only { display: none !important; }
		.mobile-only { display: flex; }
	}
</style>

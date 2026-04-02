/**
 * Tests unitaires — Store d'authentification ($lib/stores/auth.ts)
 *
 * Module testé : auth store
 * Type : Unitaire
 * Responsable : Prestataire (développement)
 *
 * Scénarios couverts :
 *   - État initial (non connecté)
 *   - Login : stockage token + user, derived stores mis à jour
 *   - Logout : nettoyage token + user + localStorage
 *   - updateUser : mise à jour partielle
 *   - Derived stores isAuthenticated, isAdmin, isMod
 *   - Persistence localStorage
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { get } from 'svelte/store';

// On réimporte à chaque test pour un état propre
let authModule: typeof import('$lib/stores/auth');

async function freshImport() {
	vi.resetModules();
	const mod = await import('$lib/stores/auth');
	return mod;
}

describe('Auth Store — État initial', () => {
	beforeEach(async () => {
		localStorage.clear();
		authModule = await freshImport();
	});

	it('user est null par défaut', () => {
		expect(get(authModule.user)).toBeNull();
	});

	it('token est null par défaut', () => {
		expect(get(authModule.token)).toBeNull();
	});

	it('isAuthenticated est false par défaut', () => {
		expect(get(authModule.isAuthenticated)).toBe(false);
	});

	it('isAdmin est false par défaut', () => {
		expect(get(authModule.isAdmin)).toBe(false);
	});

	it('isMod est false par défaut', () => {
		expect(get(authModule.isMod)).toBe(false);
	});
});

describe('Auth Store — Login', () => {
	beforeEach(async () => {
		localStorage.clear();
		authModule = await freshImport();
	});

	const mockUser = {
		id: 1,
		username: 'alice',
		email: 'alice@test.com',
		age: 25,
		avatar: 'default',
		is_active: true,
		role: 'User',
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-01-01T00:00:00Z'
	};

	it('met à jour le token après login', () => {
		authModule.login('jwt-token-123', mockUser);
		expect(get(authModule.token)).toBe('jwt-token-123');
	});

	it('met à jour le user après login', () => {
		authModule.login('jwt-token-123', mockUser);
		expect(get(authModule.user)).toEqual(mockUser);
	});

	it('isAuthenticated passe à true après login', () => {
		authModule.login('jwt-token-123', mockUser);
		expect(get(authModule.isAuthenticated)).toBe(true);
	});

	it('isAdmin reste false pour un User standard', () => {
		authModule.login('jwt-token-123', mockUser);
		expect(get(authModule.isAdmin)).toBe(false);
	});

	it('isAdmin est true pour un Admin', () => {
		authModule.login('jwt-token-123', { ...mockUser, role: 'Admin' });
		expect(get(authModule.isAdmin)).toBe(true);
	});

	it('isMod est true pour un Mod', () => {
		authModule.login('jwt-token-123', { ...mockUser, role: 'Mod' });
		expect(get(authModule.isMod)).toBe(true);
	});

	it('isMod est true pour un Admin (bypass)', () => {
		authModule.login('jwt-token-123', { ...mockUser, role: 'Admin' });
		expect(get(authModule.isMod)).toBe(true);
	});

	it('persiste le token dans localStorage', () => {
		authModule.login('jwt-token-123', mockUser);
		expect(localStorage.getItem('token')).toBe('jwt-token-123');
	});

	it('persiste le user dans localStorage (JSON)', () => {
		authModule.login('jwt-token-123', mockUser);
		const stored = localStorage.getItem('user');
		expect(stored).not.toBeNull();
		expect(JSON.parse(stored!)).toEqual(mockUser);
	});
});

describe('Auth Store — Logout', () => {
	beforeEach(async () => {
		localStorage.clear();
		authModule = await freshImport();
	});

	const mockUser = {
		id: 1, username: 'alice', email: 'a@t.com',
		age: 25, avatar: 'x', is_active: true, role: 'Admin',
		created_at: '2025-01-01T00:00:00Z', updated_at: '2025-01-01T00:00:00Z'
	};

	it('remet user à null', () => {
		authModule.login('tok', mockUser);
		authModule.logout();
		expect(get(authModule.user)).toBeNull();
	});

	it('remet token à null', () => {
		authModule.login('tok', mockUser);
		authModule.logout();
		expect(get(authModule.token)).toBeNull();
	});

	it('isAuthenticated repasse à false', () => {
		authModule.login('tok', mockUser);
		authModule.logout();
		expect(get(authModule.isAuthenticated)).toBe(false);
	});

	it('isAdmin repasse à false', () => {
		authModule.login('tok', mockUser);
		authModule.logout();
		expect(get(authModule.isAdmin)).toBe(false);
	});

	it('supprime token de localStorage', () => {
		authModule.login('tok', mockUser);
		authModule.logout();
		expect(localStorage.getItem('token')).toBeNull();
	});

	it('supprime user de localStorage', () => {
		authModule.login('tok', mockUser);
		authModule.logout();
		expect(localStorage.getItem('user')).toBeNull();
	});
});

describe('Auth Store — updateUser', () => {
	beforeEach(async () => {
		localStorage.clear();
		authModule = await freshImport();
	});

	const mockUser = {
		id: 1, username: 'alice', email: 'a@t.com',
		age: 25, avatar: 'old', is_active: true, role: 'User',
		created_at: '2025-01-01T00:00:00Z', updated_at: '2025-01-01T00:00:00Z'
	};

	it('met à jour les informations du user', () => {
		authModule.login('tok', mockUser);
		const updated = { ...mockUser, email: 'new@mail.com', avatar: 'new-avatar' };
		authModule.updateUser(updated);
		expect(get(authModule.user)!.email).toBe('new@mail.com');
		expect(get(authModule.user)!.avatar).toBe('new-avatar');
	});

	it('persiste la mise à jour dans localStorage', () => {
		authModule.login('tok', mockUser);
		const updated = { ...mockUser, email: 'new@mail.com' };
		authModule.updateUser(updated);
		const stored = JSON.parse(localStorage.getItem('user')!);
		expect(stored.email).toBe('new@mail.com');
	});
});
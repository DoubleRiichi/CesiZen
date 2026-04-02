import { writable, derived, get } from 'svelte/store';
import type { UserGet } from '$lib/types';

/**
 * Store d'authentification CESIZen.
 *
 * Chaque export est un vrai Svelte store utilisable avec $
 * dans les templates : $user, $token, $isAuthenticated, $isAdmin, $isMod
 */

// ── Stores principaux ───────────────────────────────────────
export const user = writable<UserGet | null>(null);
export const token = writable<string | null>(null);

// ── Derived stores ──────────────────────────────────────────
export const isAuthenticated = derived(token, ($t) => $t !== null);
export const isAdmin = derived(user, ($u) => $u?.role === 'Admin');
export const isMod = derived(user, ($u) => $u?.role === 'Mod' || $u?.role === 'Admin');

// ── Hydratation côté client ─────────────────────────────────
if (typeof window !== 'undefined') {
	const savedToken = localStorage.getItem('token');
	const savedUser = localStorage.getItem('user');
	if (savedToken) token.set(savedToken);
	if (savedUser) {
		try {
			user.set(JSON.parse(savedUser));
		} catch {
			localStorage.removeItem('user');
		}
	}
}

// ── Actions ─────────────────────────────────────────────────
export function login(newToken: string, newUser: UserGet) {
	token.set(newToken);
	user.set(newUser);
	if (typeof window !== 'undefined') {
		localStorage.setItem('token', newToken);
		localStorage.setItem('user', JSON.stringify(newUser));
	}
}

export function logout() {
	token.set(null);
	user.set(null);
	if (typeof window !== 'undefined') {
		localStorage.removeItem('token');
		localStorage.removeItem('user');
	}
}

export function updateUser(updatedUser: UserGet) {
	user.set(updatedUser);
	if (typeof window !== 'undefined') {
		localStorage.setItem('user', JSON.stringify(updatedUser));
	}
}

/** Raccourci pour lire l'utilisateur courant hors template */
export function getUser(): UserGet | null {
	return get(user);
}

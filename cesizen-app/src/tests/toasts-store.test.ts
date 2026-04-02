/**
 * Tests unitaires — Store de notifications ($lib/stores/toasts.ts)
 *
 * Module testé : toasts store
 * Type : Unitaire
 * Responsable : Prestataire (développement)
 *
 * Scénarios couverts :
 *   - Ajout de toasts (success, error, info, warning)
 *   - Suppression manuelle via dismiss
 *   - Auto-suppression après timeout
 *   - Unicité des IDs
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import { toasts } from '$lib/stores/toasts';

describe('Toasts Store', () => {
	beforeEach(() => {
		// Vider les toasts existants
		const current = get(toasts);
		current.forEach((t) => toasts.dismiss(t.id));
		vi.useFakeTimers();
	});

	it('est vide au départ', () => {
		expect(get(toasts)).toEqual([]);
	});

	it('ajoute un toast success', () => {
		toasts.success('Bravo !');
		const list = get(toasts);
		expect(list).toHaveLength(1);
		expect(list[0].type).toBe('success');
		expect(list[0].message).toBe('Bravo !');
	});

	it('ajoute un toast error', () => {
		toasts.error('Erreur ici');
		const list = get(toasts);
		expect(list).toHaveLength(1);
		expect(list[0].type).toBe('error');
	});

	it('ajoute un toast info', () => {
		toasts.info('Info utile');
		expect(get(toasts)[0].type).toBe('info');
	});

	it('ajoute un toast warning', () => {
		toasts.warning('Attention !');
		expect(get(toasts)[0].type).toBe('warning');
	});

	it('chaque toast a un ID unique', () => {
		toasts.success('A');
		toasts.error('B');
		toasts.info('C');
		const list = get(toasts);
		const ids = list.map((t) => t.id);
		expect(new Set(ids).size).toBe(3);
	});

	it('dismiss supprime un toast par ID', () => {
		toasts.success('À supprimer');
		const id = get(toasts)[0].id;
		toasts.dismiss(id);
		expect(get(toasts)).toHaveLength(0);
	});

	it('dismiss ne supprime que le toast ciblé', () => {
		toasts.success('Garder');
		toasts.error('Supprimer');
		const errorId = get(toasts)[1].id;
		toasts.dismiss(errorId);
		const remaining = get(toasts);
		expect(remaining).toHaveLength(1);
		expect(remaining[0].message).toBe('Garder');
	});

	it('les toasts disparaissent automatiquement après le timeout', () => {
		toasts.success('Temporaire');
		expect(get(toasts)).toHaveLength(1);

		vi.advanceTimersByTime(4100); // Timeout par défaut = 4000ms

		expect(get(toasts)).toHaveLength(0);
	});
});

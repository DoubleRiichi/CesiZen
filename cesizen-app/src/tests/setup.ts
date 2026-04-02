/**
 * Setup global pour les tests Vitest du frontend CESIZen.
 * Mock de fetch via vi.stubGlobal pour remplacer celui de jsdom.
 * localStorage est fourni nativement par jsdom.
 */
 
import { vi, beforeEach } from 'vitest';
 
// ── Mock fetch via stubGlobal (remplace le fetch de jsdom) ──
const fetchMock = vi.fn();
vi.stubGlobal('fetch', fetchMock);
 
// ── Nettoyage entre chaque test ─────────────────────────────
beforeEach(() => {
	vi.clearAllMocks();
	localStorage.clear();
});
 
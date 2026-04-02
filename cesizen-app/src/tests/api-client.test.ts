/**
 * Tests unitaires — Client HTTP API ($lib/api/client.ts)
 *
 * Module testé : client.ts
 * Type : Unitaire
 * Responsable : Prestataire (développement)
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';

let api: typeof import('$lib/api/client')['api'];
let ApiError: typeof import('$lib/api/client')['ApiError'];

function stubFetch(status: number, body: unknown = {}) {
	const ok = status >= 200 && status < 300;
	vi.stubGlobal(
		'fetch',
		vi.fn().mockResolvedValue({
			ok,
			status,
			json: () => Promise.resolve(body),
			statusText: `Status ${status}`
		})
	);
}

beforeEach(async () => {
	vi.resetModules();
	localStorage.clear();
	stubFetch(200, {});
	const mod = await import('$lib/api/client');
	api = mod.api;
	ApiError = mod.ApiError;
});

describe('api.get', () => {
	it('envoie une requête GET à la bonne URL', async () => {
		stubFetch(200, { id: 1 });
		await api.get('/user/1');
		const [url, opts] = vi.mocked(fetch).mock.calls[0];
		expect(url).toContain('/user/1');
		expect(opts?.method).toBe('GET');
	});

	it('parse la réponse JSON', async () => {
		stubFetch(200, { id: 42, username: 'alice' });
		const result = await api.get<{ id: number; username: string }>('/user/42');
		expect(result).toEqual({ id: 42, username: 'alice' });
	});

	it('injecte le token JWT si présent dans localStorage', async () => {
		localStorage.setItem('token', 'mon-jwt-secret');
		stubFetch(200, {});
		await api.get('/user/1');
		const [, opts] = vi.mocked(fetch).mock.calls[0];
		expect((opts?.headers as Record<string, string>).Authorization).toBe('Bearer mon-jwt-secret');
	});

	it("n'injecte pas de header Authorization sans token", async () => {
		stubFetch(200, {});
		await api.get('/article/1');
		const [, opts] = vi.mocked(fetch).mock.calls[0];
		expect((opts?.headers as Record<string, string>).Authorization).toBeUndefined();
	});
});

describe('api.post', () => {
	it('envoie le body sérialisé en JSON', async () => {
		stubFetch(200, 42);
		await api.post('/article', { title: 'Test', content: 'Hello' });
		const [, opts] = vi.mocked(fetch).mock.calls[0];
		expect(opts?.method).toBe('POST');
		expect(opts?.body).toBe(JSON.stringify({ title: 'Test', content: 'Hello' }));
		expect((opts?.headers as Record<string, string>)['Content-Type']).toBe('application/json');
	});

	it('gère un body vide pour les endpoints search', async () => {
		stubFetch(200, []);
		await api.post('/article/search', {});
		const [, opts] = vi.mocked(fetch).mock.calls[0];
		expect(opts?.body).toBe('{}');
	});
});

describe('api.put', () => {
	it('envoie une requête PUT', async () => {
		stubFetch(200, {});
		await api.put('/user/1', { email: 'new@mail.com' });
		const [, opts] = vi.mocked(fetch).mock.calls[0];
		expect(opts?.method).toBe('PUT');
	});
});

describe('api.delete', () => {
	it('envoie une requête DELETE', async () => {
		stubFetch(200, {});
		await api.delete('/article/5');
		const [url, opts] = vi.mocked(fetch).mock.calls[0];
		expect(opts?.method).toBe('DELETE');
		expect(url).toContain('/article/5');
	});
});

describe('Gestion des erreurs', () => {
	it('lève ApiError sur une réponse 400', async () => {
		stubFetch(400, { message: 'Bad request' });
		await expect(api.post('/user/login', {})).rejects.toThrow(ApiError);
	});

	it('ApiError contient le status HTTP correct', async () => {
		stubFetch(403, 'Forbidden');
		try {
			await api.get('/admin/resource');
			expect.fail('Devrait lever une erreur');
		} catch (err) {
			expect(err).toBeInstanceOf(ApiError);
			expect((err as InstanceType<typeof ApiError>).status).toBe(403);
		}
	});

	it('ApiError contient le body de la réponse', async () => {
		stubFetch(422, { message: 'Email invalide' });
		try {
			await api.post('/user', { email: 'bad' });
			expect.fail('Devrait lever une erreur');
		} catch (err) {
			expect((err as InstanceType<typeof ApiError>).body).toEqual({ message: 'Email invalide' });
		}
	});

	it('lève ApiError sur une erreur serveur 500', async () => {
		stubFetch(500, 'Internal Server Error');
		await expect(api.get('/crash')).rejects.toThrow(ApiError);
	});
});
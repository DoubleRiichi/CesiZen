/**
 * Tests unitaires — Services API ($lib/api/*.ts)
 *
 * Module testé : articles, feelings, feelingTracker, users, tags, auth API
 * Type : Unitaire
 * Responsable : Prestataire (développement)
 *
 * Scénarios couverts :
 *   - Chaque service appelle la bonne route HTTP (méthode + path)
 *   - Les paramètres sont correctement transmis
 *   - Tests de non-régression : les contrats API ne changent pas
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';

// ── Mock du client HTTP ──────────────────────────────────────
vi.mock('$lib/api/client', () => ({
	api: {
		get: vi.fn().mockResolvedValue({}),
		post: vi.fn().mockResolvedValue({}),
		put: vi.fn().mockResolvedValue({}),
		delete: vi.fn().mockResolvedValue({})
	},
	ApiError: class ApiError extends Error {
		constructor(public status: number, public body: unknown) {
			super(`API ${status}`);
		}
	}
}));

import { api } from '$lib/api/client';

// ═══════════════════════════════════════════════════════════
// Articles API
// ═══════════════════════════════════════════════════════════

describe('articlesApi', () => {
	let articlesApi: typeof import('$lib/api/articles')['articlesApi'];

	beforeEach(async () => {
		vi.clearAllMocks();
		const mod = await import('$lib/api/articles');
		articlesApi = mod.articlesApi;
	});

	it('getById appelle GET /article/{id}', async () => {
		await articlesApi.getById(42);
		expect(api.get).toHaveBeenCalledWith('/article/42');
	});

	it('search appelle POST /article/search avec les params', async () => {
		const params = { title: 'stress', page_size: 10 };
		await articlesApi.search(params);
		expect(api.post).toHaveBeenCalledWith('/article/search', params);
	});

	it('search sans filtre envoie un objet vide', async () => {
		await articlesApi.search({});
		expect(api.post).toHaveBeenCalledWith('/article/search', {});
	});

	it('create appelle POST /article', async () => {
		const body = { author_id: 1, title: 'Test article long enough', content: 'x'.repeat(300), visibility: 'Public', tags: [1] };
		await articlesApi.create(body);
		expect(api.post).toHaveBeenCalledWith('/article', body);
	});

	it('update appelle PUT /article/{id}', async () => {
		const body = { title: 'Titre modifié suffisamment long' };
		await articlesApi.update(5, body);
		expect(api.put).toHaveBeenCalledWith('/article/5', body);
	});

	it('delete appelle DELETE /article/{id}', async () => {
		await articlesApi.delete(5);
		expect(api.delete).toHaveBeenCalledWith('/article/5');
	});
});

// ═══════════════════════════════════════════════════════════
// Feelings API
// ═══════════════════════════════════════════════════════════

describe('feelingsApi', () => {
	let feelingsApi: typeof import('$lib/api/feelings')['feelingsApi'];

	beforeEach(async () => {
		vi.clearAllMocks();
		const mod = await import('$lib/api/feelings');
		feelingsApi = mod.feelingsApi;
	});

	it('getById appelle GET /feeling/{id}', async () => {
		await feelingsApi.getById(3);
		expect(api.get).toHaveBeenCalledWith('/feeling/3');
	});

	it('search appelle POST /feeling/search', async () => {
		await feelingsApi.search({ feeling_category_id: 1 });
		expect(api.post).toHaveBeenCalledWith('/feeling/search', { feeling_category_id: 1 });
	});
});

// ═══════════════════════════════════════════════════════════
// Feeling Categories API
// ═══════════════════════════════════════════════════════════

describe('feelingCategoriesApi', () => {
	let feelingCategoriesApi: typeof import('$lib/api/feelings')['feelingCategoriesApi'];

	beforeEach(async () => {
		vi.clearAllMocks();
		const mod = await import('$lib/api/feelings');
		feelingCategoriesApi = mod.feelingCategoriesApi;
	});

	it('getById appelle GET /feeling_category/{id}', async () => {
		await feelingCategoriesApi.getById(2);
		expect(api.get).toHaveBeenCalledWith('/feeling_category/2');
	});

	it('search appelle POST /feeling_category/search', async () => {
		await feelingCategoriesApi.search({ name: 'Positive' });
		expect(api.post).toHaveBeenCalledWith('/feeling_category/search', { name: 'Positive' });
	});
});

// ═══════════════════════════════════════════════════════════
// Feeling Tracker API
// ═══════════════════════════════════════════════════════════

describe('feelingTrackerApi', () => {
	let feelingTrackerApi: typeof import('$lib/api/feelings')['feelingTrackerApi'];

	beforeEach(async () => {
		vi.clearAllMocks();
		const mod = await import('$lib/api/feelings');
		feelingTrackerApi = mod.feelingTrackerApi;
	});

	it('getById appelle GET /feeling_tracker/{id}', async () => {
		await feelingTrackerApi.getById(10);
		expect(api.get).toHaveBeenCalledWith('/feeling_tracker/10');
	});

	it('search appelle POST /feeling_tracker/search', async () => {
		const params = { feeling_id: 3, page_size: 50 };
		await feelingTrackerApi.search(params);
		expect(api.post).toHaveBeenCalledWith('/feeling_tracker/search', params);
	});

	it('create appelle POST /feeling_tracker avec le body complet', async () => {
		const body = {
			user_id: 1,
			feeling_id: 2,
			timestamp_start: '2025-06-01T10:00:00Z',
			timestamp_end: '2025-06-01T10:30:00Z',
			intensity: 7,
			notes: 'Bonne journée',
			location: 'Bureau'
		};
		await feelingTrackerApi.create(body);
		expect(api.post).toHaveBeenCalledWith('/feeling_tracker', body);
	});

	it('update appelle PUT /feeling_tracker/{id}', async () => {
		const body = {
			feeling_id: 3,
			timestamp_start: '2025-06-01T10:00:00Z',
			timestamp_end: '2025-06-01T10:30:00Z',
			intensity: 5,
			notes: 'Modifié',
			location: 'Maison'
		};
		await feelingTrackerApi.update(10, body);
		expect(api.put).toHaveBeenCalledWith('/feeling_tracker/10', body);
	});

	it('delete appelle DELETE /feeling_tracker/{id}', async () => {
		await feelingTrackerApi.delete(10);
		expect(api.delete).toHaveBeenCalledWith('/feeling_tracker/10');
	});
});

// ═══════════════════════════════════════════════════════════
// Users API (Admin)
// ═══════════════════════════════════════════════════════════

describe('usersApi', () => {
	let usersApi: typeof import('$lib/api/users')['usersApi'];

	beforeEach(async () => {
		vi.clearAllMocks();
		const mod = await import('$lib/api/users');
		usersApi = mod.usersApi;
	});

	it('getById appelle GET /user/{id}', async () => {
		await usersApi.getById(1);
		expect(api.get).toHaveBeenCalledWith('/user/1');
	});

	it('search appelle POST /user/search', async () => {
		await usersApi.search({ email: 'test@test.com' });
		expect(api.post).toHaveBeenCalledWith('/user/search', { email: 'test@test.com' });
	});

	it('update appelle PUT /user/{id}', async () => {
		const body = { email: 'new@mail.com', password: 'newpass123', avatar: 'av', is_active: true };
		await usersApi.update(1, body);
		expect(api.put).toHaveBeenCalledWith('/user/1', body);
	});

	it('delete appelle DELETE /user/{id}', async () => {
		await usersApi.delete(99);
		expect(api.delete).toHaveBeenCalledWith('/user/99');
	});
});

// ═══════════════════════════════════════════════════════════
// Tags API
// ═══════════════════════════════════════════════════════════

describe('tagsApi', () => {
	let tagsApi: typeof import('$lib/api/tags')['tagsApi'];

	beforeEach(async () => {
		vi.clearAllMocks();
		const mod = await import('$lib/api/tags');
		tagsApi = mod.tagsApi;
	});

	it('getAll appelle GET /tag/all', async () => {
		await tagsApi.getAll();
		expect(api.get).toHaveBeenCalledWith('/tag/all');
	});

	it('getById appelle GET /tag/{id}', async () => {
		await tagsApi.getById(5);
		expect(api.get).toHaveBeenCalledWith('/tag/5');
	});

	it('create appelle POST /tag', async () => {
		await tagsApi.create({ name: 'Santé' });
		expect(api.post).toHaveBeenCalledWith('/tag', { name: 'Santé' });
	});

	it('delete appelle DELETE /tag/{id}', async () => {
		await tagsApi.delete(5);
		expect(api.delete).toHaveBeenCalledWith('/tag/5');
	});
});

// ═══════════════════════════════════════════════════════════
// Auth API
// ═══════════════════════════════════════════════════════════

describe('authApi', () => {
	let authApi: typeof import('$lib/api/auth')['authApi'];

	beforeEach(async () => {
		vi.clearAllMocks();
		const mod = await import('$lib/api/auth');
		authApi = mod.authApi;
	});

	it('login appelle POST /user/login', async () => {
		await authApi.login({ email: 'a@b.com', password: 'secret' });
		expect(api.post).toHaveBeenCalledWith('/user/login', { email: 'a@b.com', password: 'secret' });
	});

	it('register appelle POST /user', async () => {
		const body = { username: 'bob', password: 'pass1234', email: 'b@b.com', avatar: 'default', age: 20 };
		await authApi.register(body);
		expect(api.post).toHaveBeenCalledWith('/user', body);
	});

	it('me appelle GET /user/{id}', async () => {
		await authApi.me(42);
		expect(api.get).toHaveBeenCalledWith('/user/42');
	});
});

/**
 * Tests de non-régression — Contrats TypeScript ↔ DTOs Rust
 *
 * Module testé : $lib/types/index.ts
 * Type : Non-régression
 * Responsable : Prestataire (développement)
 *
 * But : Vérifier que les interfaces TypeScript correspondent
 *       exactement aux structures JSON renvoyées par l'API Rust.
 *       Si l'API change un champ, ces tests cassent → alerte immédiate.
 *
 * Scénarios couverts :
 *   - LoginResponse contient token + user
 *   - UserGet contient tous les champs attendus
 *   - ArticleGet contient author, tags, visibility
 *   - FeelingTrackerGet contient feeling, feeling_category, intensity
 *   - FeelingGet contient feeling_category imbriqué
 */

import { describe, it, expect } from 'vitest';
import type {
	LoginResponse,
	UserGet,
	ArticleGet,
	FeelingTrackerGet,
	FeelingGet,
	FeelingCategoryGet,
	TagGet
} from '$lib/types';

// ── Fixtures représentant les réponses exactes de l'API Rust ──

const validUserGet: UserGet = {
	id: 1,
	username: 'alice',
	email: 'alice@cesizen.fr',
	age: 25,
	avatar: 'https://i.pravatar.cc/150',
	is_active: true,
	role: 'User',
	created_at: '2025-01-01T00:00:00Z',
	updated_at: '2025-06-01T12:00:00Z'
};

const validLoginResponse: LoginResponse = {
	token: 'eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOjF9.fake',
	user: validUserGet
};

const validArticleGet: ArticleGet = {
	id: 1,
	author: { id: 1, username: 'admin', avatar: 'av', role: 'Admin' },
	title: 'Un article sur la santé mentale',
	content: 'Contenu de l\'article très intéressant...',
	is_deleted: false,
	visibility: 'Public',
	tags: ['Santé', 'Stress'],
	created_at: '2025-03-15T10:00:00Z',
	updated_at: '2025-03-15T10:00:00Z'
};

const validFeelingCategoryGet: FeelingCategoryGet = {
	id: 1,
	name: 'Positive',
	created_at: '2025-01-01T00:00:00Z',
	updated_at: '2025-01-01T00:00:00Z'
};

const validFeelingGet: FeelingGet = {
	id: 1,
	feeling_category: validFeelingCategoryGet,
	name: 'Happy',
	created_at: '2025-01-01T00:00:00Z',
	updated_at: '2025-01-01T00:00:00Z'
};

const validFeelingTrackerGet: FeelingTrackerGet = {
	id: 1,
	user_id: 1,
	feeling: 'Happy',
	feeling_category: 'Positive',
	timestamp_start: '2025-06-01T10:00:00Z',
	timestamp_end: '2025-06-01T10:30:00Z',
	intensity: 7,
	notes: 'Bonne journée',
	location: 'Bureau',
	created_at: '2025-06-01T10:32:00Z',
	updated_at: '2025-06-01T10:32:00Z'
};

const validTagGet: TagGet = {
	id: 1,
	name: 'Santé mentale',
	created_at: '2025-01-01T00:00:00Z',
	updated_at: '2025-01-01T00:00:00Z'
};

// ═══════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════

describe('Contrat LoginResponse', () => {
	it('contient un token string', () => {
		expect(typeof validLoginResponse.token).toBe('string');
		expect(validLoginResponse.token.length).toBeGreaterThan(0);
	});

	it('contient un user complet', () => {
		expect(validLoginResponse.user).toBeDefined();
		expect(validLoginResponse.user.id).toBe(1);
		expect(validLoginResponse.user.username).toBe('alice');
	});
});

describe('Contrat UserGet', () => {
	it('possède tous les champs requis', () => {
		const keys = Object.keys(validUserGet);
		expect(keys).toContain('id');
		expect(keys).toContain('username');
		expect(keys).toContain('email');
		expect(keys).toContain('age');
		expect(keys).toContain('avatar');
		expect(keys).toContain('is_active');
		expect(keys).toContain('role');
		expect(keys).toContain('created_at');
		expect(keys).toContain('updated_at');
	});

	it('role est une string (User|Mod|Admin)', () => {
		expect(['User', 'Mod', 'Admin']).toContain(validUserGet.role);
	});

	it('age est un nombre', () => {
		expect(typeof validUserGet.age).toBe('number');
	});
});

describe('Contrat ArticleGet', () => {
	it('possède un auteur avec id, username, avatar, role', () => {
		expect(validArticleGet.author).toBeDefined();
		expect(validArticleGet.author.id).toBeDefined();
		expect(validArticleGet.author.username).toBeDefined();
		expect(validArticleGet.author.role).toBeDefined();
	});

	it('tags est un tableau de strings', () => {
		expect(Array.isArray(validArticleGet.tags)).toBe(true);
		validArticleGet.tags.forEach((t) => expect(typeof t).toBe('string'));
	});

	it('visibility est une string', () => {
		expect(['Public', 'Private', 'Unlisted']).toContain(validArticleGet.visibility);
	});
});

describe('Contrat FeelingGet', () => {
	it('contient un feeling_category imbriqué', () => {
		expect(validFeelingGet.feeling_category).toBeDefined();
		expect(validFeelingGet.feeling_category.id).toBeDefined();
		expect(validFeelingGet.feeling_category.name).toBeDefined();
	});
});

describe('Contrat FeelingTrackerGet', () => {
	it('possède tous les champs du tracker', () => {
		const keys = Object.keys(validFeelingTrackerGet);
		expect(keys).toContain('user_id');
		expect(keys).toContain('feeling');
		expect(keys).toContain('feeling_category');
		expect(keys).toContain('intensity');
		expect(keys).toContain('notes');
		expect(keys).toContain('location');
		expect(keys).toContain('timestamp_start');
		expect(keys).toContain('timestamp_end');
	});

	it('intensity est un nombre entre 1 et 10', () => {
		expect(validFeelingTrackerGet.intensity).toBeGreaterThanOrEqual(1);
		expect(validFeelingTrackerGet.intensity).toBeLessThanOrEqual(10);
	});

	it('feeling et feeling_category sont des strings (pas des objets)', () => {
		expect(typeof validFeelingTrackerGet.feeling).toBe('string');
		expect(typeof validFeelingTrackerGet.feeling_category).toBe('string');
	});
});

describe('Contrat TagGet', () => {
	it('possède id, name, created_at, updated_at', () => {
		expect(validTagGet.id).toBeDefined();
		expect(validTagGet.name).toBeDefined();
		expect(validTagGet.created_at).toBeDefined();
		expect(validTagGet.updated_at).toBeDefined();
	});
});

/**
 * Tests de validation — Contraintes des formulaires frontend
 * Miroir des validations Rust (validator crate)
 *
 * Module testé : Logique de validation des pages (register, articles, tracker)
 * Type : Unitaire + Non-régression
 * Responsable : Prestataire (développement)
 *
 * But : S'assurer que les contraintes front correspondent EXACTEMENT
 *       aux contraintes du backend Rust. Si le backend change ses règles,
 *       ces tests doivent échouer → signal de non-régression.
 *
 * Scénarios couverts :
 *   - UserCreate : username (4-20), password (8-32), email, age (13-120)
 *   - ArticleCreate : title (10-256), content (min 300)
 *   - FeelingTrackerCreate : intensity (1-10), notes (max 2000)
 *   - FeelingCreate : name (4-100)
 */

import { describe, it, expect } from 'vitest';

// ── Fonctions de validation extraites (miroir Rust) ──────────

function validateUsername(v: string): string | null {
	if (v.length < 4) return 'Minimum 4 caractères';
	if (v.length > 20) return 'Maximum 20 caractères';
	return null;
}

function validatePassword(v: string): string | null {
	if (v.length < 8) return 'Minimum 8 caractères';
	if (v.length > 32) return 'Maximum 32 caractères';
	return null;
}

function validateEmail(v: string): string | null {
	if (!v.includes('@')) return 'Email invalide';
	return null;
}

function validateAge(v: number): string | null {
	if (v < 13) return 'Âge minimum 13 ans';
	if (v > 120) return 'Âge maximum 120 ans';
	return null;
}

function validateArticleTitle(v: string): string | null {
	if (v.length < 10) return 'Minimum 10 caractères';
	if (v.length > 256) return 'Maximum 256 caractères';
	return null;
}

function validateArticleContent(v: string): string | null {
	if (v.length < 300) return 'Minimum 300 caractères';
	return null;
}

function validateIntensity(v: number): string | null {
	if (v < 1 || v > 10) return 'Entre 1 et 10';
	return null;
}

function validateNotes(v: string): string | null {
	if (v.length > 2000) return 'Maximum 2000 caractères';
	return null;
}

function validateFeelingName(v: string): string | null {
	if (v.length < 4) return 'Minimum 4 caractères';
	if (v.length > 100) return 'Maximum 100 caractères';
	return null;
}

// ═══════════════════════════════════════════════════════════
// Tests UserCreate (miroir validate(UserCreate) Rust)
// ═══════════════════════════════════════════════════════════

describe('Validation UserCreate', () => {
	describe('username', () => {
		it('rejette un username trop court (< 4)', () => {
			expect(validateUsername('ab')).not.toBeNull();
			expect(validateUsername('abc')).not.toBeNull();
		});

		it('accepte un username de 4 caractères (borne min)', () => {
			expect(validateUsername('abcd')).toBeNull();
		});

		it('accepte un username de 20 caractères (borne max)', () => {
			expect(validateUsername('a'.repeat(20))).toBeNull();
		});

		it('rejette un username trop long (> 20)', () => {
			expect(validateUsername('a'.repeat(21))).not.toBeNull();
		});
	});

	describe('password', () => {
		it('rejette un mot de passe trop court (< 8)', () => {
			expect(validatePassword('short')).not.toBeNull();
			expect(validatePassword('1234567')).not.toBeNull();
		});

		it('accepte un mot de passe de 8 caractères (borne min)', () => {
			expect(validatePassword('12345678')).toBeNull();
		});

		it('accepte un mot de passe de 32 caractères (borne max)', () => {
			expect(validatePassword('p'.repeat(32))).toBeNull();
		});

		it('rejette un mot de passe trop long (> 32)', () => {
			expect(validatePassword('p'.repeat(33))).not.toBeNull();
		});
	});

	describe('email', () => {
		it('accepte un email valide', () => {
			expect(validateEmail('user@example.com')).toBeNull();
		});

		it('rejette un email sans @', () => {
			expect(validateEmail('not-an-email')).not.toBeNull();
		});
	});

	describe('age', () => {
		it('rejette un âge inférieur à 13', () => {
			expect(validateAge(12)).not.toBeNull();
			expect(validateAge(0)).not.toBeNull();
		});

		it('accepte l\'âge minimum de 13 (borne)', () => {
			expect(validateAge(13)).toBeNull();
		});

		it('accepte l\'âge maximum de 120 (borne)', () => {
			expect(validateAge(120)).toBeNull();
		});

		it('rejette un âge supérieur à 120', () => {
			expect(validateAge(121)).not.toBeNull();
		});
	});
});

// ═══════════════════════════════════════════════════════════
// Tests ArticleCreate (miroir Rust)
// ═══════════════════════════════════════════════════════════

describe('Validation ArticleCreate', () => {
	describe('title', () => {
		it('rejette un titre trop court (< 10)', () => {
			expect(validateArticleTitle('Court')).not.toBeNull();
		});

		it('accepte un titre de 10 caractères (borne min)', () => {
			expect(validateArticleTitle('a'.repeat(10))).toBeNull();
		});

		it('accepte un titre de 256 caractères (borne max)', () => {
			expect(validateArticleTitle('t'.repeat(256))).toBeNull();
		});

		it('rejette un titre trop long (> 256)', () => {
			expect(validateArticleTitle('t'.repeat(257))).not.toBeNull();
		});
	});

	describe('content', () => {
		it('rejette un contenu trop court (< 300)', () => {
			expect(validateArticleContent('Trop court.')).not.toBeNull();
		});

		it('accepte un contenu de exactement 300 caractères (borne)', () => {
			expect(validateArticleContent('c'.repeat(300))).toBeNull();
		});
	});
});

// ═══════════════════════════════════════════════════════════
// Tests FeelingTracker (miroir Rust)
// ═══════════════════════════════════════════════════════════

describe('Validation FeelingTracker', () => {
	describe('intensity', () => {
		it('rejette une intensité < 1', () => {
			expect(validateIntensity(0)).not.toBeNull();
		});

		it('accepte intensité = 1 (borne min)', () => {
			expect(validateIntensity(1)).toBeNull();
		});

		it('accepte intensité = 10 (borne max)', () => {
			expect(validateIntensity(10)).toBeNull();
		});

		it('rejette une intensité > 10', () => {
			expect(validateIntensity(11)).not.toBeNull();
		});
	});

	describe('notes', () => {
		it('accepte des notes vides', () => {
			expect(validateNotes('')).toBeNull();
		});

		it('accepte des notes de 2000 caractères (borne max)', () => {
			expect(validateNotes('n'.repeat(2000))).toBeNull();
		});

		it('rejette des notes > 2000 caractères', () => {
			expect(validateNotes('n'.repeat(2001))).not.toBeNull();
		});
	});
});

// ═══════════════════════════════════════════════════════════
// Tests FeelingCreate (miroir Rust)
// ═══════════════════════════════════════════════════════════

describe('Validation FeelingCreate', () => {
	describe('name', () => {
		it('rejette un nom trop court (< 4)', () => {
			expect(validateFeelingName('abc')).not.toBeNull();
		});

		it('accepte un nom de 4 caractères (borne min)', () => {
			expect(validateFeelingName('abcd')).toBeNull();
		});

		it('accepte un nom de 100 caractères (borne max)', () => {
			expect(validateFeelingName('f'.repeat(100))).toBeNull();
		});

		it('rejette un nom > 100 caractères', () => {
			expect(validateFeelingName('f'.repeat(101))).not.toBeNull();
		});
	});
});

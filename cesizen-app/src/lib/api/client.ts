/**
 * Client HTTP centralisé pour l'API CESIZen (Axum/Rust).
 * Gère l'injection du JWT, le refresh (si implémenté), et le parsing JSON.
 */

const API_BASE = import.meta.env.VITE_API_URL ?? 'http://localhost:3000';

export class ApiError extends Error {
	constructor(
		public status: number,
		public body: unknown
	) {
		super(`API ${status}`);
		this.name = 'ApiError';
	}
}

function authHeaders(): Record<string, string> {
	if (typeof window === 'undefined') return {};
	const token = localStorage.getItem('token');
	return token ? { Authorization: `Bearer ${token}` } : {};
}

async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		...authHeaders()
	};

	const res = await fetch(`${API_BASE}${path}`, {
		method,
		headers,
		body: body !== undefined ? JSON.stringify(body) : undefined
	});

	if (!res.ok) {
		const errorBody = await res.json().catch(() => res.statusText);
		throw new ApiError(res.status, errorBody);
	}

	// 204 No Content → pas de body
	if (res.status === 204) return undefined as T;

	return res.json() as Promise<T>;
}

export const api = {
	get: <T>(path: string) => request<T>('GET', path),
	post: <T>(path: string, body?: unknown) => request<T>('POST', path, body),
	put: <T>(path: string, body?: unknown) => request<T>('PUT', path, body),
	delete: <T>(path: string) => request<T>('DELETE', path)
};

// ─── Types miroirs des DTOs Rust ─────────────────────────────

// ── User ─────────────────────────────────────────────────────
export type UserRole = 'User' | 'Mod' | 'Admin';

export interface UserGet {
	id: number;
	username: string;
	email: string;
	age: number;
	avatar: string;
	is_active: boolean;
	role: string;
	created_at: string;
	updated_at: string;
}

export interface UserGetSimple {
	id: number;
	username: string;
	avatar: string;
	role: string;
}

export interface UserCreate {
	username: string;
	password: string;
	email: string;
	avatar: string;
	age: number;
}

export interface UserUpdate {
	password: string;
	email: string;
	avatar: string;
	is_active: boolean;
}

export interface UserSearchParams {
	username?: string;
	email?: string;
	age?: number;
	avatar?: string;
	is_active?: boolean;
	role?: UserRole;
	start_at?: string;
	end_at?: string;
	cursor?: string;
	page_size?: number;
}

export interface LoginRequest {
	email: string;
	password: string;
}

export interface LoginResponse {
	token: string;
	user: UserGet;
}

// ── Article ──────────────────────────────────────────────────
export interface ArticleGet {
	id: number;
	author: UserGetSimple;
	title: string;
	content: string;
	is_deleted: boolean;
	visibility: string;
	tags: string[];
	created_at: string;
	updated_at: string;
}

export interface ArticleCreate {
	author_id: number;
	title: string;
	content: string;
	visibility: string;
	tags: number[];
}

export interface ArticleUpdate {
	author_id?: number;
	title?: string;
	content?: string;
	is_deleted?: boolean;
	visibility?: string;
	tags?: number[];
}

export interface ArticleSearchParams {
	author_id?: number;
	title?: string;
	content?: string;
	start_date?: string;
	end_date?: string;
	tag_ids?: number[];
	cursor?: string;
	visibility?: string
	page_size?: number;
}

// ── Tag ──────────────────────────────────────────────────────
export interface TagGet {
	id: number;
	name: string;
	created_at: string;
	updated_at: string;
}

export interface TagCreate {
	name: string;
}

// ── Feeling ──────────────────────────────────────────────────
export interface FeelingCategoryGet {
	id: number;
	name: string;
	created_at: string;
	updated_at: string;
}

export interface FeelingCategoryCreate {
	name: string;
}

export interface FeelingCategorySearchParams {
	name?: string;
	start_at?: string;
	end_at?: string;
	cursor?: string;
	page_size?: number;
}

export interface FeelingGet {
	id: number;
	feeling_category: FeelingCategoryGet;
	name: string;
	created_at: string;
	updated_at: string;
}

export interface FeelingCreate {
	feeling_category_id: number;
	name: string;
}

export interface FeelingSearchParams {
	name?: string;
	feeling_category_id?: number;
	start_at?: string;
	end_at?: string;
	cursor?: string;
	page_size?: number;
}

// ── Feeling Tracker ──────────────────────────────────────────
export interface FeelingTrackerGet {
	id: number;
	user_id: number;
	feeling: string;
	feeling_category: string;
	timestamp_start: string;
	timestamp_end: string;
	intensity: number;
	notes: string;
	location: string;
	created_at: string;
	updated_at: string;
}

export interface FeelingTrackerCreate {
	user_id: number;
	feeling_id: number;
	timestamp_start: string;
	timestamp_end: string;
	intensity: number;
	notes: string;
	location: string;
}

export interface FeelingTrackerUpdate {
	feeling_id: number;
	timestamp_start: string;
	timestamp_end: string;
	intensity: number;
	notes: string;
	location: string;
}

export interface FeelingTrackerSearchParams {
	feeling_id?: number;
	start_date?: string;
	end_date?: string;
	cursor?: string;
	page_size?: number;
}

import { api } from '$lib/api/client';
import type { LoginRequest, LoginResponse, UserCreate, UserGet } from '$lib/types';

export const authApi = {
	login: (body: LoginRequest) => api.post<LoginResponse>('/user/login', body),

	register: (body: UserCreate) => api.post<UserGet>('/user', body),

	me: (id: number) => api.get<UserGet>(`/user/${id}`)
};

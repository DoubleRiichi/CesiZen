import { api } from '$lib/api/client';
import type { UserGet, UserUpdate, UserSearchParams } from '$lib/types';

export const usersApi = {
	getById: (id: number) => api.get<UserGet>(`/user/${id}`),

	search: (params: UserSearchParams) => api.post<UserGet[]>('/user/search', params),

	update: (id: number, body: UserUpdate) => api.put<UserGet>(`/user/${id}`, body),

	delete: (id: number) => api.delete<void>(`/user/${id}`)
};

import { api } from '$lib/api/client';
import type { TagGet, TagCreate } from '$lib/types';

export const tagsApi = {
	getById: (id: number) => api.get<TagGet>(`/tag/${id}`),

	getAll: () => api.get<TagGet[]>('/tag/all'),

	create: (body: TagCreate) => api.post<number>('/tag', body),

	delete: (id: number) => api.delete<void>(`/tag/${id}`)
};

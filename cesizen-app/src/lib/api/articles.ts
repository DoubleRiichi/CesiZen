import { api } from '$lib/api/client';
import type {
	ArticleGet,
	ArticleCreate,
	ArticleUpdate,
	ArticleSearchParams
} from '$lib/types';

export const articlesApi = {
	getById: (id: number) => api.get<ArticleGet>(`/article/${id}`),

	search: (params: ArticleSearchParams) =>
		api.post<ArticleGet[]>('/article/search', params),

	create: (body: ArticleCreate) => api.post<number>('/article', body),

	update: (id: number, body: ArticleUpdate) => api.put<void>(`/article/${id}`, body),

	delete: (id: number) => api.delete<void>(`/article/${id}`)
};

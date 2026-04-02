import { api } from '$lib/api/client';
import type {
	FeelingGet,
	FeelingSearchParams,
	FeelingCategoryGet,
	FeelingCategorySearchParams,
	FeelingTrackerGet,
	FeelingTrackerCreate,
	FeelingTrackerUpdate,
	FeelingTrackerSearchParams
} from '$lib/types';

export const feelingsApi = {
	getById: (id: number) => api.get<FeelingGet>(`/feeling/${id}`),
	search: (params: FeelingSearchParams) =>
		api.post<FeelingGet[]>('/feeling/search', params)
};

export const feelingCategoriesApi = {
	getById: (id: number) => api.get<FeelingCategoryGet>(`/feeling_category/${id}`),
	search: (params: FeelingCategorySearchParams) =>
		api.post<FeelingCategoryGet[]>('/feeling_category/search', params)
};

export const feelingTrackerApi = {
	getById: (id: number) => api.get<FeelingTrackerGet>(`/feeling_tracker/${id}`),

	search: (params: FeelingTrackerSearchParams) =>
		api.post<FeelingTrackerGet[]>('/feeling_tracker/search', params),

	create: (body: FeelingTrackerCreate) => api.post<number>('/feeling_tracker', body),

	update: (id: number, body: FeelingTrackerUpdate) =>
		api.put<void>(`/feeling_tracker/${id}`, body),

	delete: (id: number) => api.delete<void>(`/feeling_tracker/${id}`)
};

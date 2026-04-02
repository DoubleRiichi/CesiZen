import type { UserSimple } from "./user";


export enum ArticleVisibility {
    Public,
    Unlisted,
    Private
}

export interface Article {
    id: number;
    author: UserSimple;
    title: string;
    content: string;
    is_deleted: boolean;
    visibility: string;
    tags: [string];
    created_at: string;
    updated_at: string;
}

export interface ArticleCreate {
    author_id: number;
    title: string;
    content: string;
    visibility: string;
    tags: [number]
}

export interface ArticleCreate {
    author_id: number;
    title: string;
    content: string;
    visibility: string;
    is_deleted: boolean;
    tags: [number]
}


export interface ArticleSearchParams {
    author_id?: number;
    title?: string;
    content?: string;
    visibility?: string;
    is_deleted?: boolean;
    tags?: [number]
}

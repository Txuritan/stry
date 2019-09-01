import { IAuthor } from "./author";
import { IOrigin } from "./origin";
import { ITag } from "./tag";

export interface IStory {
    id: string;
    name: string;
    summary: string;
    language: string;
    square: IStorySquare;
    chapters: number;
    words: number;
    authors: IAuthor[];
    origins: IOrigin[];
    tags: ITag[];
    series: null;
    created: string;
    updated: string;
}

export interface IStorySquare {
    rating: string;
    warnings: string;
    state: string;
}
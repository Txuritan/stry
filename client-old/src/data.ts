import { IChapter } from "./models/chapter";
import { ITag } from "./models/tag";
import { IStory } from "./models/story";

export interface IResponse<T> {
    data: T;
}

export interface IStoryResponse {
    count: number;
    pages: number;
    stories: IStory[];
}

export interface IChapterResponse {
    chapter: IChapter;
    story: IStory;
}

export interface ITagResponse {
    count: number;
    pages: number;
    tags: ITag[];
}
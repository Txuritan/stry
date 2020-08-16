use {
    crate::{
        backend::{
            BackendAuthor, BackendCharacter, BackendOrigin, BackendPairing, BackendStory,
            BackendTag, BackendWarning, DataBackend,
        },
        models::{
            author::AuthorList, character::CharacterList, origin::OriginList, pairing::PairingList,
            story::StoryList, tag::TagList, warning::WarningList, Author, Character, Origin,
            Pairing, Story, Tag, Warning,
        },
    },
    juniper::FieldResult,
};

pub struct Query;

#[juniper::graphql_object(Context = DataBackend)]
impl Query {
    pub async fn all_authors(
        ctx: &DataBackend,
        offset: i32,
        limit: i32,
    ) -> FieldResult<Option<AuthorList>> {
        let authors = ctx.all_authors(offset, limit).await?;

        Ok(authors.map(Into::into))
    }

    pub async fn get_author(ctx: &DataBackend, id: String) -> FieldResult<Option<Author>> {
        let author = ctx.get_author(id.into()).await?;

        Ok(author)
    }

    pub async fn all_characters(
        ctx: &DataBackend,
        offset: i32,
        limit: i32,
    ) -> FieldResult<Option<CharacterList>> {
        let characters = ctx.all_characters(offset, limit).await?;

        Ok(characters.map(Into::into))
    }

    pub async fn get_character(ctx: &DataBackend, id: String) -> FieldResult<Option<Character>> {
        let character = ctx.get_character(id.into()).await?;

        Ok(character)
    }

    pub async fn all_origins(
        ctx: &DataBackend,
        offset: i32,
        limit: i32,
    ) -> FieldResult<Option<OriginList>> {
        let origins = ctx.all_origins(offset, limit).await?;

        Ok(origins.map(Into::into))
    }

    pub async fn get_origin(ctx: &DataBackend, id: String) -> FieldResult<Option<Origin>> {
        let origin = ctx.get_origin(id.into()).await?;

        Ok(origin)
    }

    pub async fn all_pairings(
        ctx: &DataBackend,
        offset: i32,
        limit: i32,
    ) -> FieldResult<Option<PairingList>> {
        let pairings = ctx.all_pairings(offset, limit).await?;

        Ok(pairings.map(Into::into))
    }

    pub async fn get_pairing(ctx: &DataBackend, id: String) -> FieldResult<Option<Pairing>> {
        let pairing = ctx.get_pairing(id.into()).await?;

        Ok(pairing)
    }

    pub async fn all_stories(
        ctx: &DataBackend,
        offset: i32,
        limit: i32,
    ) -> FieldResult<Option<StoryList>> {
        let stories = ctx.all_stories(offset, limit).await?;

        Ok(stories.map(Into::into))
    }

    pub async fn get_story(ctx: &DataBackend, id: String) -> FieldResult<Option<Story>> {
        let story = ctx.get_story(id.into()).await?;

        Ok(story)
    }

    pub async fn all_tags(
        ctx: &DataBackend,
        offset: i32,
        limit: i32,
    ) -> FieldResult<Option<TagList>> {
        let tags = ctx.all_tags(offset, limit).await?;

        Ok(tags.map(Into::into))
    }

    pub async fn get_tag(ctx: &DataBackend, id: String) -> FieldResult<Option<Tag>> {
        let tag = ctx.get_tag(id.into()).await?;

        Ok(tag)
    }

    pub async fn all_warnings(
        ctx: &DataBackend,
        offset: i32,
        limit: i32,
    ) -> FieldResult<Option<WarningList>> {
        let warnings = ctx.all_warnings(offset, limit).await?;

        Ok(warnings.map(Into::into))
    }

    pub async fn get_warning(ctx: &DataBackend, id: String) -> FieldResult<Option<Warning>> {
        let warning = ctx.get_warning(id.into()).await?;

        Ok(warning)
    }
}

// #[juniper::graphql_object(Context = DataBackend)]
// impl Author {
//     pub async fn stories(
//         ctx: &DataBackend,
//         offset: i32,
//         limit: i32,
//     ) -> FieldResult<Option<StoryList>> {
//         let stories = ctx.author_stories(id, offset, limit)
//     }
// }

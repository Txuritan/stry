use {
    crate::{
        backend::{
            sqlite::test_utils::setup,
            test_helpers::{character, StoryBuilder},
            BackendCharacter,
        },
        models::{Character, List, Rating, State, Story},
    },
    tokio::runtime::Runtime,
};

#[test]
pub fn get() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<Character>> {
        let backend = setup()?;

        let character = backend.get_character("2crUDM".into()).await?;

        Ok(character)
    }

    assert_eq!(
        Some(character("2crUDM", "character 1")),
        rt.block_on(run())?
    );

    Ok(())
}

#[test]
pub fn get_all() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<List<Character>>> {
        let backend = setup()?;

        let characters = backend.all_characters(0, 10).await?;

        Ok(characters)
    }

    assert_eq!(
        Some(List {
            total: 4,
            items: vec![
                character("2crUDM", "character 1"),
                character("9Tb66w", "character 2"),
                character("iV5yY4", "character 3"),
                character("SqWCU9", "character 4"),
            ]
        }),
        rt.block_on(run())?,
    );

    Ok(())
}

#[test]
pub fn get_stories() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<List<Story>>> {
        let backend = setup()?;

        let stories = backend.character_stories("2crUDM".into(), 0, 10).await?;

        Ok(stories)
    }

    assert_eq!(
        Some(List {
            total: 1,
            items: vec![StoryBuilder::new(
                "pS8LfM",
                "story 1",
                "example story",
                Rating::Teen,
                State::InProgress,
                2,
                6,
            )
            .with_author("ZqYCf8", "author 1")
            .with_origin("Nb4ynY", "origin 1")
            .with_warning("brVRkN", "warning 1")
            .with_tag("V3VEAR", "tag 1")
            .with_tag("fMNi7A", "tag 2")
            .with_tag("A38isy", "tag 3")
            .with_character("2crUDM", "character 1")
            .with_character("9Tb66w", "character 2")
            .with_character("iV5yY4", "character 3")
            .with_character("SqWCU9", "character 4")
            .with_pairing("FLR49G", false, |pairing| {
                pairing
                    .with_character("2crUDM", "character 1")
                    .with_character("9Tb66w", "character 2")
            })
            .with_pairing("SeUBQq", false, |pairing| {
                pairing
                    .with_character("2crUDM", "character 1")
                    .with_character("iV5yY4", "character 3")
            })
            .with_pairing("3TETzP", false, |pairing| {
                pairing
                    .with_character("2crUDM", "character 1")
                    .with_character("SqWCU9", "character 4")
            })
            .finish()],
        }),
        rt.block_on(run())?,
    );

    Ok(())
}

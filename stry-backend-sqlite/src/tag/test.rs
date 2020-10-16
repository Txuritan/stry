use {
    crate::test_utils::setup,
    stry_models::{story::StoryBuilder, List, Rating, State, Story, Tag},
    tokio::runtime::Runtime,
};

#[test]
pub fn get() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<Tag>> {
        let backend = setup()?;

        let tag = backend.get_tag("V3VEAR".into()).await?;

        Ok(tag)
    }

    assert_eq!(Some(Tag::new_test("V3VEAR", "tag 1")), rt.block_on(run())?);

    Ok(())
}

#[test]
pub fn get_all() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<List<Tag>>> {
        let backend = setup()?;

        let tags = backend.all_tags(0, 10).await?;

        Ok(tags)
    }

    assert_eq!(
        Some(List {
            total: 4,
            items: vec![
                Tag::new_test("V3VEAR", "tag 1"),
                Tag::new_test("fMNi7A", "tag 2"),
                Tag::new_test("A38isy", "tag 3"),
                Tag::new_test("7TnYys", "tag 4"),
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

        let stories = backend.tag_stories("V3VEAR".into(), 0, 10).await?;

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

use {
    crate::test_utils::setup,
    stry_models::{story::StoryBuilder, List, Origin, Rating, State, Story},
    tokio::runtime::Runtime,
};

#[test]
pub fn get() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<Origin>> {
        let backend = setup()?;

        let origin = backend.get_origin("Nb4ynY".into()).await?;

        Ok(origin)
    }

    assert_eq!(
        Some(Origin::new_test("Nb4ynY", "origin 1")),
        rt.block_on(run())?
    );

    Ok(())
}

#[test]
pub fn get_all() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;

    async fn run() -> anyhow::Result<Option<List<Origin>>> {
        let backend = setup()?;

        let origins = backend.all_origins(0, 10).await?;

        Ok(origins)
    }

    assert_eq!(
        Some(List {
            total: 2,
            items: vec![
                Origin::new_test("Nb4ynY", "origin 1"),
                Origin::new_test("J2Ej2P", "origin 2"),
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

        let stories = backend.origin_stories("Nb4ynY".into(), 0, 10).await?;

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

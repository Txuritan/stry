use web_sys::Storage;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SiteData {
    pub dark: bool,
}

impl SiteData {
    pub fn get() -> Self {
        web_sys::window()
            .and_then(|window| match window.local_storage() {
                Ok(Some(storage)) => match storage.get_item("data") {
                    Ok(Some(data)) => serde_json::from_str::<SiteData>(&data).ok().map(|data| {
                        let dark = data.dark;

                        web_sys::window().map(|window| {
                            window.document().map(|document| {
                                document.body().map(|body| {
                                    let _ = body.set_attribute(
                                        "theme",
                                        if dark { "dark" } else { "light" },
                                    );
                                });
                            });
                        });

                        data
                    }),
                    Err(_) | Ok(None) => None,
                },
                Err(_) | Ok(None) => None,
            })
            .unwrap_or_else(|| Self::default())
    }

    pub fn store(&self, storage: &Storage) {
        storage
            .set_item("data", &serde_json::to_string(&self).unwrap())
            .unwrap();
    }
}

impl Default for SiteData {
    fn default() -> SiteData {
        SiteData { dark: false }
    }
}

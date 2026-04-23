//#[cfg(all(test, target_os = "linux"))]
#[cfg(test)]
mod tests {
    use g_tools::bin_xournalpp;
    use g_tools::config::*;
    use g_tools::*;

    #[test]
    fn test_config() {
        // MacOS: /Users/geraldo/pdf_images/index.txt
        let path = shellexpand::tilde("~/pdf_images/index.txt").to_string();
        initialize_mutable_config("~/pdf_images/".to_string());
        let index_txt_path = &MUTABLE_CONFIG
            .get()
            .expect("Error in config")
            .lock()
            .unwrap()
            .index_txt;
        assert_eq!(index_txt_path, &path);
    }

    #[test]
    fn test_bin_xournalpp() {
        use std::env::consts::OS;

        match OS {
            "linux" => {
                let path = bin_xournalpp();
                assert_eq!(path, "/usr/bin/xournalpp");
            }
            "macos" => {
                let path = bin_xournalpp();
                assert_eq!(path, "/Applications/Xournal++.app/Contents/MacOS/xournalpp");
            }
            _ => panic!("Unsupported OS: {}", OS),
        }
    }

    // #[test]
    // fn test_search_text() {
    //     // Create temp directory with test files
    //     let tmp_dir = std::env::temp_dir().join("g_tools_test_");
    //     std::fs::create_dir_all(&tmp_dir).unwrap();
    //
    //     let index_path = tmp_dir.join("index.txt");
    //     let bookmarks_path = tmp_dir.join("bookmarks.txt");
    //
    //     // Write test data
    //     std::fs::write(&index_path, "abc123 doc1.pdf abc456 doc2.pdf").unwrap();
    //     std::fs::write(&bookmarks_path, "abc123 Chapter 1\nabc456 Section A").unwrap();
    //
    //     // Temporarily override config paths for testing
    //     let expanded_dir = shellexpand::tilde(&format!("~/pdf_images/"));
    //     let pdf_images = std::path::PathBuf::from(expanded_dir.into_owned());
    //     let index_txt = pdf_images.join("index.txt");
    //     let bookmarks_txt = pdf_images.join("bookmarks.txt");
    //
    //     // Update config to use our test paths
    //     if let Some(config_lock) = MUTABLE_CONFIG.get() {
    //         let mut config = config_lock.lock().unwrap();
    //         config.index_txt = index_path.clone();
    //         config.bookmarks_txt = bookmarks_path.clone();
    //     }
    //
    //     // Test search
    //     let result = search_text(&"doc".to_string());
    //     assert!(result.is_some());
    //
    //     // Cleanup
    //     std::fs::remove_dir_all(&tmp_dir).ok();
    // }

    #[test]
    #[ignore]
    fn test_show_bookmark() {
        let tmp_dir = std::env::temp_dir().join("g_tools_test_bm_");
        std::fs::create_dir_all(&tmp_dir).unwrap();

        let bookmarks_path = tmp_dir.join("bookmarks.txt");
        std::fs::write(&bookmarks_path, "abc12345 Chapter 1\nabc456 Section A").unwrap();

        // Update config to use test path
        if let Some(config_lock) = MUTABLE_CONFIG.get() {
            let mut config = config_lock.lock().unwrap();
            config.bookmarks_txt = bookmarks_path.clone();
        }

        let result = show_bookmark(&"abc123".to_string());
        assert!(result.is_some());

        std::fs::remove_dir_all(&tmp_dir).ok();
    }

    // FIXME: refactor this test for existing pdf
    #[test]
    #[ignore]
    fn test_locate_related_file() {
        let tmp_dir = std::env::temp_dir().join("g_tools_test_loc_");
        std::fs::create_dir_all(&tmp_dir).unwrap();

        let index_path = tmp_dir.join("index.txt");
        std::fs::write(&index_path, "abc123 doc1.pdf abc456 doc2.pdf").unwrap();

        // Update config to use test path
        if let Some(config_lock) = MUTABLE_CONFIG.get() {
            let mut config = config_lock.lock().unwrap();
            config.index_txt = index_path.clone();
        }

        let result = locate_related_file("abc123");
        assert!(result.is_some());

        std::fs::remove_dir_all(&tmp_dir).ok();
    }

    #[test]
    fn test_locate_related_file_not_found() {
        let tmp_dir = std::env::temp_dir().join("g_tools_test_loc2_");
        std::fs::create_dir_all(&tmp_dir).unwrap();

        let index_path = tmp_dir.join("index.txt");
        std::fs::write(&index_path, "abc123 doc1.pdf abc456 doc2.pdf").unwrap();

        if let Some(config_lock) = MUTABLE_CONFIG.get() {
            let mut config = config_lock.lock().unwrap();
            config.index_txt = index_path.clone();
        }

        let result = locate_related_file("xyz999");
        assert!(result.is_none());

        std::fs::remove_dir_all(&tmp_dir).ok();
    }

    #[test]
    fn test_copy_text_to_clipboard() {
        let text1 = "Ipsum lorem".to_string();
        let _ = copy_text_to_clipboard(text1);
        let text2 = copy_text_from_clipboard();
        assert_eq!(text2.unwrap(), "Ipsum lorem".to_string());
    }

    #[test]
    #[ignore]
    fn test_cmd_xournal_search() {
        let tmp_dir = std::env::temp_dir().join("g_tools_test_search_");
        std::fs::create_dir_all(&tmp_dir).unwrap();

        let index_path = tmp_dir.join("index.txt");
        std::fs::write(&index_path, "abc123 doc1.pdf abc456 doc2.pdf").unwrap();

        if let Some(config_lock) = MUTABLE_CONFIG.get() {
            let mut config = config_lock.lock().unwrap();
            config.index_txt = index_path.clone();
        }

        let result = cmd_xournal(
            XournalAction::Search {
                text: "doc".to_string(),
            },
            false,
        );
        assert!(result.is_ok());

        std::fs::remove_dir_all(&tmp_dir).ok();
    }

    // Refactor this test for existing pdf and hash
    #[test]
    fn test_cmd_xournal() {
        let result = cmd_xournal(
            XournalAction::Open {
                hash: "12345678".to_string(),
            },
            false,
        );
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, "Hash not found at index.txt");
    }

    #[test]
    fn test_cmd_xournal_open_not_found() {
        let result = cmd_xournal(
            XournalAction::Open {
                hash: "12345678".to_string(),
            },
            false,
        );
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, "Hash not found at index.txt");
    }

    #[test]
    fn test_cmd_microci_install() {
        // Just verify the function exists and compiles
        let result = cmd_microci(MicroCIAction::Install);
        assert!(result.is_ok());
    }
}

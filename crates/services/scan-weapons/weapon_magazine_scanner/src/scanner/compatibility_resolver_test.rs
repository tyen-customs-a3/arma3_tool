#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{WeaponInfo, MagazineWellInfo};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn create_test_magazine_well(name: &str, magazines: Vec<(&str, Vec<&str>)>) -> MagazineWellInfo {
        let mut mag_map = HashMap::new();
        for (group, mags) in magazines {
            mag_map.insert(group.to_string(), mags.iter().map(|s| s.to_string()).collect());
        }
        
        MagazineWellInfo {
            name: name.to_string(),
            file_path: PathBuf::from("test.cpp"),
            magazines: mag_map,
        }
    }

    fn create_test_weapon(name: &str, wells: Vec<&str>) -> WeaponInfo {
        WeaponInfo {
            name: name.to_string(),
            parent: None,
            file_path: PathBuf::from("test.cpp"),
            magazine_wells: wells.iter().map(|s| s.to_string()).collect(),
            compatible_magazines: Vec::new(),
        }
    }

    #[test]
    fn test_magazine_cache_basic_functionality() {
        let mut cache = MagazineWellCache::new();
        
        let magazines = vec!["mag1".to_string(), "mag2".to_string()];
        cache.insert("well1".to_string(), magazines.clone());
        
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
        
        let retrieved = cache.get("well1").unwrap();
        assert_eq!(retrieved, &magazines);
        
        assert_eq!(cache.stats().cache_hits, 1);
        assert_eq!(cache.stats().cache_misses, 0);
        
        // Test cache miss
        assert!(cache.get("nonexistent").is_none());
        assert_eq!(cache.stats().cache_misses, 1);
    }

    #[test]
    fn test_extract_sorted_magazines() {
        let well = create_test_magazine_well("test_well", vec![
            ("group1", vec!["mag_c", "mag_a"]),
            ("group2", vec!["mag_b", "mag_a"]), // Duplicate mag_a
        ]);

        let sorted_magazines = CompatibilityResolver::extract_sorted_magazines(&well);
        
        // Should be sorted and deduplicated
        assert_eq!(sorted_magazines, vec!["mag_a", "mag_b", "mag_c"]);
    }

    #[test]
    fn test_compatibility_resolution_correctness() {
        let mut magazine_wells = HashMap::new();
        
        // Create test magazine wells
        magazine_wells.insert("well1".to_string(), create_test_magazine_well("well1", vec![
            ("group1", vec!["mag1", "mag2"]),
        ]));
        magazine_wells.insert("well2".to_string(), create_test_magazine_well("well2", vec![
            ("group2", vec!["mag3", "mag4"]),
        ]));
        magazine_wells.insert("well3".to_string(), create_test_magazine_well("well3", vec![
            ("group3", vec!["mag2", "mag5"]), // mag2 overlaps with well1
        ]));

        // Create test weapons
        let mut weapons = vec![
            create_test_weapon("weapon1", vec!["well1"]),
            create_test_weapon("weapon2", vec!["well1", "well2"]),
            create_test_weapon("weapon3", vec!["well2", "well3"]),
            create_test_weapon("weapon4", vec![]), // No wells
        ];

        // Run compatibility resolution
        CompatibilityResolver::resolve_compatibility_optimized(&mut weapons, &magazine_wells);

        // Verify results
        assert_eq!(weapons[0].compatible_magazines, vec!["mag1", "mag2"]);
        assert_eq!(weapons[1].compatible_magazines, vec!["mag1", "mag2", "mag3", "mag4"]);
        assert_eq!(weapons[2].compatible_magazines, vec!["mag2", "mag3", "mag4", "mag5"]);
        assert_eq!(weapons[3].compatible_magazines, Vec::<String>::new());
    }

    #[test]
    fn test_performance_metrics() {
        let mut magazine_wells = HashMap::new();
        magazine_wells.insert("well1".to_string(), create_test_magazine_well("well1", vec![
            ("group1", vec!["mag1", "mag2", "mag3"]),
        ]));

        let weapons = vec![
            create_test_weapon("weapon1", vec!["well1"]),
            create_test_weapon("weapon2", vec!["well1"]),
        ];

        let metrics = CompatibilityResolver::get_performance_metrics(&weapons, &magazine_wells);
        
        assert!(metrics.contains_key("avg_magazines_per_well"));
        assert!(metrics.contains_key("avg_wells_per_weapon"));
        assert!(metrics.contains_key("theoretical_speedup"));
        
        // Verify calculated values
        assert_eq!(metrics["avg_magazines_per_well"], 3.0);
        assert_eq!(metrics["avg_wells_per_weapon"], 1.0);
    }

    #[test]
    fn test_compatibility_stats() {
        let mut magazine_wells = HashMap::new();
        magazine_wells.insert("well1".to_string(), create_test_magazine_well("well1", vec![
            ("group1", vec!["mag1", "mag2"]),
        ]));

        let mut weapons = vec![
            create_test_weapon("weapon1", vec!["well1"]),
            create_test_weapon("weapon2", vec!["well1"]),
        ];

        // Manually set compatible magazines for testing
        weapons[0].compatible_magazines = vec!["mag1".to_string(), "mag2".to_string()];
        weapons[1].compatible_magazines = vec!["mag1".to_string(), "mag2".to_string()];

        let stats = CompatibilityResolver::get_compatibility_stats(&weapons, &magazine_wells);
        
        assert_eq!(stats["total_weapons"], 2);
        assert_eq!(stats["total_magazine_wells"], 1);
        assert_eq!(stats["unique_wells_used"], 1);
        assert_eq!(stats["most_popular_well_usage"], 2);
        assert_eq!(stats["total_magazine_mappings"], 4);
    }

    #[test]
    fn test_empty_data_handling() {
        let mut empty_weapons = Vec::new();
        let empty_wells = HashMap::new();

        // Should not panic with empty data
        CompatibilityResolver::resolve_compatibility_optimized(&mut empty_weapons, &empty_wells);

        let mut weapons = vec![create_test_weapon("weapon1", vec![])];
        CompatibilityResolver::resolve_compatibility_optimized(&mut weapons, &empty_wells);
        
        assert!(weapons[0].compatible_magazines.is_empty());
    }

    #[test] 
    fn test_large_dataset_simulation() {
        use std::time::Instant;

        let mut magazine_wells = HashMap::new();
        
        // Create 50 magazine wells with 10 magazines each
        for i in 0..50 {
            let well_name = format!("well_{}", i);
            let magazines: Vec<&str> = (0..10).map(|j| Box::leak(format!("mag_{}_{}", i, j).into_boxed_str())).collect();
            magazine_wells.insert(well_name.clone(), create_test_magazine_well(&well_name, vec![
                ("group1", magazines),
            ]));
        }

        // Create 200 weapons, each using 3-5 random wells
        let mut weapons = Vec::new();
        for i in 0..200 {
            let well_count = 3 + (i % 3); // 3-5 wells per weapon
            let wells: Vec<&str> = (0..well_count).map(|j| {
                let well_idx = (i * 7 + j) % 50; // Pseudo-random well selection
                Box::leak(format!("well_{}", well_idx).into_boxed_str())
            }).collect();
            weapons.push(create_test_weapon(&format!("weapon_{}", i), wells));
        }

        // Measure performance
        let start = Instant::now();
        CompatibilityResolver::resolve_compatibility_optimized(&mut weapons, &magazine_wells);
        let duration = start.elapsed();

        println!("Processed {} weapons with {} wells in {:?}", 
                weapons.len(), magazine_wells.len(), duration);

        // Verify all weapons got their magazines
        for weapon in &weapons {
            if !weapon.magazine_wells.is_empty() {
                assert!(!weapon.compatible_magazines.is_empty(), 
                       "Weapon {} should have compatible magazines", weapon.name);
            }
        }

        // Should complete in reasonable time (< 100ms for this small test dataset)
        assert!(duration.as_millis() < 100, "Performance test took too long: {:?}", duration);
    }
}
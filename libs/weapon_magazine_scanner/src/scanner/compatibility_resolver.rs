use std::collections::{HashMap, HashSet};
use crate::models::{WeaponInfo, MagazineWellInfo};
use super::MagazineExtractor;

const INITIAL_MAGAZINE_CAPACITY: usize = 32; // Typical number of magazines per weapon

/// Cache for pre-computed magazine well data
#[derive(Debug, Clone)]
pub struct MagazineWellCache {
    well_magazines: HashMap<String, Vec<String>>,
}

/// Statistics about magazine compatibility
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_weapons: usize,
    pub total_magazine_wells: usize,
    pub most_used_wells: usize,
    pub most_popular_well_usage: usize,
}

/// Handles resolution of magazine compatibility between weapons and magazine wells
pub struct CompatibilityResolver {
    cache: MagazineWellCache,
}

impl MagazineWellCache {
    /// Create new cache from magazine wells
    fn new(magazine_wells: &HashMap<String, MagazineWellInfo>) -> Self {
        let well_magazines = magazine_wells.iter()
            .map(|(name, well)| {
                (name.clone(), MagazineExtractor::get_all_magazines_from_well(well))
            })
            .collect();

        Self { well_magazines }
    }

    /// Get compatible magazines for a well
    fn get_magazines(&self, well_name: &str) -> Option<&Vec<String>> {
        self.well_magazines.get(well_name)
    }
}

impl CompatibilityResolver {
    /// Create new resolver with magazine well cache
    pub fn new(magazine_wells: &HashMap<String, MagazineWellInfo>) -> Self {
        Self {
            cache: MagazineWellCache::new(magazine_wells)
        }
    }

    /// Resolve magazine compatibility for weapons using cached data
    pub fn resolve_compatibility(&self, weapons: &mut [WeaponInfo]) {
        let total_weapons = weapons.len();
        log::info!("Starting compatibility resolution for {} weapons", total_weapons);
        
        for (index, weapon) in weapons.iter_mut().enumerate() {
            if index % 100 == 0 {  // Log progress every 100 weapons
                log::info!("Resolving compatibility: {}/{} weapons processed", index, total_weapons);
            }
            
            let weapon_name = weapon.name.clone();
            let well_count = weapon.magazine_wells.len();
            log::debug!("Processing weapon: {} (with {} magazine wells)",
                weapon_name, well_count);
            
            // Calculate total magazines for pre-allocation
            let total_capacity = weapon.magazine_wells.iter()
                .filter_map(|well_name| self.cache.get_magazines(well_name))
                .map(|magazines| magazines.len())
                .sum::<usize>()
                .max(INITIAL_MAGAZINE_CAPACITY);

            let mut compatible_magazines = HashSet::with_capacity(total_capacity);
            
            // Efficiently collect all compatible magazines
            for well_name in &weapon.magazine_wells {
                if let Some(magazines) = self.cache.get_magazines(well_name) {
                    log::debug!("Found {} compatible magazines in well '{}'",
                        magazines.len(), well_name);
                    compatible_magazines.extend(magazines.iter().cloned());
                } else {
                    log::warn!("No magazines found for well '{}' in weapon '{}'",
                        well_name, weapon_name);
                }
            }
            
            // Convert to sorted vector with exact capacity
            let mut magazines_vec = Vec::with_capacity(compatible_magazines.len());
            magazines_vec.extend(compatible_magazines);
            magazines_vec.sort_unstable(); // Faster than standard sort
            
            log::debug!("Resolved {} total compatible magazines for weapon '{}'",
                magazines_vec.len(), weapon_name);
                
            weapon.compatible_magazines = magazines_vec;
        }
        
        log::info!("Compatibility resolution completed for all weapons");
    }

    /// Get magazine compatibility statistics using cached data
    pub fn get_stats(&self, weapons: &[WeaponInfo]) -> CacheStats {
        // Use single pass counting for efficiency
        let mut well_usage = HashMap::new();
        let mut max_usage = 0;
        
        // Count usage in single pass
        for weapon in weapons {
            for well_name in &weapon.magazine_wells {
                let count = well_usage.entry(well_name.clone()).or_insert(0);
                *count += 1;
                max_usage = max_usage.max(*count);
            }
        }
        
        CacheStats {
            total_weapons: weapons.len(),
            total_magazine_wells: self.cache.well_magazines.len(),
            most_used_wells: well_usage.len(),
            most_popular_well_usage: max_usage,
        }
    }
}

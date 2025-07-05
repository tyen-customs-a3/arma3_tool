use anyhow::Result;
use sqf_analyzer::{Args, analyze_sqf};
use std::path::PathBuf;
use std::collections::HashSet;

#[test]
fn test_fn_assigngear_curated_extraction() -> Result<()> {
    // Create args for analyzing the fn_assignGearCurated.sqf file
    let mut args = Args {
        path: PathBuf::from("test-files/fn_assignGearCurated.sqf"),
        output: "text".to_string(),
        full_paths: false,
        include_vars: false,
        
        functions: None,
    };
    
    // Test general extraction
    let all_items = analyze_sqf(&args)?;
    
    // Check for specific items we know should be in the file
    let expected_specific_items = [
        "ACE_fieldDressing",
        "ACE_packingBandage",
        "ACE_epinephrine",
        "ACE_morphine",
        "ACE_tourniquet",
        "ACE_splint",
        "rhs_mag_rgd5",
        "rhs_mag_rdg2_white",
        "rhs_tsh4",
        "rhs_rpg_empty",
        "rhs_weap_rpg7",
        "rhs_rpg7_PG7VL_mag",
    ];
    
    for item in &expected_specific_items {
        assert!(all_items.contains(*item), "Failed to extract expected item: {}", item);
    }
    
    // Test with all equipment functions
    args.functions = Some("addItemToUniform,addItemToVest,addItemToBackpack,addItem,addWeapon,addWeaponItem,addMagazine,addMagazineCargo,addWeaponCargo,addItemCargo,forceAddUniform,addVest,addHeadgear,addGoggles,addBackpack".to_string());
    
    let uniform_items = analyze_sqf(&args)?;
    
    // Should contain only the items added to uniform
    let expected_uniform_items = [
        "ACE_fieldDressing",
        "ACE_packingBandage", 
        "ACE_epinephrine",
        "ACE_morphine",
        "ACE_tourniquet",
        "ACE_splint",
    ];
    
    for item in &expected_uniform_items {
        assert!(uniform_items.contains(*item), "Failed to extract expected uniform item: {}", item);
    }
    
    // Check that items not added to uniform aren't included
    assert!(!uniform_items.contains("rhs_mag_rgd5"), "Found magazine item in uniform items");
    
    // Test with another function (addMagazine)
    args.functions = Some("addMagazine".to_string());
    let magazine_items = analyze_sqf(&args)?;
    
    let expected_magazine_items = [
        "rhs_mag_rgd5",
        "rhs_mag_rdg2_white",
    ];
    
    for item in &expected_magazine_items {
        assert!(magazine_items.contains(*item), "Failed to extract expected magazine: {}", item);
    }
    
    // Make sure uniform items aren't in the magazine results
    assert!(!magazine_items.contains("ACE_fieldDressing"), "Found uniform item in magazine items");
    
    // Test with multiple specific functions
    args.functions = Some("addItemToBackpack,addWeapon".to_string());
    let backpack_weapon_items = analyze_sqf(&args)?;
    
    let expected_backpack_weapon_items = [
        "rhs_rpg7_PG7VL_mag", // from addItemToBackpack
        "rhs_weap_rpg7",      // from addWeapon
    ];
    
    for item in &expected_backpack_weapon_items {
        assert!(backpack_weapon_items.contains(*item), "Failed to extract expected backpack/weapon item: {}", item);
    }
    
    Ok(())
}

#[test]
fn test_whitehead_extraction() -> Result<()> {
    // Create args for analyzing the fn_assignGearCurated.sqf file
    let args = Args {
        path: PathBuf::from("test-files/fn_assignGearCurated.sqf"),
        output: "text".to_string(),
        full_paths: false,
        include_vars: true, // Include all items to ensure we find the WhiteHead values
        functions: None, // No function filter to get all string literals
    };
    
    // Run the analyzer
    let all_items = analyze_sqf(&args)?;
    
    // Check that we extract all the WhiteHead items from the face array
    let whitehead_items: Vec<_> = all_items.iter()
        .filter(|item| item.starts_with("WhiteHead_"))
        .collect();
    
    // There should be 31 WhiteHead items
    assert_eq!(whitehead_items.len(), 31, "Failed to extract expected number of WhiteHead items");
    
    // Check for some specific WhiteHead items
    assert!(all_items.contains("WhiteHead_01"), "Missing WhiteHead_01");
    assert!(all_items.contains("WhiteHead_15"), "Missing WhiteHead_15");
    assert!(all_items.contains("WhiteHead_32"), "Missing WhiteHead_32");
    
    Ok(())
}

#[test]
fn test_exact_extraction_count() -> Result<()> {
    // Test that we get exactly the items we expect from fn_assignGearCurated.sqf
    // using different extraction modes
    
    // Setup args
    let mut args = Args {
        path: PathBuf::from("test-files/fn_assignGearCurated.sqf"),
        output: "text".to_string(),
        full_paths: false,
        include_vars: false,
        
        functions: None,
    };
    
    // 1. Test with addItemToUniform function only
    args.functions = Some("addItemToUniform".to_string());
    let uniform_items = analyze_sqf(&args)?;
    assert_eq!(uniform_items.len(), 6, "Expected exactly 6 uniform items");
    
    // 2. Test with addMagazine function only
    args.functions = Some("addMagazine".to_string());
    let magazine_items = analyze_sqf(&args)?;
    assert_eq!(magazine_items.len(), 2, "Expected exactly 2 magazine items");
    
    // 3. Test with addHeadgear function only
    args.functions = Some("addHeadgear".to_string());
    let headgear_items = analyze_sqf(&args)?;
    assert_eq!(headgear_items.len(), 1, "Expected exactly 1 headgear item");
    assert!(headgear_items.contains("rhs_tsh4"), "Expected rhs_tsh4 in headgear items");
    
    // 4. Test with all equipment functions
    args.functions = Some("addItemToUniform,addItemToVest,addItemToBackpack,addItem,addWeapon,addWeaponItem,addMagazine,addMagazineCargo,addWeaponCargo,addItemCargo,forceAddUniform,addVest,addHeadgear,addGoggles,addBackpack".to_string());
    let equipment_items = analyze_sqf(&args)?;
    assert!(equipment_items.len() >= 9, "Expected at least 9 equipment items");
    
    // Verify each set of items contains exactly what we expect
    let expected_uniform_items = HashSet::from([
        "ACE_fieldDressing".to_string(),
        "ACE_packingBandage".to_string(),
        "ACE_epinephrine".to_string(),
        "ACE_morphine".to_string(),
        "ACE_tourniquet".to_string(),
        "ACE_splint".to_string(),
    ]);
    
    let expected_magazine_items = HashSet::from([
        "rhs_mag_rgd5".to_string(),
        "rhs_mag_rdg2_white".to_string(),
    ]);
    
    let expected_backpack_weapon_items = HashSet::from([
        "rhs_rpg7_PG7VL_mag".to_string(),
        "rhs_weap_rpg7".to_string(),
        "rhs_rpg_empty".to_string(),
    ]);
    
    args.functions = Some("addItemToUniform".to_string());
    let actual_uniform_items = analyze_sqf(&args)?;
    assert_eq!(actual_uniform_items, expected_uniform_items, "Uniform items don't match expected set");
    
    args.functions = Some("addMagazine".to_string());
    let actual_magazine_items = analyze_sqf(&args)?;
    assert_eq!(actual_magazine_items, expected_magazine_items, "Magazine items don't match expected set");
    
    args.functions = Some("addBackpack,addWeapon,addItemToBackpack".to_string());
    let actual_backpack_weapon_items = analyze_sqf(&args)?;
    for item in &expected_backpack_weapon_items {
        assert!(actual_backpack_weapon_items.contains(item), "Missing expected backpack/weapon item: {}", item);
    }
    
    Ok(())
} 
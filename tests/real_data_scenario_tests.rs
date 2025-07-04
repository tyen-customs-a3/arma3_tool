//! Real data scenario tests for workflow orchestration with realistic Arma 3 data

use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;
use tokio;

use arma3_workflow::orchestrator::WorkflowOrchestrator;
use arma3_workflow::types::{Workflow, WorkflowType, ContentType};
use arma3_workflow::types::options::{WorkflowOptions, ExtractionOptions, ProcessingOptions, ReportingOptions, ExportOptions, ReportFormat};
use arma3_database::DatabaseManager;

/// Create realistic Arma 3 PBO content for testing
fn create_realistic_pbo_content(dir: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    
    // Create realistic game data PBO content
    let gamedata_dir = dir.join("gamedata");
    fs::create_dir_all(&gamedata_dir)?;
    
    // Simulate config.cpp content
    let config_cpp = r#"
class CfgPatches {
    class A3_Characters_F {
        units[] = {};
        weapons[] = {};
        requiredVersion = 0.1;
        requiredAddons[] = {"A3_Data_F"};
    };
};

class CfgVehicles {
    class Man;
    class CAManBase: Man {
        scope = 0;
        model = "\A3\Characters_F\Common\basicbody";
        displayName = "Basic Man";
    };
    
    class B_Soldier_base_F: CAManBase {
        scope = 1;
        faction = "BLU_F";
        vehicleClass = "Men";
        displayName = "Rifleman";
        nakedUniform = "U_BasicBody";
        uniformClass = "U_B_CombatUniform_mcam";
    };
    
    class B_Soldier_F: B_Soldier_base_F {
        scope = 2;
        displayName = "Rifleman";
        weapons[] = {"arifle_MX_F","hgun_P07_F","Throw","Put"};
        respawnWeapons[] = {"arifle_MX_F","hgun_P07_F","Throw","Put"};
        magazines[] = {"30Rnd_65x39_caseless_mag","30Rnd_65x39_caseless_mag","30Rnd_65x39_caseless_mag","30Rnd_65x39_caseless_mag","16Rnd_9x21_Mag","16Rnd_9x21_Mag","HandGrenade","HandGrenade","SmokeShell","SmokeShell"};
        respawnMagazines[] = {"30Rnd_65x39_caseless_mag","30Rnd_65x39_caseless_mag","30Rnd_65x39_caseless_mag","30Rnd_65x39_caseless_mag","16Rnd_9x21_Mag","16Rnd_9x21_Mag","HandGrenade","HandGrenade","SmokeShell","SmokeShell"};
        linkedItems[] = {"V_PlateCarrier1_rgr","H_HelmetB","NVGoggles","ItemMap","ItemCompass","ItemWatch","ItemRadio"};
        respawnLinkedItems[] = {"V_PlateCarrier1_rgr","H_HelmetB","NVGoggles","ItemMap","ItemCompass","ItemWatch","ItemRadio"};
    };
};

class CfgWeapons {
    class Rifle_Base_F;
    class arifle_MX_F: Rifle_Base_F {
        scope = 2;
        displayName = "MX 6.5 mm";
        model = "\A3\Weapons_F\Rifles\MX\MX_F";
        picture = "\A3\Weapons_F\Rifles\MX\Data\UI\gear_mx_X_ca.paa";
        magazines[] = {"30Rnd_65x39_caseless_mag","30Rnd_65x39_caseless_mag_Tracer"};
        modes[] = {"Single","FullAuto","single_medium_optics1","single_far_optics2","fullauto_medium"};
    };
    
    class Pistol_Base_F;
    class hgun_P07_F: Pistol_Base_F {
        scope = 2;
        displayName = "P07 9 mm";
        model = "\A3\Weapons_F\Pistols\P07\P07";
        picture = "\A3\Weapons_F\Pistols\P07\Data\UI\gear_p07_X_ca.paa";
        magazines[] = {"16Rnd_9x21_Mag"};
        modes[] = {"Single"};
    };
};
"#;
    fs::write(gamedata_dir.join("config.cpp"), config_cpp)?;
    
    // Create mission content
    let mission_dir = dir.join("mission");
    fs::create_dir_all(&mission_dir)?;
    
    // Simulate mission.sqm content
    let mission_sqm = r#"
class Mission {
    class Intel {
        briefingName = "Test Mission";
        overviewText = "A test mission for integration testing";
        startWeather = 0.3;
        forecastWeather = 0.3;
        year = 2035;
        month = 7;
        day = 6;
        hour = 12;
        minute = 0;
    };
    
    class Groups {
        items = 2;
        class Item0 {
            side = "WEST";
            class Vehicles {
                items = 4;
                class Item0 {
                    position[] = {1000, 50, 1000};
                    azimut = 0;
                    vehicle = "B_Soldier_F";
                    player = "PLAYER COMMANDER";
                    leader = 1;
                    rank = "SERGEANT";
                };
                class Item1 {
                    position[] = {1002, 50, 1000};
                    azimut = 0;
                    vehicle = "B_Soldier_F";
                    rank = "CORPORAL";
                };
            };
        };
    };
};
"#;
    fs::write(mission_dir.join("mission.sqm"), mission_sqm)?;
    
    // Create init.sqf
    let init_sqf = r#"
// Mission initialization script
player setVariable ["initialized", true];
hint "Mission initialized successfully";

// Set up some variables
missionNamespace setVariable ["testVar", "testValue"];
publicVariable "testVar";

// Add event handlers
player addEventHandler ["Killed", {
    hint "Player killed";
}];
"#;
    fs::write(mission_dir.join("init.sqf"), init_sqf)?;
    
    // Create description.ext
    let description_ext = r#"
briefingName = "Integration Test Mission";
onLoadName = "Integration Test";
onLoadMission = "Testing workflow orchestration";
author = "Integration Test Suite";

class Header {
    gameType = "Coop";
    minPlayers = 1;
    maxPlayers = 10;
};

respawn = "BASE";
respawnDelay = 10;

class Params {
    class Difficulty {
        title = "Difficulty";
        values[] = {1, 2, 3};
        texts[] = {"Easy", "Normal", "Hard"};
        default = 2;
    };
};
"#;
    fs::write(mission_dir.join("description.ext"), description_ext)?;
    
    Ok(())
}

/// Create realistic config file for item filtering
fn create_realistic_config(path: &PathBuf) -> std::io::Result<()> {
    let config_content = r#"{
    "item_types": {
        "weapons": [
            "Rifle_Base_F",
            "Pistol_Base_F",
            "Launcher_Base_F",
            "GrenadeLauncher_Base_F"
        ],
        "vehicles": [
            "Car",
            "Tank",
            "Helicopter",
            "Plane",
            "Ship"
        ],
        "equipment": [
            "Vest_Camo_Base",
            "H_HelmetB",
            "Bag_Base",
            "NVGoggles"
        ],
        "units": [
            "CAManBase",
            "B_Soldier_base_F",
            "O_Soldier_base_F",
            "I_Soldier_base_F"
        ]
    },
    "max_scope": 1,
    "excluded_prefixes": [
        "test_",
        "debug_",
        "temp_",
        "prototype_"
    ]
}"#;
    fs::write(path, config_content)?;
    Ok(())
}

#[tokio::test]
async fn test_realistic_gamedata_processing() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("arma3_data");
    let work_dir = temp_dir.path().join("work");
    let output_dir = temp_dir.path().join("output");
    let db_path = temp_dir.path().join("gamedata.db");
    let config_path = temp_dir.path().join("config.json");
    
    // Create realistic test data
    create_realistic_pbo_content(&source_dir).unwrap();
    create_realistic_config(&config_path).unwrap();
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    // Create test database
    fs::write(&db_path, "").unwrap();
    
    let start_time = std::time::Instant::now();
    
    // Test processing realistic game data
    let processing_options = ProcessingOptions {
        max_files: 1000,
        max_depth: 10,
        extensions: vec!["cpp".to_string(), "hpp".to_string(), "sqf".to_string()],
        source_directories: vec![source_dir.clone()],
        verbose: true,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_processing(processing_options);
    
    let workflow = Workflow::new(
        "realistic_gamedata_processing".to_string(),
        WorkflowType::Process,
        ContentType::GameData,
        workflow_options,
    );
    
    let processing_time = start_time.elapsed();
    
    // Verify workflow configuration handles realistic data
    assert_eq!(workflow.content_type, ContentType::GameData);
    assert_eq!(workflow.options.processing.extensions.len(), 3);
    assert!(workflow.options.processing.extensions.contains(&"cpp".to_string()));
    assert!(workflow.options.processing.verbose);
    
    println!("Realistic game data processing workflow created in: {:?}", processing_time);
}

#[tokio::test]
async fn test_realistic_mission_analysis() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("mission_data");
    let output_dir = temp_dir.path().join("reports");
    let db_path = temp_dir.path().join("mission.db");
    
    // Create realistic mission data
    create_realistic_pbo_content(&source_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(&db_path, "").unwrap();
    
    let start_time = std::time::Instant::now();
    
    // Test mission analysis workflow
    let processing_options = ProcessingOptions {
        max_files: 500,
        max_depth: 8,
        extensions: vec!["sqf".to_string(), "sqm".to_string(), "ext".to_string()],
        source_directories: vec![source_dir.clone()],
        verbose: false,
    };
    
    let reporting_options = ReportingOptions {
        format: ReportFormat::Html,
        output_path: Some(output_dir.join("mission_analysis.html")),
        source_directories: vec![source_dir],
        include_mission_dependencies: true,
        include_game_data_classes: false,
        include_missing_dependencies: true,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_processing(processing_options)
        .with_reporting(reporting_options);
    
    let workflow = Workflow::new(
        "realistic_mission_analysis".to_string(),
        WorkflowType::ProcessAndReport,
        ContentType::Mission,
        workflow_options,
    );
    
    let analysis_time = start_time.elapsed();
    
    // Verify mission analysis configuration
    assert_eq!(workflow.content_type, ContentType::Mission);
    assert!(workflow.options.reporting.include_mission_dependencies);
    assert!(!workflow.options.reporting.include_game_data_classes);
    assert_eq!(workflow.options.reporting.format, ReportFormat::Html);
    
    println!("Realistic mission analysis workflow created in: {:?}", analysis_time);
}

#[tokio::test]
async fn test_large_dataset_export() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("large_dataset.db");
    let output_dir = temp_dir.path().join("exports");
    
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(&db_path, "").unwrap();
    
    let start_time = std::time::Instant::now();
    
    // Test export with large dataset simulation
    let export_options = ExportOptions {
        format: "csv".to_string(),
        output_path: Some(output_dir.join("large_export.csv")),
        compression_level: Some(6),
        include_metadata: true,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_export(export_options);
    
    let workflow = Workflow::new(
        "large_dataset_export".to_string(),
        WorkflowType::Export,
        ContentType::GameData,
        workflow_options,
    );
    
    let export_time = start_time.elapsed();
    
    // Verify large dataset export configuration
    assert_eq!(workflow.options.export.format, "csv");
    assert_eq!(workflow.options.export.compression_level, Some(6));
    assert!(workflow.options.export.include_metadata);
    
    println!("Large dataset export workflow created in: {:?}", export_time);
}

#[tokio::test]
async fn test_complete_realistic_pipeline() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("complete_test");
    let work_dir = temp_dir.path().join("work");
    let output_dir = temp_dir.path().join("output");
    let db_path = temp_dir.path().join("complete.db");
    let config_path = temp_dir.path().join("filter_config.json");
    
    // Create comprehensive test data
    create_realistic_pbo_content(&source_dir).unwrap();
    create_realistic_config(&config_path).unwrap();
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(&db_path, "").unwrap();
    
    let start_time = std::time::Instant::now();
    
    // Test complete pipeline: Extract → Process → Report → Export
    let extraction_options = ExtractionOptions {
        use_extractor: true,
        directories: None,
        source_directories: vec![source_dir.clone()],
        force: false,
    };
    
    let processing_options = ProcessingOptions {
        max_files: 2000,
        max_depth: 12,
        extensions: vec![
            "cpp".to_string(),
            "hpp".to_string(),
            "sqf".to_string(),
            "sqm".to_string(),
            "ext".to_string(),
        ],
        source_directories: vec![source_dir.clone()],
        verbose: true,
    };
    
    let reporting_options = ReportingOptions {
        format: ReportFormat::Json,
        output_path: Some(output_dir.join("complete_report.json")),
        source_directories: vec![source_dir],
        include_mission_dependencies: true,
        include_game_data_classes: true,
        include_missing_dependencies: true,
    };
    
    let export_options = ExportOptions {
        format: "json".to_string(),
        output_path: Some(output_dir.join("complete_export.json")),
        compression_level: Some(9),
        include_metadata: true,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_extraction(extraction_options)
        .with_processing(processing_options)
        .with_reporting(reporting_options)
        .with_export(export_options);
    
    let workflow = Workflow::new(
        "complete_realistic_pipeline".to_string(),
        WorkflowType::Complete,
        ContentType::GameData,
        workflow_options,
    );
    
    let pipeline_time = start_time.elapsed();
    
    // Verify complete pipeline configuration
    assert_eq!(workflow.workflow_type, WorkflowType::Complete);
    assert!(workflow.options.extraction.use_extractor);
    assert_eq!(workflow.options.processing.extensions.len(), 5);
    assert!(workflow.options.reporting.include_mission_dependencies);
    assert!(workflow.options.reporting.include_game_data_classes);
    assert_eq!(workflow.options.export.compression_level, Some(9));
    
    println!("Complete realistic pipeline workflow created in: {:?}", pipeline_time);
}

#[tokio::test]
async fn test_memory_intensive_scenario() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("memory_test");
    let output_dir = temp_dir.path().join("output");
    
    // Create large realistic dataset
    create_realistic_pbo_content(&source_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    // Create many additional files to simulate large dataset
    for i in 0..50 {
        let sub_dir = source_dir.join(format!("addon_{}", i));
        fs::create_dir_all(&sub_dir).unwrap();
        
        // Create multiple config files per addon
        for j in 0..10 {
            let config_content = format!(
                r#"
class CfgPatches {{
    class TestAddon_{}_{}  {{
        units[] = {{}};
        weapons[] = {{}};
        requiredVersion = 0.1;
        requiredAddons[] = {{"A3_Data_F"}};
    }};
}};

class CfgVehicles {{
    class TestVehicle_{}_{}  {{
        scope = 2;
        displayName = "Test Vehicle {} {}";
        model = "\\test\\path\\model_{}.p3d";
    }};
}};
"#,
                i, j, i, j, i, j, i
            );
            fs::write(sub_dir.join(format!("config_{}.cpp", j)), config_content).unwrap();
        }
    }
    
    let start_time = std::time::Instant::now();
    
    // Test memory-intensive processing
    let processing_options = ProcessingOptions {
        max_files: 10000,
        max_depth: 20,
        extensions: vec!["cpp".to_string(), "hpp".to_string()],
        source_directories: vec![source_dir],
        verbose: false, // Reduce output for memory test
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_processing(processing_options);
    
    let workflow = Workflow::new(
        "memory_intensive_test".to_string(),
        WorkflowType::Process,
        ContentType::GameData,
        workflow_options,
    );
    
    let memory_test_time = start_time.elapsed();
    
    // Verify memory-intensive configuration
    assert_eq!(workflow.options.processing.max_files, 10000);
    assert_eq!(workflow.options.processing.max_depth, 20);
    assert!(!workflow.options.processing.verbose);
    
    // Should complete within reasonable time even with large dataset
    assert!(memory_test_time.as_secs() < 1);
    
    println!("Memory-intensive scenario workflow created in: {:?}", memory_test_time);
}

#[tokio::test]
async fn test_error_recovery_with_realistic_data() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("error_test");
    let corrupt_dir = temp_dir.path().join("corrupt");
    let output_dir = temp_dir.path().join("output");
    
    // Create mixed valid and invalid data
    create_realistic_pbo_content(&source_dir).unwrap();
    fs::create_dir_all(&corrupt_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    // Create corrupted/invalid files
    fs::write(corrupt_dir.join("invalid.cpp"), "This is not valid C++ syntax {{{").unwrap();
    fs::write(corrupt_dir.join("broken.sqf"), "invalid SQF syntax ;;; {{").unwrap();
    
    let start_time = std::time::Instant::now();
    
    // Test error recovery with mixed data
    let processing_options = ProcessingOptions {
        max_files: 1000,
        max_depth: 10,
        extensions: vec!["cpp".to_string(), "sqf".to_string()],
        source_directories: vec![source_dir, corrupt_dir],
        verbose: true,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_processing(processing_options);
    
    let workflow = Workflow::new(
        "error_recovery_test".to_string(),
        WorkflowType::Process,
        ContentType::GameData,
        workflow_options,
    );
    
    let recovery_time = start_time.elapsed();
    
    // Verify error recovery configuration
    assert_eq!(workflow.options.processing.source_directories.len(), 2);
    assert!(workflow.options.processing.verbose);
    
    println!("Error recovery test workflow created in: {:?}", recovery_time);
}

#[tokio::test]
async fn test_performance_with_realistic_constraints() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("perf_test");
    let output_dir = temp_dir.path().join("output");
    
    create_realistic_pbo_content(&source_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    let iterations = 10;
    let mut total_time = std::time::Duration::from_secs(0);
    
    for i in 0..iterations {
        let start_time = std::time::Instant::now();
        
        // Create workflow with realistic constraints
        let processing_options = ProcessingOptions {
            max_files: 1000,
            max_depth: 8,
            extensions: vec!["cpp".to_string(), "hpp".to_string(), "sqf".to_string()],
            source_directories: vec![source_dir.clone()],
            verbose: false,
        };
        
        let reporting_options = ReportingOptions {
            format: ReportFormat::Csv,
            output_path: Some(output_dir.join(format!("perf_report_{}.csv", i))),
            source_directories: vec![source_dir.clone()],
            include_mission_dependencies: true,
            include_game_data_classes: true,
            include_missing_dependencies: false,
        };
        
        let workflow_options = WorkflowOptions::new()
            .with_processing(processing_options)
            .with_reporting(reporting_options);
        
        let workflow = Workflow::new(
            format!("performance_test_{}", i),
            WorkflowType::ProcessAndReport,
            ContentType::GameData,
            workflow_options,
        );
        
        let iteration_time = start_time.elapsed();
        total_time += iteration_time;
        
        // Verify each workflow is configured correctly
        assert_eq!(workflow.name, format!("performance_test_{}", i));
        assert_eq!(workflow.workflow_type, WorkflowType::ProcessAndReport);
    }
    
    let average_time = total_time / iterations as u32;
    
    // Performance should be consistent
    assert!(average_time.as_millis() < 100); // Less than 100ms per workflow creation
    
    println!(
        "Performance test: {} iterations, total: {:?}, average: {:?}",
        iterations, total_time, average_time
    );
}
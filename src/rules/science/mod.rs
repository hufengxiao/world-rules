//! 科学规则模块 - 涵盖自然科学、社会科学和工程科学的基础规则
//!
//! 本模块包含科学领域的基础规则和定律，覆盖：
//! - **自然科学**: 物理定律、化学定律、生物定律等
//! - **数学规则**: 代数规则、几何规则、概率规则等
//! - **工程科学**: 机械工程、电气工程、土木工程等
//! - **计算机科学**: 算法规则、数据结构规则、网络规则等
//! - **社会科学**: 经济学规则、心理学规则、社会学规则等
//! - **医学科学**: 生理学规则、病理学规则、药理学规则等
//!
//! # 模块结构
//!
//! ```text
//! science/
//! ├── physics          # 物理学
//! ├── chemistry        # 化学
//! ├── biology          # 生物科学
//! ├── math             # 数学
//! ├── computer_science # 计算机科学
//! ├── engineering      # 各类工程科学
//! ├── medical_science  # 医学科学
//! └── social_science   # 社会科学
//! ```
//!
//! # Examples
//!
//! 使用规则示例：
//!
//! ```rust
//! use world_rules::rules::science::{PhysicsRules, ChemistryRules};
//! use world_rules::rules::core::Rule;
//!
//! // 物理规则
//! let physics = PhysicsRules::new();
//! println!("规则: {}", physics.metadata().name);
//! println!("分类: {:?}", physics.category());
//!
//! // 化学规则
//! let chemistry = ChemistryRules::new();
//! let explanation = chemistry.explain();
//! assert!(!explanation.is_empty());
//! ```
//!
//! # 规则统计
//!
//! 当前包含数百条科学规则，覆盖：
//! - 100+ 条物理定律和规则
//! - 80+ 条化学定律和规则
//! - 60+ 条生物定律和规则
//! - 50+ 条数学规则
//! - 40+ 条计算机科学规则
//! - 30+ 条工程科学规则
//! - 20+ 条医学科学规则
//! - 20+ 条社会科学规则

pub mod acoustics;
pub mod acoustics_detailed;
pub mod aerospace_engineering;
pub mod aerospace_engineering_detailed;
pub mod aesthetics;
pub mod ai_ethics;
pub mod air_quality;
pub mod algebra_math;
pub mod algorithms;
pub mod analytical_chemistry;
pub mod analytical_mechanics;
pub mod anatomy;
pub mod animal_behavior;
pub mod antenna_theory;
pub mod anthropology;
pub mod archaeology;
pub mod architecture;
pub mod art_science;
pub mod artificial_intelligence;
pub mod astronomy;
pub mod astrophysics;
pub mod astrophysics_detailed;
pub mod atomic_physics;
pub mod behavioral_economics;
pub mod bilinear_algebra;
pub mod biochemistry;
pub mod biochemistry_detailed;
pub mod biodiversity;
pub mod bioengineering;
pub mod bioethics;
pub mod biogeography;
pub mod bioinformatics;
pub mod biology;
pub mod biomedical_engineering;
pub mod biophysics;
pub mod biostatistics;
pub mod blockchain_detailed;
pub mod blockchain_theory;
pub mod botany;
pub mod botany_detailed;
pub mod calculus;
pub mod carbon_cycle;
pub mod category_theory;
pub mod celestial_mechanics;
pub mod cell_biology;
pub mod cell_biology_detailed;
pub mod chaos_theory;
pub mod chemical_engineering;
pub mod chemical_engineering_detailed;
pub mod chemistry;
pub mod circuit_theory;
pub mod civil_engineering;
pub mod civil_engineering_detailed;
pub mod climate_science;
pub mod climatology;
pub mod cognitive_psychology;
pub mod communication;
pub mod compiler_theory;
pub mod complex_analysis;
pub mod complexity_science;
pub mod computational_complexity;
pub mod computational_linguistics;
pub mod computational_mechanics;
pub mod computer_graphics;
pub mod computer_networks;
pub mod computer_science;
pub mod condensed_matter;
pub mod condensed_matter_detailed;
pub mod conservation_biology;
pub mod control_engineering;
pub mod control_engineering_detailed;
pub mod control_theory;
pub mod cosmology;
pub mod cosmology_detailed;
pub mod cryptography;
pub mod cryptography_detailed;
pub mod crystallography;
pub mod cv_detailed;
pub mod cv_theory;
pub mod cybersecurity;
pub mod data_ethics;
pub mod data_science_detailed;
pub mod data_science_theory;
pub mod data_structures;
pub mod database_theory;
pub mod decision_science;
pub mod deep_learning;
pub mod deep_learning_detailed;
pub mod dentistry;
pub mod dermatology;
pub mod devops_theory;
pub mod differential_equations;
pub mod differential_geometry;
pub mod discrete_math;
pub mod distributed_systems;
pub mod distributed_systems_detailed;
pub mod dynamics;
pub mod ecology;
pub mod ecology_detailed;
pub mod ecology_detailed2;
pub mod economics;
pub mod education;
pub mod electrical_engineering;
pub mod electrical_engineering_detailed;
pub mod electrochemistry;
pub mod electrodynamics;
pub mod electromagnetic_compatibility;
pub mod electromagnetic_induction;
pub mod electromagnetic_wave_propagation;
pub mod electromagnetism;
pub mod electrostatics;
pub mod energy_engineering;
pub mod environmental_engineering;
pub mod environmental_ethics;
pub mod environmental_science;
pub mod environmental_science_detailed;
pub mod epidemiology;
pub mod epidemiology_detailed;
pub mod equation_theory;
pub mod evolution_detailed;
pub mod evolutionary_biology;
pub mod exercise_physiology;
pub mod exercise_science;
pub mod field_theory;
pub mod financial_economics;
pub mod fluid_dynamics;
pub mod fluid_mechanics;
pub mod fluid_mechanics_detailed;
pub mod functional_analysis;
pub mod game_theory;
pub mod general_relativity;
pub mod genetics;
pub mod genetics_detailed;
pub mod genomic_laws;
pub mod geography;
pub mod geology;
pub mod geometry_math;
pub mod geomorphology;
pub mod geophysics;
pub mod geophysics_detailed;
pub mod geoscience;
pub mod geriatrics;
pub mod gis;
pub mod graph_theory;
pub mod group_theory;
pub mod history;
pub mod hydrology;
pub mod immunology;
pub mod immunology_detailed;
pub mod information_science;
pub mod information_theory;
pub mod information_theory_detailed;
pub mod inorganic_chemistry;
pub mod kinematics;
pub mod linear_algebra;
pub mod linguistics;
pub mod literature;
pub mod machine_learning_detailed;
pub mod magnetostatics;
pub mod management_science;
pub mod marine_biology;
pub mod marine_biology_detailed;
pub mod marketing_theory;
pub mod material_science;
pub mod materials_engineering;
pub mod materials_mechanics;
pub mod materials_science_detailed;
pub mod math;
pub mod matrix_algebra;
pub mod maxwell_equations;
pub mod measure_theory;
pub mod mechanical_engineering;
pub mod mechanical_engineering_detailed;
pub mod mechanics_analytical;
pub mod mechanics_classical;
pub mod medical_imaging;
pub mod medical_science;
pub mod meteorology_detailed;
pub mod microbiology;
pub mod microbiology_detailed;
pub mod microwave_technology;
pub mod mineralogy;
pub mod mineralogy_detailed;
pub mod ml_theory;
pub mod module_theory;
pub mod molecular_biology;
pub mod music_theory;
pub mod musicology;
pub mod nanotechnology;
pub mod nanotechnology_detailed;
pub mod network_science;
pub mod neuroscience;
pub mod neuroscience_detailed;
pub mod neuroscience_detailed2;
pub mod nlp_detailed;
pub mod nlp_theory;
pub mod noise_pollution_science;
pub mod norm_theory;
pub mod nuclear_physics;
pub mod nuclear_physics_detailed;
pub mod number_theory;
pub mod numerical_analysis;
pub mod nutrition_science_detailed;
pub mod obstetrics;
pub mod oceanography;
pub mod oceanography_detailed;
pub mod operating_systems;
pub mod ophthalmology;
pub mod optical_basics;
pub mod optics;
pub mod optics_detailed;
pub mod optimization;
pub mod organic_chemistry;
pub mod organizational_behavior;
pub mod paleontology;
pub mod paleontology_detailed;
pub mod paleontology_earth;
pub mod particle_physics;
pub mod particle_physics_detailed;
pub mod particle_physics_standard_model;
pub mod pathology;
pub mod pediatrics;
pub mod petrology;
pub mod pharmacology;
pub mod pharmacology_detailed;
pub mod philosophy;
pub mod photochemistry;
pub mod physical_chemistry;
pub mod physics;
pub mod physiology;
pub mod plant_physiology;
pub mod plasma_physics;
pub mod plasma_physics_detailed;
pub mod political_science;
pub mod polymer_chemistry;
pub mod polynomial_algebra;
pub mod probability_theory;
pub mod proteomics;
pub mod psychiatry;
pub mod psychology;
pub mod quantum_computing;
pub mod quantum_computing_detailed;
pub mod quantum_field_theory;
pub mod quantum_mechanics;
pub mod quantum_mechanics_detailed;
pub mod real_analysis;
pub mod relativity;
pub mod relativity_general;
pub mod relativity_special;
pub mod remote_sensing;
pub mod renewable_energy;
pub mod research_ethics;
pub mod rigid_body_dynamics;
pub mod ring_theory;
pub mod robotics_detailed;
pub mod robotics_theory;
pub mod seismology;
pub mod signal_processing;
pub mod social_psychology;
pub mod sociology;
pub mod software_engineering;
pub mod soil_science;
pub mod soil_science_detailed;
pub mod solid_mechanics;
pub mod solid_mechanics_detailed;
pub mod statics;
pub mod statistical_mechanics;
pub mod statistical_physics;
pub mod statistical_physics_detailed;
pub mod statistics;
pub mod stratigraphy;
pub mod surgery;
pub mod sustainability_science;
pub mod synthetic_biology;
pub mod systems_science;
pub mod thermochemistry;
pub mod thermodynamics;
pub mod thermodynamics_detailed;
pub mod topology;
pub mod traditional_chinese_medicine;
pub mod urban_geography;
pub mod vector_space;
pub mod vibration_wave;
pub mod volcanology;
pub mod volcanology_detailed;
pub mod waste_management;
pub mod water_resources;
pub mod zoology;
pub mod zoology_detailed;

pub use acoustics::AcousticsRules;
pub use acoustics_detailed::AcousticsDetailedRules;
pub use aerospace_engineering::AerospaceEngineeringLaws;
pub use aerospace_engineering_detailed::AerospaceEngineeringDetailedRules;
pub use aesthetics::AestheticsRules;
pub use ai_ethics::AiEthicsRules;
pub use air_quality::AirQualityRules;
pub use algebra_math::AlgebraMathRules;
pub use algorithms::AlgorithmsRules;
pub use analytical_chemistry::AnalyticalChemistryLaws;
pub use analytical_mechanics::AnalyticalMechanicsRules;
pub use anatomy::AnatomyRules;
pub use animal_behavior::AnimalBehaviorRules;
pub use antenna_theory::AntennaTheoryRules;
pub use anthropology::AnthropologyLaws;
pub use archaeology::ArchaeologyLaws;
pub use architecture::ArchitectureLaws;
pub use art_science::ArtScienceLaws;
pub use artificial_intelligence::ArtificialIntelligenceRules;
pub use astronomy::AstronomyRules;
pub use astrophysics::AstrophysicsRules;
pub use astrophysics_detailed::AstrophysicsDetailedRules;
pub use atomic_physics::AtomicPhysicsRules;
pub use behavioral_economics::BehavioralEconomicsRules;
pub use bilinear_algebra::BilinearAlgebraRules;
pub use biochemistry::BiochemistryLaws;
pub use biochemistry_detailed::BiochemistryDetailedRules;
pub use biodiversity::BiodiversityRules;
pub use bioengineering::BioengineeringLaws;
pub use bioethics::BioethicsRules;
pub use biogeography::BiogeographyRules;
pub use bioinformatics::BioinformaticsRules;
pub use biology::BiologyRules;
pub use biomedical_engineering::BiomedicalEngineeringRules;
pub use biophysics::BiophysicsRules;
pub use biostatistics::BiostatisticsRules;
pub use blockchain_detailed::BlockchainDetailedRules;
pub use blockchain_theory::BlockchainTheoryRules;
pub use botany::BotanyLaws;
pub use botany_detailed::BotanyDetailedRules;
pub use calculus::CalculusRules;
pub use carbon_cycle::CarbonCycleRules;
pub use category_theory::CategoryTheoryRules;
pub use celestial_mechanics::CelestialMechanicsRules;
pub use cell_biology::CellBiologyLaws;
pub use cell_biology_detailed::CellBiologyDetailedRules;
pub use chaos_theory::ChaosTheoryLaws;
pub use chemical_engineering::ChemicalEngineeringLaws;
pub use chemical_engineering_detailed::ChemicalEngineeringDetailedRules;
pub use chemistry::ChemistryRules;
pub use circuit_theory::CircuitTheoryRules;
pub use civil_engineering::CivilEngineeringLaws;
pub use civil_engineering_detailed::CivilEngineeringDetailedRules;
pub use climate_science::ClimateScienceRules;
pub use climatology::ClimatologyRules;
pub use cognitive_psychology::CognitivePsychologyRules;
pub use communication::CommunicationLaws;
pub use compiler_theory::CompilerTheoryRules;
pub use complex_analysis::ComplexAnalysisRules;
pub use complexity_science::ComplexityScienceLaws;
pub use computational_complexity::ComputationalComplexityRules;
pub use computational_linguistics::ComputationalLinguisticsRules;
pub use computational_mechanics::ComputationalMechanicsRules;
pub use computer_graphics::ComputerGraphicsRules;
pub use computer_networks::ComputerNetworksRules;
pub use computer_science::ComputerScienceLaws;
pub use condensed_matter::CondensedMatterRules;
pub use condensed_matter_detailed::CondensedMatterDetailedRules;
pub use conservation_biology::ConservationBiologyRules;
pub use control_engineering::ControlEngineeringLaws;
pub use control_engineering_detailed::ControlEngineeringDetailedRules;
pub use control_theory::ControlTheoryRules;
pub use cosmology::CosmologyRules;
pub use cosmology_detailed::CosmologyDetailedRules;
pub use cryptography::CryptographyRules;
pub use cryptography_detailed::CryptographyDetailedRules;
pub use crystallography::CrystallographyRules;
pub use cv_detailed::CvDetailedRules;
pub use cv_theory::CvTheoryRules;
pub use cybersecurity::CybersecurityRules;
pub use data_ethics::DataEthicsRules;
pub use data_science_detailed::DataScienceDetailedRules;
pub use data_science_theory::DataScienceTheoryRules;
pub use data_structures::DataStructuresRules;
pub use database_theory::DatabaseTheoryRules;
pub use decision_science::DecisionScienceLaws;
pub use deep_learning::DeepLearningRules;
pub use deep_learning_detailed::DeepLearningDetailedRules;
pub use dentistry::DentistryRules;
pub use dermatology::DermatologyRules;
pub use devops_theory::DevopsTheoryRules;
pub use differential_equations::DifferentialEquationsRules;
pub use differential_geometry::DifferentialGeometryRules;
pub use discrete_math::DiscreteMathRules;
pub use distributed_systems::DistributedSystemsRules;
pub use distributed_systems_detailed::DistributedSystemsDetailedRules;
pub use dynamics::DynamicsRules;
pub use ecology::EcologyLaws;
pub use ecology_detailed::EcologyDetailedRules;
pub use ecology_detailed2::EcologyDetailed2Rules;
pub use economics::EconomicsRules;
pub use education::EducationLaws;
pub use electrical_engineering::ElectricalEngineeringLaws;
pub use electrical_engineering_detailed::ElectricalEngineeringDetailedRules;
pub use electrochemistry::ElectrochemistryRules;
pub use electrodynamics::ElectrodynamicsRules;
pub use electromagnetic_compatibility::ElectromagneticCompatibilityRules;
pub use electromagnetic_induction::ElectromagneticInductionRules;
pub use electromagnetic_wave_propagation::ElectromagneticWavePropagationRules;
pub use electromagnetism::ElectromagnetismLaws;
pub use electrostatics::ElectrostaticsRules;
pub use energy_engineering::EnergyEngineeringRules;
pub use environmental_engineering::EnvironmentalEngineeringRules;
pub use environmental_ethics::EnvironmentalEthicsRules;
pub use environmental_science::EnvironmentalScienceLaws;
pub use environmental_science_detailed::EnvironmentalScienceDetailedRules;
pub use epidemiology::EpidemiologyRules;
pub use epidemiology_detailed::EpidemiologyDetailedRules;
pub use evolution_detailed::EvolutionDetailedRules;
pub use evolutionary_biology::EvolutionaryBiologyLaws;
pub use exercise_physiology::ExercisePhysiologyRules;
pub use exercise_science::ExerciseScienceRules;
pub use field_theory::FieldTheoryRules;
pub use financial_economics::FinancialEconomicsRules;
pub use fluid_dynamics::FluidDynamicsRules;
pub use fluid_mechanics::FluidMechanicsLaws;
pub use fluid_mechanics_detailed::FluidMechanicsDetailedRules;
pub use functional_analysis::FunctionalAnalysisRules;
pub use game_theory::GameTheoryLaws;
pub use general_relativity::GeneralRelativityRules;
pub use genetics::GeneticsLaws;
pub use genetics_detailed::GeneticsDetailedRules;
pub use genomic_laws::GenomicLawsRules;
pub use geography::GeographyLaws;
pub use geology::GeologyRules;
pub use geometry_math::GeometryMathRules;
pub use geomorphology::GeomorphologyRules;
pub use geophysics::GeophysicsRules;
pub use geophysics_detailed::GeophysicsDetailedRules;
pub use geoscience::GeoscienceLaws;
pub use geriatrics::GeriatricsRules;
pub use gis::GisRules;
pub use graph_theory::GraphTheoryRules;
pub use history::HistoryLaws;
pub use hydrology::HydrologyRules;
pub use immunology::ImmunologyLaws;
pub use immunology_detailed::ImmunologyDetailedRules;
pub use information_science::InformationScienceLaws;
pub use information_theory::InformationTheoryRules;
pub use information_theory_detailed::InformationTheoryDetailedRules;
pub use inorganic_chemistry::InorganicChemistryLaws;
pub use kinematics::KinematicsRules;
pub use linear_algebra::LinearAlgebraRules;
pub use linguistics::LinguisticsLaws;
pub use literature::LiteratureLaws;
pub use machine_learning_detailed::MachineLearningDetailedRules;
pub use magnetostatics::MagnetostaticsRules;
pub use management_science::ManagementScienceLaws;
pub use marine_biology::MarineBiologyRules;
pub use marine_biology_detailed::MarineBiologyDetailedRules;
pub use marketing_theory::MarketingTheoryRules;
pub use material_science::MaterialScienceLaws;
pub use materials_engineering::MaterialsEngineeringRules;
pub use materials_mechanics::MaterialsMechanicsRules;
pub use materials_science_detailed::MaterialsScienceDetailedRules;
pub use math::MathRules;
pub use matrix_algebra::MatrixAlgebraRules;
pub use maxwell_equations::MaxwellEquationsRules;
pub use measure_theory::MeasureTheoryRules;
pub use mechanical_engineering::MechanicalEngineeringLaws;
pub use mechanical_engineering_detailed::MechanicalEngineeringDetailedRules;
pub use mechanics_analytical::MechanicsAnalyticalRules;
pub use mechanics_classical::MechanicsClassicalRules;
pub use medical_imaging::MedicalImagingRules;
pub use medical_science::MedicalScienceLaws;
pub use meteorology_detailed::MeteorologyDetailedRules;
pub use microbiology::MicrobiologyLaws;
pub use microbiology_detailed::MicrobiologyDetailedRules;
pub use microwave_technology::MicrowaveTechnologyRules;
pub use mineralogy::MineralogyLaws;
pub use mineralogy_detailed::MineralogyDetailedRules;
pub use ml_theory::MlTheoryRules;
pub use module_theory::ModuleTheoryRules;
pub use molecular_biology::MolecularBiologyRules;
pub use music_theory::MusicTheoryRules;
pub use musicology::MusicologyLaws;
pub use nanotechnology::NanotechnologyRules;
pub use nanotechnology_detailed::NanotechnologyDetailedRules;
pub use network_science::NetworkScienceLaws;
pub use neuroscience::NeuroscienceLaws;
pub use neuroscience_detailed::NeuroscienceDetailedRules;
pub use neuroscience_detailed2::NeuroscienceDetailed2Rules;
pub use nlp_detailed::NlpDetailedRules;
pub use nlp_theory::NlpTheoryRules;
pub use noise_pollution_science::NoisePollutionScienceRules;
pub use norm_theory::NormTheoryRules;
pub use nuclear_physics::NuclearPhysicsLaws;
pub use nuclear_physics_detailed::NuclearPhysicsDetailedRules;
pub use number_theory::NumberTheoryRules;
pub use numerical_analysis::NumericalAnalysisRules;
pub use nutrition_science_detailed::NutritionScienceDetailedRules;
pub use obstetrics::ObstetricsRules;
pub use oceanography::OceanographyRules;
pub use oceanography_detailed::OceanographyDetailedRules;
pub use operating_systems::OperatingSystemsRules;
pub use ophthalmology::OphthalmologyRules;
pub use optical_basics::OpticalBasicsRules;
pub use optics::OpticsLaws;
pub use optics_detailed::OpticsDetailedRules;
pub use optimization::OptimizationRules;
pub use organic_chemistry::OrganicChemistryLaws;
pub use organizational_behavior::OrganizationalBehaviorRules;
pub use paleontology::PaleontologyLaws;
pub use paleontology_detailed::PaleontologyDetailedRules;
pub use paleontology_earth::PaleontologyEarthRules;
pub use particle_physics::ParticlePhysicsRules;
pub use particle_physics_detailed::ParticlePhysicsDetailedRules;
pub use particle_physics_standard_model::ParticlePhysicsStandardModelRules;
pub use pathology::PathologyRules;
pub use pediatrics::PediatricsRules;
pub use petrology::PetrologyRules;
pub use pharmacology::PharmacologyRules;
pub use pharmacology_detailed::PharmacologyDetailedRules;
pub use philosophy::PhilosophyLaws;
pub use photochemistry::PhotochemistryRules;
pub use physical_chemistry::PhysicalChemistryLaws;
pub use physics::PhysicsLaws;
pub use physiology::PhysiologyRules;
pub use plant_physiology::PlantPhysiologyRules;
pub use plasma_physics::PlasmaPhysicsRules;
pub use plasma_physics_detailed::PlasmaPhysicsDetailedRules;
pub use political_science::PoliticalScienceLaws;
pub use polymer_chemistry::PolymerChemistryRules;
pub use polynomial_algebra::PolynomialAlgebraRules;
pub use probability_theory::ProbabilityTheoryRules;
pub use proteomics::ProteomicsRules;
pub use psychiatry::PsychiatryRules;
pub use psychology::PsychologyRules;
pub use quantum_computing::QuantumComputingRules;
pub use quantum_computing_detailed::QuantumComputingDetailedRules;
pub use quantum_field_theory::QuantumFieldTheoryRules;
pub use quantum_mechanics::QuantumMechanicsLaws;
pub use quantum_mechanics_detailed::QuantumMechanicsDetailedRules;
pub use real_analysis::RealAnalysisRules;
pub use relativity::RelativityLaws;
pub use relativity_general::RelativityGeneralRules;
pub use relativity_special::RelativitySpecialRules;
pub use remote_sensing::RemoteSensingRules;
pub use renewable_energy::RenewableEnergyRules;
pub use research_ethics::ResearchEthicsRules;
pub use rigid_body_dynamics::RigidBodyDynamicsRules;
pub use robotics_detailed::RoboticsDetailedRules;
pub use robotics_theory::RoboticsTheoryRules;
pub use seismology::SeismologyRules;
pub use signal_processing::SignalProcessingRules;
pub use social_psychology::SocialPsychologyRules;
pub use sociology::SociologyLaws;
pub use software_engineering::SoftwareEngineeringRules;
pub use soil_science::SoilScienceRules;
pub use soil_science_detailed::SoilScienceDetailedRules;
pub use solid_mechanics::SolidMechanicsLaws;
pub use solid_mechanics_detailed::SolidMechanicsDetailedRules;
pub use statics::StaticsRules;
pub use statistical_mechanics::StatisticalMechanicsRules;
pub use statistical_physics::StatisticalPhysicsRules;
pub use statistical_physics_detailed::StatisticalPhysicsDetailedRules;
pub use statistics::StatisticsRules;
pub use stratigraphy::StratigraphyRules;
pub use surgery::SurgeryRules;
pub use sustainability_science::SustainabilityScienceRules;
pub use synthetic_biology::SyntheticBiologyRules;
pub use systems_science::SystemsScienceLaws;
pub use thermochemistry::ThermochemistryRules;
pub use thermodynamics::ThermodynamicsLaws;
pub use thermodynamics_detailed::ThermodynamicsDetailedRules;
pub use topology::TopologyRules;
pub use traditional_chinese_medicine::TraditionalChineseMedicineRules;
pub use urban_geography::UrbanGeographyRules;
pub use vibration_wave::VibrationWaveRules;
pub use volcanology::VolcanologyRules;
pub use volcanology_detailed::VolcanologyDetailedRules;
pub use waste_management::WasteManagementRules;
pub use water_resources::WaterResourcesRules;
pub use zoology::ZoologyLaws;
pub use zoology_detailed::ZoologyDetailedRules;

pub fn all_rules() -> Vec<(
    &'static str,
    crate::rules::core::RuleMetadata,
    crate::rules::core::RuleCategory,
    String,
)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    {
        let r = AcousticsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AerospaceEngineeringLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AestheticsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AlgebraMathRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AnalyticalChemistryLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AnimalBehaviorRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AnthropologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ArchaeologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ArchitectureLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ArtScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AstronomyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AstrophysicsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BehavioralEconomicsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BiochemistryLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BioengineeringLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BiogeographyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BioinformaticsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BiologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BiostatisticsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BlockchainTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BotanyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CategoryTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CellBiologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChaosTheoryLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChemicalEngineeringLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChemistryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CivilEngineeringLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ClimateScienceRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CognitivePsychologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CommunicationLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CompilerTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ComplexityScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ComputationalLinguisticsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ComputerScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CondensedMatterRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ControlEngineeringLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ControlTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CryptographyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CrystallographyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CvTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DataScienceTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DecisionScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DeepLearningRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DevopsTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DifferentialGeometryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GeometryMathRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DistributedSystemsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EcologyDetailedRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EcologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EconomicsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EducationLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElectricalEngineeringLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElectrochemistryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElectromagnetismLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EnvironmentalScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EpidemiologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EvolutionaryBiologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ExerciseScienceRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FinancialEconomicsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FluidMechanicsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FunctionalAnalysisRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GameTheoryLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GeneralRelativityRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GeneticsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GenomicLawsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GeographyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GeoscienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HistoryLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ImmunologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InformationScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InformationTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InorganicChemistryLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LinguisticsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LiteratureLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ManagementScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MarineBiologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MarketingTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MaterialScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MaterialsEngineeringRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MathRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MeasureTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MechanicalEngineeringLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MedicalScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MicrobiologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MineralogyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MlTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MusicTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MusicologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NanotechnologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NetworkScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NeuroscienceDetailedRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NeuroscienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NlpTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NuclearPhysicsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NumberTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OceanographyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OpticsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OrganicChemistryLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OrganizationalBehaviorRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PaleontologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ParticlePhysicsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PharmacologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PhilosophyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PhotochemistryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PhysicalChemistryLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PhysicsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PlantPhysiologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PlasmaPhysicsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PoliticalScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PolymerChemistryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ProbabilityTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ProteomicsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PsychologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = QuantumComputingRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = QuantumFieldTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = QuantumMechanicsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RelativityLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RoboticsTheoryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SignalProcessingRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SocialPsychologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SociologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SoilScienceRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SolidMechanicsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = StatisticalMechanicsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = StatisticsRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SyntheticBiologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SystemsScienceLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ThermochemistryRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ThermodynamicsLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TopologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = UrbanGeographyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VolcanologyRules::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ZoologyLaws::new();
        rules.push(("science", r.metadata().clone(), r.category(), r.explain()));
    }
    rules
}

//! 体育规则模块

pub mod aikido;
pub mod air_hockey;
pub mod airship_racing;
pub mod alpine_skiing;
pub mod american_football_detailed;
pub mod archery;
pub mod archery_detailed;
pub mod athletics;
pub mod athletics_detailed;
pub mod australian_football;
pub mod badminton;
pub mod badminton_detailed;
pub mod ballroom_dancing;
pub mod base_jumping;
pub mod baseball;
pub mod baseball_detailed;
pub mod basketball;
pub mod basketball_3x3_detail;
pub mod basketball_detailed;
pub mod beach_handball;
pub mod beach_soccer;
pub mod beach_volleyball;
pub mod beach_volleyball_detail;
pub mod biathlon;
pub mod billiards;
pub mod billiards_detailed;
pub mod bjj;
pub mod blind_football;
pub mod bmx;
pub mod bobsleigh;
pub mod bocce;
pub mod bodybuilding;
pub mod bodybuilding_detailed;
pub mod bowling;
pub mod bowling_detailed;
pub mod boxing;
pub mod boxing_detailed;
pub mod breakdancing;
pub mod bungee_jumping;
pub mod canoe_polo;
pub mod capoeira;
pub mod climbing;
pub mod cornhole;
pub mod cricket;
pub mod cricket_detailed;
pub mod cross_country_skiing;
pub mod curling;
pub mod curling_detailed;
pub mod cycling;
pub mod darts;
pub mod darts_detailed;
pub mod decathlon;
pub mod disc_golf;
pub mod disc_golf_detail;
pub mod disc_golf_detailed;
pub mod discus;
pub mod diving;
pub mod diving_sport;
pub mod drag_racing;
pub mod dragon_boat;
pub mod drone_racing;
pub mod equestrian;
pub mod esports;
pub mod esports_detailed;
pub mod f1;
pub mod fencing;
pub mod fencing_detailed;
pub mod field_archery;
pub mod figure_skating;
pub mod figure_skating_detailed;
pub mod fitness_competition;
pub mod five_aside_football;
pub mod floorball;
pub mod foosball;
pub mod football;
pub mod football_detailed;
pub mod free_diving;
pub mod freestyle_skiing;
pub mod gaelic_football;
pub mod gateball;
pub mod golf;
pub mod golf_detailed;
pub mod gymnastics;
pub mod gymnastics_detailed;
pub mod hammer_throw;
pub mod handball;
pub mod handball_detailed;
pub mod handball_detailed_rules;
pub mod hapkido;
pub mod heptathlon;
pub mod high_jump;
pub mod hockey;
pub mod horseshoes;
pub mod hot_air_balloon;
pub mod hurdles;
pub mod hurling;
pub mod iaido;
pub mod ice_climbing;
pub mod ice_dancing;
pub mod ice_hockey;
pub mod ice_hockey_detailed;
pub mod ice_sailing;
pub mod indoor_archery;
pub mod indoor_soccer;
pub mod javelin;
pub mod jet_ski;
pub mod jiu_jitsu;
pub mod judo;
pub mod judo_detailed;
pub mod kabaddi;
pub mod kalaripayattu;
pub mod karate;
pub mod karate_detailed;
pub mod kayaking;
pub mod kendo;
pub mod kickboxing;
pub mod kitesurfing;
pub mod krav_maga;
pub mod lacrosse;
pub mod lacrosse_detailed;
pub mod latin_dance;
pub mod lethwei;
pub mod long_jump;
pub mod luge;
pub mod marathon;
pub mod model_airplane;
pub mod modern_pentathlon;
pub mod motogp;
pub mod motogp_detailed;
pub mod motorboat_racing;
pub mod mounted_archery;
pub mod muay_thai;
pub mod mud_racing;
pub mod nascar;
pub mod netball;
pub mod nine_ball_detailed;
pub mod nordic_combined;
pub mod orienteering;
pub mod orienteering_detailed;
pub mod paddleboarding;
pub mod pair_skating;
pub mod paragliding;
pub mod parkour;
pub mod petanque;
pub mod pole_vault;
pub mod polo;
pub mod powerlifting;
pub mod powerlifting_detailed;
pub mod race_walking;
pub mod rally_racing;
pub mod relay;
pub mod rhythmic_gymnastics;
pub mod roller_skating;
pub mod rowing;
pub mod rugby;
pub mod rugby_detailed;
pub mod rugby_sevens;
pub mod sailing;
pub mod sambo;
pub mod sand_motorcycle;
pub mod sand_volleyball;
pub mod sanda;
pub mod savate;
pub mod sepak_takraw;
pub mod seven_aside_football;
pub mod shaolin_kung_fu;
pub mod shooting;
pub mod shooting_detailed;
pub mod short_track;
pub mod shot_put;
pub mod silat;
pub mod sitting_volleyball;
pub mod skateboarding;
pub mod skeleton;
pub mod ski_jumping;
pub mod skiing;
pub mod skydiving;
pub mod slalom_kayaking;
pub mod snooker_detailed;
pub mod snowboarding;
pub mod softball;
pub mod softball_detailed;
pub mod speed_skating;
pub mod speed_skating_detailed;
pub mod squash;
pub mod steeplechase;
pub mod sumo;
pub mod surfing;
pub mod swimming;
pub mod swimming_detailed;
pub mod synchronized_swimming;
pub mod table_tennis;
pub mod table_tennis_detailed;
pub mod taekwondo;
pub mod taekwondo_detailed;
pub mod tai_chi_push_hands;
pub mod taiwanese_slingshot;
pub mod tennis;
pub mod tennis_detailed;
pub mod thai_boxing;
pub mod three_x_three_basketball;
pub mod trampoline;
pub mod triathlon;
pub mod triathlon_detailed;
pub mod triple_jump;
pub mod ultimate_frisbee;
pub mod volleyball;
pub mod volleyball_detailed;
pub mod wakeboarding;
pub mod water_polo;
pub mod water_polo_detailed;
pub mod water_skiing;
pub mod water_skiing_detailed;
pub mod water_slide_racing;
pub mod weightlifting;
pub mod weightlifting_detailed;
pub mod wheelchair_basketball;
pub mod wheelchair_racing;
pub mod wheelchair_tennis;
pub mod white_water_rafting;
pub mod windsurfing;
pub mod wrestling;
pub mod wushu;
pub mod yoga_competition;

pub use aikido::AikidoRules;
pub use air_hockey::AirHockeyRules;
pub use airship_racing::AirshipRacingRules;
pub use alpine_skiing::AlpineSkiingRules;
pub use american_football_detailed::AmericanFootballDetailedRules;
pub use archery::ArcheryRules;
pub use archery_detailed::ArcheryDetailedRules;
pub use athletics::{AthleticsEvent, AthleticsRules};
pub use athletics_detailed::AthleticsDetailedRules;
pub use australian_football::AustralianFootballRules;
pub use badminton::BadmintonRules;
pub use badminton_detailed::BadmintonDetailedRules;
pub use ballroom_dancing::BallroomDancingRules;
pub use base_jumping::BaseJumpingRules;
pub use baseball::BaseballRules;
pub use baseball_detailed::BaseballDetailedRules;
pub use basketball::BasketballRules;
pub use basketball_3x3_detail::Basketball3x3DetailRules;
pub use basketball_detailed::BasketballDetailedRules;
pub use beach_handball::BeachHandballRules;
pub use beach_soccer::BeachSoccerRules;
pub use beach_volleyball::BeachVolleyballRules;
pub use beach_volleyball_detail::BeachVolleyballDetailRules;
pub use biathlon::BiathlonRules;
pub use billiards::{BilliardsRules, BilliardsType};
pub use billiards_detailed::BilliardsDetailedRules;
pub use bjj::BjjRules;
pub use blind_football::BlindFootballRules;
pub use bmx::BmxRules;
pub use bobsleigh::BobsleighRules;
pub use bocce::BocceRules;
pub use bodybuilding::BodybuildingRules;
pub use bodybuilding_detailed::BodybuildingDetailedRules;
pub use bowling::BowlingRules;
pub use bowling_detailed::BowlingDetailedRules;
pub use boxing::BoxingRules;
pub use boxing_detailed::BoxingDetailedRules;
pub use breakdancing::BreakdancingRules;
pub use bungee_jumping::BungeeJumpingRules;
pub use canoe_polo::CanoePoloRules;
pub use capoeira::CapoeiraRules;
pub use climbing::ClimbingRules;
pub use cornhole::CornholeRules;
pub use cricket::CricketRules;
pub use cricket_detailed::CricketDetailedRules;
pub use cross_country_skiing::CrossCountrySkiingRules;
pub use curling::CurlingRules;
pub use curling_detailed::CurlingDetailedRules;
pub use cycling::{CyclingRules, CyclingType};
pub use darts::DartsRules;
pub use darts_detailed::DartsDetailedRules;
pub use decathlon::DecathlonRules;
pub use disc_golf::DiscGolfRules;
pub use disc_golf_detail::DiscGolfDetailRules;
pub use disc_golf_detailed::DiscGolfDetailedRules;
pub use discus::DiscusRules;
pub use diving::DivingRules;
pub use diving_sport::DivingSportRules;
pub use drag_racing::DragRacingRules;
pub use dragon_boat::DragonBoatRules;
pub use drone_racing::DroneRacingRules;
pub use equestrian::EquestrianRules;
pub use esports::EsportsRules;
pub use esports_detailed::EsportsDetailedRules;
pub use f1::F1Rules;
pub use fencing::FencingRules;
pub use fencing_detailed::FencingDetailedRules;
pub use field_archery::FieldArcheryRules;
pub use figure_skating::FigureSkatingRules;
pub use figure_skating_detailed::FigureSkatingDetailedRules;
pub use fitness_competition::FitnessCompetitionRules;
pub use five_aside_football::FiveAsideFootballRules;
pub use floorball::FloorballRules;
pub use foosball::FoosballRules;
pub use football::FootballRules;
pub use football_detailed::FootballDetailedRules;
pub use free_diving::FreeDivingRules;
pub use freestyle_skiing::FreestyleSkiingRules;
pub use gaelic_football::GaelicFootballRules;
pub use gateball::GateballRules;
pub use golf::GolfRules;
pub use golf_detailed::GolfDetailedRules;
pub use gymnastics::{GymnasticsRules, GymnasticsType};
pub use gymnastics_detailed::GymnasticsDetailedRules;
pub use hammer_throw::HammerThrowRules;
pub use handball::HandballRules;
pub use handball_detailed::HandballDetailedRules;
pub use handball_detailed_rules::HandballDetailedRulesRules;
pub use hapkido::HapkidoRules;
pub use heptathlon::HeptathlonRules;
pub use high_jump::HighJumpRules;
pub use hockey::HockeyRules;
pub use horseshoes::HorseshoesRules;
pub use hot_air_balloon::HotAirBalloonRules;
pub use hurdles::HurdlesRules;
pub use hurling::HurlingRules;
pub use iaido::IaidoRules;
pub use ice_climbing::IceClimbingRules;
pub use ice_dancing::IceDancingRules;
pub use ice_hockey::IceHockeyRules;
pub use ice_hockey_detailed::IceHockeyDetailedRules;
pub use ice_sailing::IceSailingRules;
pub use indoor_archery::IndoorArcheryRules;
pub use indoor_soccer::IndoorSoccerRules;
pub use javelin::JavelinRules;
pub use jet_ski::JetSkiRules;
pub use jiu_jitsu::JiuJitsuRules;
pub use judo::JudoRules;
pub use judo_detailed::JudoDetailedRules;
pub use kabaddi::KabaddiRules;
pub use kalaripayattu::KalaripayattuRules;
pub use karate::KarateRules;
pub use karate_detailed::KarateDetailedRules;
pub use kayaking::KayakingRules;
pub use kendo::KendoRules;
pub use kickboxing::KickboxingRules;
pub use kitesurfing::KitesurfingRules;
pub use krav_maga::KravMagaRules;
pub use lacrosse::LacrosseRules;
pub use lacrosse_detailed::LacrosseDetailedRules;
pub use latin_dance::LatinDanceRules;
pub use lethwei::LethweiRules;
pub use long_jump::LongJumpRules;
pub use luge::LugeRules;
pub use marathon::MarathonRules;
pub use model_airplane::ModelAirplaneRules;
pub use modern_pentathlon::ModernPentathlonRules;
pub use motogp::MotoGPRules;
pub use motogp_detailed::MotogpDetailedRules;
pub use motorboat_racing::MotorboatRacingRules;
pub use mounted_archery::MountedArcheryRules;
pub use muay_thai::MuayThaiRules;
pub use mud_racing::MudRacingRules;
pub use nascar::NASCARRules;
pub use netball::NetballRules;
pub use nine_ball_detailed::NineBallDetailedRules;
pub use nordic_combined::NordicCombinedRules;
pub use orienteering::OrienteeringRules;
pub use orienteering_detailed::OrienteeringDetailedRules;
pub use paddleboarding::PaddleboardingRules;
pub use pair_skating::PairSkatingRules;
pub use paragliding::ParaglidingRules;
pub use parkour::ParkourRules;
pub use petanque::PetanqueRules;
pub use pole_vault::PoleVaultRules;
pub use polo::PoloRules;
pub use powerlifting::PowerliftingRules;
pub use powerlifting_detailed::PowerliftingDetailedRules;
pub use race_walking::RaceWalkingRules;
pub use rally_racing::RallyRacingRules;
pub use relay::RelayRules;
pub use rhythmic_gymnastics::RhythmicGymnasticsRules;
pub use roller_skating::RollerSkatingRules;
pub use rowing::RowingRules;
pub use rugby::RugbyRules;
pub use rugby_detailed::RugbyDetailedRules;
pub use rugby_sevens::RugbySevensRules;
pub use sailing::SailingRules;
pub use sambo::SamboRules;
pub use sand_motorcycle::SandMotorcycleRules;
pub use sand_volleyball::SandVolleyballRules;
pub use sanda::SandaRules;
pub use savate::SavateRules;
pub use sepak_takraw::SepakTakrawRules;
pub use seven_aside_football::SevenAsideFootballRules;
pub use shaolin_kung_fu::ShaolinKungFuRules;
pub use shooting::ShootingRules;
pub use shooting_detailed::ShootingDetailedRules;
pub use short_track::ShortTrackRules;
pub use shot_put::ShotPutRules;
pub use silat::SilatRules;
pub use sitting_volleyball::SittingVolleyballRules;
pub use skateboarding::SkateboardingRules;
pub use skeleton::SkeletonRules;
pub use ski_jumping::SkiJumpingRules;
pub use skiing::{SkiingRules, SkiingType};
pub use skydiving::SkydivingRules;
pub use slalom_kayaking::SlalomKayakingRules;
pub use snooker_detailed::SnookerDetailedRules;
pub use snowboarding::SnowboardingRules;
pub use softball::SoftballRules;
pub use softball_detailed::SoftballDetailedRules;
pub use speed_skating::SpeedSkatingRules;
pub use speed_skating_detailed::SpeedSkatingDetailedRules;
pub use squash::SquashRules;
pub use steeplechase::SteeplechaseRules;
pub use sumo::SumoRules;
pub use surfing::SurfingRules;
pub use swimming::{SwimmingRules, SwimmingStyle};
pub use swimming_detailed::SwimmingDetailedRules;
pub use synchronized_swimming::SynchronizedSwimmingRules;
pub use table_tennis::TableTennisRules;
pub use table_tennis_detailed::TableTennisDetailedRules;
pub use taekwondo::TaekwondoRules;
pub use taekwondo_detailed::TaekwondoDetailedRules;
pub use tai_chi_push_hands::TaiChiPushHandsRules;
pub use taiwanese_slingshot::TaiwaneseSlingshotRules;
pub use tennis::TennisRules;
pub use tennis_detailed::TennisDetailedRules;
pub use thai_boxing::ThaiBoxingRules;
pub use three_x_three_basketball::ThreeXThreeBasketballRules;
pub use trampoline::TrampolineRules;
pub use triathlon::TriathlonRules;
pub use triathlon_detailed::TriathlonDetailedRules;
pub use triple_jump::TripleJumpRules;
pub use ultimate_frisbee::UltimateFrisbeeRules;
pub use volleyball::VolleyballRules;
pub use volleyball_detailed::VolleyballDetailedRules;
pub use wakeboarding::WakeboardingRules;
pub use water_polo::WaterPoloRules;
pub use water_polo_detailed::WaterPoloDetailedRules;
pub use water_skiing::WaterSkiingRules;
pub use water_skiing_detailed::WaterSkiingDetailedRules;
pub use water_slide_racing::WaterSlideRacingRules;
pub use weightlifting::WeightliftingRules;
pub use weightlifting_detailed::WeightliftingDetailedRules;
pub use wheelchair_basketball::WheelchairBasketballRules;
pub use wheelchair_racing::WheelchairRacingRules;
pub use wheelchair_tennis::WheelchairTennisRules;
pub use white_water_rafting::WhiteWaterRaftingRules;
pub use windsurfing::WindsurfingRules;
pub use wrestling::WrestlingRules;
pub use wushu::WushuRules;
pub use yoga_competition::YogaCompetitionRules;

pub fn all_rules() -> Vec<(
    &'static str,
    crate::rules::core::RuleMetadata,
    crate::rules::core::RuleCategory,
    String,
)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    {
        let r = AikidoRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AirHockeyRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AirshipRacingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AlpineSkiingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AmericanFootballDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ArcheryDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ArcheryRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AthleticsDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AthleticsRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AustralianFootballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BadmintonDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BadmintonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BallroomDancingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BaseJumpingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BaseballDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BaseballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = Basketball3x3DetailRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BasketballDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BasketballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BeachHandballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BeachSoccerRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BeachVolleyballDetailRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BeachVolleyballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BiathlonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BilliardsDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r =
            BilliardsRules::new(crate::rules::sports::billiards::BilliardsType::ChineseEightBall);
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BjjRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BlindFootballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BmxRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BobsleighRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BocceRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BodybuildingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BodybuildingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BowlingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BowlingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BoxingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BoxingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BreakdancingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BungeeJumpingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CanoePoloRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CapoeiraRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ClimbingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CornholeRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CricketDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CricketRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CrossCountrySkiingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CurlingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CurlingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CyclingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DartsDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DartsRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DecathlonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DiscGolfDetailRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DiscGolfDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DiscGolfRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DiscusRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DivingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DivingSportRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DragRacingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DragonBoatRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DroneRacingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EquestrianRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EsportsDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EsportsRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = F1Rules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FencingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FencingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FieldArcheryRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FigureSkatingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FigureSkatingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FitnessCompetitionRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FiveAsideFootballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FloorballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FoosballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FootballDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FootballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FreeDivingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FreestyleSkiingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GaelicFootballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GateballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GolfDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GolfRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GymnasticsDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GymnasticsRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HammerThrowRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HandballDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HandballDetailedRulesRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HandballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HapkidoRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HeptathlonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HighJumpRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HockeyRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HorseshoesRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HotAirBalloonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HurdlesRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HurlingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IaidoRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IceClimbingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IceDancingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IceHockeyDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IceHockeyRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IceSailingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IndoorArcheryRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = IndoorSoccerRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = JavelinRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = JetSkiRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = JiuJitsuRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = JudoDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = JudoRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KabaddiRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KalaripayattuRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KarateDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KarateRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KayakingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KendoRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KickboxingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KitesurfingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KravMagaRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LacrosseDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LacrosseRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LatinDanceRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LethweiRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LongJumpRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LugeRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MarathonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ModelAirplaneRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ModernPentathlonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MotoGPRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MotogpDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MotorboatRacingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MountedArcheryRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MuayThaiRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MudRacingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NASCARRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NetballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NineBallDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NordicCombinedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OrienteeringDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OrienteeringRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PaddleboardingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PairSkatingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ParaglidingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ParkourRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PetanqueRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PoleVaultRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PoloRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PowerliftingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PowerliftingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RaceWalkingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RallyRacingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RelayRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RhythmicGymnasticsRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RollerSkatingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RowingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RugbyDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RugbyRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = RugbySevensRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SailingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SamboRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SandMotorcycleRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SandVolleyballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SandaRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SavateRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SepakTakrawRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SevenAsideFootballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ShaolinKungFuRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ShootingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ShootingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ShortTrackRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ShotPutRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SilatRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SittingVolleyballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SkateboardingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SkeletonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SkiJumpingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SkiingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SkydivingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SlalomKayakingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SnookerDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SnowboardingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SoftballDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SoftballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SpeedSkatingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SpeedSkatingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SquashRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SteeplechaseRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SumoRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SurfingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SwimmingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SwimmingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SynchronizedSwimmingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TableTennisDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TableTennisRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaekwondoDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaekwondoRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaiChiPushHandsRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TaiwaneseSlingshotRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TennisDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TennisRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ThaiBoxingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ThreeXThreeBasketballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TrampolineRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TriathlonDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TriathlonRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TripleJumpRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = UltimateFrisbeeRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VolleyballDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VolleyballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WakeboardingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WaterPoloDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WaterPoloRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WaterSkiingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WaterSkiingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WaterSlideRacingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WeightliftingDetailedRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WeightliftingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WheelchairBasketballRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WheelchairRacingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WheelchairTennisRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WhiteWaterRaftingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WindsurfingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WrestlingRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WushuRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = YogaCompetitionRules::new();
        rules.push(("sports", r.metadata().clone(), r.category(), r.explain()));
    }
    rules
}

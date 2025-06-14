use clap::Subcommand;
use space_traders_sdk::{
    space_traders_client::SpaceTradersClient,
    system::{
        waypoint::{WaypointData, WaypointTraitSymbol, WaypointType},
        System,
    },
};

use crate::Application;
use clap::ValueEnum;

#[derive(Subcommand, Debug)]
pub enum SystemCommand {
    /// Show information for a given agent
    ListWaypoints {
        #[arg(long)]
        r#type: Option<WaypointTypeArg>,
        #[arg(long)]
        r#trait: Option<WaypointTraitSymbolArg>,
    },
}

fn display_waypoint(waypoint: &WaypointData) {
    println!("Waypoint: {}", waypoint.symbol);
    println!("------------------------------");
    println!(
        "Type: {}",
        serde_json::to_string(&waypoint.waypoint_type).unwrap()
    );
    println!("System Symbol: {}", waypoint.system_symbol);
    println!("X: {}", waypoint.x);
    println!("Y: {}", waypoint.y);

    print!("Orbitals: ");
    if !waypoint.orbitals.is_empty() {
        let symbols: Vec<_> = waypoint
            .orbitals
            .iter()
            .map(|o| o.symbol.as_str())
            .collect();
        println!("{}", symbols.join(", "));
    } else {
        println!("None");
    }

    print!("Traits: ");
    if !waypoint.traits.is_empty() {
        let names: Vec<_> = waypoint
            .traits
            .iter()
            .map(|t| serde_json::to_string(&t.symbol).unwrap())
            .collect();
        println!("{}", names.join(", "));
    } else {
        println!("None");
    }

    print!("Faction: ");
    if let Some(faction) = &waypoint.faction {
        println!("{}", serde_json::to_string(&faction.symbol).unwrap());
    } else {
        println!("None");
    }

    print!("Chart: ");
    if let Some(chart) = &waypoint.chart {
        println!("{}", chart.submitted_by.as_deref().unwrap_or("Unknown"));
    } else {
        println!("None");
    }

    println!();
}

fn display_waypoint_short(waypoint: &WaypointData) {
    let trait_names: Vec<_> = waypoint
        .traits
        .iter()
        .map(|t| serde_json::to_string(&t.symbol).unwrap())
        .collect();
    println!(
        "{} [{}] with traits {}",
        waypoint.symbol,
        serde_json::to_string(&waypoint.waypoint_type).unwrap(),
        if trait_names.is_empty() {
            String::from("No traits")
        } else {
            trait_names.join(", ")
        }
    );
}

impl SystemCommand {
    pub async fn handle(
        &self,
        application: &mut Application,
        symbol: String,
    ) -> anyhow::Result<()> {
        match self {
            SystemCommand::ListWaypoints { r#type, r#trait } => {
                let type_converted = r#type.as_ref().map(|t| WaypointType::from(t.clone()));
                let trait_converted = r#trait
                    .as_ref()
                    .map(|t| WaypointTraitSymbol::from(t.clone()));
                match System::new(SpaceTradersClient::new(None).into(), &symbol)
                    .list_waypoints(type_converted.clone(), trait_converted.clone())
                    .await
                {
                    Ok(waypoints) => {
                        println!("");
                        print!("Waypoints in system {}", symbol);
                        if let Some(waypoint_type) = type_converted {
                            print!(
                                ", with type {}",
                                serde_json::to_string(&waypoint_type).unwrap()
                            );
                        }
                        if let Some(waypoint_trait) = trait_converted {
                            print!(
                                ", with trait {}",
                                serde_json::to_string(&waypoint_trait).unwrap()
                            );
                        }
                        println!("");
                        println!("------------------------------");
                        waypoints
                            .iter()
                            .for_each(|waypoint| display_waypoint_short(&waypoint.data))
                    }
                    Err(e) => eprintln!("Error listing waypoints: {}", e),
                }
            }
        }

        Ok(())
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum WaypointTypeArg {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Asteroid,
    EngineeredAsteroid,
    AsteroidBase,
    Nebula,
    DebrisField,
    GravityWell,
    ArtificialGravityWell,
    FuelStation,
}

impl From<WaypointTypeArg> for WaypointType {
    fn from(arg: WaypointTypeArg) -> Self {
        match arg {
            WaypointTypeArg::Planet => WaypointType::Planet,
            WaypointTypeArg::GasGiant => WaypointType::GasGiant,
            WaypointTypeArg::Moon => WaypointType::Moon,
            WaypointTypeArg::OrbitalStation => WaypointType::OrbitalStation,
            WaypointTypeArg::JumpGate => WaypointType::JumpGate,
            WaypointTypeArg::AsteroidField => WaypointType::AsteroidField,
            WaypointTypeArg::Asteroid => WaypointType::Asteroid,
            WaypointTypeArg::EngineeredAsteroid => WaypointType::EngineeredAsteroid,
            WaypointTypeArg::AsteroidBase => WaypointType::AsteroidBase,
            WaypointTypeArg::Nebula => WaypointType::Nebula,
            WaypointTypeArg::DebrisField => WaypointType::DebrisField,
            WaypointTypeArg::GravityWell => WaypointType::GravityWell,
            WaypointTypeArg::ArtificialGravityWell => WaypointType::ArtificialGravityWell,
            WaypointTypeArg::FuelStation => WaypointType::FuelStation,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum WaypointTraitSymbolArg {
    Uncharted,
    UnderConstruction,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    PirateBase,
    Overcrowded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    PreciousMetalDeposits,
    RareMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshClouds,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
    ThinAtmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Radioactive,
    MicroGravityAnomalies,
    DebrisCluster,
    DeepCraters,
    ShallowCraters,
    UnstableComposition,
    HollowedInterior,
    Stripped,
}

impl From<WaypointTraitSymbolArg> for WaypointTraitSymbol {
    fn from(arg: WaypointTraitSymbolArg) -> Self {
        match arg {
            WaypointTraitSymbolArg::Uncharted => WaypointTraitSymbol::Uncharted,
            WaypointTraitSymbolArg::UnderConstruction => WaypointTraitSymbol::UnderConstruction,
            WaypointTraitSymbolArg::Marketplace => WaypointTraitSymbol::Marketplace,
            WaypointTraitSymbolArg::Shipyard => WaypointTraitSymbol::Shipyard,
            WaypointTraitSymbolArg::Outpost => WaypointTraitSymbol::Outpost,
            WaypointTraitSymbolArg::ScatteredSettlements => {
                WaypointTraitSymbol::ScatteredSettlements
            }
            WaypointTraitSymbolArg::SprawlingCities => WaypointTraitSymbol::SprawlingCities,
            WaypointTraitSymbolArg::MegaStructures => WaypointTraitSymbol::MegaStructures,
            WaypointTraitSymbolArg::PirateBase => WaypointTraitSymbol::PirateBase,
            WaypointTraitSymbolArg::Overcrowded => WaypointTraitSymbol::Overcrowded,
            WaypointTraitSymbolArg::HighTech => WaypointTraitSymbol::HighTech,
            WaypointTraitSymbolArg::Corrupt => WaypointTraitSymbol::Corrupt,
            WaypointTraitSymbolArg::Bureaucratic => WaypointTraitSymbol::Bureaucratic,
            WaypointTraitSymbolArg::TradingHub => WaypointTraitSymbol::TradingHub,
            WaypointTraitSymbolArg::Industrial => WaypointTraitSymbol::Industrial,
            WaypointTraitSymbolArg::BlackMarket => WaypointTraitSymbol::BlackMarket,
            WaypointTraitSymbolArg::ResearchFacility => WaypointTraitSymbol::ResearchFacility,
            WaypointTraitSymbolArg::MilitaryBase => WaypointTraitSymbol::MilitaryBase,
            WaypointTraitSymbolArg::SurveillanceOutpost => WaypointTraitSymbol::SurveillanceOutpost,
            WaypointTraitSymbolArg::ExplorationOutpost => WaypointTraitSymbol::ExplorationOutpost,
            WaypointTraitSymbolArg::MineralDeposits => WaypointTraitSymbol::MineralDeposits,
            WaypointTraitSymbolArg::CommonMetalDeposits => WaypointTraitSymbol::CommonMetalDeposits,
            WaypointTraitSymbolArg::PreciousMetalDeposits => {
                WaypointTraitSymbol::PreciousMetalDeposits
            }
            WaypointTraitSymbolArg::RareMetalDeposits => WaypointTraitSymbol::RareMetalDeposits,
            WaypointTraitSymbolArg::MethanePools => WaypointTraitSymbol::MethanePools,
            WaypointTraitSymbolArg::IceCrystals => WaypointTraitSymbol::IceCrystals,
            WaypointTraitSymbolArg::ExplosiveGases => WaypointTraitSymbol::ExplosiveGases,
            WaypointTraitSymbolArg::StrongMagnetosphere => WaypointTraitSymbol::StrongMagnetosphere,
            WaypointTraitSymbolArg::VibrantAuroras => WaypointTraitSymbol::VibrantAuroras,
            WaypointTraitSymbolArg::SaltFlats => WaypointTraitSymbol::SaltFlats,
            WaypointTraitSymbolArg::Canyons => WaypointTraitSymbol::Canyons,
            WaypointTraitSymbolArg::PerpetualDaylight => WaypointTraitSymbol::PerpetualDaylight,
            WaypointTraitSymbolArg::PerpetualOvercast => WaypointTraitSymbol::PerpetualOvercast,
            WaypointTraitSymbolArg::DrySeabeds => WaypointTraitSymbol::DrySeabeds,
            WaypointTraitSymbolArg::MagmaSeas => WaypointTraitSymbol::MagmaSeas,
            WaypointTraitSymbolArg::Supervolcanoes => WaypointTraitSymbol::Supervolcanoes,
            WaypointTraitSymbolArg::AshClouds => WaypointTraitSymbol::AshClouds,
            WaypointTraitSymbolArg::VastRuins => WaypointTraitSymbol::VastRuins,
            WaypointTraitSymbolArg::MutatedFlora => WaypointTraitSymbol::MutatedFlora,
            WaypointTraitSymbolArg::Terraformed => WaypointTraitSymbol::Terraformed,
            WaypointTraitSymbolArg::ExtremeTemperatures => WaypointTraitSymbol::ExtremeTemperatures,
            WaypointTraitSymbolArg::ExtremePressure => WaypointTraitSymbol::ExtremePressure,
            WaypointTraitSymbolArg::DiverseLife => WaypointTraitSymbol::DiverseLife,
            WaypointTraitSymbolArg::ScarceLife => WaypointTraitSymbol::ScarceLife,
            WaypointTraitSymbolArg::Fossils => WaypointTraitSymbol::Fossils,
            WaypointTraitSymbolArg::WeakGravity => WaypointTraitSymbol::WeakGravity,
            WaypointTraitSymbolArg::StrongGravity => WaypointTraitSymbol::StrongGravity,
            WaypointTraitSymbolArg::CrushingGravity => WaypointTraitSymbol::CrushingGravity,
            WaypointTraitSymbolArg::ToxicAtmosphere => WaypointTraitSymbol::ToxicAtmosphere,
            WaypointTraitSymbolArg::CorrosiveAtmosphere => WaypointTraitSymbol::CorrosiveAtmosphere,
            WaypointTraitSymbolArg::BreathableAtmosphere => {
                WaypointTraitSymbol::BreathableAtmosphere
            }
            WaypointTraitSymbolArg::ThinAtmosphere => WaypointTraitSymbol::ThinAtmosphere,
            WaypointTraitSymbolArg::Jovian => WaypointTraitSymbol::Jovian,
            WaypointTraitSymbolArg::Rocky => WaypointTraitSymbol::Rocky,
            WaypointTraitSymbolArg::Volcanic => WaypointTraitSymbol::Volcanic,
            WaypointTraitSymbolArg::Frozen => WaypointTraitSymbol::Frozen,
            WaypointTraitSymbolArg::Swamp => WaypointTraitSymbol::Swamp,
            WaypointTraitSymbolArg::Barren => WaypointTraitSymbol::Barren,
            WaypointTraitSymbolArg::Temperate => WaypointTraitSymbol::Temperate,
            WaypointTraitSymbolArg::Jungle => WaypointTraitSymbol::Jungle,
            WaypointTraitSymbolArg::Ocean => WaypointTraitSymbol::Ocean,
            WaypointTraitSymbolArg::Radioactive => WaypointTraitSymbol::Radioactive,
            WaypointTraitSymbolArg::MicroGravityAnomalies => {
                WaypointTraitSymbol::MicroGravityAnomalies
            }
            WaypointTraitSymbolArg::DebrisCluster => WaypointTraitSymbol::DebrisCluster,
            WaypointTraitSymbolArg::DeepCraters => WaypointTraitSymbol::DeepCraters,
            WaypointTraitSymbolArg::ShallowCraters => WaypointTraitSymbol::ShallowCraters,
            WaypointTraitSymbolArg::UnstableComposition => WaypointTraitSymbol::UnstableComposition,
            WaypointTraitSymbolArg::HollowedInterior => WaypointTraitSymbol::HollowedInterior,
            WaypointTraitSymbolArg::Stripped => WaypointTraitSymbol::Stripped,
        }
    }
}

use asr::settings::Gui;
use asr::settings::gui::Title;
use crate::data::Area;

#[derive(Gui)]
pub struct Settings {

    // ENDAR SPIRE

    /// Endar Spire Splits
    title_endar_spire: Title,

    /// END_M01AA - Command Module
    #[default = false]
    pub END_M01AA: bool,

    /// END_M01AA - Command Module Unlimited Splits
    #[default = false]
    pub END_M01AA_unlim: bool,

    /// END_M01AB - Starboard Section
    #[default = false]
    pub END_M01AB: bool,

    /// END_M01AB - Starboard Section Unlimited Splits
    #[default = false]
    pub END_M01AB_unlim: bool,


    // TARIS

    /// Taris Splits
    title_taris: Title,

    /// TAR_M02AF - Hideout
    #[default = false]
    pub TAR_M02AF: bool,

    /// TAR_M02AF - Hideout Unlimited Splits
    #[default = false]
    pub TAR_M02AF_unlim: bool,

    /// TAR_M02AA - South Apartments
    #[default = false]
    pub TAR_M02AA: bool,

    /// TAR_M02AA - South Apartments Unlimited Splits
    #[default = false]
    pub TAR_M02AA_unlim: bool,

    /// TAR_M02AC - Upper City South
    #[default = false]
    pub TAR_M02AC: bool,

    /// TAR_M02AC - Upper City South Unlimited Splits
    #[default = false]
    pub TAR_M02AC_unlim: bool,

    /// TAR_M02AE - Upper City Cantina
    #[default = false]
    pub TAR_M02AE: bool,

    /// TAR_M02AE - Upper City Cantina Unlimited Splits
    #[default = false]
    pub TAR_M02AE_unlim: bool,

    /// TAR_M02AB - Upper City North
    #[default = false]
    pub TAR_M02AB: bool,

    /// TAR_M02AB - Upper City North Unlimited Splits
    #[default = false]
    pub TAR_M02AB_unlim: bool,

    /// TAR_M02AD - North Apartments
    #[default = false]
    pub TAR_M02AD: bool,

    /// TAR_M02AD - North Apartments Unlimited Splits
    #[default = false]
    pub TAR_M02AD_unlim: bool,

    /// TAR_M03AA - Lower City
    #[default = false]
    pub TAR_M03AA: bool,

    /// TAR_M03AA - Lower City Unlimited Splits
    #[default = false]
    pub TAR_M03AA_unlim: bool,

    /// TAR_M03AD - Lower City Apartments (East)
    #[default = false]
    pub TAR_M03AD: bool,

    /// TAR_M03AD - Lower City Apartments (East) Unlimited Splits
    #[default = false]
    pub TAR_M03AD_unlim: bool,

    /// TAR_M03AE - Javyar's Cantina
    #[default = false]
    pub TAR_M03AE: bool,

    /// TAR_M03AE - Javyar's Cantina Unlimited Splits
    #[default = false]
    pub TAR_M03AE_unlim: bool,

    /// TAR_M03AB - Lower City Apartments (West)
    #[default = false]
    pub TAR_M03AB: bool,

    /// TAR_M03AB - Lower City Apartments (West) Unlimited Splits
    #[default = false]
    pub TAR_M03AB_unlim: bool,

    /// TAR_M11AA - Hidden Bek Base
    #[default = false]
    pub TAR_M11AA: bool,

    /// TAR_M11AA - Hidden Bek Base Unlimited Splits
    #[default = false]
    pub TAR_M11AA_unlim: bool,

    /// TAR_M11AB - Gadon Thek's Office
    #[default = false]
    pub TAR_M11AB: bool,

    /// TAR_M11AB - Gadon Thek's Office Unlimited Splits
    #[default = false]
    pub TAR_M11AB_unlim: bool,

    /// TAR_M04AA - Undercity
    #[default = false]
    pub TAR_M04AA: bool,

    /// TAR_M04AA - Undercity Unlimited Splits
    #[default = false]
    pub TAR_M04AA_unlim: bool,

    /// TAR_M05AA - Lower Sewers
    #[default = false]
    pub TAR_M05AA: bool,

    /// TAR_M05AA - Lower Sewers Unlimited Splits
    #[default = false]
    pub TAR_M05AA_unlim: bool,

    /// TAR_M05AB - Upper Sewers
    #[default = false]
    pub TAR_M05AB: bool,

    /// TAR_M05AB - Upper Sewers Unlimited Splits
    #[default = false]
    pub TAR_M05AB_unlim: bool,

    /// TAR_M10AA - Vulkar Base
    #[default = false]
    pub TAR_M10AA: bool,

    /// TAR_M10AA - Vulkar Base Unlimited Splits
    #[default = false]
    pub TAR_M10AA_unlim: bool,

    /// TAR_M10AB - Vulkar Spice Lab
    #[default = false]
    pub TAR_M10AB: bool,

    /// TAR_M10AB - Vulkar Spice Lab Unlimited Splits
    #[default = false]
    pub TAR_M10AB_unlim: bool,

    /// TAR_M10AC - Vulkar Garage
    #[default = false]
    pub TAR_M10AC: bool,

    /// TAR_M10AC - Vulkar Garage Unlimited Splits
    #[default = false]
    pub TAR_M10AC_unlim: bool,

    /// TAR_M03AF - Swoop Platform
    #[default = false]
    pub TAR_M03AF: bool,

    /// TAR_M03AF - Swoop Platform Unlimited Splits
    #[default = false]
    pub TAR_M03AF_unlim: bool,

    /// TAR_M03MG - Swoop Racing
    #[default = false]
    pub TAR_M03MG: bool,

    /// TAR_M03MG - Swoop Racing Unlimited Splits
    #[default = false]
    pub TAR_M03MG_unlim: bool,

    /// TAR_M09AA - Sith Base
    #[default = false]
    pub TAR_M09AA: bool,

    /// TAR_M09AA - Sith Base Unlimited Splits
    #[default = false]
    pub TAR_M09AA_unlim: bool,

    /// TAR_M09AB - Governor's Office
    #[default = false]
    pub TAR_M09AB: bool,

    /// TAR_M09AB - Governor's Office Unlimited Splits
    #[default = false]
    pub TAR_M09AB_unlim: bool,

    /// TAR_M08AA - Davik's Estate
    #[default = false]
    pub TAR_M08AA: bool,

    /// TAR_M08AA - Davik's Estate Unlimited Splits
    #[default = false]
    pub TAR_M08AA_unlim: bool,


    // DANTOOINE

    /// Dantooine Splits
    title_dantooine: Title,

    /// DANM13 - Jedi Enclave
    #[default = false]
    pub DANM13: bool,

    /// DANM13 - Jedi Enclave Unlimited Splits
    #[default = false]
    pub DANM13_unlim: bool,

    /// DANM14AA - Courtyard
    #[default = false]
    pub DANM14AA: bool,

    /// DANM14AA - Courtyard Unlimited Splits
    #[default = false]
    pub DANM14AA_unlim: bool,

    /// DANM14AB - Matale Grounds
    #[default = false]
    pub DANM14AB: bool,

    /// DANM14AB - Matale Grounds Unlimited Splits
    #[default = false]
    pub DANM14AB_unlim: bool,

    /// DANM14AC - Sandral Grounds
    #[default = false]
    pub DANM14AC: bool,

    /// DANM14AC - Sandral Grounds Unlimited Splits
    #[default = false]
    pub DANM14AC_unlim: bool,

    /// DANM14AD - Crystal Cave
    #[default = false]
    pub DANM14AD: bool,

    /// DANM14AD - Crystal Cave Unlimited Splits
    #[default = false]
    pub DANM14AD_unlim: bool,

    /// DANM14AE - Grove
    #[default = false]
    pub DANM14AE: bool,

    /// DANM14AE - Grove Unlimited Splits
    #[default = false]
    pub DANM14AE_unlim: bool,

    /// DANM16 - Ruins
    #[default = false]
    pub DANM16: bool,

    /// DANM16 - Ruins Unlimited Splits
    #[default = false]
    pub DANM16_unlim: bool,

    /// DANM15 - Sandral Estate
    #[default = false]
    pub DANM15: bool,

    /// DANM15 - Sandral Estate Unlimited Splits
    #[default = false]
    pub DANM15_unlim: bool,


    // MAANAAN

    /// Manaan Splits
    title_maanaan: Title,

    /// MANM26AD - Ahto East
    #[default = false]
    pub MANM26AD: bool,

    /// MANM26AD - Ahto East Unlimited Splits
    #[default = false]
    pub MANM26AD_unlim: bool,

    /// MANM26AC - Ahto West
    #[default = false]
    pub MANM26AC: bool,

    /// MANM26AC - Ahto West Unlimited Splits
    #[default = false]
    pub MANM26AC_unlim: bool,

    /// MANM26AA - Ahto Central
    #[default = false]
    pub MANM26AA: bool,

    /// MANM26AA - Ahto Central Unlimited Splits
    #[default = false]
    pub MANM26AA_unlim: bool,

    /// MANM26AE - East Central Courtyard
    #[default = false]
    pub MANM26AE: bool,

    /// MANM26AE - East Central Courtyard Unlimited Splits
    #[default = false]
    pub MANM26AE_unlim: bool,

    /// MANM26AB - West Central Courtyard
    #[default = false]
    pub MANM26AB: bool,

    /// MANM26AB - West Central Courtyard Unlimited Splits
    #[default = false]
    pub MANM26AB_unlim: bool,

    /// MANM26MG - Swoop Track
    #[default = false]
    pub MANM26MG: bool,

    /// MANM26MG - Swoop Track Unlimited Splits
    #[default = false]
    pub MANM26MG_unlim: bool,

    /// MANM27AA - Sith Base
    #[default = false]
    pub MANM27AA: bool,

    /// MANM27AA - Sith Base Unlimited Splits
    #[default = false]
    pub MANM27AA_unlim: bool,

    /// MANM28AA - Hrakert Station
    #[default = false]
    pub MANM28AA: bool,

    /// MANM28AA - Hrakert Station Unlimited Splits
    #[default = false]
    pub MANM28AA_unlim: bool,

    /// MANM28AB - Sea Floor
    #[default = false]
    pub MANM28AB: bool,

    /// MANM28AB - Sea Floor Unlimited Splits
    #[default = false]
    pub MANM28AB_unlim: bool,

    /// MANM28AC - Kolto Control
    #[default = false]
    pub MANM28AC: bool,

    /// MANM28AC - Kolto Control Unlimited Splits
    #[default = false]
    pub MANM28AC_unlim: bool,

    /// MANM28AD - Submersible
    #[default = false]
    pub MANM28AD: bool,

    /// MANM28AD - Submersible Unlimited Splits
    #[default = false]
    pub MANM28AD_unlim: bool,


    // KORRIBAN

    /// Korriban Splits
    title_korriban: Title,

    /// KORR_M33AA - Valley of Dark Lords
    #[default = false]
    pub KORR_M33AA: bool,

    /// KORR_M33AA - Valley of Dark Lords Unlimited Splits
    #[default = false]
    pub KORR_M33AA_unlim: bool,

    /// KORR_M33AB - Dreshdae
    #[default = false]
    pub KORR_M33AB: bool,

    /// KORR_M33AB - Dreshdae Unlimited Splits
    #[default = false]
    pub KORR_M33AB_unlim: bool,

    /// KORR_M35AA - Sith Academy Entrance
    #[default = false]
    pub KORR_M35AA: bool,

    /// KORR_M35AA - Sith Academy Entrance Unlimited Splits
    #[default = false]
    pub KORR_M35AA_unlim: bool,

    /// KORR_M36AA - Sith Academy
    #[default = false]
    pub KORR_M36AA: bool,

    /// KORR_M36AA - Sith Academy Unlimited Splits
    #[default = false]
    pub KORR_M36AA_unlim: bool,

    /// KORR_M34AA - Tomb of Ajunta Pall
    #[default = false]
    pub KORR_M34AA: bool,

    /// KORR_M34AA - Tomb of Ajunta Pall Unlimited Splits
    #[default = false]
    pub KORR_M34AA_unlim: bool,

    /// KORR_M38AA - Tomb of Marka Ragnos
    #[default = false]
    pub KORR_M38AA: bool,

    /// KORR_M38AA - Tomb of Marka Ragnos Unlimited Splits
    #[default = false]
    pub KORR_M38AA_unlim: bool,

    /// KORR_M37AA - Tomb of Naga Sadow
    #[default = false]
    pub KORR_M37AA: bool,

    /// KORR_M37AA - Tomb of Naga Sadow Unlimited Splits
    #[default = false]
    pub KORR_M37AA_unlim: bool,

    /// KORR_M38AB - Shyrack Caves
    #[default = false]
    pub KORR_M38AB: bool,

    /// KORR_M38AB - Shyrack Caves Unlimited Splits
    #[default = false]
    pub KORR_M38AB_unlim: bool,

    /// KORR_M39AA - Tomb of Tulak Hord
    #[default = false]
    pub KORR_M39AA: bool,

    /// KORR_M39AA - Tomb of Tulak Hord Unlimited Splits
    #[default = false]
    pub KORR_M39AA_unlim: bool,


    // KASHYYYK

    /// Kashyyyk Splits
    title_kashyyyk: Title,

    /// KAS_M22AA - Czerka Landing Port
    #[default = false]
    pub KAS_M22AA: bool,

    /// KAS_M22AA - Czerka Landing Port Unlimited Splits
    #[default = false]
    pub KAS_M22AA_unlim: bool,

    /// KAS_M22AB - Wookiee Village
    #[default = false]
    pub KAS_M22AB: bool,

    /// KAS_M22AB - Wookiee Village Unlimited Splits
    #[default = false]
    pub KAS_M22AB_unlim: bool,

    /// KAS_M23AA - Upper Shadowlands
    #[default = false]
    pub KAS_M23AA: bool,

    /// KAS_M23AA - Upper Shadowlands Unlimited Splits
    #[default = false]
    pub KAS_M23AA_unlim: bool,

    /// KAS_M23AB - Lower Shadowlands
    #[default = false]
    pub KAS_M23AB: bool,

    /// KAS_M23AB - Lower Shadowlands Unlimited Splits
    #[default = false]
    pub KAS_M23AB_unlim: bool,

    /// KAS_M23AC - Czerka Camp
    #[default = false]
    pub KAS_M23AC: bool,

    /// KAS_M23AC - Czerka Camp Unlimited Splits
    #[default = false]
    pub KAS_M23AC_unlim: bool,

    /// KAS_M24AA - Great Walkway
    #[default = false]
    pub KAS_M24AA: bool,

    /// KAS_M24AA - Great Walkway Unlimited Splits
    #[default = false]
    pub KAS_M24AA_unlim: bool,

    /// KAS_M25AA - Upper Wroshyr Tree
    #[default = false]
    pub KAS_M25AA: bool,

    /// KAS_M25AA - Upper Wroshyr Tree Unlimited Splits
    #[default = false]
    pub KAS_M25AA_unlim: bool,

    /// KAS_M23AD - Lower Wroshyr Tree
    #[default = false]
    pub KAS_M23AD: bool,

    /// KAS_M23AD - Lower Wroshyr Tree Unlimited Splits
    #[default = false]
    pub KAS_M23AD_unlim: bool,


    // TATOOINE

    /// Tatooine Splits
    title_tatooine: Title,

    /// TAT_M17AB - Anchorhead
    #[default = false]
    pub TAT_M17AB: bool,

    /// TAT_M17AB - Anchorhead Unlimited Splits
    #[default = false]
    pub TAT_M17AB_unlim: bool,

    /// TAT_M17AA - Hunting Lodge
    #[default = false]
    pub TAT_M17AA: bool,

    /// TAT_M17AA - Hunting Lodge Unlimited Splits
    #[default = false]
    pub TAT_M17AA_unlim: bool,

    /// TAT_M17AD - Czerka Office
    #[default = false]
    pub TAT_M17AD: bool,

    /// TAT_M17AD - Czerka Office Unlimited Splits
    #[default = false]
    pub TAT_M17AD_unlim: bool,

    /// TAT_M17AG - Cantina
    #[default = false]
    pub TAT_M17AG: bool,

    /// TAT_M17AG - Cantina Unlimited Splits
    #[default = false]
    pub TAT_M17AG_unlim: bool,

    /// TAT_M17AE - Droid Shop
    #[default = false]
    pub TAT_M17AE: bool,

    /// TAT_M17AE - Droid Shop Unlimited Splits
    #[default = false]
    pub TAT_M17AE_unlim: bool,

    /// TAT_M17MG - Swoop Platform
    #[default = false]
    pub TAT_M17MG: bool,

    /// TAT_M17MG - Swoop Platform Unlimited Splits
    #[default = false]
    pub TAT_M17MG_unlim: bool,

    /// TAT_M17AF - Docking Bay
    #[default = false]
    pub TAT_M17AF: bool,

    /// TAT_M17AF - Docking Bay Unlimited Splits
    #[default = false]
    pub TAT_M17AF_unlim: bool,

    /// TAT_M17AC - Czerka Compound
    #[default = false]
    pub TAT_M17AC: bool,

    /// TAT_M17AC - Czerka Compound Unlimited Splits
    #[default = false]
    pub TAT_M17AC_unlim: bool,

    /// TAT_M18AA - Dune Sea
    #[default = false]
    pub TAT_M18AA: bool,

    /// TAT_M18AA - Dune Sea Unlimited Splits
    #[default = false]
    pub TAT_M18AA_unlim: bool,

    /// TAT_M18AB - Sand People Territory
    #[default = false]
    pub TAT_M18AB: bool,

    /// TAT_M18AB - Sand People Territory Unlimited Splits
    #[default = false]
    pub TAT_M18AB_unlim: bool,

    /// TAT_M20AA - Sand People Enclave
    #[default = false]
    pub TAT_M20AA: bool,

    /// TAT_M20AA - Sand People Enclave Unlimited Splits
    #[default = false]
    pub TAT_M20AA_unlim: bool,

    /// TAT_M18AC - Krayt Dragon Cave
    #[default = false]
    pub TAT_M18AC: bool,

    /// TAT_M18AC - Krayt Dragon Cave Unlimited Splits
    #[default = false]
    pub TAT_M18AC_unlim: bool,


    // LEVIATHAN

    /// Leviathan Splits
    title_leviathan: Title,

    /// LEV_M40AA - Prison Block
    #[default = false]
    pub LEV_M40AA: bool,

    /// LEV_M40AA - Prison Block Unlimited Splits
    #[default = false]
    pub LEV_M40AA_unlim: bool,

    /// LEV_M40AB - Command Deck
    #[default = false]
    pub LEV_M40AB: bool,

    /// LEV_M40AB - Command Deck Unlimited Splits
    #[default = false]
    pub LEV_M40AB_unlim: bool,

    /// LEV_M40AD - Hangar
    #[default = false]
    pub LEV_M40AD: bool,

    /// LEV_M40AD - Hangar Unlimited Splits
    #[default = false]
    pub LEV_M40AD_unlim: bool,

    /// LEV_M40AC - Bridge
    #[default = false]
    pub LEV_M40AC: bool,

    /// LEV_M40AC - Bridge Unlimited Splits
    #[default = false]
    pub LEV_M40AC_unlim: bool,


    // LEHON

    /// Lehon Splits
    title_lehon: Title,

    /// UNK_M41AA - Beach
    #[default = false]
    pub UNK_M41AA: bool,

    /// UNK_M41AA - Beach Unlimited Splits
    #[default = false]
    pub UNK_M41AA_unlim: bool,

    /// UNK_M41AC - North Beach
    #[default = false]
    pub UNK_M41AC: bool,

    /// UNK_M41AC - North Beach Unlimited Splits
    #[default = false]
    pub UNK_M41AC_unlim: bool,

    /// UNK_M43AA - Temple Exterior
    #[default = false]
    pub UNK_M43AA: bool,

    /// UNK_M43AA - Temple Exterior Unlimited Splits
    #[default = false]
    pub UNK_M43AA_unlim: bool,

    /// UNK_M41AD - Elder Settlement
    #[default = false]
    pub UNK_M41AD: bool,

    /// UNK_M41AD - Elder Settlement Unlimited Splits
    #[default = false]
    pub UNK_M41AD_unlim: bool,

    /// UNK_M41AB - Rakatan Settlement
    #[default = false]
    pub UNK_M41AB: bool,

    /// UNK_M41AB - Rakatan Settlement Unlimited Splits
    #[default = false]
    pub UNK_M41AB_unlim: bool,

    /// UNK_M42AA - Temple Main Floor
    #[default = false]
    pub UNK_M42AA: bool,

    /// UNK_M42AA - Temple Main Floor Unlimited Splits
    #[default = false]
    pub UNK_M42AA_unlim: bool,

    /// UNK_M44AA - Temple Catacombs
    #[default = false]
    pub UNK_M44AA: bool,

    /// UNK_M44AA - Temple Catacombs Unlimited Splits
    #[default = false]
    pub UNK_M44AA_unlim: bool,

    /// UNK_M44AB - Temple Lower Level
    #[default = false]
    pub UNK_M44AB: bool,

    /// UNK_M44AB - Temple Lower Level Unlimited Splits
    #[default = false]
    pub UNK_M44AB_unlim: bool,

    /// UNK_M44AC - Temple Summit
    #[default = false]
    pub UNK_M44AC: bool,

    /// UNK_M44AC - Temple Summit Unlimited Splits
    #[default = false]
    pub UNK_M44AC_unlim: bool,


    // STAR FORGE

    /// Star Forge Splits
    title_star_forge: Title,

    /// STA_M45AA - Deck One
    #[default = false]
    pub STA_M45AA: bool,

    /// STA_M45AA - Deck One Unlimited Splits
    #[default = false]
    pub STA_M45AA_unlim: bool,

    /// STA_M45AB - Deck Two
    #[default = false]
    pub STA_M45AB: bool,

    /// STA_M45AB - Deck Two Unlimited Splits
    #[default = false]
    pub STA_M45AB_unlim: bool,

    /// STA_M45AC - Deck Three
    #[default = false]
    pub STA_M45AC: bool,

    /// STA_M45AC - Deck Three Unlimited Splits
    #[default = false]
    pub STA_M45AC_unlim: bool,

    /// STA_M45AD - Command Center
    #[default = false]
    pub STA_M45AD: bool,

    /// STA_M45AD - Command Center Unlimited Splits
    #[default = false]
    pub STA_M45AD_unlim: bool,


    // EBON HAWK

    /// Ebon Hawk Splits
    title_ebon_hawk: Title,

    /// EBO_M12AA - Ebon Hawk Interior
    #[default = false]
    pub EBO_M12AA: bool,

    /// EBO_M12AA - Ebon Hawk Interior Unlimited Splits
    #[default = false]
    pub EBO_M12AA_unlim: bool,

    /// EBO_M41AA - Ebon Hawk Cockpit
    #[default = false]
    pub EBO_M41AA: bool,

    /// EBO_M41AA - Ebon Hawk Cockpit Unlimited Splits
    #[default = false]
    pub EBO_M41AA_unlim: bool,

    /// EBO_M46AB - Unknown Location
    #[default = false]
    pub EBO_M46AB: bool,

    /// EBO_M46AB - Unknown Location Unlimited Splits
    #[default = false]
    pub EBO_M46AB_unlim: bool,


    // MISC

      /// Miscellaneous Splits
    title_misc_main: Title,

    /// LIV_M99AA - Yavin Station
    #[default = false]
    pub LIV_M99AA: bool,

    /// LIV_M99AA - Yavin Station Unlimited Splits
    #[default = false]
    pub LIV_M99AA_unlim: bool,

    /// STUNT_00 - Dream Sequence
    #[default = false]
    pub STUNT_00: bool,

    /// STUNT_00 - Dream Sequence Unlimited Splits
    #[default = false]
    pub STUNT_00_unlim: bool,

    /// STUNT_03A - Taris Leviathan Bridge Cutscene
    #[default = false]
    pub STUNT_03A: bool,

    /// STUNT_03A - Taris Leviathan Bridge Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_03A_unlim: bool,

    /// STUNT_06 - Taris Destroyed Cutscene
    #[default = false]
    pub STUNT_06: bool,

    /// STUNT_06 - Taris Destroyed Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_06_unlim: bool,

    /// STUNT_07 - Escaping Taris Conversation
    #[default = false]
    pub STUNT_07: bool,

    /// STUNT_07 - Escaping Taris Conversation Unlimited Splits
    #[default = false]
    pub STUNT_07_unlim: bool,

    /// M12AB - Ebon Hawk Gunner
    #[default = false]
    pub M12AB: bool,

    /// M12AB - Ebon Hawk Gunner Unlimited Splits
    #[default = false]
    pub M12AB_unlim: bool,

    /// STUNT_12 - Calo Nord Leviathan Cutscene
    #[default = false]
    pub STUNT_12: bool,

    /// STUNT_12 - Calo Nord Leviathan Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_12_unlim: bool,

    /// STUNT_14 - Darth Bandon Leviathan Cutscene
    #[default = false]
    pub STUNT_14: bool,

    /// STUNT_14 - Darth Bandon Leviathan Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_14_unlim: bool,

    /// STUNT_16 - Leviathan Capture Cutscene
    #[default = false]
    pub STUNT_16: bool,

    /// STUNT_16 - Leviathan Capture Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_16_unlim: bool,

    /// STUNT_19 - Star Forge Lehon Temple Cutscene
    #[default = false]
    pub STUNT_19: bool,

    /// STUNT_19 - Star Forge Lehon Temple Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_19_unlim: bool,

    /// EBO_M40AA - Leviathan Game Plan Cutscene
    #[default = false]
    pub EBO_M40AA: bool,

    /// EBO_M40AA - Leviathan Game Plan Cutscene Unlimited Splits
    #[default = false]
    pub EBO_M40AA_unlim: bool,

    /// STUNT_31B - Revan Reveal Cutscene
    #[default = false]
    pub STUNT_31B: bool,

    /// STUNT_31B - Revan Reveal Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_31B_unlim: bool,

    /// EBO_M40AD - Leviathan Escape Cutscene
    #[default = false]
    pub EBO_M40AD: bool,

    /// EBO_M40AD - Leviathan Escape Cutscene Unlimited Splits
    #[default = false]
    pub EBO_M40AD_unlim: bool,

    /// STUNT_18 - Bastilla Torture Cutscene
    #[default = false]
    pub STUNT_18: bool,

    /// STUNT_18 - Bastilla Torture Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_18_unlim: bool,

    /// STUNT_35 - Lehon Arrival Cutscene
    #[default = false]
    pub STUNT_35: bool,

    /// STUNT_35 - Lehon Arrival Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_35_unlim: bool,

    /// STUNT_42 - Lehon Departure Cutscene (LS)
    #[default = false]
    pub STUNT_42: bool,

    /// STUNT_42 - Lehon Departure Cutscene (LS) Unlimited Splits
    #[default = false]
    pub STUNT_42_unlim: bool,

    /// STUNT_44 - Lehon Departure Cutscene (DS)
    #[default = false]
    pub STUNT_44: bool,

    /// STUNT_44 - Lehon Departure Cutscene (DS) Unlimited Splits
    #[default = false]
    pub STUNT_44_unlim: bool,

    /// STUNT_34 - Star Forge Arrival Cutscene
    #[default = false]
    pub STUNT_34: bool,

    /// STUNT_34 - Star Forge Arrival Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_34_unlim: bool,

    /// STUNT_50A - Green Squadron Cutscene
    #[default = false]
    pub STUNT_50A: bool,

    /// STUNT_50A - Green Squadron Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_50A_unlim: bool,

    /// STUNT_54A - The Republic is Doomed Cutscene
    #[default = false]
    pub STUNT_54A: bool,

    /// STUNT_54A - The Republic is Doomed Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_54A_unlim: bool,

    /// STUNT_55A - All Hail Lord Revan Cutscene
    #[default = false]
    pub STUNT_55A: bool,

    /// STUNT_55A - All Hail Lord Revan Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_55A_unlim: bool,

    /// STUNT_56A - Star Forge Destroyed Cutscene
    #[default = false]
    pub STUNT_56A: bool,

    /// STUNT_56A - Star Forge Destroyed Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_56A_unlim: bool,

    /// STUNT_57A - The Sith are Defeated Cutscene
    #[default = false]
    pub STUNT_57A: bool,

    /// STUNT_57A - The Sith are Defeated Cutscene Unlimited Splits
    #[default = false]
    pub STUNT_57A_unlim: bool,

    /// Bastilla Saved
    #[default = false]
    pub BastilaSaved: bool,


}


impl Settings {
    /// Return (enabled, unlim) tuple for a given Area variant.
    /// Keeps all split lookups compile-time-safe.
    pub fn for_area(&self, area: &Area) -> (bool, bool) {
        match area {
            Area::END_M01AA => (self.END_M01AA, self.END_M01AA_unlim),
            Area::END_M01AB => (self.END_M01AB, self.END_M01AB_unlim),
            Area::TAR_M02AF => (self.TAR_M02AF, self.TAR_M02AF_unlim),
            Area::TAR_M02AA => (self.TAR_M02AA, self.TAR_M02AA_unlim),
            Area::TAR_M02AC => (self.TAR_M02AC, self.TAR_M02AC_unlim),
            Area::TAR_M02AE => (self.TAR_M02AE, self.TAR_M02AE_unlim),
            Area::TAR_M02AB => (self.TAR_M02AB, self.TAR_M02AB_unlim),
            Area::TAR_M02AD => (self.TAR_M02AD, self.TAR_M02AD_unlim),
            Area::TAR_M03AA => (self.TAR_M03AA, self.TAR_M03AA_unlim),
            Area::TAR_M03AD => (self.TAR_M03AD, self.TAR_M03AD_unlim),
            Area::TAR_M03AE => (self.TAR_M03AE, self.TAR_M03AE_unlim),
            Area::TAR_M03AB => (self.TAR_M03AB, self.TAR_M03AB_unlim),
            Area::TAR_M11AA => (self.TAR_M11AA, self.TAR_M11AA_unlim),
            Area::TAR_M11AB => (self.TAR_M11AB, self.TAR_M11AB_unlim),
            Area::TAR_M04AA => (self.TAR_M04AA, self.TAR_M04AA_unlim),
            Area::TAR_M05AA => (self.TAR_M05AA, self.TAR_M05AA_unlim),
            Area::TAR_M05AB => (self.TAR_M05AB, self.TAR_M05AB_unlim),
            Area::TAR_M10AA => (self.TAR_M10AA, self.TAR_M10AA_unlim),
            Area::TAR_M10AB => (self.TAR_M10AB, self.TAR_M10AB_unlim),
            Area::TAR_M10AC => (self.TAR_M10AC, self.TAR_M10AC_unlim),
            Area::TAR_M03AF => (self.TAR_M03AF, self.TAR_M03AF_unlim),
            Area::TAR_M03MG => (self.TAR_M03MG, self.TAR_M03MG_unlim),
            Area::TAR_M09AA => (self.TAR_M09AA, self.TAR_M09AA_unlim),
            Area::TAR_M09AB => (self.TAR_M09AB, self.TAR_M09AB_unlim),
            Area::TAR_M08AA => (self.TAR_M08AA, self.TAR_M08AA_unlim),
            Area::DANM13 => (self.DANM13, self.DANM13_unlim),
            Area::DANM14AA => (self.DANM14AA, self.DANM14AA_unlim),
            Area::DANM14AB => (self.DANM14AB, self.DANM14AB_unlim),
            Area::DANM14AC => (self.DANM14AC, self.DANM14AC_unlim),
            Area::DANM14AD => (self.DANM14AD, self.DANM14AD_unlim),
            Area::DANM14AE => (self.DANM14AE, self.DANM14AE_unlim),
            Area::DANM16 => (self.DANM16, self.DANM16_unlim),
            Area::DANM15 => (self.DANM15, self.DANM15_unlim),
            Area::MANM26AD => (self.MANM26AD, self.MANM26AD_unlim),
            Area::MANM26AC => (self.MANM26AC, self.MANM26AC_unlim),
            Area::MANM26AA => (self.MANM26AA, self.MANM26AA_unlim),
            Area::MANM26AE => (self.MANM26AE, self.MANM26AE_unlim),
            Area::MANM26AB => (self.MANM26AB, self.MANM26AB_unlim),
            Area::MANM26MG => (self.MANM26MG, self.MANM26MG_unlim),
            Area::MANM27AA => (self.MANM27AA, self.MANM27AA_unlim),
            Area::MANM28AA => (self.MANM28AA, self.MANM28AA_unlim),
            Area::MANM28AB => (self.MANM28AB, self.MANM28AB_unlim),
            Area::MANM28AC => (self.MANM28AC, self.MANM28AC_unlim),
            Area::MANM28AD => (self.MANM28AD, self.MANM28AD_unlim),
            Area::KORR_M33AA => (self.KORR_M33AA, self.KORR_M33AA_unlim),
            Area::KORR_M33AB => (self.KORR_M33AB, self.KORR_M33AB_unlim),
            Area::KORR_M35AA => (self.KORR_M35AA, self.KORR_M35AA_unlim),
            Area::KORR_M36AA => (self.KORR_M36AA, self.KORR_M36AA_unlim),
            Area::KORR_M34AA => (self.KORR_M34AA, self.KORR_M34AA_unlim),
            Area::KORR_M38AA => (self.KORR_M38AA, self.KORR_M38AA_unlim),
            Area::KORR_M37AA => (self.KORR_M37AA, self.KORR_M37AA_unlim),
            Area::KORR_M38AB => (self.KORR_M38AB, self.KORR_M38AB_unlim),
            Area::KORR_M39AA => (self.KORR_M39AA, self.KORR_M39AA_unlim),
            Area::KAS_M22AA => (self.KAS_M22AA, self.KAS_M22AA_unlim),
            Area::KAS_M22AB => (self.KAS_M22AB, self.KAS_M22AB_unlim),
            Area::KAS_M23AA => (self.KAS_M23AA, self.KAS_M23AA_unlim),
            Area::KAS_M23AB => (self.KAS_M23AB, self.KAS_M23AB_unlim),
            Area::KAS_M23AC => (self.KAS_M23AC, self.KAS_M23AC_unlim),
            Area::KAS_M24AA => (self.KAS_M24AA, self.KAS_M24AA_unlim),
            Area::KAS_M25AA => (self.KAS_M25AA, self.KAS_M25AA_unlim),
            Area::KAS_M23AD => (self.KAS_M23AD, self.KAS_M23AD_unlim),
            Area::TAT_M17AB => (self.TAT_M17AB, self.TAT_M17AB_unlim),
            Area::TAT_M17AA => (self.TAT_M17AA, self.TAT_M17AA_unlim),
            Area::TAT_M17AD => (self.TAT_M17AD, self.TAT_M17AD_unlim),
            Area::TAT_M17AG => (self.TAT_M17AG, self.TAT_M17AG_unlim),
            Area::TAT_M17AE => (self.TAT_M17AE, self.TAT_M17AE_unlim),
            Area::TAT_M17MG => (self.TAT_M17MG, self.TAT_M17MG_unlim),
            Area::TAT_M17AF => (self.TAT_M17AF, self.TAT_M17AF_unlim),
            Area::TAT_M17AC => (self.TAT_M17AC, self.TAT_M17AC_unlim),
            Area::TAT_M18AA => (self.TAT_M18AA, self.TAT_M18AA_unlim),
            Area::TAT_M18AB => (self.TAT_M18AB, self.TAT_M18AB_unlim),
            Area::TAT_M20AA => (self.TAT_M20AA, self.TAT_M20AA_unlim),
            Area::TAT_M18AC => (self.TAT_M18AC, self.TAT_M18AC_unlim),
            Area::LEV_M40AA => (self.LEV_M40AA, self.LEV_M40AA_unlim),
            Area::LEV_M40AB => (self.LEV_M40AB, self.LEV_M40AB_unlim),
            Area::LEV_M40AD => (self.LEV_M40AD, self.LEV_M40AD_unlim),
            Area::LEV_M40AC => (self.LEV_M40AC, self.LEV_M40AC_unlim),
            Area::UNK_M41AA => (self.UNK_M41AA, self.UNK_M41AA_unlim),
            Area::UNK_M41AC => (self.UNK_M41AC, self.UNK_M41AC_unlim),
            Area::UNK_M43AA => (self.UNK_M43AA, self.UNK_M43AA_unlim),
            Area::UNK_M41AD => (self.UNK_M41AD, self.UNK_M41AD_unlim),
            Area::UNK_M41AB => (self.UNK_M41AB, self.UNK_M41AB_unlim),
            Area::UNK_M42AA => (self.UNK_M42AA, self.UNK_M42AA_unlim),
            Area::UNK_M44AA => (self.UNK_M44AA, self.UNK_M44AA_unlim),
            Area::UNK_M44AB => (self.UNK_M44AB, self.UNK_M44AB_unlim),
            Area::UNK_M44AC => (self.UNK_M44AC, self.UNK_M44AC_unlim),
            Area::STA_M45AA => (self.STA_M45AA, self.STA_M45AA_unlim),
            Area::STA_M45AB => (self.STA_M45AB, self.STA_M45AB_unlim),
            Area::STA_M45AC => (self.STA_M45AC, self.STA_M45AC_unlim),
            Area::STA_M45AD => (self.STA_M45AD, self.STA_M45AD_unlim),
            Area::EBO_M12AA => (self.EBO_M12AA, self.EBO_M12AA_unlim),
            Area::EBO_M41AA => (self.EBO_M41AA, self.EBO_M41AA_unlim),
            Area::EBO_M46AB => (self.EBO_M46AB, self.EBO_M46AB_unlim),
            Area::MISC_CREDITS => (self.MISC_CREDITS, self.MISC_CREDITS_unlim),
            Area::MISC_PRELUDE => (self.MISC_PRELUDE, self.MISC_PRELUDE_unlim),
            Area::MISC_TUTORIAL => (self.MISC_TUTORIAL, self.MISC_TUTORIAL_unlim),
            Area::MISC_JEDITEST => (self.MISC_JEDITEST, self.MISC_JEDITEST_unlim),
            Area::MISC_EARLYACCESS => (self.MISC_EARLYACCESS, self.MISC_EARLYACCESS_unlim),
            Area::Unknown(_) => (false, false),
        }
    }
}

pub type EpsgCode = u16;

pub const EPSG_WGS84_GEOGRAPHIC_2D: EpsgCode = 4326;
pub const EPSG_WGS84_GEOGRAPHIC_3D: EpsgCode = 4979;
pub const EPSG_WGS84_GEOCENTRIC: EpsgCode = 4978;

// Web Mercator
pub const EPSG_WEB_MERCATOR: EpsgCode = 3857;

/// JGD2011
pub const EPSG_JGD2011_GEOGRAPHIC_2D: EpsgCode = 6668;

/// JGD2011 + JGD2011 (vertical) height
pub const EPSG_JGD2011_GEOGRAPHIC_3D: EpsgCode = 6697;

// JGD2011 / Japan Plane Rectangular CS + JGD2011 (vertical) height
// Note: Only I - XIII are defined (XIV - XIX does not exist)
pub const EPSG_JGD2011_JPRECT_I_JGD2011_HEIGHT: EpsgCode = 10162;
pub const EPSG_JGD2011_JPRECT_II_JGD2011_HEIGHT: EpsgCode = 10163;
pub const EPSG_JGD2011_JPRECT_III_JGD2011_HEIGHT: EpsgCode = 10164;
pub const EPSG_JGD2011_JPRECT_IV_JGD2011_HEIGHT: EpsgCode = 10165;
pub const EPSG_JGD2011_JPRECT_V_JGD2011_HEIGHT: EpsgCode = 10166;
pub const EPSG_JGD2011_JPRECT_VI_JGD2011_HEIGHT: EpsgCode = 10167;
pub const EPSG_JGD2011_JPRECT_VII_JGD2011_HEIGHT: EpsgCode = 10168;
pub const EPSG_JGD2011_JPRECT_VIII_JGD2011_HEIGHT: EpsgCode = 10169;
pub const EPSG_JGD2011_JPRECT_IX_JGD2011_HEIGHT: EpsgCode = 10170;
pub const EPSG_JGD2011_JPRECT_X_JGD2011_HEIGHT: EpsgCode = 10171;
pub const EPSG_JGD2011_JPRECT_XI_JGD2011_HEIGHT: EpsgCode = 10172;
pub const EPSG_JGD2011_JPRECT_XII_JGD2011_HEIGHT: EpsgCode = 10173;
pub const EPSG_JGD2011_JPRECT_XIII_JGD2011_HEIGHT: EpsgCode = 10174;

// JGD2011 / Japan Plane Rectangular CS
pub const EPSG_JGD2011_JPRECT_I: EpsgCode = 6669;
pub const EPSG_JGD2011_JPRECT_II: EpsgCode = 6670;
pub const EPSG_JGD2011_JPRECT_III: EpsgCode = 6671;
pub const EPSG_JGD2011_JPRECT_IV: EpsgCode = 6672;
pub const EPSG_JGD2011_JPRECT_V: EpsgCode = 6673;
pub const EPSG_JGD2011_JPRECT_VI: EpsgCode = 6674;
pub const EPSG_JGD2011_JPRECT_VII: EpsgCode = 6675;
pub const EPSG_JGD2011_JPRECT_VIII: EpsgCode = 6676;
pub const EPSG_JGD2011_JPRECT_IX: EpsgCode = 6677;
pub const EPSG_JGD2011_JPRECT_X: EpsgCode = 6678;
pub const EPSG_JGD2011_JPRECT_XI: EpsgCode = 6679;
pub const EPSG_JGD2011_JPRECT_XII: EpsgCode = 6680;
pub const EPSG_JGD2011_JPRECT_XIII: EpsgCode = 6681;
pub const EPSG_JGD2011_JPRECT_XIV: EpsgCode = 6682;
pub const EPSG_JGD2011_JPRECT_XV: EpsgCode = 6683;
pub const EPSG_JGD2011_JPRECT_XVI: EpsgCode = 6684;
pub const EPSG_JGD2011_JPRECT_XVII: EpsgCode = 6685;
pub const EPSG_JGD2011_JPRECT_XVIII: EpsgCode = 6686;
pub const EPSG_JGD2011_JPRECT_XIX: EpsgCode = 6687;

// JGD2000 / Japan Plane Rectangular CS
pub const EPSG_JGD2000_JPRECT_I: EpsgCode = 2443;
pub const EPSG_JGD2000_JPRECT_II: EpsgCode = 2444;
pub const EPSG_JGD2000_JPRECT_III: EpsgCode = 2445;
pub const EPSG_JGD2000_JPRECT_IV: EpsgCode = 2446;
pub const EPSG_JGD2000_JPRECT_V: EpsgCode = 2447;
pub const EPSG_JGD2000_JPRECT_VI: EpsgCode = 2448;
pub const EPSG_JGD2000_JPRECT_VII: EpsgCode = 2449;
pub const EPSG_JGD2000_JPRECT_VIII: EpsgCode = 2450;
pub const EPSG_JGD2000_JPRECT_IX: EpsgCode = 2451;
pub const EPSG_JGD2000_JPRECT_X: EpsgCode = 2452;
pub const EPSG_JGD2000_JPRECT_XI: EpsgCode = 2453;
pub const EPSG_JGD2000_JPRECT_XII: EpsgCode = 2454;
pub const EPSG_JGD2000_JPRECT_XIII: EpsgCode = 2455;
pub const EPSG_JGD2000_JPRECT_XIV: EpsgCode = 2456;
pub const EPSG_JGD2000_JPRECT_XV: EpsgCode = 2457;
pub const EPSG_JGD2000_JPRECT_XVI: EpsgCode = 2458;
pub const EPSG_JGD2000_JPRECT_XVII: EpsgCode = 2459;
pub const EPSG_JGD2000_JPRECT_XVIII: EpsgCode = 2460;
pub const EPSG_JGD2000_JPRECT_XIX: EpsgCode = 2461;

// Tokyo / Japan Plane Rectangular CS
pub const EPSG_TOKYO_JPRECT_I: EpsgCode = 30161;
pub const EPSG_TOKYO_JPRECT_II: EpsgCode = 30162;
pub const EPSG_TOKYO_JPRECT_III: EpsgCode = 30163;
pub const EPSG_TOKYO_JPRECT_IV: EpsgCode = 30164;
pub const EPSG_TOKYO_JPRECT_V: EpsgCode = 30165;
pub const EPSG_TOKYO_JPRECT_VI: EpsgCode = 30166;
pub const EPSG_TOKYO_JPRECT_VII: EpsgCode = 30167;
pub const EPSG_TOKYO_JPRECT_VIII: EpsgCode = 30168;
pub const EPSG_TOKYO_JPRECT_IX: EpsgCode = 30169;
pub const EPSG_TOKYO_JPRECT_X: EpsgCode = 30170;
pub const EPSG_TOKYO_JPRECT_XI: EpsgCode = 30171;
pub const EPSG_TOKYO_JPRECT_XII: EpsgCode = 30172;
pub const EPSG_TOKYO_JPRECT_XIII: EpsgCode = 30173;
pub const EPSG_TOKYO_JPRECT_XIV: EpsgCode = 30174;
pub const EPSG_TOKYO_JPRECT_XV: EpsgCode = 30175;
pub const EPSG_TOKYO_JPRECT_XVI: EpsgCode = 30176;
pub const EPSG_TOKYO_JPRECT_XVII: EpsgCode = 30177;
pub const EPSG_TOKYO_JPRECT_XVIII: EpsgCode = 30178;
pub const EPSG_TOKYO_JPRECT_XIX: EpsgCode = 30179;

// =============================================================================
// GLOBAL COORDINATE SYSTEMS
// =============================================================================

// -----------------------------------------------------------------------------
// UTM (Universal Transverse Mercator) - WGS84 based
// -----------------------------------------------------------------------------
// Northern Hemisphere (EPSG:32601 - 32660)
pub const EPSG_WGS84_UTM_1N: EpsgCode = 32601;
pub const EPSG_WGS84_UTM_2N: EpsgCode = 32602;
pub const EPSG_WGS84_UTM_3N: EpsgCode = 32603;
pub const EPSG_WGS84_UTM_4N: EpsgCode = 32604;
pub const EPSG_WGS84_UTM_5N: EpsgCode = 32605;
pub const EPSG_WGS84_UTM_6N: EpsgCode = 32606;
pub const EPSG_WGS84_UTM_7N: EpsgCode = 32607;
pub const EPSG_WGS84_UTM_8N: EpsgCode = 32608;
pub const EPSG_WGS84_UTM_9N: EpsgCode = 32609;
pub const EPSG_WGS84_UTM_10N: EpsgCode = 32610;
pub const EPSG_WGS84_UTM_11N: EpsgCode = 32611;
pub const EPSG_WGS84_UTM_12N: EpsgCode = 32612;
pub const EPSG_WGS84_UTM_13N: EpsgCode = 32613;
pub const EPSG_WGS84_UTM_14N: EpsgCode = 32614;
pub const EPSG_WGS84_UTM_15N: EpsgCode = 32615;
pub const EPSG_WGS84_UTM_16N: EpsgCode = 32616;
pub const EPSG_WGS84_UTM_17N: EpsgCode = 32617;
pub const EPSG_WGS84_UTM_18N: EpsgCode = 32618;
pub const EPSG_WGS84_UTM_19N: EpsgCode = 32619;
pub const EPSG_WGS84_UTM_20N: EpsgCode = 32620;
pub const EPSG_WGS84_UTM_21N: EpsgCode = 32621;
pub const EPSG_WGS84_UTM_22N: EpsgCode = 32622;
pub const EPSG_WGS84_UTM_23N: EpsgCode = 32623;
pub const EPSG_WGS84_UTM_24N: EpsgCode = 32624;
pub const EPSG_WGS84_UTM_25N: EpsgCode = 32625;
pub const EPSG_WGS84_UTM_26N: EpsgCode = 32626;
pub const EPSG_WGS84_UTM_27N: EpsgCode = 32627;
pub const EPSG_WGS84_UTM_28N: EpsgCode = 32628;
pub const EPSG_WGS84_UTM_29N: EpsgCode = 32629;
pub const EPSG_WGS84_UTM_30N: EpsgCode = 32630;
pub const EPSG_WGS84_UTM_31N: EpsgCode = 32631;
pub const EPSG_WGS84_UTM_32N: EpsgCode = 32632;
pub const EPSG_WGS84_UTM_33N: EpsgCode = 32633;
pub const EPSG_WGS84_UTM_34N: EpsgCode = 32634;
pub const EPSG_WGS84_UTM_35N: EpsgCode = 32635;
pub const EPSG_WGS84_UTM_36N: EpsgCode = 32636;
pub const EPSG_WGS84_UTM_37N: EpsgCode = 32637;
pub const EPSG_WGS84_UTM_38N: EpsgCode = 32638;
pub const EPSG_WGS84_UTM_39N: EpsgCode = 32639;
pub const EPSG_WGS84_UTM_40N: EpsgCode = 32640;
pub const EPSG_WGS84_UTM_41N: EpsgCode = 32641;
pub const EPSG_WGS84_UTM_42N: EpsgCode = 32642;
pub const EPSG_WGS84_UTM_43N: EpsgCode = 32643;
pub const EPSG_WGS84_UTM_44N: EpsgCode = 32644;
pub const EPSG_WGS84_UTM_45N: EpsgCode = 32645;
pub const EPSG_WGS84_UTM_46N: EpsgCode = 32646;
pub const EPSG_WGS84_UTM_47N: EpsgCode = 32647;
pub const EPSG_WGS84_UTM_48N: EpsgCode = 32648;
pub const EPSG_WGS84_UTM_49N: EpsgCode = 32649;
pub const EPSG_WGS84_UTM_50N: EpsgCode = 32650;
pub const EPSG_WGS84_UTM_51N: EpsgCode = 32651;
pub const EPSG_WGS84_UTM_52N: EpsgCode = 32652;
pub const EPSG_WGS84_UTM_53N: EpsgCode = 32653;
pub const EPSG_WGS84_UTM_54N: EpsgCode = 32654;
pub const EPSG_WGS84_UTM_55N: EpsgCode = 32655;
pub const EPSG_WGS84_UTM_56N: EpsgCode = 32656;
pub const EPSG_WGS84_UTM_57N: EpsgCode = 32657;
pub const EPSG_WGS84_UTM_58N: EpsgCode = 32658;
pub const EPSG_WGS84_UTM_59N: EpsgCode = 32659;
pub const EPSG_WGS84_UTM_60N: EpsgCode = 32660;

// Southern Hemisphere (EPSG:32701 - 32760)
pub const EPSG_WGS84_UTM_1S: EpsgCode = 32701;
pub const EPSG_WGS84_UTM_2S: EpsgCode = 32702;
pub const EPSG_WGS84_UTM_3S: EpsgCode = 32703;
pub const EPSG_WGS84_UTM_4S: EpsgCode = 32704;
pub const EPSG_WGS84_UTM_5S: EpsgCode = 32705;
pub const EPSG_WGS84_UTM_6S: EpsgCode = 32706;
pub const EPSG_WGS84_UTM_7S: EpsgCode = 32707;
pub const EPSG_WGS84_UTM_8S: EpsgCode = 32708;
pub const EPSG_WGS84_UTM_9S: EpsgCode = 32709;
pub const EPSG_WGS84_UTM_10S: EpsgCode = 32710;
pub const EPSG_WGS84_UTM_11S: EpsgCode = 32711;
pub const EPSG_WGS84_UTM_12S: EpsgCode = 32712;
pub const EPSG_WGS84_UTM_13S: EpsgCode = 32713;
pub const EPSG_WGS84_UTM_14S: EpsgCode = 32714;
pub const EPSG_WGS84_UTM_15S: EpsgCode = 32715;
pub const EPSG_WGS84_UTM_16S: EpsgCode = 32716;
pub const EPSG_WGS84_UTM_17S: EpsgCode = 32717;
pub const EPSG_WGS84_UTM_18S: EpsgCode = 32718;
pub const EPSG_WGS84_UTM_19S: EpsgCode = 32719;
pub const EPSG_WGS84_UTM_20S: EpsgCode = 32720;
pub const EPSG_WGS84_UTM_21S: EpsgCode = 32721;
pub const EPSG_WGS84_UTM_22S: EpsgCode = 32722;
pub const EPSG_WGS84_UTM_23S: EpsgCode = 32723;
pub const EPSG_WGS84_UTM_24S: EpsgCode = 32724;
pub const EPSG_WGS84_UTM_25S: EpsgCode = 32725;
pub const EPSG_WGS84_UTM_26S: EpsgCode = 32726;
pub const EPSG_WGS84_UTM_27S: EpsgCode = 32727;
pub const EPSG_WGS84_UTM_28S: EpsgCode = 32728;
pub const EPSG_WGS84_UTM_29S: EpsgCode = 32729;
pub const EPSG_WGS84_UTM_30S: EpsgCode = 32730;
pub const EPSG_WGS84_UTM_31S: EpsgCode = 32731;
pub const EPSG_WGS84_UTM_32S: EpsgCode = 32732;
pub const EPSG_WGS84_UTM_33S: EpsgCode = 32733;
pub const EPSG_WGS84_UTM_34S: EpsgCode = 32734;
pub const EPSG_WGS84_UTM_35S: EpsgCode = 32735;
pub const EPSG_WGS84_UTM_36S: EpsgCode = 32736;
pub const EPSG_WGS84_UTM_37S: EpsgCode = 32737;
pub const EPSG_WGS84_UTM_38S: EpsgCode = 32738;
pub const EPSG_WGS84_UTM_39S: EpsgCode = 32739;
pub const EPSG_WGS84_UTM_40S: EpsgCode = 32740;
pub const EPSG_WGS84_UTM_41S: EpsgCode = 32741;
pub const EPSG_WGS84_UTM_42S: EpsgCode = 32742;
pub const EPSG_WGS84_UTM_43S: EpsgCode = 32743;
pub const EPSG_WGS84_UTM_44S: EpsgCode = 32744;
pub const EPSG_WGS84_UTM_45S: EpsgCode = 32745;
pub const EPSG_WGS84_UTM_46S: EpsgCode = 32746;
pub const EPSG_WGS84_UTM_47S: EpsgCode = 32747;
pub const EPSG_WGS84_UTM_48S: EpsgCode = 32748;
pub const EPSG_WGS84_UTM_49S: EpsgCode = 32749;
pub const EPSG_WGS84_UTM_50S: EpsgCode = 32750;
pub const EPSG_WGS84_UTM_51S: EpsgCode = 32751;
pub const EPSG_WGS84_UTM_52S: EpsgCode = 32752;
pub const EPSG_WGS84_UTM_53S: EpsgCode = 32753;
pub const EPSG_WGS84_UTM_54S: EpsgCode = 32754;
pub const EPSG_WGS84_UTM_55S: EpsgCode = 32755;
pub const EPSG_WGS84_UTM_56S: EpsgCode = 32756;
pub const EPSG_WGS84_UTM_57S: EpsgCode = 32757;
pub const EPSG_WGS84_UTM_58S: EpsgCode = 32758;
pub const EPSG_WGS84_UTM_59S: EpsgCode = 32759;
pub const EPSG_WGS84_UTM_60S: EpsgCode = 32760;

// -----------------------------------------------------------------------------
// North America - NAD83 (North American Datum 1983)
// -----------------------------------------------------------------------------
pub const EPSG_NAD83_GEOGRAPHIC_2D: EpsgCode = 4269;
pub const EPSG_NAD83_GEOGRAPHIC_3D: EpsgCode = 4955;

// NAD83 State Plane (US) - Sample of commonly used zones
pub const EPSG_NAD83_CALIFORNIA_ZONE_1: EpsgCode = 26941;
pub const EPSG_NAD83_CALIFORNIA_ZONE_2: EpsgCode = 26942;
pub const EPSG_NAD83_CALIFORNIA_ZONE_3: EpsgCode = 26943;
pub const EPSG_NAD83_CALIFORNIA_ZONE_4: EpsgCode = 26944;
pub const EPSG_NAD83_CALIFORNIA_ZONE_5: EpsgCode = 26945;
pub const EPSG_NAD83_CALIFORNIA_ZONE_6: EpsgCode = 26946;
pub const EPSG_NAD83_TEXAS_NORTH: EpsgCode = 32138;
pub const EPSG_NAD83_TEXAS_CENTRAL: EpsgCode = 32139;
pub const EPSG_NAD83_TEXAS_SOUTH: EpsgCode = 32141;
pub const EPSG_NAD83_FLORIDA_EAST: EpsgCode = 26960;
pub const EPSG_NAD83_FLORIDA_WEST: EpsgCode = 26961;
pub const EPSG_NAD83_FLORIDA_NORTH: EpsgCode = 26962;
pub const EPSG_NAD83_NEW_YORK_LONG_ISLAND: EpsgCode = 32118;
pub const EPSG_NAD83_WASHINGTON_NORTH: EpsgCode = 32148;
pub const EPSG_NAD83_WASHINGTON_SOUTH: EpsgCode = 32149;

// NAD83(2011) - Modern NAD83 realization
pub const EPSG_NAD83_2011_GEOGRAPHIC_2D: EpsgCode = 6318;
pub const EPSG_NAD83_2011_GEOGRAPHIC_3D: EpsgCode = 6319;

// NAD27 (North American Datum 1927) - Legacy
pub const EPSG_NAD27_GEOGRAPHIC: EpsgCode = 4267;

// -----------------------------------------------------------------------------
// Europe - ETRS89 (European Terrestrial Reference System 1989)
// -----------------------------------------------------------------------------
pub const EPSG_ETRS89_GEOGRAPHIC_2D: EpsgCode = 4258;
pub const EPSG_ETRS89_GEOGRAPHIC_3D: EpsgCode = 4937;

// ETRS89 UTM Zones (Europe)
pub const EPSG_ETRS89_UTM_28N: EpsgCode = 25828;
pub const EPSG_ETRS89_UTM_29N: EpsgCode = 25829;
pub const EPSG_ETRS89_UTM_30N: EpsgCode = 25830;
pub const EPSG_ETRS89_UTM_31N: EpsgCode = 25831;
pub const EPSG_ETRS89_UTM_32N: EpsgCode = 25832;
pub const EPSG_ETRS89_UTM_33N: EpsgCode = 25833;
pub const EPSG_ETRS89_UTM_34N: EpsgCode = 25834;
pub const EPSG_ETRS89_UTM_35N: EpsgCode = 25835;
pub const EPSG_ETRS89_UTM_36N: EpsgCode = 25836;
pub const EPSG_ETRS89_UTM_37N: EpsgCode = 25837;
pub const EPSG_ETRS89_UTM_38N: EpsgCode = 25838;

// United Kingdom
pub const EPSG_OSGB36_BRITISH_NATIONAL_GRID: EpsgCode = 27700;
pub const EPSG_OSGB36_GEOGRAPHIC: EpsgCode = 4277;

// France
pub const EPSG_RGF93_GEOGRAPHIC: EpsgCode = 4171;
pub const EPSG_RGF93_LAMBERT_93: EpsgCode = 2154;
pub const EPSG_NTF_LAMBERT_II_ETENDU: EpsgCode = 27572; // Legacy

// Germany
pub const EPSG_DHDN_GAUSS_KRUGER_ZONE_2: EpsgCode = 31466;
pub const EPSG_DHDN_GAUSS_KRUGER_ZONE_3: EpsgCode = 31467;
pub const EPSG_DHDN_GAUSS_KRUGER_ZONE_4: EpsgCode = 31468;
pub const EPSG_DHDN_GAUSS_KRUGER_ZONE_5: EpsgCode = 31469;

// Switzerland
pub const EPSG_CH1903_LV03: EpsgCode = 21781;
pub const EPSG_CH1903_PLUS_LV95: EpsgCode = 2056;

// Netherlands
pub const EPSG_AMERSFOORT_RD_NEW: EpsgCode = 28992;

// Spain
pub const EPSG_ED50_UTM_30N: EpsgCode = 23030;
pub const EPSG_ED50_UTM_31N: EpsgCode = 23031;

// Italy
pub const EPSG_ROME40_WEST_ZONE: EpsgCode = 26591;
pub const EPSG_ROME40_CENTRAL_ZONE: EpsgCode = 26592;

// -----------------------------------------------------------------------------
// Asia-Pacific
// -----------------------------------------------------------------------------

// China
pub const EPSG_CGCS2000_GEOGRAPHIC_2D: EpsgCode = 4490;
pub const EPSG_CGCS2000_GEOGRAPHIC_3D: EpsgCode = 4491;
pub const EPSG_CGCS2000_GAUSS_KRUGER_CM_75E: EpsgCode = 4491;
pub const EPSG_CGCS2000_3_DEGREE_GK_CM_75E: EpsgCode = 4513;

// South Korea
pub const EPSG_KOREA_2000_GEOGRAPHIC: EpsgCode = 4737;
pub const EPSG_KOREA_2000_CENTRAL_BELT: EpsgCode = 5186;
pub const EPSG_KOREA_2000_WEST_BELT: EpsgCode = 5185;
pub const EPSG_KOREA_2000_EAST_BELT: EpsgCode = 5187;

// Thailand
pub const EPSG_WGS84_UTM_47N_THAILAND: EpsgCode = 32647;
pub const EPSG_WGS84_UTM_48N_THAILAND: EpsgCode = 32648;

// India
pub const EPSG_WGS84_UTM_43N_INDIA: EpsgCode = 32643;
pub const EPSG_WGS84_UTM_44N_INDIA: EpsgCode = 32644;

// Singapore
pub const EPSG_SVY21_SINGAPORE_TM: EpsgCode = 3414;

// Hong Kong
pub const EPSG_HONG_KONG_1980_GRID: EpsgCode = 2326;

// -----------------------------------------------------------------------------
// Australia and Oceania
// -----------------------------------------------------------------------------

// Australia - GDA2020 (Geocentric Datum of Australia 2020)
pub const EPSG_GDA2020_GEOGRAPHIC_2D: EpsgCode = 7844;
pub const EPSG_GDA2020_GEOGRAPHIC_3D: EpsgCode = 7843;

// GDA2020 MGA (Map Grid of Australia) Zones
pub const EPSG_GDA2020_MGA_ZONE_46: EpsgCode = 7846;
pub const EPSG_GDA2020_MGA_ZONE_47: EpsgCode = 7847;
pub const EPSG_GDA2020_MGA_ZONE_48: EpsgCode = 7848;
pub const EPSG_GDA2020_MGA_ZONE_49: EpsgCode = 7849;
pub const EPSG_GDA2020_MGA_ZONE_50: EpsgCode = 7850;
pub const EPSG_GDA2020_MGA_ZONE_51: EpsgCode = 7851;
pub const EPSG_GDA2020_MGA_ZONE_52: EpsgCode = 7852;
pub const EPSG_GDA2020_MGA_ZONE_53: EpsgCode = 7853;
pub const EPSG_GDA2020_MGA_ZONE_54: EpsgCode = 7854;
pub const EPSG_GDA2020_MGA_ZONE_55: EpsgCode = 7855;
pub const EPSG_GDA2020_MGA_ZONE_56: EpsgCode = 7856;
pub const EPSG_GDA2020_MGA_ZONE_57: EpsgCode = 7857;
pub const EPSG_GDA2020_MGA_ZONE_58: EpsgCode = 7858;

// GDA94 (Legacy)
pub const EPSG_GDA94_GEOGRAPHIC: EpsgCode = 4283;

// New Zealand
pub const EPSG_NZGD2000_GEOGRAPHIC_2D: EpsgCode = 4167;
pub const EPSG_NZGD2000_GEOGRAPHIC_3D: EpsgCode = 4959;
pub const EPSG_NZGD2000_NEW_ZEALAND_TM: EpsgCode = 2193;

// -----------------------------------------------------------------------------
// South America
// -----------------------------------------------------------------------------

// Brazil
pub const EPSG_SIRGAS2000_GEOGRAPHIC: EpsgCode = 4674;
pub const EPSG_SIRGAS2000_UTM_18S: EpsgCode = 31978;
pub const EPSG_SIRGAS2000_UTM_19S: EpsgCode = 31979;
pub const EPSG_SIRGAS2000_UTM_20S: EpsgCode = 31980;
pub const EPSG_SIRGAS2000_UTM_21S: EpsgCode = 31981;
pub const EPSG_SIRGAS2000_UTM_22S: EpsgCode = 31982;
pub const EPSG_SIRGAS2000_UTM_23S: EpsgCode = 31983;
pub const EPSG_SIRGAS2000_UTM_24S: EpsgCode = 31984;
pub const EPSG_SIRGAS2000_UTM_25S: EpsgCode = 31985;

// Argentina
pub const EPSG_POSGAR94_GEOGRAPHIC: EpsgCode = 4694;
pub const EPSG_POSGAR2007_GEOGRAPHIC: EpsgCode = 5340;

// -----------------------------------------------------------------------------
// Africa
// -----------------------------------------------------------------------------

// South Africa
pub const EPSG_HARTEBEESTHOEK94_GEOGRAPHIC: EpsgCode = 4148;
pub const EPSG_HARTEBEESTHOEK94_LO15: EpsgCode = 2046;
pub const EPSG_HARTEBEESTHOEK94_LO17: EpsgCode = 2047;
pub const EPSG_HARTEBEESTHOEK94_LO19: EpsgCode = 2048;

// -----------------------------------------------------------------------------
// Polar Regions
// -----------------------------------------------------------------------------

// Arctic
pub const EPSG_WGS84_NSIDC_SEA_ICE_POLAR_STEREOGRAPHIC_NORTH: EpsgCode = 3413;

// Antarctic
pub const EPSG_WGS84_ANTARCTIC_POLAR_STEREOGRAPHIC: EpsgCode = 3031;

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Check if EPSG code is a Japan-specific coordinate system
pub fn is_japan_crs(epsg: EpsgCode) -> bool {
    matches!(
        epsg,
        EPSG_JGD2011_GEOGRAPHIC_2D
            | EPSG_JGD2011_GEOGRAPHIC_3D
            | EPSG_JGD2011_JPRECT_I_JGD2011_HEIGHT..=EPSG_JGD2011_JPRECT_XIII_JGD2011_HEIGHT
            | EPSG_JGD2011_JPRECT_I..=EPSG_JGD2011_JPRECT_XIX
            | EPSG_JGD2000_JPRECT_I..=EPSG_JGD2000_JPRECT_XIX
            | EPSG_TOKYO_JPRECT_I..=EPSG_TOKYO_JPRECT_XIX
    )
}

/// Check if EPSG code is WGS84-based
pub fn is_wgs84_based(epsg: EpsgCode) -> bool {
    matches!(
        epsg,
        EPSG_WGS84_GEOGRAPHIC_2D
            | EPSG_WGS84_GEOGRAPHIC_3D
            | EPSG_WGS84_GEOCENTRIC
            | EPSG_WEB_MERCATOR
            | EPSG_WGS84_UTM_1N..=EPSG_WGS84_UTM_60N
            | EPSG_WGS84_UTM_1S..=EPSG_WGS84_UTM_60S
    )
}

/// Check if EPSG code is a UTM zone
pub fn is_utm(epsg: EpsgCode) -> bool {
    matches!(
        epsg,
        EPSG_WGS84_UTM_1N..=EPSG_WGS84_UTM_60N | EPSG_WGS84_UTM_1S..=EPSG_WGS84_UTM_60S
    )
}

/// Get UTM zone number from EPSG code (1-60), returns None if not a UTM CRS
pub fn get_utm_zone(epsg: EpsgCode) -> Option<u8> {
    match epsg {
        32601..=32660 => Some((epsg - 32600) as u8), // Northern hemisphere
        32701..=32760 => Some((epsg - 32700) as u8), // Southern hemisphere
        _ => None,
    }
}

/// Check if UTM zone is in northern hemisphere
pub fn is_utm_north(epsg: EpsgCode) -> bool {
    matches!(epsg, EPSG_WGS84_UTM_1N..=EPSG_WGS84_UTM_60N)
}

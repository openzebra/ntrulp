#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{P, Q, Q12, SEEDS_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{P, Q, Q12, SEEDS_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{P, Q, Q12, SEEDS_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{P, Q, Q12, SEEDS_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{P, Q, Q12, SEEDS_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{P, Q, Q12, SEEDS_BYTES};

use super::errors::NTRUErrors;
use crate::{math::nums::u32_mod_u14, ntru::aes::aes256_ctr_crypto_stream};

pub fn expand(k: &[u8; SEEDS_BYTES]) -> Result<[u32; P], NTRUErrors<'static>> {
    let out = match aes256_ctr_crypto_stream(k) {
        Ok(o) => o,
        Err(_) => return Err(NTRUErrors::LPRExpandError("aes error")),
    };

    Ok(out)
}

pub fn generator(k: &[u8; SEEDS_BYTES]) -> Result<[i16; P], NTRUErrors<'static>> {
    let l = expand(k)?;
    let mut g = [0i16; P];

    for i in 0..P {
        let value = u32_mod_u14(l[i], Q as u16);

        g[i] = value as i16 - Q12 as i16;
    }

    Ok(g)
}

// static void ZKeyGen(unsigned char *pk, unsigned char *sk) {
//   Fq A[p];
//   small a[p];
//
//   XKeyGen(pk, A, a);
//   pk += Seeds_bytes;
//   Rounded_encode(pk, A);
//   Small_encode(sk, a);
// }
//
// static void XKeyGen(unsigned char *S, Fq *A, small *a) {
//   Fq G[p];
//
//   Seeds_random(S);
//   Generator(G, S);
//   KeyGen(A, a, G);
// }
//
// static void Generator(Fq *G, const unsigned char *k) {
//   uint32 L[p];
//   int i;
//
//   Expand(L, k);
//   for (i = 0; i < p; ++i)
//     G[i] = uint32_mod_uint14(L[i], q) - q12;
// }

#[cfg(feature = "ntrulpr761")]
#[test]
fn test_expand() {
    let key: [u8; SEEDS_BYTES] = [
        151, 153, 142, 125, 236, 255, 169, 87, 52, 34, 151, 78, 0, 78, 108, 210, 125, 3, 77, 69,
        58, 198, 92, 127, 159, 29, 250, 66, 226, 127, 151, 93,
    ];
    let out = expand(&key).unwrap();

    assert_eq!(
        out,
        [
            466886879, 3805536058, 418619842, 579911611, 55227098, 251810133, 3880337202, 23230866,
            2069518008, 572406649, 3923686849, 4156412155, 3326446113, 3656173862, 1700019479,
            3650756319, 943794036, 3763192377, 883121252, 1119550060, 2768283731, 3960652040,
            4175009575, 3567815233, 1939544378, 3825902593, 1221699488, 3545163055, 1299573848,
            2439323875, 3539875924, 1703639252, 3110587955, 4002497537, 3451828540, 2300838408,
            2773791834, 1422688412, 2999887733, 2936224976, 4290964913, 4241238115, 1691844228,
            3747696356, 1215650914, 315071458, 3085451559, 1990129377, 2850768024, 2640435072,
            4024929728, 3546593652, 1414554654, 792579194, 3866516183, 3493336219, 1672807091,
            1923738882, 384616086, 898922910, 2433615999, 2938821512, 3548222867, 3908794191,
            1438071027, 3165626442, 3060964463, 2610243496, 3946858902, 1575612737, 83283233,
            424084922, 2901425927, 2057157842, 1314376139, 1795712397, 2804032562, 1448567575,
            130228779, 2591396973, 3090851707, 246691342, 1086566522, 197562439, 3280131291,
            485479622, 475566621, 719484734, 589020573, 4057318297, 9283402, 279072121, 176327087,
            2021189063, 4221137902, 4049862874, 1622484427, 1504439069, 3207051896, 545937194,
            569550167, 2925506614, 785942155, 441261648, 782940657, 3841159120, 1453633844,
            3504529100, 244336236, 3293331789, 2695193762, 821143605, 544111856, 1695728203,
            1263539489, 3868420458, 3614981285, 233393244, 1461100243, 2731472066, 544661308,
            3722766356, 1706299164, 2571111243, 2343791348, 3594541963, 302685807, 1270551849,
            3222458371, 3007351958, 3523579909, 3468005917, 1255623660, 2409522402, 491569706,
            1559414985, 914403202, 3967451290, 1532066218, 1636151661, 1772549698, 2060093251,
            1803309857, 580037805, 276575985, 3058413438, 1874793755, 4201921294, 1132799319,
            1171470274, 783904935, 2860558944, 3767752697, 1270921225, 496774458, 1115820692,
            2074657077, 1652235203, 3689654324, 758871595, 1006590741, 3053088007, 425768528,
            645938025, 2291710050, 643978858, 500909233, 1828931763, 4083818219, 2074333866,
            3716867552, 3035148826, 3402275135, 1913546067, 2795743317, 2935228677, 2233905986,
            42199901, 1357765419, 543964922, 1340129639, 3760421959, 1926512041, 295855417,
            1655485353, 529017114, 1880942431, 2612936111, 377914471, 4215787066, 3537132414,
            855463524, 3805276695, 3221515969, 3174993371, 2996697056, 2619425672, 1474451657,
            1003134736, 2927552113, 2755902894, 1901110224, 377776981, 3251459772, 3044489619,
            1830948724, 3924146195, 2470567302, 2940016553, 840374040, 3909920513, 1962485402,
            2559572655, 2184607735, 1312685919, 3303277891, 924625972, 144095020, 3252091362,
            1084917915, 954106350, 436531801, 1061159866, 2627443756, 2973045282, 2303583128,
            2867046378, 2917975308, 2846170574, 992166104, 565046532, 3783974182, 839933275,
            2055591815, 3652359896, 2399924434, 2044670074, 1579438049, 2814643281, 1758954702,
            3202089567, 3800920700, 3956738146, 1052362497, 3630634012, 3293330287, 1768287634,
            2413853514, 1585794750, 832161361, 1061816355, 2040289773, 2130732370, 965671566,
            1258184636, 3530078300, 3601196759, 3623040640, 1009784757, 2500279677, 248338828,
            2163820374, 2940134928, 914972892, 4254318215, 2860342304, 913988925, 3846614145,
            444757205, 953597647, 2429191203, 2244995502, 763060880, 2071452502, 2524094937,
            2634019274, 2642359503, 2235461787, 3737934417, 3365926754, 2280728930, 1476700354,
            810355849, 1410822208, 1379732417, 688306192, 2890129768, 3690136828, 2396374896,
            4210003162, 2695157022, 3006102800, 1464083627, 292131015, 1719364161, 2556473846,
            52750103, 96964250, 1851358863, 3499763808, 3643801370, 3899242804, 2070991210,
            4280849076, 1118480464, 2233475271, 3753653399, 988427917, 929388359, 610741165,
            4148212126, 3482189782, 2062208381, 2581311242, 4258417376, 2240876564, 2522799724,
            2344962361, 120596746, 138968622, 1770811560, 2279493028, 3494440215, 7964682,
            1326280443, 543112575, 1110073595, 1099713366, 3468982676, 3427716623, 579178048,
            4110986686, 1135687741, 306276538, 4046068988, 2276541441, 220349873, 936737166,
            2623689409, 2680152398, 712060359, 668572205, 376766700, 1513944390, 792497333,
            3419281980, 1600395449, 2959453536, 442616071, 3920499186, 1060413080, 28342744,
            3937816682, 975917075, 2613754579, 2848335958, 1686497149, 139341040, 3594120228,
            4098247577, 2262408096, 2102196741, 180570428, 3273589661, 3724777079, 3143922349,
            2842728644, 2946934316, 2490519046, 2800952911, 2946399152, 4248487363, 2176154892,
            592404890, 269236007, 1409778913, 1548583734, 3370600910, 900800736, 1646395334,
            1076386908, 640197714, 3435629221, 2151443550, 100179827, 484050310, 2849953782,
            4045122693, 3184587086, 3382929000, 1850663466, 1818004455, 808126065, 478586799,
            2542670087, 2530252666, 1241055426, 2830826254, 2248218355, 1964523723, 834227478,
            521023839, 2278534726, 1249465975, 3012442826, 20127816, 624035826, 3008774113,
            4230543143, 570020864, 1093069396, 243044026, 2579901971, 1369780218, 3257891680,
            64432701, 2973202190, 885697139, 4162025585, 3806665965, 3271130680, 678382125,
            818901291, 3507582256, 94117230, 1583325906, 618354970, 3078785439, 597488871,
            2484475943, 2889872621, 686345735, 396197795, 649991542, 3192401291, 1251467154,
            3326595080, 4113941342, 2723316435, 3045913940, 2918710060, 3263317585, 677605406,
            2795325990, 2542638515, 4192301817, 1599826367, 2270008852, 4086190186, 2644828887,
            607165570, 1476654081, 207511556, 4081460941, 2551866999, 102453865, 775758395,
            1340904282, 2058259725, 3322184154, 2685018480, 928444823, 911054450, 333798479,
            2249195615, 1830260626, 3342379109, 310940851, 389095703, 2980553213, 3539128704,
            1030507067, 1685035294, 391728187, 3920808696, 413609542, 4031471111, 1356137422,
            120910936, 66789510, 2419453664, 848489333, 3712516249, 959981572, 3821618568,
            2930440773, 3503731188, 3416921278, 3948366823, 1614291518, 166114612, 1134404065,
            2205611094, 31312060, 298754501, 427078075, 2970627058, 3794449874, 4110602300,
            3560042991, 1786837567, 330847564, 680644225, 240336138, 941370909, 4175370387,
            4101454691, 2664795991, 1507711889, 3861718764, 1778204703, 1984955318, 4073825873,
            1684826630, 564110149, 2120878235, 3603003437, 1797200620, 2517716030, 362248765,
            3503344422, 2038479759, 2440269317, 3271873262, 2034583765, 139114769, 602390152,
            1494857886, 1194118842, 2606899258, 4114698795, 4167865276, 970922986, 3960036339,
            3461340537, 2856316087, 887633502, 3208770894, 3037677537, 2543508652, 1386202813,
            1840196985, 2149382532, 2440613000, 4167596829, 807878118, 4215863779, 4133387272,
            2626044893, 1544300710, 2293830547, 685901456, 211530050, 2178531408, 3872147157,
            473090830, 3537329454, 1632338837, 4252514121, 3512600217, 748203473, 1834030416,
            2132182063, 1562482210, 3877213703, 4264554152, 1612968974, 3884361102, 1304060471,
            709488464, 3514281017, 1860283956, 1856615667, 3856072442, 3582232258, 1912591153,
            1354666590, 1917168237, 88700624, 1207559709, 3237241690, 3807616795, 3947000239,
            3456211015, 2825748677, 303862481, 4224380819, 1777763925, 2921433327, 3791286854,
            2380381619, 2683922820, 2513476498, 294015664, 1590817519, 1109350484, 2467314056,
            199120301, 434587717, 2580432675, 76533004, 1011452290, 2345789451, 3206964806,
            1622505920, 4276476216, 480362007, 2767545350, 3459353851, 1962018712, 3904862121,
            2206790123, 1788140308, 1524910658, 1347465126, 4237996157, 1428799973, 361135310,
            772034210, 233998960, 3059218072, 1171681722, 535867001, 3875698085, 3810346129,
            24166927, 303286233, 2143763681, 3135157764, 4222626988, 2228348421, 1815019058,
            975388695, 2490908501, 2672163133, 1052083715, 2636472320, 841588580, 1069865960,
            1014346346, 121748005, 2804380254, 2279944523, 1993933882, 4032038161, 3734211858,
            533520499, 1376560072, 2034040301, 2712540244, 3261098670, 1262238177, 1535149993,
            926854940, 3554366049, 4127939399, 2913006622, 1284797325, 3043393405, 350195090,
            2285560990, 2539421571, 1707087462, 3986108688, 208309423, 4017966359, 1255345995,
            724196699, 612258874, 1767447034, 1203528801, 3838678256, 1958012094, 2140009258,
            442368125, 1350473455, 1535564504, 37351404, 668459583, 3134614302, 46875539,
            265909078, 1817788097, 2914742116, 3943637645, 1634666165, 139099245, 517202117,
            1567235181, 2671372876, 752658974, 3547771036, 2173427320, 3754648522, 3053180226,
            467687114, 310361313, 2542984831, 2881483368, 291476801, 4136296010, 1202330773,
            1305869811, 871164914, 2336748407, 3814363380, 3604704301, 2080868039, 1901252122,
            3970434344, 2312700935, 988295512, 3432145780, 1881105817, 2788446677, 124055936,
            3229833788, 279756044, 3142336849, 946058322, 4066041062, 3698567164, 315293119,
            2373509228, 2756818535, 1417198777, 970644890, 1926623972, 1634122561, 3027787327,
            3882228045, 4053460479, 92022443, 1788359520, 679295031, 272825427, 584199484,
            1555087955, 691402373, 784076897, 3473828475, 1448415803, 4098616614, 41084115,
            1861336180, 4179502698, 3817482234, 890910179, 2546366041, 2771906763, 3609714066,
            617574853, 1867647508, 741914319, 3813370619, 4118650663, 3647364684, 4043614021,
            1728905357, 3779180866, 3726904908,
        ]
    );
}
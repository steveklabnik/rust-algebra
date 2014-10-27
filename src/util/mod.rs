#![cfg(test)]

// local crates
extern crate semigroup;

// external imports
use std::num;
use std::rand::{
    SeedableRng,
    StdRng,
};

// local imports
use semigroup::{
    Semigroup,
};

const SEED_BENCHES: [uint, ..256] = [9148059157905455617, 10725543981611425273, 12994069146683094067, 5776043534355732689, 11392463831482025972, 12336849770500457800, 7037662043743085263, 833908646579283805, 12573440333398132938, 13182463127749284232, 11583719268506847629, 13133629327460420694, 6250515940094000721, 12111525503859328386, 5198993598043463244, 419962882995434500, 279877011613549533, 18368292993051950427, 932261554946075086, 5687664411084759556, 16731932866752721728, 16683573073931728358, 2516943696453133828, 2144239263614322157, 9374531008807836475, 14992738956237865930, 6134202656452533271, 16739214489880234116, 17048153085071188711, 1959115663349859917, 3226749211885997014, 2305184412625701204, 6024004081259882430, 7794270732654221401, 7083967700007484491, 1461051872447164914, 12572998894805849553, 15406869282920024626, 13296436045666307925, 16734108470553426562, 7166841803455060821, 13324474766546613079, 4580685862960082126, 12648750685071527169, 12746797037325867625, 9103018660971262795, 1094341440304935913, 7118331286341158072, 13524439391154663442, 17739428692876205620, 11606275083547679661, 17047790325727991541, 11243546768169972921, 12415712744063754805, 5929721271583879608, 17131090304870093096, 1826369141890088100, 10807835466737568957, 17052491242427727505, 8744618762644171491, 4268782808275854187, 4093203929817298687, 5703219369729887976, 276720859499741651, 10520197993108890442, 12489155139928354541, 16187333613989108541, 1650747563372842885, 10874847714594902481, 3735708711546341198, 13619700827865119768, 2982381037769505233, 14172561437291531322, 5687885064562342780, 12529543670376714142, 11984649053001402462, 12150162308293955482, 18162668372065908245, 18048973266517797658, 16160176460709766294, 11293919228456761357, 3231526885343105118, 7057948102043197541, 8334683381231720628, 1306307178313104277, 11268985196141727342, 18203251861576101989, 1741399357467469019, 15824248265086038128, 1456342306469694183, 13554982041588507462, 8889498245841343981, 13361823037735987959, 15277135258724517883, 17917485230770665636, 779328640414185551, 17455002077014706083, 15906007917377414973, 9829739497336156943, 4880995549652116953, 5186380455936455458, 8462632850119129405, 8360843911679319682, 10156961621765312673, 16593052459813327735, 18251741133234626681, 10076511309185129670, 9012693211285248012, 5218197772175722824, 14186159823151803817, 13617425390727232043, 10864025009370591812, 10666791845811859186, 10728776518546677058, 17342175482425836576, 15494813205810496697, 2062586495133207546, 6224893422869792478, 706651024084317665, 15013807789520861629, 11645406454532490747, 17505252095011576198, 2170047252750189350, 10351033504322514099, 370951774784349726, 12522159394848465355, 4361528464840481134, 363858166827194393, 1666883221494225744, 5631648522366971833, 2811149876075065319, 2802843801906169712, 11129850884845736869, 12791020411819541754, 15206611890418188285, 17568602177873219504, 15286712008759969564, 5086601799136505118, 437875535527505051, 890336681007534847, 13640579654125949710, 17353277581706049632, 15440944439058482107, 8019866680793912339, 5576994154232470355, 201127468048188513, 3134622665209919811, 3080302845997706037, 5136503897657133658, 8892958809998305016, 15657016918411236208, 13675741803755239873, 7838653387929415035, 5211500603028466273, 7050823797584494907, 8982099648137605728, 9783255995794738608, 15978639735303823971, 12848757877355080911, 14532266986647298992, 5510225331709503821, 6034524295142163097, 6276234799431752509, 14274064211436437941, 9589427691735429308, 9433071987916206327, 14943141208652650613, 17009960659650319140, 13602755064040084541, 13731374986446873919, 5020251246763314244, 6991250510534226423, 11593277643980676428, 16672104516923071229, 15217608179049903781, 12710051014061475612, 9399882320429587931, 9429669261053949206, 4534908546024145284, 5795323112627296986, 9110813366803613293, 15029322311573389371, 13254879757450910963, 7699842814314880129, 9306378276159844237, 1799409470411625924, 16326958277721816417, 288667178595135827, 13949682083512986305, 13002828835817266469, 3880553549463723871, 2743524932339889498, 16388579613845677946, 8173967391412602362, 10178707904602260576, 15157411645580465958, 17706983960831010921, 8184233084330368282, 4108894146644386025, 4591039456581298507, 13278345216655371268, 14491425197357674230, 12093416483470938907, 17862346794723171688, 2029464226277298393, 6473580603312094138, 3833015894132178723, 16978549599557869553, 5516940215214084760, 11330830035699492123, 15025894547124347155, 10703801187751402398, 12455548799501845262, 13871813359546559886, 15527684358601034344, 3052611880150902923, 16029170313189852802, 3177917987477547118, 3116585995573740689, 7592700843576778898, 7337283015687595823, 17346932366518168956, 2255052425630873241, 6127733376797907695, 4566541386821918723, 14038498388856259627, 11650239057600029922, 8835810362020297739, 2940733064265192632, 6946543493272368849, 4015048133230458500, 10388339960321465273, 11718148130512758841, 1635489472115945603, 15827990529223825497, 3564577611999816380, 12346252738084807844, 6846949170665629737, 15521347442862805545, 5707551348793064751, 516961692149958781, 14587970763765844106, 85819459614620708, 7265258877208378720, 1101857990170607363, 18133440744164739752, 16169476335573170196, 6505569603951002727, 13960891824487580323, 2701238007857542583, 10845129642466404314, 17885165300556661684, 6707353915006147004, 8345637133095979995, 17426585060678930714, 4632567845104863997];

#[allow(dead_code)]
#[inline]
pub fn seeded_rng() -> StdRng {
    SeedableRng::from_seed(SEED_BENCHES.as_slice())
}

#[allow(dead_code)]
#[inline]
pub fn pownz_builtin<A>(base:A, exp:uint) -> A
    where
        A:num::One,
{
    num::pow(base, exp + 1)
}

#[inline]
pub fn pownz_naive<A>(base:A, mut exp:uint) -> A
    where
        A:Clone,
        A:Semigroup,
{
    let mut acc = base.clone();
    exp = exp + 1;
    while exp > 1 {
        acc = acc.op(&base);
        exp = exp - 1
    }
    acc
}

#[inline]
pub fn product_naive<A,F>(it:&mut F, mut acc:A) -> A
    where
        A:Semigroup,
        F:Iterator<A>,
{
    for x in it { acc = acc.op(&x) }
    acc
}

use super::curve::AffinePoint;
use plonky2_field::{extension::quintic::QuinticExtension, goldilocks_field::GoldilocksField};

pub(crate) const MUL_TABLE_G0: [AffinePoint; 16] = [
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(12883135586176881569),
            GoldilocksField(4356519642755055268),
            GoldilocksField(5248930565894896907),
            GoldilocksField(2165973894480315022),
            GoldilocksField(2448410071095648785),
        ]),
        u: QuinticExtension([
            GoldilocksField(13835058052060938241),
            GoldilocksField(0),
            GoldilocksField(0),
            GoldilocksField(0),
            GoldilocksField(0),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16517537419581740386),
            GoldilocksField(6962630169123120981),
            GoldilocksField(12147752690379666704),
            GoldilocksField(16637325971742264607),
            GoldilocksField(2335078582315237010),
        ]),
        u: QuinticExtension([
            GoldilocksField(8457587110646932172),
            GoldilocksField(138591869800252458),
            GoldilocksField(3187444967472352324),
            GoldilocksField(18179149801168653736),
            GoldilocksField(9453003655195557048),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(4546139357324501584),
            GoldilocksField(1393728687664685160),
            GoldilocksField(15208040286522119521),
            GoldilocksField(7903224051455420834),
            GoldilocksField(12463930627278381774),
        ]),
        u: QuinticExtension([
            GoldilocksField(16373828487211693378),
            GoldilocksField(5899455736915524900),
            GoldilocksField(17616512450102495476),
            GoldilocksField(17643201028570366669),
            GoldilocksField(2833280130550676525),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(4341836049185169731),
            GoldilocksField(9111482874850194930),
            GoldilocksField(7798994609726992878),
            GoldilocksField(12619124383509403661),
            GoldilocksField(13047834166950680886),
        ]),
        u: QuinticExtension([
            GoldilocksField(3584786391427904733),
            GoldilocksField(1717626083626375072),
            GoldilocksField(16549008311909030594),
            GoldilocksField(17550175197111849143),
            GoldilocksField(18374971670674568416),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(18121072711119258927),
            GoldilocksField(3394315639035318724),
            GoldilocksField(2648370499809919556),
            GoldilocksField(13348924736921714137),
            GoldilocksField(3428166646246873447),
        ]),
        u: QuinticExtension([
            GoldilocksField(9264305576790077869),
            GoldilocksField(7426254234280836405),
            GoldilocksField(5107777768036114824),
            GoldilocksField(9390769538758625122),
            GoldilocksField(9788182195111344062),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(11080635543643017332),
            GoldilocksField(3122290570793204485),
            GoldilocksField(16632474826839786439),
            GoldilocksField(14883711538614796285),
            GoldilocksField(10396852362099782295),
        ]),
        u: QuinticExtension([
            GoldilocksField(14253916706639980511),
            GoldilocksField(15728038457561632290),
            GoldilocksField(3947138785484546318),
            GoldilocksField(4740958322851071718),
            GoldilocksField(17384736114265519442),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(4763058716218401568),
            GoldilocksField(17879823368956058516),
            GoldilocksField(13578954599286698938),
            GoldilocksField(8634670560943921567),
            GoldilocksField(13706660844700767685),
        ]),
        u: QuinticExtension([
            GoldilocksField(3354778288360932917),
            GoldilocksField(13842278303693121409),
            GoldilocksField(4717821645259836467),
            GoldilocksField(7978743897613094276),
            GoldilocksField(10118963888992569394),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(4026958896735257282),
            GoldilocksField(13595990041314210204),
            GoldilocksField(11499471878438064392),
            GoldilocksField(10019455879458851233),
            GoldilocksField(11986847968355927330),
        ]),
        u: QuinticExtension([
            GoldilocksField(14532821659997761913),
            GoldilocksField(9582789969382797985),
            GoldilocksField(3082219099923033594),
            GoldilocksField(2859656980617778370),
            GoldilocksField(3746047816071136016),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(15935900828168308224),
            GoldilocksField(8668680449802005535),
            GoldilocksField(491315506768012688),
            GoldilocksField(6584881037682113026),
            GoldilocksField(12386385009372860460),
        ]),
        u: QuinticExtension([
            GoldilocksField(13217832923050551864),
            GoldilocksField(51671271962049328),
            GoldilocksField(15400792709153778477),
            GoldilocksField(6752203529649104660),
            GoldilocksField(2855313280735340066),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(8473506523195244465),
            GoldilocksField(2446964921175324878),
            GoldilocksField(17962771942831363202),
            GoldilocksField(6949608686158330138),
            GoldilocksField(9315492999547366751),
        ]),
        u: QuinticExtension([
            GoldilocksField(5171814696081600409),
            GoldilocksField(3025466154945175207),
            GoldilocksField(453302446979841822),
            GoldilocksField(14135305892339872079),
            GoldilocksField(2556388051049291052),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(3960231187580500028),
            GoldilocksField(3695840168764199059),
            GoldilocksField(2914577777792670911),
            GoldilocksField(9249939676680902688),
            GoldilocksField(17553522813502241416),
        ]),
        u: QuinticExtension([
            GoldilocksField(3015152305907361949),
            GoldilocksField(10730034543155667220),
            GoldilocksField(3314242046485170944),
            GoldilocksField(1984395553885795852),
            GoldilocksField(13781645774758249860),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(11575997426281090678),
            GoldilocksField(1534495174840625570),
            GoldilocksField(7539338128385981583),
            GoldilocksField(10393042019577161985),
            GoldilocksField(10667466219175771157),
        ]),
        u: QuinticExtension([
            GoldilocksField(16681365912970185037),
            GoldilocksField(11287896019745355117),
            GoldilocksField(11069899752345274504),
            GoldilocksField(15487604769605237513),
            GoldilocksField(13467978440572613228),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(11192179397773394280),
            GoldilocksField(3555953455665397909),
            GoldilocksField(5346523552109387121),
            GoldilocksField(4514445299325204396),
            GoldilocksField(3932728981135688453),
        ]),
        u: QuinticExtension([
            GoldilocksField(5421638117266109845),
            GoldilocksField(204299445119713184),
            GoldilocksField(6067390115784997081),
            GoldilocksField(16191134954342419157),
            GoldilocksField(4139938600224417293),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(13189785832536261642),
            GoldilocksField(8777097377506996162),
            GoldilocksField(17497140949916325738),
            GoldilocksField(15140279769427597032),
            GoldilocksField(15517274717131999881),
        ]),
        u: QuinticExtension([
            GoldilocksField(1040464435413162742),
            GoldilocksField(9262701069034606854),
            GoldilocksField(2990438819650713743),
            GoldilocksField(18129195737333990255),
            GoldilocksField(12490074042478236606),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(17716508479149156535),
            GoldilocksField(14351380558651795729),
            GoldilocksField(3644546258883003807),
            GoldilocksField(5171318241596472386),
            GoldilocksField(294806796132518330),
        ]),
        u: QuinticExtension([
            GoldilocksField(7535225611936271281),
            GoldilocksField(14682077054502188499),
            GoldilocksField(784215514926156349),
            GoldilocksField(5280586574139275596),
            GoldilocksField(14407528916988559545),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(8681294642569802563),
            GoldilocksField(7751765660802747503),
            GoldilocksField(16382129702876313971),
            GoldilocksField(7447155060842833278),
            GoldilocksField(6859908403876474879),
        ]),
        u: QuinticExtension([
            GoldilocksField(9674486254207846385),
            GoldilocksField(5248970165164951259),
            GoldilocksField(3611784478790504991),
            GoldilocksField(18437168019170350173),
            GoldilocksField(3537959913875671086),
        ]),
    },
];
pub(crate) const MUL_TABLE_G40: [AffinePoint; 16] = [
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6996996617034310847),
            GoldilocksField(1312534891996392328),
            GoldilocksField(1967056454231743182),
            GoldilocksField(12432745115107639465),
            GoldilocksField(8188918658769983203),
        ]),
        u: QuinticExtension([
            GoldilocksField(9779151955752388390),
            GoldilocksField(12827693252247248589),
            GoldilocksField(8299002358494291091),
            GoldilocksField(10057624387258292793),
            GoldilocksField(9561932552523598817),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(12727761422252171591),
            GoldilocksField(11715233354058649362),
            GoldilocksField(11258110171296383015),
            GoldilocksField(4946612044061620143),
            GoldilocksField(10674140266605092092),
        ]),
        u: QuinticExtension([
            GoldilocksField(13968556698015688219),
            GoldilocksField(9764817221409883159),
            GoldilocksField(6009815048702102249),
            GoldilocksField(928542484379469501),
            GoldilocksField(17548136021451934003),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(1449347403861973234),
            GoldilocksField(1268700206465777189),
            GoldilocksField(68931803832001930),
            GoldilocksField(508124187869777281),
            GoldilocksField(14966299269768645002),
        ]),
        u: QuinticExtension([
            GoldilocksField(12519156548432608657),
            GoldilocksField(1830718924858545317),
            GoldilocksField(8290101973558828816),
            GoldilocksField(6963396969528752135),
            GoldilocksField(5027294278125306748),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(7543060992125635458),
            GoldilocksField(13154269169488238929),
            GoldilocksField(13038629689209617296),
            GoldilocksField(17607665244721587028),
            GoldilocksField(4076634695833139117),
        ]),
        u: QuinticExtension([
            GoldilocksField(16474278336963843968),
            GoldilocksField(7342735040871703005),
            GoldilocksField(11822823161099820577),
            GoldilocksField(15838689010341349421),
            GoldilocksField(8387592947884092077),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6956288471673670775),
            GoldilocksField(10363521468804730232),
            GoldilocksField(1618310362752793530),
            GoldilocksField(16886810269476841179),
            GoldilocksField(4982980062158920723),
        ]),
        u: QuinticExtension([
            GoldilocksField(13688045661223437644),
            GoldilocksField(17947601766473933193),
            GoldilocksField(7138906029562123225),
            GoldilocksField(14564553876341839060),
            GoldilocksField(4126496432434298977),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(800685292854622487),
            GoldilocksField(11040079590365906652),
            GoldilocksField(1466305609865524328),
            GoldilocksField(8372552820474238249),
            GoldilocksField(10874913568038030998),
        ]),
        u: QuinticExtension([
            GoldilocksField(4703724548613471267),
            GoldilocksField(16058989380922585526),
            GoldilocksField(8365972383552432650),
            GoldilocksField(12321780682158893877),
            GoldilocksField(2418487585371688136),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(12518522439291713316),
            GoldilocksField(18265859802652833974),
            GoldilocksField(4355197864757715454),
            GoldilocksField(16333106890933317228),
            GoldilocksField(7860825917869078801),
        ]),
        u: QuinticExtension([
            GoldilocksField(12169474924601364130),
            GoldilocksField(1427729574788767322),
            GoldilocksField(3451823787886833090),
            GoldilocksField(4595725973834664846),
            GoldilocksField(5636506224235047729),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(7016248550036618856),
            GoldilocksField(14664150699534918598),
            GoldilocksField(7289196844394571239),
            GoldilocksField(3733481542224777638),
            GoldilocksField(12940819275544993154),
        ]),
        u: QuinticExtension([
            GoldilocksField(5962170105887193190),
            GoldilocksField(7757792046810148121),
            GoldilocksField(17754145760690637154),
            GoldilocksField(5608151523576337415),
            GoldilocksField(10158975094989974837),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9657667902075638078),
            GoldilocksField(1738398137082324930),
            GoldilocksField(3309896085425426006),
            GoldilocksField(5244596195331513559),
            GoldilocksField(11098614916240915598),
        ]),
        u: QuinticExtension([
            GoldilocksField(10176686769986870501),
            GoldilocksField(17149616066773579692),
            GoldilocksField(16557806655360885458),
            GoldilocksField(6409371822017281510),
            GoldilocksField(447032970886916415),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(2000298634105946483),
            GoldilocksField(7990437998469847294),
            GoldilocksField(13891384442822604159),
            GoldilocksField(3400617664053350732),
            GoldilocksField(17650120710895099722),
        ]),
        u: QuinticExtension([
            GoldilocksField(9011065287270146338),
            GoldilocksField(9712006535696787670),
            GoldilocksField(5197636265344816024),
            GoldilocksField(14644619822912127741),
            GoldilocksField(5091497898426581809),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(4353689210628214181),
            GoldilocksField(11629282537514442736),
            GoldilocksField(519301038092536110),
            GoldilocksField(17451856528277649540),
            GoldilocksField(8053963837814854762),
        ]),
        u: QuinticExtension([
            GoldilocksField(16247175863676166340),
            GoldilocksField(13321024650071188595),
            GoldilocksField(13226465566647040787),
            GoldilocksField(15830701216342305199),
            GoldilocksField(10171768200911815007),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(8618084654605828418),
            GoldilocksField(2932134432657893516),
            GoldilocksField(14040921219429416616),
            GoldilocksField(8539270659386774297),
            GoldilocksField(8223174716536738537),
        ]),
        u: QuinticExtension([
            GoldilocksField(2118173438466787625),
            GoldilocksField(17017456632539625481),
            GoldilocksField(3822614388660837302),
            GoldilocksField(18012676134277779138),
            GoldilocksField(14555233257002087745),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(14104364668871773315),
            GoldilocksField(4671583541374529339),
            GoldilocksField(14595315310536253921),
            GoldilocksField(12293043219805252275),
            GoldilocksField(11083273927620890457),
        ]),
        u: QuinticExtension([
            GoldilocksField(13013197605833180311),
            GoldilocksField(6369553806055216484),
            GoldilocksField(13715364943719691230),
            GoldilocksField(832870131890809214),
            GoldilocksField(2834204446065110889),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(17610707880266706457),
            GoldilocksField(8946198449628536102),
            GoldilocksField(14056449117236625467),
            GoldilocksField(6751468363564694789),
            GoldilocksField(10581122285882655867),
        ]),
        u: QuinticExtension([
            GoldilocksField(16822879694511882841),
            GoldilocksField(7030889609682609080),
            GoldilocksField(1819733726510865699),
            GoldilocksField(1477354361991598818),
            GoldilocksField(3060932650955723086),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16383078186728412911),
            GoldilocksField(17336793750234608284),
            GoldilocksField(10282704501742138249),
            GoldilocksField(8902952211247569575),
            GoldilocksField(10036728575538225007),
        ]),
        u: QuinticExtension([
            GoldilocksField(980771758638014650),
            GoldilocksField(8822864673362619613),
            GoldilocksField(1247272673889574430),
            GoldilocksField(8049338215992656959),
            GoldilocksField(5754772454101411592),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(4793164719180728081),
            GoldilocksField(10337661541467018847),
            GoldilocksField(4370608981419008671),
            GoldilocksField(8309057178611279515),
            GoldilocksField(11967697131554357119),
        ]),
        u: QuinticExtension([
            GoldilocksField(17586180332786867000),
            GoldilocksField(10992062529780955862),
            GoldilocksField(4283639578773926288),
            GoldilocksField(10598406479331979533),
            GoldilocksField(13292632801372322468),
        ]),
    },
];
pub(crate) const MUL_TABLE_G80: [AffinePoint; 16] = [
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(13832685079504880268),
            GoldilocksField(18013036221761440296),
            GoldilocksField(1301626881083565265),
            GoldilocksField(9139126253053898429),
            GoldilocksField(4505395467569954655),
        ]),
        u: QuinticExtension([
            GoldilocksField(7359813255592029850),
            GoldilocksField(16688014242518042008),
            GoldilocksField(4399996806448279465),
            GoldilocksField(5271684552135959425),
            GoldilocksField(11652444551874101645),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(10957597387983347508),
            GoldilocksField(15279113224094632648),
            GoldilocksField(16636004563247846338),
            GoldilocksField(139361871129849794),
            GoldilocksField(14913244377905888101),
        ]),
        u: QuinticExtension([
            GoldilocksField(7004241227096627206),
            GoldilocksField(639096603853214644),
            GoldilocksField(17343971022152731708),
            GoldilocksField(11127082727624914758),
            GoldilocksField(6961420809959752544),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(1678194015101167575),
            GoldilocksField(10443033913340861968),
            GoldilocksField(15723961754453665965),
            GoldilocksField(466551946746500778),
            GoldilocksField(1384638131140679955),
        ]),
        u: QuinticExtension([
            GoldilocksField(7911659739613756657),
            GoldilocksField(9008449922226900897),
            GoldilocksField(8828649835406020350),
            GoldilocksField(12804093940915848836),
            GoldilocksField(5168873490743917498),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9668801441369446334),
            GoldilocksField(1618542760290755427),
            GoldilocksField(15806911258918325259),
            GoldilocksField(14508945524557601221),
            GoldilocksField(8400708218360666510),
        ]),
        u: QuinticExtension([
            GoldilocksField(2070702333293922760),
            GoldilocksField(6249392735673775978),
            GoldilocksField(5221268220067076678),
            GoldilocksField(12830382095618421300),
            GoldilocksField(6798253292813277552),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(613541603487791685),
            GoldilocksField(13807376311954113152),
            GoldilocksField(4937154484322350324),
            GoldilocksField(3044864073363788260),
            GoldilocksField(10659806245468237672),
        ]),
        u: QuinticExtension([
            GoldilocksField(11268721606331277338),
            GoldilocksField(14114972563238185761),
            GoldilocksField(15134656524184558801),
            GoldilocksField(8109827563124888851),
            GoldilocksField(3238236749755375190),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(3653303296020985061),
            GoldilocksField(16968816356701165477),
            GoldilocksField(3537989784194419792),
            GoldilocksField(6048563117397703739),
            GoldilocksField(13275594789417281589),
        ]),
        u: QuinticExtension([
            GoldilocksField(15320572452406052803),
            GoldilocksField(423975947193335924),
            GoldilocksField(9786061404780445812),
            GoldilocksField(113935901661183202),
            GoldilocksField(17462508908451992614),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(5575647366699441601),
            GoldilocksField(2189227564735866743),
            GoldilocksField(1686208091749425593),
            GoldilocksField(6736750915939348632),
            GoldilocksField(17433930427527644213),
        ]),
        u: QuinticExtension([
            GoldilocksField(7057911563532867792),
            GoldilocksField(16566118262655927325),
            GoldilocksField(12888897205414551370),
            GoldilocksField(14415855073450397097),
            GoldilocksField(1147090766535755807),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(15658118616953616040),
            GoldilocksField(2263539327323250613),
            GoldilocksField(2715487874579250798),
            GoldilocksField(11933376952724039169),
            GoldilocksField(17769318666901826029),
        ]),
        u: QuinticExtension([
            GoldilocksField(16745623139313228390),
            GoldilocksField(9536464142142244411),
            GoldilocksField(12504946243788089281),
            GoldilocksField(704708129354743638),
            GoldilocksField(14573477780244357666),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(13701409545959547673),
            GoldilocksField(11537398060095127371),
            GoldilocksField(18304316093185449069),
            GoldilocksField(16990637176495122248),
            GoldilocksField(8300454239136955447),
        ]),
        u: QuinticExtension([
            GoldilocksField(12946536999123301864),
            GoldilocksField(16028271018248917226),
            GoldilocksField(14442669626987508876),
            GoldilocksField(8204605677104061293),
            GoldilocksField(13012677989830312429),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(8977822175893189913),
            GoldilocksField(8385291758088962932),
            GoldilocksField(6459781748990922334),
            GoldilocksField(10500670301259390474),
            GoldilocksField(8148745850566531944),
        ]),
        u: QuinticExtension([
            GoldilocksField(648416469448933683),
            GoldilocksField(2018140447090876597),
            GoldilocksField(11059355864713025945),
            GoldilocksField(17171402628974174968),
            GoldilocksField(720667133464111689),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(533755834279203303),
            GoldilocksField(9133223350344638107),
            GoldilocksField(6496913004501565984),
            GoldilocksField(5070553496917221248),
            GoldilocksField(10026395429516732342),
        ]),
        u: QuinticExtension([
            GoldilocksField(17311718290481148297),
            GoldilocksField(12616184711972987746),
            GoldilocksField(16195499951758316636),
            GoldilocksField(8118955923598298529),
            GoldilocksField(16774524951584936403),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(8977932331742105562),
            GoldilocksField(9135241432935976918),
            GoldilocksField(8762554005485625681),
            GoldilocksField(14767442741287060847),
            GoldilocksField(9223537459805575058),
        ]),
        u: QuinticExtension([
            GoldilocksField(15269989054854026299),
            GoldilocksField(11931086694777575213),
            GoldilocksField(1979657370606607924),
            GoldilocksField(10082554692183350114),
            GoldilocksField(4573690475951190900),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(12857842861752747447),
            GoldilocksField(11647180289644065286),
            GoldilocksField(17408779236478002670),
            GoldilocksField(5917000661832739376),
            GoldilocksField(1047056879360966448),
        ]),
        u: QuinticExtension([
            GoldilocksField(60118689797675542),
            GoldilocksField(1664328840457595492),
            GoldilocksField(7075936368160047305),
            GoldilocksField(13974115771952400562),
            GoldilocksField(11318108364890349009),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(17452797179483233405),
            GoldilocksField(6882955852043132316),
            GoldilocksField(9304840691925828603),
            GoldilocksField(981483665863638676),
            GoldilocksField(11024236439678964632),
        ]),
        u: QuinticExtension([
            GoldilocksField(2608844450889021414),
            GoldilocksField(2862891036050959369),
            GoldilocksField(9059816914007502053),
            GoldilocksField(16849128770451662626),
            GoldilocksField(54944805734402483),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(7304267395998600281),
            GoldilocksField(12651945702824162423),
            GoldilocksField(12034846251704181244),
            GoldilocksField(14535937891251268540),
            GoldilocksField(16446125823956689442),
        ]),
        u: QuinticExtension([
            GoldilocksField(14013745143822621484),
            GoldilocksField(13346293440957348839),
            GoldilocksField(14559163781616146382),
            GoldilocksField(10079303505894311335),
            GoldilocksField(13316971442260780794),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(2073268421803561746),
            GoldilocksField(13903239987062959402),
            GoldilocksField(6595765789009484358),
            GoldilocksField(12734389031510939331),
            GoldilocksField(14507055985845886345),
        ]),
        u: QuinticExtension([
            GoldilocksField(6178525556615612),
            GoldilocksField(5187104181066643307),
            GoldilocksField(2097004975629951488),
            GoldilocksField(3624702972881058018),
            GoldilocksField(15835733836057682299),
        ]),
    },
];
pub(crate) const MUL_TABLE_G120: [AffinePoint; 16] = [
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9358418073545563325),
            GoldilocksField(6201803925005767184),
            GoldilocksField(17525836657555505989),
            GoldilocksField(18172103331346227979),
            GoldilocksField(11525670089424228174),
        ]),
        u: QuinticExtension([
            GoldilocksField(15389027580004038174),
            GoldilocksField(17425413276694524614),
            GoldilocksField(15639145503384753087),
            GoldilocksField(15041017306226520945),
            GoldilocksField(7937401073912193639),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(424871884768762681),
            GoldilocksField(13522556051462729987),
            GoldilocksField(12578037128032095483),
            GoldilocksField(15478027026291985081),
            GoldilocksField(3107357372380600388),
        ]),
        u: QuinticExtension([
            GoldilocksField(139609698330600720),
            GoldilocksField(13047471464877067976),
            GoldilocksField(14569000597615364817),
            GoldilocksField(2241769726453036433),
            GoldilocksField(15809930333584099827),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(14490559385422698658),
            GoldilocksField(9192132350820542857),
            GoldilocksField(5174647998374408373),
            GoldilocksField(14517418341859680382),
            GoldilocksField(17127851909541764338),
        ]),
        u: QuinticExtension([
            GoldilocksField(10617869578552630251),
            GoldilocksField(15452062022333822112),
            GoldilocksField(74217513813449143),
            GoldilocksField(7065334431037916517),
            GoldilocksField(1908363005628198785),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(14158363767375738516),
            GoldilocksField(10881488396819614845),
            GoldilocksField(9845083246403658682),
            GoldilocksField(308084846693439896),
            GoldilocksField(2258456665285229766),
        ]),
        u: QuinticExtension([
            GoldilocksField(10189353602169967163),
            GoldilocksField(3307134994579671177),
            GoldilocksField(15193472587506759411),
            GoldilocksField(1522949334698619656),
            GoldilocksField(10335076055833410122),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6988575191781662507),
            GoldilocksField(6763011815163702392),
            GoldilocksField(12700583067108401780),
            GoldilocksField(10889091046959437472),
            GoldilocksField(14563326032896307580),
        ]),
        u: QuinticExtension([
            GoldilocksField(12122806272622858917),
            GoldilocksField(17957572904440664730),
            GoldilocksField(1371105162549165938),
            GoldilocksField(7050159476133204977),
            GoldilocksField(14174648605675469597),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(11134953890003368271),
            GoldilocksField(1950173651962543315),
            GoldilocksField(15717245132850143332),
            GoldilocksField(3404453732698149788),
            GoldilocksField(5301672891646287185),
        ]),
        u: QuinticExtension([
            GoldilocksField(3625450390591129442),
            GoldilocksField(7246221686985732698),
            GoldilocksField(883169685721066424),
            GoldilocksField(4890159692945065594),
            GoldilocksField(5846189492174531971),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(17779324141575511119),
            GoldilocksField(2222820233157145199),
            GoldilocksField(14311299573357024128),
            GoldilocksField(6091166172492559467),
            GoldilocksField(13251122054512244755),
        ]),
        u: QuinticExtension([
            GoldilocksField(13595785608342218333),
            GoldilocksField(5346420442473779380),
            GoldilocksField(15973815498598602014),
            GoldilocksField(17570023165337986853),
            GoldilocksField(4489084688781803549),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(18066103166710948306),
            GoldilocksField(17952420946495149357),
            GoldilocksField(6895726862811180419),
            GoldilocksField(5250742026743142449),
            GoldilocksField(5546233908977317256),
        ]),
        u: QuinticExtension([
            GoldilocksField(13627730136315133390),
            GoldilocksField(16318021942381891511),
            GoldilocksField(17522263726824223313),
            GoldilocksField(2960524358953784315),
            GoldilocksField(9229420628457238614),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(1295063301573260180),
            GoldilocksField(2809464405706641890),
            GoldilocksField(16876063007570590190),
            GoldilocksField(414980406456848047),
            GoldilocksField(8882993381636093379),
        ]),
        u: QuinticExtension([
            GoldilocksField(14084704505090840803),
            GoldilocksField(1455438701125484684),
            GoldilocksField(7140138141300391159),
            GoldilocksField(3304135812365795152),
            GoldilocksField(2617025679312300128),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6275460045614886806),
            GoldilocksField(3390801146218874506),
            GoldilocksField(17247998212939720068),
            GoldilocksField(14133145208463656732),
            GoldilocksField(3920522032578446900),
        ]),
        u: QuinticExtension([
            GoldilocksField(7628461038336051188),
            GoldilocksField(5939897916270777659),
            GoldilocksField(9105159200762125376),
            GoldilocksField(13546478897675664577),
            GoldilocksField(10279072558522952380),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(11630988947772602329),
            GoldilocksField(12620300429100070711),
            GoldilocksField(79628975116792272),
            GoldilocksField(17920472109136769182),
            GoldilocksField(5826732348459131885),
        ]),
        u: QuinticExtension([
            GoldilocksField(2736111763898189506),
            GoldilocksField(14407691554344511345),
            GoldilocksField(10405697919259369402),
            GoldilocksField(2951539272691560626),
            GoldilocksField(17028604616981679777),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9988698078321716091),
            GoldilocksField(429119229031363106),
            GoldilocksField(7711926677839955310),
            GoldilocksField(14843425123144395632),
            GoldilocksField(2253491748118774140),
        ]),
        u: QuinticExtension([
            GoldilocksField(17190043005790419516),
            GoldilocksField(13808981798094567902),
            GoldilocksField(4645442529701115361),
            GoldilocksField(10360499666917437943),
            GoldilocksField(13003321814463836887),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(5428331454841389691),
            GoldilocksField(5911352608778299689),
            GoldilocksField(12033745745356201095),
            GoldilocksField(14100994707656604830),
            GoldilocksField(2886042088926452362),
        ]),
        u: QuinticExtension([
            GoldilocksField(12925133128294153456),
            GoldilocksField(6458535650167456730),
            GoldilocksField(8582452901418814402),
            GoldilocksField(9403948375821725222),
            GoldilocksField(4166244923628463342),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(14049231651863941702),
            GoldilocksField(9994040187462027964),
            GoldilocksField(16602305579940231446),
            GoldilocksField(8805602289491330699),
            GoldilocksField(544940053745291275),
        ]),
        u: QuinticExtension([
            GoldilocksField(18184165264127619754),
            GoldilocksField(11557606822284913524),
            GoldilocksField(7784129138807937081),
            GoldilocksField(11583517824597488539),
            GoldilocksField(7002309200501552489),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(814125571699125593),
            GoldilocksField(113298508670324430),
            GoldilocksField(3553512439231149575),
            GoldilocksField(5722734149611317431),
            GoldilocksField(13535892466294020417),
        ]),
        u: QuinticExtension([
            GoldilocksField(10718151468633124775),
            GoldilocksField(1411760656056230045),
            GoldilocksField(2150017719245220876),
            GoldilocksField(14735221082549759933),
            GoldilocksField(13642901740019011009),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16483184730500681046),
            GoldilocksField(9673757784055259057),
            GoldilocksField(7760528659260895061),
            GoldilocksField(14112860811008950766),
            GoldilocksField(14165735631683045125),
        ]),
        u: QuinticExtension([
            GoldilocksField(16200754974233622593),
            GoldilocksField(15775772353572942080),
            GoldilocksField(8728522175126988968),
            GoldilocksField(14337787208807512369),
            GoldilocksField(6870309312996910338),
        ]),
    },
];
pub(crate) const MUL_TABLE_G160: [AffinePoint; 16] = [
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(4048997798593065056),
            GoldilocksField(8401406543098379712),
            GoldilocksField(8471972887547353150),
            GoldilocksField(11271856534362959532),
            GoldilocksField(11485893719004138771),
        ]),
        u: QuinticExtension([
            GoldilocksField(9981895593163975663),
            GoldilocksField(16506992680199754648),
            GoldilocksField(9795990766132909080),
            GoldilocksField(14537323266760073360),
            GoldilocksField(16786980505293186490),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16515152542557678971),
            GoldilocksField(2820879738576535933),
            GoldilocksField(14546871004256087775),
            GoldilocksField(8067774721434663075),
            GoldilocksField(5547758516300176370),
        ]),
        u: QuinticExtension([
            GoldilocksField(13156895577790221631),
            GoldilocksField(14079823781876329633),
            GoldilocksField(3663423310046916033),
            GoldilocksField(8256729270602146828),
            GoldilocksField(8025936200066564880),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(7199041597631769070),
            GoldilocksField(6507380560962078664),
            GoldilocksField(8741238648067440929),
            GoldilocksField(5032023372661133788),
            GoldilocksField(1471499738040488525),
        ]),
        u: QuinticExtension([
            GoldilocksField(16127942173059622373),
            GoldilocksField(17662578881118466367),
            GoldilocksField(5426223217353814653),
            GoldilocksField(12687076501536075723),
            GoldilocksField(11700332978843695966),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(15540762768637989229),
            GoldilocksField(5977836013802283595),
            GoldilocksField(17611433126093706192),
            GoldilocksField(1869784237181444322),
            GoldilocksField(262436190082189342),
        ]),
        u: QuinticExtension([
            GoldilocksField(16646868690306195484),
            GoldilocksField(2492778147148350975),
            GoldilocksField(12994887025011189709),
            GoldilocksField(18073347299788553346),
            GoldilocksField(16182392324261935778),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(14451630909414751763),
            GoldilocksField(2682316750096868275),
            GoldilocksField(15784810705353479256),
            GoldilocksField(9913396490753039039),
            GoldilocksField(17084522528101355432),
        ]),
        u: QuinticExtension([
            GoldilocksField(13015512373883463322),
            GoldilocksField(18140315257280584894),
            GoldilocksField(5133502996496697434),
            GoldilocksField(581463011694460141),
            GoldilocksField(10720904114857970130),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(1736453911989778095),
            GoldilocksField(2522761214792928606),
            GoldilocksField(8490724482968195082),
            GoldilocksField(3061517266849590914),
            GoldilocksField(7560708607478466898),
        ]),
        u: QuinticExtension([
            GoldilocksField(2559867271025702686),
            GoldilocksField(8279186716530148418),
            GoldilocksField(9394033500068495079),
            GoldilocksField(15391096564340037389),
            GoldilocksField(15441682874751040991),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(10406668092884681305),
            GoldilocksField(12237016771705070337),
            GoldilocksField(6310848257747044271),
            GoldilocksField(4113320295997237722),
            GoldilocksField(16814466981743832206),
        ]),
        u: QuinticExtension([
            GoldilocksField(3167328430596803860),
            GoldilocksField(3373273028644416665),
            GoldilocksField(13236218152769964416),
            GoldilocksField(15816058495228292851),
            GoldilocksField(8001858317475143616),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(18177821317840763398),
            GoldilocksField(7208454950216370358),
            GoldilocksField(14780008596150434388),
            GoldilocksField(1996209710201147400),
            GoldilocksField(15053807226826426393),
        ]),
        u: QuinticExtension([
            GoldilocksField(15715262971932204033),
            GoldilocksField(8714266598318325282),
            GoldilocksField(16219555901832677748),
            GoldilocksField(245656264630859564),
            GoldilocksField(4633621313248689546),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(1841517068532537972),
            GoldilocksField(18159427598377627852),
            GoldilocksField(7101751901448687743),
            GoldilocksField(17419951806778701769),
            GoldilocksField(15872135176100603181),
        ]),
        u: QuinticExtension([
            GoldilocksField(14938501378296161155),
            GoldilocksField(1475670735048314023),
            GoldilocksField(16050270748361767813),
            GoldilocksField(14927165212644255889),
            GoldilocksField(11393545566941110440),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(11016016673823483108),
            GoldilocksField(10346862381783788104),
            GoldilocksField(15413371668364281620),
            GoldilocksField(3789574685442821016),
            GoldilocksField(10327416280296530490),
        ]),
        u: QuinticExtension([
            GoldilocksField(1471176826026130963),
            GoldilocksField(4450232675785892534),
            GoldilocksField(1999057422912905727),
            GoldilocksField(1862118471196890026),
            GoldilocksField(9836667920542412877),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6731903398643108337),
            GoldilocksField(9730751811492293683),
            GoldilocksField(14448312389075081409),
            GoldilocksField(12232946634578520226),
            GoldilocksField(5149728844990350383),
        ]),
        u: QuinticExtension([
            GoldilocksField(13304407859937134355),
            GoldilocksField(9591204855047500826),
            GoldilocksField(7113854334829183285),
            GoldilocksField(11795800474402144172),
            GoldilocksField(15515700514512556333),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16206498404205552972),
            GoldilocksField(7467982993043588737),
            GoldilocksField(954111054908556531),
            GoldilocksField(15259102337638072429),
            GoldilocksField(12916169332967261393),
        ]),
        u: QuinticExtension([
            GoldilocksField(10986259094443142549),
            GoldilocksField(918816446526617182),
            GoldilocksField(10678622673672003543),
            GoldilocksField(9174304313393317665),
            GoldilocksField(7047157651466091392),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(5524339637429426536),
            GoldilocksField(13012396000187524883),
            GoldilocksField(8701257881797351665),
            GoldilocksField(7601128411527015893),
            GoldilocksField(16817462731082877836),
        ]),
        u: QuinticExtension([
            GoldilocksField(17315109416612436252),
            GoldilocksField(8903947754371488039),
            GoldilocksField(11080795620793054950),
            GoldilocksField(12186542410997831530),
            GoldilocksField(10711958746278079839),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6264744119896948359),
            GoldilocksField(10601340541921400101),
            GoldilocksField(14657733949024428965),
            GoldilocksField(4449426502181859631),
            GoldilocksField(15315608631820517742),
        ]),
        u: QuinticExtension([
            GoldilocksField(2840761601268004671),
            GoldilocksField(5838696228978373234),
            GoldilocksField(8592255273635329784),
            GoldilocksField(11032000397652854764),
            GoldilocksField(10686619933707435695),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(18403739294830496585),
            GoldilocksField(5395321127948182079),
            GoldilocksField(4362930334215698085),
            GoldilocksField(3891000203325226477),
            GoldilocksField(7114568565526390560),
        ]),
        u: QuinticExtension([
            GoldilocksField(3975610669088804605),
            GoldilocksField(17233183788370721900),
            GoldilocksField(3024806945190232867),
            GoldilocksField(12024175965114418277),
            GoldilocksField(16612390798970961761),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16856493815994067196),
            GoldilocksField(14785817845078013270),
            GoldilocksField(2731707303337213832),
            GoldilocksField(5700860421257465777),
            GoldilocksField(16512159307408461032),
        ]),
        u: QuinticExtension([
            GoldilocksField(15635761722509813626),
            GoldilocksField(9688917495669656811),
            GoldilocksField(13645873987197106712),
            GoldilocksField(11818012498378673433),
            GoldilocksField(5396192277875563403),
        ]),
    },
];
pub(crate) const MUL_TABLE_G200: [AffinePoint; 16] = [
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16579624480310836700),
            GoldilocksField(4654009893788464381),
            GoldilocksField(1879948550255376688),
            GoldilocksField(7165778526988411257),
            GoldilocksField(10027404176497435516),
        ]),
        u: QuinticExtension([
            GoldilocksField(14672132787094789329),
            GoldilocksField(8134912716135063128),
            GoldilocksField(1681926888624011127),
            GoldilocksField(3090601642585073427),
            GoldilocksField(5082367180675620723),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(12935460095664666921),
            GoldilocksField(13688862829955708769),
            GoldilocksField(9016000768964455819),
            GoldilocksField(18074632734324577885),
            GoldilocksField(10067423399607639746),
        ]),
        u: QuinticExtension([
            GoldilocksField(1891495241569347963),
            GoldilocksField(9444474092325495302),
            GoldilocksField(10783243003245947999),
            GoldilocksField(15112298120192081012),
            GoldilocksField(8489851093422035711),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(3971319576308477519),
            GoldilocksField(11846884681248592704),
            GoldilocksField(7252181329490053993),
            GoldilocksField(2837725043423724233),
            GoldilocksField(554712018738433440),
        ]),
        u: QuinticExtension([
            GoldilocksField(2283716927456626486),
            GoldilocksField(4678849096118793201),
            GoldilocksField(7064207072633614681),
            GoldilocksField(1293928013652227803),
            GoldilocksField(4122458298059420843),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(17348512312529533060),
            GoldilocksField(8639109793604178953),
            GoldilocksField(15818212126524653050),
            GoldilocksField(17950462779586277033),
            GoldilocksField(15262813331997381824),
        ]),
        u: QuinticExtension([
            GoldilocksField(5107077498491622223),
            GoldilocksField(4004992081347212098),
            GoldilocksField(13489481871700798330),
            GoldilocksField(1439663511274537768),
            GoldilocksField(2916087242841422420),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6922005184759837266),
            GoldilocksField(4816940458101140735),
            GoldilocksField(14348527682434315092),
            GoldilocksField(16536303157525555702),
            GoldilocksField(8297623336197847715),
        ]),
        u: QuinticExtension([
            GoldilocksField(3550074682900750241),
            GoldilocksField(5900089539068431592),
            GoldilocksField(15881485277116367548),
            GoldilocksField(4494234475833006435),
            GoldilocksField(698663099382505402),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16241061472728124620),
            GoldilocksField(6131873802153215223),
            GoldilocksField(16831672901554690917),
            GoldilocksField(17254464099918200920),
            GoldilocksField(11185076059758094886),
        ]),
        u: QuinticExtension([
            GoldilocksField(4298246016297961963),
            GoldilocksField(7189403662133590696),
            GoldilocksField(9418905817123278198),
            GoldilocksField(14531204622533844239),
            GoldilocksField(8541862381303549676),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(15447868881789362057),
            GoldilocksField(5508196216379753233),
            GoldilocksField(16485811425555940264),
            GoldilocksField(13222911319795183488),
            GoldilocksField(3159840448703036170),
        ]),
        u: QuinticExtension([
            GoldilocksField(5204165561238044016),
            GoldilocksField(13468232899848292870),
            GoldilocksField(17191293205041837891),
            GoldilocksField(18246478932776430841),
            GoldilocksField(15325962678017236259),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(10621030779181154903),
            GoldilocksField(17136235063952920846),
            GoldilocksField(9314218722796245020),
            GoldilocksField(9368391401101022200),
            GoldilocksField(8555825846071793318),
        ]),
        u: QuinticExtension([
            GoldilocksField(7664574944617879289),
            GoldilocksField(16231709863136830941),
            GoldilocksField(6590015398331881523),
            GoldilocksField(14621275666438800255),
            GoldilocksField(6788090867699016859),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(720425677629731910),
            GoldilocksField(558256443318656554),
            GoldilocksField(5541446382714187419),
            GoldilocksField(16510886437312776757),
            GoldilocksField(10390794562653129460),
        ]),
        u: QuinticExtension([
            GoldilocksField(13345062980903998097),
            GoldilocksField(13515598458298192134),
            GoldilocksField(6777126340206327673),
            GoldilocksField(14815170113495224049),
            GoldilocksField(1808065573577174046),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(8512864041521600225),
            GoldilocksField(13632121416118897300),
            GoldilocksField(15219847883797542830),
            GoldilocksField(6281672652132756722),
            GoldilocksField(12690075810246041331),
        ]),
        u: QuinticExtension([
            GoldilocksField(15240394429738581893),
            GoldilocksField(13526765963383505570),
            GoldilocksField(385005971031338975),
            GoldilocksField(17055827423572264183),
            GoldilocksField(9799789282342151082),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16433318271062727305),
            GoldilocksField(1053347735296699652),
            GoldilocksField(5450980641165850759),
            GoldilocksField(4054013846223550412),
            GoldilocksField(2947006303368928546),
        ]),
        u: QuinticExtension([
            GoldilocksField(17928731966205396993),
            GoldilocksField(17380491217072802345),
            GoldilocksField(11249617314541463800),
            GoldilocksField(13746866206588898967),
            GoldilocksField(12571294391280109436),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(18390535763930288817),
            GoldilocksField(13956792141128102015),
            GoldilocksField(14894537812189538984),
            GoldilocksField(5333289079328326940),
            GoldilocksField(10076243009389690036),
        ]),
        u: QuinticExtension([
            GoldilocksField(8250298621563594676),
            GoldilocksField(5752869300366626776),
            GoldilocksField(3645497280270257308),
            GoldilocksField(12900372348275640100),
            GoldilocksField(16885169851778635393),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(1544673605232492218),
            GoldilocksField(661976301925838846),
            GoldilocksField(18424167365360574137),
            GoldilocksField(3246102959607711481),
            GoldilocksField(5211114398364569488),
        ]),
        u: QuinticExtension([
            GoldilocksField(4460870711092798561),
            GoldilocksField(3451028986412879783),
            GoldilocksField(17189436277480328087),
            GoldilocksField(16695916816719405476),
            GoldilocksField(712205578119358045),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6293897706165296494),
            GoldilocksField(1058852550215266328),
            GoldilocksField(5340568372786241917),
            GoldilocksField(5059226109913370799),
            GoldilocksField(3526604109990729657),
        ]),
        u: QuinticExtension([
            GoldilocksField(11157536197710362632),
            GoldilocksField(12986275077072906620),
            GoldilocksField(3545776948579292831),
            GoldilocksField(11785840473114906984),
            GoldilocksField(10099190834060857641),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(11775056308366361359),
            GoldilocksField(797408970642010187),
            GoldilocksField(11281697728680415953),
            GoldilocksField(15083731163311144943),
            GoldilocksField(2591402698173474283),
        ]),
        u: QuinticExtension([
            GoldilocksField(11008763107345506753),
            GoldilocksField(5488815957510229275),
            GoldilocksField(14952094509887379098),
            GoldilocksField(11189563823429936956),
            GoldilocksField(5358872537390699328),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(7693346203646808522),
            GoldilocksField(3196464325266151639),
            GoldilocksField(1785222888514983133),
            GoldilocksField(4961084796993397121),
            GoldilocksField(16651160545753804914),
        ]),
        u: QuinticExtension([
            GoldilocksField(12113910774037980879),
            GoldilocksField(7934748119329239619),
            GoldilocksField(14520318444063438710),
            GoldilocksField(1372113091606068548),
            GoldilocksField(11259415352488711270),
        ]),
    },
];
pub(crate) const MUL_TABLE_G240: [AffinePoint; 16] = [
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(12150973993870501418),
            GoldilocksField(4223924024756880744),
            GoldilocksField(12164602482423882598),
            GoldilocksField(10110827219574637558),
            GoldilocksField(7454721448521923322),
        ]),
        u: QuinticExtension([
            GoldilocksField(8223067178251187472),
            GoldilocksField(14791411048736217143),
            GoldilocksField(6548050514357003677),
            GoldilocksField(14101051606185056042),
            GoldilocksField(9723051335063761713),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(2309936888761803695),
            GoldilocksField(10528374492977918782),
            GoldilocksField(2909110930817727979),
            GoldilocksField(14140458781369438628),
            GoldilocksField(14608954252678341690),
        ]),
        u: QuinticExtension([
            GoldilocksField(13315057417082143829),
            GoldilocksField(2875970576192442492),
            GoldilocksField(10204753160271556880),
            GoldilocksField(2528165599636440836),
            GoldilocksField(15588626368559095887),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(10285645489096016828),
            GoldilocksField(6826665230734386908),
            GoldilocksField(3643430412114742539),
            GoldilocksField(3525069461824492670),
            GoldilocksField(9265259914130088255),
        ]),
        u: QuinticExtension([
            GoldilocksField(18429224257556970829),
            GoldilocksField(16335577406386351411),
            GoldilocksField(1444816108348712587),
            GoldilocksField(532410028340092104),
            GoldilocksField(16527851406835121471),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(5704484539924172300),
            GoldilocksField(7404153046704669007),
            GoldilocksField(10464550607197363861),
            GoldilocksField(6247473471694475226),
            GoldilocksField(5115259736150878416),
        ]),
        u: QuinticExtension([
            GoldilocksField(17579790539786983406),
            GoldilocksField(2637134544147945869),
            GoldilocksField(4816977865203371123),
            GoldilocksField(248089872468508433),
            GoldilocksField(4531777203898089043),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(4467537824888613905),
            GoldilocksField(558399825893683724),
            GoldilocksField(4701759200819649961),
            GoldilocksField(16655886253669319016),
            GoldilocksField(14976096788667951951),
        ]),
        u: QuinticExtension([
            GoldilocksField(4542754722443867895),
            GoldilocksField(17838455475085664297),
            GoldilocksField(3856006738985174470),
            GoldilocksField(3695500756395218282),
            GoldilocksField(10605666420204608788),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9600870350287519277),
            GoldilocksField(2943222643218798300),
            GoldilocksField(7504369701746722369),
            GoldilocksField(3618345531898965921),
            GoldilocksField(7996994629944741723),
        ]),
        u: QuinticExtension([
            GoldilocksField(1279526320710392206),
            GoldilocksField(4757632433269403318),
            GoldilocksField(12420546729136568420),
            GoldilocksField(17056471951401952929),
            GoldilocksField(16063059997803195687),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(38797529778910718),
            GoldilocksField(5399283910211167400),
            GoldilocksField(14916560969855131779),
            GoldilocksField(682297961769392616),
            GoldilocksField(18182112167162978281),
        ]),
        u: QuinticExtension([
            GoldilocksField(4851072938181616220),
            GoldilocksField(1768095373600786914),
            GoldilocksField(16165351733290258071),
            GoldilocksField(16718741168144865753),
            GoldilocksField(3387411250674432260),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(2078420199699477832),
            GoldilocksField(8841464556389390192),
            GoldilocksField(11642290600245563238),
            GoldilocksField(3963148268515541490),
            GoldilocksField(12483849286604430921),
        ]),
        u: QuinticExtension([
            GoldilocksField(13785261307443076347),
            GoldilocksField(8468941646155066103),
            GoldilocksField(3174688882704239544),
            GoldilocksField(10801045692115252746),
            GoldilocksField(12996585990193072559),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9988106987444016195),
            GoldilocksField(550486750301142863),
            GoldilocksField(16147691900152849957),
            GoldilocksField(11482331300775839937),
            GoldilocksField(18210139443246491531),
        ]),
        u: QuinticExtension([
            GoldilocksField(17280645777130171058),
            GoldilocksField(5143971509316066734),
            GoldilocksField(9444564929039929588),
            GoldilocksField(2353260944176421839),
            GoldilocksField(7465399806142043858),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6553362623203065460),
            GoldilocksField(17774755178623960848),
            GoldilocksField(8170189958890581873),
            GoldilocksField(16479723056180470829),
            GoldilocksField(9052786989344840129),
        ]),
        u: QuinticExtension([
            GoldilocksField(5051652642644768336),
            GoldilocksField(8142249998939619774),
            GoldilocksField(6620402268383223033),
            GoldilocksField(15441849186338064088),
            GoldilocksField(17835312998647746744),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(12874799067286536937),
            GoldilocksField(5111767032451732361),
            GoldilocksField(9488358619465533651),
            GoldilocksField(2298827191559954790),
            GoldilocksField(15515015915136216385),
        ]),
        u: QuinticExtension([
            GoldilocksField(9934501897778259341),
            GoldilocksField(4862857445330881324),
            GoldilocksField(7191492445992175174),
            GoldilocksField(12588576141673201363),
            GoldilocksField(16820074689985814838),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(5506619209793629300),
            GoldilocksField(7913282297782618980),
            GoldilocksField(17035818002685942678),
            GoldilocksField(12219904669712698298),
            GoldilocksField(4701091471601382843),
        ]),
        u: QuinticExtension([
            GoldilocksField(15628068501760254685),
            GoldilocksField(9969915731376118609),
            GoldilocksField(4006095342913065224),
            GoldilocksField(11418313546696146922),
            GoldilocksField(9535581122323707943),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(660873376897289838),
            GoldilocksField(2332132165476400730),
            GoldilocksField(4904481730668053625),
            GoldilocksField(17592889807182765803),
            GoldilocksField(1775714498923493702),
        ]),
        u: QuinticExtension([
            GoldilocksField(8278491921012401650),
            GoldilocksField(255948487882786297),
            GoldilocksField(18072518402211877989),
            GoldilocksField(5587324201809627359),
            GoldilocksField(7916932786454127987),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(14880598518989845991),
            GoldilocksField(207047178807534206),
            GoldilocksField(8949411863433895830),
            GoldilocksField(15987292979823109393),
            GoldilocksField(10450748763888590480),
        ]),
        u: QuinticExtension([
            GoldilocksField(10555084898033032496),
            GoldilocksField(11149020781750632904),
            GoldilocksField(12754167684588738056),
            GoldilocksField(6203699237453069783),
            GoldilocksField(8397897173241663238),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9626581523142877484),
            GoldilocksField(1014282284030781401),
            GoldilocksField(12559760948477539740),
            GoldilocksField(1719475860010180104),
            GoldilocksField(12167893974497751844),
        ]),
        u: QuinticExtension([
            GoldilocksField(10039328052268687164),
            GoldilocksField(16635482332793119899),
            GoldilocksField(5022923182724434224),
            GoldilocksField(13591886545913812687),
            GoldilocksField(4895263026932926029),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(15155727891419849945),
            GoldilocksField(3461317531761180793),
            GoldilocksField(16979908481664688625),
            GoldilocksField(15684045230954038314),
            GoldilocksField(12877152996799011381),
        ]),
        u: QuinticExtension([
            GoldilocksField(10120254644770986491),
            GoldilocksField(12192410531100649784),
            GoldilocksField(10938806981692604655),
            GoldilocksField(12172717977579895996),
            GoldilocksField(4275232645621155364),
        ]),
    },
];
pub(crate) const MUL_TABLE_G280: [AffinePoint; 16] = [
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(16213503882573976174),
            GoldilocksField(17168097236575729409),
            GoldilocksField(5196518270315815888),
            GoldilocksField(11117797779066091728),
            GoldilocksField(8133486084301919302),
        ]),
        u: QuinticExtension([
            GoldilocksField(11377245759937335205),
            GoldilocksField(4469833894127669069),
            GoldilocksField(9013706759438268290),
            GoldilocksField(1420430480105358672),
            GoldilocksField(16254559763550257786),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6770577214447522289),
            GoldilocksField(17115295583004483100),
            GoldilocksField(6277430884428490111),
            GoldilocksField(9367148506334403125),
            GoldilocksField(6974282321669735675),
        ]),
        u: QuinticExtension([
            GoldilocksField(1707192586757379005),
            GoldilocksField(16379422115255719397),
            GoldilocksField(9601719515238438547),
            GoldilocksField(18382556734962308004),
            GoldilocksField(9816132397810204232),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9294285004075832264),
            GoldilocksField(17130662520620891577),
            GoldilocksField(17784936778486874242),
            GoldilocksField(7903567741417559125),
            GoldilocksField(13438250367827046909),
        ]),
        u: QuinticExtension([
            GoldilocksField(14848083910737694210),
            GoldilocksField(3876659422633582058),
            GoldilocksField(17589777829179053297),
            GoldilocksField(13255998440838131261),
            GoldilocksField(16836576774480954338),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6484667483670905518),
            GoldilocksField(14676664710267712890),
            GoldilocksField(943902085975544717),
            GoldilocksField(9447580128743448969),
            GoldilocksField(16970743407772865788),
        ]),
        u: QuinticExtension([
            GoldilocksField(6954165327706188094),
            GoldilocksField(8649474865423322710),
            GoldilocksField(2874401123529251159),
            GoldilocksField(6791369587301962541),
            GoldilocksField(4682935506184263557),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(17851907028754343206),
            GoldilocksField(804547578244465260),
            GoldilocksField(9836036663990543574),
            GoldilocksField(2782503787318970554),
            GoldilocksField(11029394870653732940),
        ]),
        u: QuinticExtension([
            GoldilocksField(17369554270592567524),
            GoldilocksField(11522732789192066880),
            GoldilocksField(10532626382274872331),
            GoldilocksField(15084091109637533903),
            GoldilocksField(12335999220635744679),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(17927819688989708837),
            GoldilocksField(988383065726890993),
            GoldilocksField(17134368434216800793),
            GoldilocksField(3745722005614877274),
            GoldilocksField(12079981168859675058),
        ]),
        u: QuinticExtension([
            GoldilocksField(16726774574446090464),
            GoldilocksField(16696890676634414315),
            GoldilocksField(1768034342698142990),
            GoldilocksField(5182686366441226421),
            GoldilocksField(12905524404643926664),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(5238880119446176935),
            GoldilocksField(7632489756740258264),
            GoldilocksField(13186772342659187888),
            GoldilocksField(5373269322406587505),
            GoldilocksField(4770529397079489612),
        ]),
        u: QuinticExtension([
            GoldilocksField(906031890843250730),
            GoldilocksField(2524575321869066878),
            GoldilocksField(1749353240118753004),
            GoldilocksField(8401611932919350607),
            GoldilocksField(13809067453022178888),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(3463236991239759171),
            GoldilocksField(9418504154128111760),
            GoldilocksField(16646612147210445285),
            GoldilocksField(8048290712504722073),
            GoldilocksField(4003118648580238214),
        ]),
        u: QuinticExtension([
            GoldilocksField(565998296113403270),
            GoldilocksField(5639331094891259297),
            GoldilocksField(3505572540820256764),
            GoldilocksField(828191569017542887),
            GoldilocksField(2857618747433407780),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(10579753460617358122),
            GoldilocksField(1425995342210623751),
            GoldilocksField(4437515648943912607),
            GoldilocksField(9208066594954079254),
            GoldilocksField(8133603054721359271),
        ]),
        u: QuinticExtension([
            GoldilocksField(5608659290599426924),
            GoldilocksField(14668957524891276508),
            GoldilocksField(3526636595086144132),
            GoldilocksField(16193609694652077957),
            GoldilocksField(14814479961293040846),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(6518242324859070685),
            GoldilocksField(10266975352945110329),
            GoldilocksField(7108371607348108785),
            GoldilocksField(14640665666349949967),
            GoldilocksField(6626682978649692287),
        ]),
        u: QuinticExtension([
            GoldilocksField(14497090493935686223),
            GoldilocksField(2359285113881335421),
            GoldilocksField(8857893277062885351),
            GoldilocksField(17134727430531764861),
            GoldilocksField(2965117089847599750),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(3234104227668768457),
            GoldilocksField(3225149987207949460),
            GoldilocksField(7086354559915607869),
            GoldilocksField(12877313486906605274),
            GoldilocksField(11561630661277747287),
        ]),
        u: QuinticExtension([
            GoldilocksField(2820816680951282830),
            GoldilocksField(16276533358496276797),
            GoldilocksField(8457579751747007027),
            GoldilocksField(227223614176367695),
            GoldilocksField(9322033005858872072),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(9250294019343021345),
            GoldilocksField(1766812231170355307),
            GoldilocksField(13463388795950836322),
            GoldilocksField(13360210575495818786),
            GoldilocksField(8017191608807726449),
        ]),
        u: QuinticExtension([
            GoldilocksField(2255099676665350867),
            GoldilocksField(9521276765352132752),
            GoldilocksField(2720865867697005972),
            GoldilocksField(11524331848149102745),
            GoldilocksField(1853382383896073031),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(2978539147748114010),
            GoldilocksField(8802332783321234516),
            GoldilocksField(8411186022628666251),
            GoldilocksField(8965939348601447933),
            GoldilocksField(114587463394660272),
        ]),
        u: QuinticExtension([
            GoldilocksField(10264182682530549009),
            GoldilocksField(9417221851288332824),
            GoldilocksField(6749125336792502108),
            GoldilocksField(5965179916047598634),
            GoldilocksField(5374111552073601171),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(11268491403320804445),
            GoldilocksField(3223633055331054259),
            GoldilocksField(4794616428042030393),
            GoldilocksField(9408476598474014080),
            GoldilocksField(9670252162084106811),
        ]),
        u: QuinticExtension([
            GoldilocksField(513179742804655698),
            GoldilocksField(6696701674770108433),
            GoldilocksField(15069423489583433755),
            GoldilocksField(8503371514376466366),
            GoldilocksField(9365587857178664019),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(8819593048375407845),
            GoldilocksField(6714540590843281274),
            GoldilocksField(12089230224772664935),
            GoldilocksField(17126824728760033332),
            GoldilocksField(15606471085374753320),
        ]),
        u: QuinticExtension([
            GoldilocksField(18421631694011443474),
            GoldilocksField(14887724257413490347),
            GoldilocksField(16780540965430713114),
            GoldilocksField(10821826807107857648),
            GoldilocksField(6670989693469822701),
        ]),
    },
    AffinePoint {
        x: QuinticExtension([
            GoldilocksField(3738402919300376943),
            GoldilocksField(3839435231606872366),
            GoldilocksField(9950700849677152681),
            GoldilocksField(13599115104880397736),
            GoldilocksField(8170114904882828684),
        ]),
        u: QuinticExtension([
            GoldilocksField(17403178918799426162),
            GoldilocksField(12276995036775716338),
            GoldilocksField(135085417294263740),
            GoldilocksField(6813815177605214808),
            GoldilocksField(15873392253765702165),
        ]),
    },
];

// TODO: separate this into a proper script
#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::{curve::Point, scalar_field::Scalar};
    use num::{BigUint, FromPrimitive};
    use plonky2_field::types::{Field, PrimeField64};

    // For k = 40*j (j = 0 to 7), constant Gk[] is an array of 16 points in
    // affine coordinates, with Gk[i] = (i+1)*(2^k)*G for the conventional
    // generator point G.

    fn compute_table(j: u64) -> [AffinePoint; 16] {
        let k = 40 * j;
        let mut table = [Point::NEUTRAL; 16];

        for i in 0..16 {
            let s_biguint =
                (BigUint::from_u64(1).unwrap() << k) * BigUint::from_usize(i + 1).unwrap();
            let s = Scalar::from_noncanonical_biguint(s_biguint);
            table[i] = Point::GENERATOR * s;
        }

        let mut res = [AffinePoint::NEUTRAL; 16];
        res.copy_from_slice(&Point::batch_to_affine(&table));

        res
    }

    fn print_table(table: &[AffinePoint; 16], name: &str) {
        println!("pub(crate) const {}: [AffinePoint; 16] = [", name);

        for i in 0..table.len() {
            let x_limbs = table[i]
                .x
                .0
                .map(|x| format!("GoldilocksField({})", x.to_canonical_u64()))
                .join(", ");
            let u_limbs = table[i]
                .u
                .0
                .map(|u| format!("GoldilocksField({})", u.to_canonical_u64()))
                .join(", ");

            println!(
                "    AffinePoint {{ x: QuinticExtension([{}]), u: QuinticExtension([{}]) }},",
                x_limbs, u_limbs
            );
        }

        println!("];");
    }

    #[ignore]
    #[test]
    fn print_mul_table() {
        for j in 0..8 {
            let table = compute_table(j);
            print_table(&table, &format!("MUL_TABLE_G{}", j * 40));
        }

        panic!();
    }
}

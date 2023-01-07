// this is mostly copied from https://github.com/pornin/ecgfp5
// and adapted to build atop plonky2 primitives

// For k = 40*j (j = 0 to 7)])), constant Gk[] is an array of 16 points in
// affine coordinates), with Gk[i] = (i+1)*(2^k)*G for the conventional
// generator point G.

use plonky2_field::extension::FieldExtension;
use plonky2_field::types::Field;
use plonky2_field::{extension::quintic::QuinticExtension, goldilocks_field::GoldilocksField};

use super::curve::AffinePoint;

type GFp5 = QuinticExtension<GoldilocksField>;
type F = GoldilocksField;

pub(crate) const G0: [AffinePoint; 16] = [
    AffinePoint {
        /* 1 */
        x: QuinticExtension([
            GoldilocksField(0xB2CA178ECF4453A1),
            GoldilocksField(0x3C757788836D3EA4),
            GoldilocksField(0x48D7F28A26DAFD0B),
            GoldilocksField(0x1E0F15C7FD44C28E),
            GoldilocksField(0x21FA7FFCC8252211),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xBFFFFFFF40000001),
            GoldilocksField(0x0000000000000000),
            GoldilocksField(0x0000000000000000),
            GoldilocksField(0x0000000000000000),
            GoldilocksField(0x0000000000000000),
        ]),
    },
    AffinePoint {
        /* 2 */
        x: QuinticExtension([
            GoldilocksField(0xE53A14C4FA645562),
            GoldilocksField(0x60A03B3FCD006755),
            GoldilocksField(0xA8957CB5D9603510),
            GoldilocksField(0xE6E3A7D2AE140D1F),
            GoldilocksField(0x2067DD994D85BE92),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x755F62AABB0862CC),
            GoldilocksField(0x01EC60924033842A),
            GoldilocksField(0x2C3C144F634BCC44),
            GoldilocksField(0xFC49506D49454DA8),
            GoldilocksField(0x832FD0BD80667CB8),
        ]),
    },
    AffinePoint {
        /* 3 */
        x: QuinticExtension([
            GoldilocksField(0x3F1721A8B4BFE250),
            GoldilocksField(0x135784DB6D866068),
            GoldilocksField(0xD30DD0109FFB1561),
            GoldilocksField(0x6DADE466D752E9A2),
            GoldilocksField(0xACF8C6DE680A22CE),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE33B8641C16CB542),
            GoldilocksField(0x51DF13BC6EFD7D24),
            GoldilocksField(0xF47A6CBCB19B08F4),
            GoldilocksField(0xF4D93DDB98BE42CD),
            GoldilocksField(0x2751D54472A53C2D),
        ]),
    },
    AffinePoint {
        /* 4 */
        x: QuinticExtension([
            GoldilocksField(0x3C414CE1F8AAE143),
            GoldilocksField(0x7E727D66648035F2),
            GoldilocksField(0x6C3B9846D5F6B1EE),
            GoldilocksField(0xAF2022C48F8C580D),
            GoldilocksField(0xB513380B614CD936),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x31BFB83AE0B6F4DD),
            GoldilocksField(0x17D63BD29B7ED7A0),
            GoldilocksField(0xE5A9E36118531AC2),
            GoldilocksField(0xF38EBF5B61B910B7),
            GoldilocksField(0xFF010361346C74E0),
        ]),
    },
    AffinePoint {
        /* 5 */
        x: QuinticExtension([
            GoldilocksField(0xFB7AFB9F0749C52F),
            GoldilocksField(0x2F1B08170040D1C4),
            GoldilocksField(0x24C0E6F01ACB4244),
            GoldilocksField(0xB940E85295A4BDD9),
            GoldilocksField(0x2F934B66CC631967),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x80916CD6733E31AD),
            GoldilocksField(0x670F5AE14E0CBD35),
            GoldilocksField(0x46E278D15F6E2188),
            GoldilocksField(0x8252B7241D444762),
            GoldilocksField(0x87D69BD8B06A37BE),
        ]),
    },
    AffinePoint {
        /* 6 */
        x: QuinticExtension([
            GoldilocksField(0x99C65351B868D074),
            GoldilocksField(0x2B549ABB45AE3B05),
            GoldilocksField(0xE6D26BBB7D6983C7),
            GoldilocksField(0xCE8D90C97E847FFD),
            GoldilocksField(0x90490A2371237297),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xC5D015B9654827DF),
            GoldilocksField(0xDA4537A57CA4CE22),
            GoldilocksField(0x36C70DD003F9210E),
            GoldilocksField(0x41CB4479D87846E6),
            GoldilocksField(0xF142FD633CB37552),
        ]),
    },
    AffinePoint {
        /* 7 */
        x: QuinticExtension([
            GoldilocksField(0x4219C8AAA886A320),
            GoldilocksField(0xF821E49F695A9B94),
            GoldilocksField(0xBC7223437FA3F3BA),
            GoldilocksField(0x77D483207195D99F),
            GoldilocksField(0xBE37D76AD12515C5),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x2E8E911431F24635),
            GoldilocksField(0xC019A6C715B9A781),
            GoldilocksField(0x417911CA4F5C8C33),
            GoldilocksField(0x6EBA314D3B0B4D84),
            GoldilocksField(0x8C6DC80B62684832),
        ]),
    },
    AffinePoint {
        /* 8 */
        x: QuinticExtension([
            GoldilocksField(0x37E2A1C7E50102C2),
            GoldilocksField(0xBCAEA8E86E48319C),
            GoldilocksField(0x9F9654BF22E51108),
            GoldilocksField(0x8B0C420979D809A1),
            GoldilocksField(0xA659D6B7DA77C522),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xC9AEF44D3444F179),
            GoldilocksField(0x84FCE8B4923CF2A1),
            GoldilocksField(0x2AC63DF0EAD62DFA),
            GoldilocksField(0x27AF8ADF6D3054C2),
            GoldilocksField(0x33FCA2A42C8C4310),
        ]),
    },
    AffinePoint {
        /* 9 */
        x: QuinticExtension([
            GoldilocksField(0xDD27B15F06D34600),
            GoldilocksField(0x784D56F0C1D26C1F),
            GoldilocksField(0x06D180D9AD8C6590),
            GoldilocksField(0x5B62326A633F7A02),
            GoldilocksField(0xABE5478A46E4902C),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB76F2D032EE42A38),
            GoldilocksField(0x00B792BF8263FB30),
            GoldilocksField(0xD5BA9B5F40896B2D),
            GoldilocksField(0x5DB4A555E3DFA714),
            GoldilocksField(0x27A01C4CFCAF6622),
        ]),
    },
    AffinePoint {
        /* 10 */
        x: QuinticExtension([
            GoldilocksField(0x7597F149CD01FFB1),
            GoldilocksField(0x21F55DA1987F38CE),
            GoldilocksField(0xF94895E941D11482),
            GoldilocksField(0x6071F8478A6FB91A),
            GoldilocksField(0x8147478709A8AD5F),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x47C5FA108C16AF99),
            GoldilocksField(0x29FC9D7035795EA7),
            GoldilocksField(0x064A742C800C7F1E),
            GoldilocksField(0xC42AB1D12867714F),
            GoldilocksField(0x237A1D635AD89D2C),
        ]),
    },
    AffinePoint {
        /* 11 */
        x: QuinticExtension([
            GoldilocksField(0x36F591486C28B83C),
            GoldilocksField(0x334A430D55CF5093),
            GoldilocksField(0x2872A90BFB945CBF),
            GoldilocksField(0x805E6320AE5B5820),
            GoldilocksField(0xF39AA3FF0F843A88),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x29D7F90C194EE89D),
            GoldilocksField(0x94E8BD911C76E114),
            GoldilocksField(0x2DFE8D94E8698B00),
            GoldilocksField(0x1B89FD3E133A360C),
            GoldilocksField(0xBF423DD058BA8584),
        ]),
    },
    AffinePoint {
        /* 12 */
        x: QuinticExtension([
            GoldilocksField(0xA0A63453B2520E76),
            GoldilocksField(0x154B9F3B3DED29A2),
            GoldilocksField(0x68A11C1528013C8F),
            GoldilocksField(0x903B80A6F09AE901),
            GoldilocksField(0x940A7401F7737C15),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE7801DEA8AE5C94D),
            GoldilocksField(0x9CA6A99FC677B56D),
            GoldilocksField(0x99A02F2C85B98888),
            GoldilocksField(0xD6EF067B29F73B09),
            GoldilocksField(0xBAE7DF0908F3326C),
        ]),
    },
    AffinePoint {
        /* 13 */
        x: QuinticExtension([
            GoldilocksField(0x9B529BDD12BA8168),
            GoldilocksField(0x315948D3E2C24095),
            GoldilocksField(0x4A32AAD7F2BD5D71),
            GoldilocksField(0x3EA68814C835F7AC),
            GoldilocksField(0x3693DC2BFE7A0305),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x4B3D8724805D3595),
            GoldilocksField(0x02D5D1434E2B7FA0),
            GoldilocksField(0x5433B329F514B8D9),
            GoldilocksField(0xE0B27774E7FD8AD5),
            GoldilocksField(0x39740438FA4DE60D),
        ]),
    },
    AffinePoint {
        /* 14 */
        x: QuinticExtension([
            GoldilocksField(0xB70B88550BCDC40A),
            GoldilocksField(0x79CE838FD1512FC2),
            GoldilocksField(0xF2D254FD78B4176A),
            GoldilocksField(0xD21D143BD0C3D6E8),
            GoldilocksField(0xD7586F2424CD8A89),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x0E7078E4323A9AF6),
            GoldilocksField(0x808BB98BE0705106),
            GoldilocksField(0x29802C432FD1B88F),
            GoldilocksField(0xFB97D7787351836F),
            GoldilocksField(0xAD55A82A983E13BE),
        ]),
    },
    AffinePoint {
        /* 15 */
        x: QuinticExtension([
            GoldilocksField(0xF5DDAE987FE1A8B7),
            GoldilocksField(0xC72A5894CAE32911),
            GoldilocksField(0x329407834833459F),
            GoldilocksField(0x47C4368ABADAFC42),
            GoldilocksField(0x04175D3818D989BA),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x68927FC54DE30BB1),
            GoldilocksField(0xCBC1374AC68435D3),
            GoldilocksField(0x0AE217DB1A860A3D),
            GoldilocksField(0x49486982ED44B94C),
            GoldilocksField(0xC7F1D337344580B9),
        ]),
    },
    AffinePoint {
        /* 16 */
        x: QuinticExtension([
            GoldilocksField(0x787A277B60DFF343),
            GoldilocksField(0x6B93CDCC42380C6F),
            GoldilocksField(0xE359042B24517973),
            GoldilocksField(0x67599C1234CA257E),
            GoldilocksField(0x5F334A5914C62FFF),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x8642ADFD967547F1),
            GoldilocksField(0x48D8168E1347C6DB),
            GoldilocksField(0x321FA2D8EF93D21F),
            GoldilocksField(0xFFDDFAA10ED7545D),
            GoldilocksField(0x31195BCBF821882E),
        ]),
    },
];

pub(crate) const G40: [AffinePoint; 16] = [
    AffinePoint {
        /* 1 */
        x: QuinticExtension([
            GoldilocksField(0x611A5359FCF0A4BF),
            GoldilocksField(0x12370F87B8C02B88),
            GoldilocksField(0x1B4C636C579F5ACE),
            GoldilocksField(0xAC89FBCF88F6C0A9),
            GoldilocksField(0x71A4E222268D96E3),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x87B686E43F300726),
            GoldilocksField(0xB2051F0C83DF5ECD),
            GoldilocksField(0x732BFAACD02E6493),
            GoldilocksField(0x8B93DC17B6D61A39),
            GoldilocksField(0x84B2CEFEC769CBE1),
        ]),
    },
    AffinePoint {
        /* 2 */
        x: QuinticExtension([
            GoldilocksField(0xB0A217943742E147),
            GoldilocksField(0xA294DEAA61794F12),
            GoldilocksField(0x9C3CD78D82554027),
            GoldilocksField(0x44A5E5721FF637AF),
            GoldilocksField(0x94222A04B9B09CFC),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xC1DA484EEA20E61B),
            GoldilocksField(0x87839986D2C0C417),
            GoldilocksField(0x536726F207AF8AE9),
            GoldilocksField(0x0CE2D8784C4C06BD),
            GoldilocksField(0xF38780BCCCE10933),
        ]),
    },
    AffinePoint {
        /* 3 */
        x: QuinticExtension([
            GoldilocksField(0x141D1DC8EB53D8F2),
            GoldilocksField(0x119B541DACDB7E25),
            GoldilocksField(0x00F4E51C4122258A),
            GoldilocksField(0x070D3841B7C30D81),
            GoldilocksField(0xCFB2F9E418BF898A),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xADBCFA8D2CD4BD91),
            GoldilocksField(0x196805299FC754A5),
            GoldilocksField(0x730C5BD275BD1F10),
            GoldilocksField(0x60A2F4A66ECAF807),
            GoldilocksField(0x45C489829D03437C),
        ]),
    },
    AffinePoint {
        /* 4 */
        x: QuinticExtension([
            GoldilocksField(0x68AE5601EAC10B82),
            GoldilocksField(0xB68D5A1D87381951),
            GoldilocksField(0xB4F2849EE7337B90),
            GoldilocksField(0xF45AFE402B9F2354),
            GoldilocksField(0x38931DA82C2AB7AD),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE4A064DCEDA49780),
            GoldilocksField(0x65E6A29DBEF8A1DD),
            GoldilocksField(0xA4131B06679D9E21),
            GoldilocksField(0xDBCE53BCE947882D),
            GoldilocksField(0x7466B758C333EAAD),
        ]),
    },
    AffinePoint {
        /* 5 */
        x: QuinticExtension([
            GoldilocksField(0x6089B3824D410C77),
            GoldilocksField(0x8FD29FDE1A3D8D78),
            GoldilocksField(0x167564B36436BBBA),
            GoldilocksField(0xEA5A00784D95E6DB),
            GoldilocksField(0x452719F72FD61C13),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xBDF5B501AA8CC54C),
            GoldilocksField(0xF912B0B754DA2189),
            GoldilocksField(0x63127D35D8F1D3D9),
            GoldilocksField(0xCA1FB0958F64D4D4),
            GoldilocksField(0x394442A46629C061),
        ]),
    },
    AffinePoint {
        /* 6 */
        x: QuinticExtension([
            GoldilocksField(0x0B1C9B08197AB917),
            GoldilocksField(0x99363DE503CCC6DC),
            GoldilocksField(0x14595D2EF0327468),
            GoldilocksField(0x7431486E378FD529),
            GoldilocksField(0x96EB7445B9FB8E96),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x4146FC8E3963FC23),
            GoldilocksField(0xDEDCFDC1FDA931B6),
            GoldilocksField(0x7419E78EDA5EEA0A),
            GoldilocksField(0xAAFFC23F3059EB35),
            GoldilocksField(0x219031A52BC58CC8),
        ]),
    },
    AffinePoint {
        /* 7 */
        x: QuinticExtension([
            GoldilocksField(0xADBAB9D522BD3724),
            GoldilocksField(0xFD7D5EB6BE3578B6),
            GoldilocksField(0x3C70C562176C61FE),
            GoldilocksField(0xE2AADA2E4890AE6C),
            GoldilocksField(0x6D1743861BC53911),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xA8E2A8F79FE34EA2),
            GoldilocksField(0x13D0507BDC9A9E5A),
            GoldilocksField(0x2FE75772503CEDC2),
            GoldilocksField(0x3FC74C6C8B738B8E),
            GoldilocksField(0x4E38E48C25CEEB31),
        ]),
    },
    AffinePoint {
        /* 8 */
        x: QuinticExtension([
            GoldilocksField(0x615EB8E1F0C83E68),
            GoldilocksField(0xCB81875E029323C6),
            GoldilocksField(0x65286DE9000965E7),
            GoldilocksField(0x33CFFDAE8BC911A6),
            GoldilocksField(0xB397069155631D82),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x52BDE21F53DBF866),
            GoldilocksField(0x6BA936C397782D19),
            GoldilocksField(0xF6636580FC790962),
            GoldilocksField(0x4DD42818F1555407),
            GoldilocksField(0x8CFBEE063AEAED35),
        ]),
    },
    AffinePoint {
        /* 9 */
        x: QuinticExtension([
            GoldilocksField(0x8606EDC9D5E9113E),
            GoldilocksField(0x182007E52EFE27C2),
            GoldilocksField(0x2DEF1CF3FF3D0E56),
            GoldilocksField(0x48C88C73DD199CD7),
            GoldilocksField(0x9A0633769D643C8E),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x8D3ADAB2EB7C1CE5),
            GoldilocksField(0xEDFFACF33E5EF7AC),
            GoldilocksField(0xE5C9256D106262D2),
            GoldilocksField(0x58F2A9B6BCBCFDE6),
            GoldilocksField(0x06342E1E589A293F),
        ]),
    },
    AffinePoint {
        /* 10 */
        x: QuinticExtension([
            GoldilocksField(0x1BC27D027A958173),
            GoldilocksField(0x6EE3BD06429418FE),
            GoldilocksField(0xC0C81C8D10D4357F),
            GoldilocksField(0x2F316BBF8AD1914C),
            GoldilocksField(0xF4F1D345AFAADF4A),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x7D0DBC22D7972122),
            GoldilocksField(0x86C7FA7CE07AA8D6),
            GoldilocksField(0x4821B6A52390D398),
            GoldilocksField(0xCB3C24236F5242FD),
            GoldilocksField(0x46A8A25CDA5D4331),
        ]),
    },
    AffinePoint {
        /* 11 */
        x: QuinticExtension([
            GoldilocksField(0x3C6B69452DF20DA5),
            GoldilocksField(0xA16382DA71339FF0),
            GoldilocksField(0x0734ED8AF2FF492E),
            GoldilocksField(0xF231730C6F123884),
            GoldilocksField(0x6FC56D7052CBD46A),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE179905DE5B7E0C4),
            GoldilocksField(0xB8DDC95767B72473),
            GoldilocksField(0xB78DD85B25EEBB13),
            GoldilocksField(0xDBB1F2E1B16D69AF),
            GoldilocksField(0x8D29614950F6815F),
        ]),
    },
    AffinePoint {
        /* 12 */
        x: QuinticExtension([
            GoldilocksField(0x7799965532068142),
            GoldilocksField(0x28B108BB3C43E88C),
            GoldilocksField(0xC2DB5F750355BEA8),
            GoldilocksField(0x7681956BF5AA1319),
            GoldilocksField(0x721E95D6212552E9),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x1D6543850C7B0929),
            GoldilocksField(0xEC2A26A51B23B809),
            GoldilocksField(0x350CA7888D6D1BB6),
            GoldilocksField(0xF9F9E18245C88EC2),
            GoldilocksField(0xC9FE9387C28FB541),
        ]),
    },
    AffinePoint {
        /* 13 */
        x: QuinticExtension([
            GoldilocksField(0xC3BCC4F02EAC0C83),
            GoldilocksField(0x40D4CC7AD230AF3B),
            GoldilocksField(0xCA8CF9F2155561E1),
            GoldilocksField(0xAA99A9AD55042AB3),
            GoldilocksField(0x99CFB2EA5C310F59),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB4982A468FA68497),
            GoldilocksField(0x5865337073A89564),
            GoldilocksField(0xBE56C3BFCB41F3DE),
            GoldilocksField(0x0B8EF2F8E362697E),
            GoldilocksField(0x27551DED6C8B2F69),
        ]),
    },
    AffinePoint {
        /* 14 */
        x: QuinticExtension([
            GoldilocksField(0xF465CD82F6782E19),
            GoldilocksField(0x7C27481748425D26),
            GoldilocksField(0xC31289FF743E003B),
            GoldilocksField(0x5DB208B4B28FBD05),
            GoldilocksField(0x92D7B2A86701447B),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE976DFF362834259),
            GoldilocksField(0x6192BCD9481DE3B8),
            GoldilocksField(0x1940FE2ED27A5D23),
            GoldilocksField(0x14809DF701E212E2),
            GoldilocksField(0x2A7A9E074D9C514E),
        ]),
    },
    AffinePoint {
        /* 15 */
        x: QuinticExtension([
            GoldilocksField(0xE35C62CF3F65D6EF),
            GoldilocksField(0xF098AA0FC2F2329C),
            GoldilocksField(0x8EB38143A4A67389),
            GoldilocksField(0x7B8DA3DDC3B09AA7),
            GoldilocksField(0x8B499F766BF7DF6F),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x0D9C66B7D75AA0BA),
            GoldilocksField(0x7A711CACC51F2CDD),
            GoldilocksField(0x114F33E3793D5E1E),
            GoldilocksField(0x6FB4FE75D138043F),
            GoldilocksField(0x4FDD0F0E91EAE708),
        ]),
    },
    AffinePoint {
        /* 16 */
        x: QuinticExtension([
            GoldilocksField(0x4284BDEAB4B3C711),
            GoldilocksField(0x8F76C066935BD65F),
            GoldilocksField(0x3CA785B64D9CA29F),
            GoldilocksField(0x734FB37B0D88E69B),
            GoldilocksField(0xA615CD2231C23F7F),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xF40EA9D64BB8DB38),
            GoldilocksField(0x988BA6A1E479BAD6),
            GoldilocksField(0x3B728B804E9FB190),
            GoldilocksField(0x93151A8A49C7F10D),
            GoldilocksField(0xB878EB1AFF80BAA4),
        ]),
    },
];

pub(crate) const G80: [AffinePoint; 16] = [
    AffinePoint {
        /* 1 */
        x: QuinticExtension([
            GoldilocksField(0xBFF791CA85BC028C),
            GoldilocksField(0xF9FB2901AC55CA28),
            GoldilocksField(0x12104EC08515E4D1),
            GoldilocksField(0x7ED4B2E7EEB96EBD),
            GoldilocksField(0x3E86614EA1297F5F),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x66234F2978C62E9A),
            GoldilocksField(0xE797BC8963F28598),
            GoldilocksField(0x3D0FEDC88533EFA9),
            GoldilocksField(0x4928C92B69FA6381),
            GoldilocksField(0xA1B5CC952764418D),
        ]),
    },
    AffinePoint {
        /* 2 */
        x: QuinticExtension([
            GoldilocksField(0x981134C4BE7EDF34),
            GoldilocksField(0xD40A50866D224AC8),
            GoldilocksField(0xE6DEF6024A2A53C2),
            GoldilocksField(0x01EF1CE227EDAFC2),
            GoldilocksField(0xCEF67CBF79ABFB65),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x61341049363D9806),
            GoldilocksField(0x08DE86F9A1D453B4),
            GoldilocksField(0xF0B229C09A150C3C),
            GoldilocksField(0x9A6B56C97677B746),
            GoldilocksField(0x609BEF57EDA9C360),
        ]),
    },
    AffinePoint {
        /* 3 */
        x: QuinticExtension([
            GoldilocksField(0x174A2490B64AA7D7),
            GoldilocksField(0x90ED1C03BE0A1E10),
            GoldilocksField(0xDA36BBE81409E8AD),
            GoldilocksField(0x067986860F1796AA),
            GoldilocksField(0x1337390B4D19D913),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x6DCBDC9D8B6FFCF1),
            GoldilocksField(0x7D047179B73ACFA1),
            GoldilocksField(0x7A85AA3FC5AFCEFE),
            GoldilocksField(0xB1B14799AB054684),
            GoldilocksField(0x47BB870DD0B97BBA),
        ]),
    },
    AffinePoint {
        /* 4 */
        x: QuinticExtension([
            GoldilocksField(0x862E7BAEF49C57BE),
            GoldilocksField(0x16763810A8E92363),
            GoldilocksField(0xDB5D6E0A97F30C0B),
            GoldilocksField(0xC95A211538BFD9C5),
            GoldilocksField(0x74954F9DA117998E),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x1CBC9CCD138E89C8),
            GoldilocksField(0x56BA4D9517BDF76A),
            GoldilocksField(0x4875ABC85E225A46),
            GoldilocksField(0xB20EAC89A5DE3234),
            GoldilocksField(0x5E583F59E343D970),
        ]),
    },
    AffinePoint {
        /* 5 */
        x: QuinticExtension([
            GoldilocksField(0x0883BCD672D03A45),
            GoldilocksField(0xBF9DA799E22B0280),
            GoldilocksField(0x44844BD86742B4F4),
            GoldilocksField(0x2A4187BE0A3291E4),
            GoldilocksField(0x93EF3D4D5B9FEF68),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x9C628A98C225201A),
            GoldilocksField(0xC3E274C30E326B21),
            GoldilocksField(0xD20919EC01919CD1),
            GoldilocksField(0x708BE53371BB1513),
            GoldilocksField(0x2CF0872ABFC8DE56),
        ]),
    },
    AffinePoint {
        /* 6 */
        x: QuinticExtension([
            GoldilocksField(0x32B323FDE01834E5),
            GoldilocksField(0xEB7D58926002D7A5),
            GoldilocksField(0x311976F6B1DE8C50),
            GoldilocksField(0x53F0D01BCE04FC3B),
            GoldilocksField(0xB83C631FB53E6835),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xD49D9B793D4C47C3),
            GoldilocksField(0x05E243E0ADC4C074),
            GoldilocksField(0x87CF12FFB6194074),
            GoldilocksField(0x0194C81951ADBCE2),
            GoldilocksField(0xF2574B54F7601426),
        ]),
    },
    AffinePoint {
        /* 7 */
        x: QuinticExtension([
            GoldilocksField(0x4D60ADBD2CF65DC1),
            GoldilocksField(0x1E61B2DF00932777),
            GoldilocksField(0x17669D53561D21B9),
            GoldilocksField(0x5D7DBF43E4DA3098),
            GoldilocksField(0xF1F1C35ADBA8C435),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x61F2BD2C307F98D0),
            GoldilocksField(0xE5E6ACC9F0B7C41D),
            GoldilocksField(0xB2DE8FB853425F4A),
            GoldilocksField(0xC80F67CF92860DA9),
            GoldilocksField(0x0FEB48F9807E881F),
        ]),
    },
    AffinePoint {
        /* 8 */
        x: QuinticExtension([
            GoldilocksField(0xD94CCFEBFE7376A8),
            GoldilocksField(0x1F69B506843A9FB5),
            GoldilocksField(0x25AF59D5AC2D3E6E),
            GoldilocksField(0xA59BDF1CDDF32601),
            GoldilocksField(0xF6994D2E8166A9ED),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE8646785E9539A66),
            GoldilocksField(0x8458539C65A3863B),
            GoldilocksField(0xAD8A7E5ACC5827C1),
            GoldilocksField(0x09C7A04F5884FF56),
            GoldilocksField(0xCA3F64D3D9836222),
        ]),
    },
    AffinePoint {
        /* 9 */
        x: QuinticExtension([
            GoldilocksField(0xBE252F638D10A319),
            GoldilocksField(0xA01D12686C403F4B),
            GoldilocksField(0xFE05FE819EC8DC6D),
            GoldilocksField(0xEBCADE7DE489A748),
            GoldilocksField(0x733123271E24BC37),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB3AB56CE983235E8),
            GoldilocksField(0xDE6FDB91D66A50EA),
            GoldilocksField(0xC86EAB8154E9F48C),
            GoldilocksField(0x71DC9D64B9F0DB6D),
            GoldilocksField(0xB49651B0086141ED),
        ]),
    },
    AffinePoint {
        /* 10 */
        x: QuinticExtension([
            GoldilocksField(0x7C97A1B3D2BCF519),
            GoldilocksField(0x745E8A6D42E74374),
            GoldilocksField(0x59A5C1467B27CE5E),
            GoldilocksField(0x91B9E0010A0F5E0A),
            GoldilocksField(0x7116292D5BF4DB68),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x08FFA357F0044133),
            GoldilocksField(0x1C01E00B47B320B5),
            GoldilocksField(0x997AB99060DA5999),
            GoldilocksField(0xEE4D13B68B36E2F8),
            GoldilocksField(0x0A0052F08B6D6A49),
        ]),
    },
    AffinePoint {
        /* 11 */
        x: QuinticExtension([
            GoldilocksField(0x0768481A730111E7),
            GoldilocksField(0x7EBFBA3F4B985E9B),
            GoldilocksField(0x5A29ABF499EE8220),
            GoldilocksField(0x465E398A5E5D9F80),
            GoldilocksField(0x8B24E98561F1E7B6),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xF03F941053C2F189),
            GoldilocksField(0xAF15B126F1DD2362),
            GoldilocksField(0xE0C1F9661069145C),
            GoldilocksField(0x70AC53657D214DA1),
            GoldilocksField(0xE8CB1591772591D3),
        ]),
    },
    AffinePoint {
        /* 12 */
        x: QuinticExtension([
            GoldilocksField(0x7C9805E37AFB8FDA),
            GoldilocksField(0x7EC6E5AEC3944FD6),
            GoldilocksField(0x799AD8712625A151),
            GoldilocksField(0xCCF07EEE9356056F),
            GoldilocksField(0x8000967388385792),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xD3E9E6243AAD543B),
            GoldilocksField(0xA593BC22A29B572D),
            GoldilocksField(0x1B7927E3CFFD9E34),
            GoldilocksField(0x8BEC6E12A8095B62),
            GoldilocksField(0x3F79034163A87374),
        ]),
    },
    AffinePoint {
        /* 13 */
        x: QuinticExtension([
            GoldilocksField(0xB2703BF5809085B7),
            GoldilocksField(0xA1A318C3972A7206),
            GoldilocksField(0xF198687ADB77A5EE),
            GoldilocksField(0x521D68C0984AAA30),
            GoldilocksField(0x0E87E4AF29F07F30),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x00D595A13DC09A16),
            GoldilocksField(0x1718E2432CE9AA64),
            GoldilocksField(0x6232C6A30D7280C9),
            GoldilocksField(0xC1EE0841AD6808B2),
            GoldilocksField(0x9D11FF978AAA9DD1),
        ]),
    },
    AffinePoint {
        /* 14 */
        x: QuinticExtension([
            GoldilocksField(0xF234CA90DC099C7D),
            GoldilocksField(0x5F852BE145BA1D9C),
            GoldilocksField(0x81216F4F65D6C3FB),
            GoldilocksField(0x0D9EEE31A8A67E94),
            GoldilocksField(0x98FDF4A20AFB5398),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x24347A34B17D37E6),
            GoldilocksField(0x27BB083AA277AC09),
            GoldilocksField(0x7DBAEF7B797E2CE5),
            GoldilocksField(0xE9D42158A195E722),
            GoldilocksField(0x00C334027C0D49B3),
        ]),
    },
    AffinePoint {
        /* 15 */
        x: QuinticExtension([
            GoldilocksField(0x655DF87F3AF6CC59),
            GoldilocksField(0xAF94BD95765D3477),
            GoldilocksField(0xA7045CE70F44E1FC),
            GoldilocksField(0xC9BA067F552C2BBC),
            GoldilocksField(0xE43C604D34512222),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xC27AD2F5D6671B2C),
            GoldilocksField(0xB9378F2C40637BE7),
            GoldilocksField(0xCA0C8A5255F697CE),
            GoldilocksField(0x8BE0E122DC2E05A7),
            GoldilocksField(0xB8CF62F86BF9A6FA),
        ]),
    },
    AffinePoint {
        /* 16 */
        x: QuinticExtension([
            GoldilocksField(0x1CC5BAA51DA2D312),
            GoldilocksField(0xC0F23B1B0FE4ED2A),
            GoldilocksField(0x5B88DE0A0B959A46),
            GoldilocksField(0xB0B9A35ABED716C3),
            GoldilocksField(0xC9536A8EB50C5D89),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x0015F3561509A9BC),
            GoldilocksField(0x47FC4BC52D6BB76B),
            GoldilocksField(0x1D1A0EEA38A66200),
            GoldilocksField(0x324D88263AE0B8E2),
            GoldilocksField(0xDBC3D405C7DA217B),
        ]),
    },
];

pub(crate) const G120: [AffinePoint; 16] = [
    AffinePoint {
        /* 1 */
        x: QuinticExtension([
            GoldilocksField(0x81DFC7A7A5475CBD),
            GoldilocksField(0x56113BCFAC979210),
            GoldilocksField(0xF33847958D1D8345),
            GoldilocksField(0xFC3047B32E95A30B),
            GoldilocksField(0x9FF367E17679CF4E),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xD590CF0CA8D4761E),
            GoldilocksField(0xF1D3810D25C0B2C6),
            GoldilocksField(0xD90967F9BFE553BF),
            GoldilocksField(0xD0BC6D8C9E227371),
            GoldilocksField(0x6E275038FA572267),
        ]),
    },
    AffinePoint {
        /* 2 */
        x: QuinticExtension([
            GoldilocksField(0x05E572BA66B15B39),
            GoldilocksField(0xBBA9C5154EBEB103),
            GoldilocksField(0xAE8E2A206C070CFB),
            GoldilocksField(0xD6CCFF9306078AB9),
            GoldilocksField(0x2B1F8D1112EFD844),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x01EFFE47EC31E510),
            GoldilocksField(0xB511EE2B390B0EC8),
            GoldilocksField(0xCA2F7CDA89514AD1),
            GoldilocksField(0x1F1C5DB055753591),
            GoldilocksField(0xDB6827DFB0F409F3),
        ]),
    },
    AffinePoint {
        /* 3 */
        x: QuinticExtension([
            GoldilocksField(0xC918CEFC9CD298A2),
            GoldilocksField(0x7F9103ABCB08E989),
            GoldilocksField(0x47D00AF024CAF0B5),
            GoldilocksField(0xC9783B10BA49607E),
            GoldilocksField(0xEDB25A9081BB9CF2),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x935A4020350E5BEB),
            GoldilocksField(0xD670C08A7082F4A0),
            GoldilocksField(0x0107AC6F82F699B7),
            GoldilocksField(0x620D1C3B355A0565),
            GoldilocksField(0x1A7BDE0AECF72F81),
        ]),
    },
    AffinePoint {
        /* 4 */
        x: QuinticExtension([
            GoldilocksField(0xC47C9CD50B313694),
            GoldilocksField(0x9702D00B57F7CC7D),
            GoldilocksField(0x88A0C30D160A23BA),
            GoldilocksField(0x0446898927D7D998),
            GoldilocksField(0x1F57A65F0AF07CC6),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x8D67DB1D9C30123B),
            GoldilocksField(0x2DE54DC1702F4889),
            GoldilocksField(0xD2DA0ED1E2F9B6F3),
            GoldilocksField(0x15229A59D9A0F308),
            GoldilocksField(0x8F6D90EA4A987A4A),
        ]),
    },
    AffinePoint {
        /* 5 */
        x: QuinticExtension([
            GoldilocksField(0x60FC681C2040DF2B),
            GoldilocksField(0x5DDB0B69F69A2C78),
            GoldilocksField(0xB04189024B566274),
            GoldilocksField(0x971DD29D44E756A0),
            GoldilocksField(0xCA1B53DDFAFBB17C),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xA83CDC13589012A5),
            GoldilocksField(0xF9361D69CD5D929A),
            GoldilocksField(0x130724E183727F72),
            GoldilocksField(0x61D732B0B0C613F1),
            GoldilocksField(0xC4B677CE6D45BB1D),
        ]),
    },
    AffinePoint {
        /* 6 */
        x: QuinticExtension([
            GoldilocksField(0x9A874D914401394F),
            GoldilocksField(0x1B10689A9F8DBCD3),
            GoldilocksField(0xDA1EDF2CBF452064),
            GoldilocksField(0x2F3F0CA1DFB9D39C),
            GoldilocksField(0x49935367CDE51D51),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x32502FEBF9FC4762),
            GoldilocksField(0x648FC03B7084325A),
            GoldilocksField(0x0C41A62661CC2BB8),
            GoldilocksField(0x43DD5654E0EC867A),
            GoldilocksField(0x5121D65E62AF2D83),
        ]),
    },
    AffinePoint {
        /* 7 */
        x: QuinticExtension([
            GoldilocksField(0xF6BCD91B9D229C4F),
            GoldilocksField(0x1ED90B399CE1F26F),
            GoldilocksField(0xC69BF32330251380),
            GoldilocksField(0x54882B5C8B4B746B),
            GoldilocksField(0xB7E5714D168BF813),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xBCADEEFA2B9B265D),
            GoldilocksField(0x4A324D10DD8950B4),
            GoldilocksField(0xDDAE64902599551E),
            GoldilocksField(0xF3D542FA995AE725),
            GoldilocksField(0x3E4C6EBD817C001D),
        ]),
    },
    AffinePoint {
        /* 8 */
        x: QuinticExtension([
            GoldilocksField(0xFAB7B11C9F5DA1D2),
            GoldilocksField(0xF923CFBC1E20152D),
            GoldilocksField(0x5FB28B0BEDFAAD83),
            GoldilocksField(0x48DE620DBF3D5831),
            GoldilocksField(0x4CF82E5AE1D1B588),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xBD1F6C5B2DF2FDCE),
            GoldilocksField(0xE275428000B3CFB7),
            GoldilocksField(0xF32B9605CA11F251),
            GoldilocksField(0x2915E5380C54D3FB),
            GoldilocksField(0x80157D2979DB3056),
        ]),
    },
    AffinePoint {
        /* 9 */
        x: QuinticExtension([
            GoldilocksField(0x11F8FD3613EB3394),
            GoldilocksField(0x26FD38FDE73F91E2),
            GoldilocksField(0xEA33D1E4655AF9EE),
            GoldilocksField(0x05C24E7B239B8EAF),
            GoldilocksField(0x7B46BB6A9E90B5C3),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xC376EC1F984898E3),
            GoldilocksField(0x1432C1C994F0C08C),
            GoldilocksField(0x6316DDCF376D7CF7),
            GoldilocksField(0x2DDAA603E9B5BF50),
            GoldilocksField(0x24518AFD5F7C2860),
        ]),
    },
    AffinePoint {
        /* 10 */
        x: QuinticExtension([
            GoldilocksField(0x5716E9A9A5082F96),
            GoldilocksField(0x2F0E8BAD605D068A),
            GoldilocksField(0xEF5D32FDA8D82584),
            GoldilocksField(0xC42304AFBD7D871C),
            GoldilocksField(0x36687E044DEE4634),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x69DDBCE5A4ED87F4),
            GoldilocksField(0x526EC1AF1567153B),
            GoldilocksField(0x7E5C060D48CE5C40),
            GoldilocksField(0xBBFEC2C8FD487CC1),
            GoldilocksField(0x8EA69A07F739EABC),
        ]),
    },
    AffinePoint {
        /* 11 */
        x: QuinticExtension([
            GoldilocksField(0xA16992D30A7AF7D9),
            GoldilocksField(0xAF24506003383737),
            GoldilocksField(0x011AE62187AA31D0),
            GoldilocksField(0xF8B24E6FC064709E),
            GoldilocksField(0x50DCB633172F23ED),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x25F89F2718EB46C2),
            GoldilocksField(0xC7F267222A19A771),
            GoldilocksField(0x906877203579DFBA),
            GoldilocksField(0x28F5F954A16530B2),
            GoldilocksField(0xEC51C1AD7FC50EA1),
        ]),
    },
    AffinePoint {
        /* 12 */
        x: QuinticExtension([
            GoldilocksField(0x8A9EFBFAD6FE2B7B),
            GoldilocksField(0x05F489AA4CB43222),
            GoldilocksField(0x6B0644743682456E),
            GoldilocksField(0xCDFE708173216B70),
            GoldilocksField(0x1F4602CE2EE4C17C),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xEE8F4D0979D37A3C),
            GoldilocksField(0xBFA35BC84142F9DE),
            GoldilocksField(0x4077ED5E3F53B5E1),
            GoldilocksField(0x8FC7E38E062AB1F7),
            GoldilocksField(0xB475144BB33472D7),
        ]),
    },
    AffinePoint {
        /* 13 */
        x: QuinticExtension([
            GoldilocksField(0x4B554EB298E6827B),
            GoldilocksField(0x520957E0C44F2929),
            GoldilocksField(0xA70073FF75531C87),
            GoldilocksField(0xC3B0CBF9F188FC9E),
            GoldilocksField(0x280D47FD2D499A8A),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB35F4C19929B3CF0),
            GoldilocksField(0x59A153F47D6D83DA),
            GoldilocksField(0x771AFF712A96CBC2),
            GoldilocksField(0x828189393AF72E26),
            GoldilocksField(0x39D179AF302624EE),
        ]),
    },
    AffinePoint {
        /* 14 */
        x: QuinticExtension([
            GoldilocksField(0xC2F8E5C058865246),
            GoldilocksField(0x8AB1F6998CA70ABC),
            GoldilocksField(0xE6673CF650D2B916),
            GoldilocksField(0x7A33C8A0D27F468B),
            GoldilocksField(0x07900417775ED00B),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xFC5B21F6C152F2AA),
            GoldilocksField(0xA064DE2B87EC6B74),
            GoldilocksField(0x6C06C835AC6A1839),
            GoldilocksField(0xA0C0EC16E089F39B),
            GoldilocksField(0x612D331E37583569),
        ]),
    },
    AffinePoint {
        /* 15 */
        x: QuinticExtension([
            GoldilocksField(0x0B4C5AE4DFB02D59),
            GoldilocksField(0x01928464B2E04ECE),
            GoldilocksField(0x31509CBC754DF607),
            GoldilocksField(0x4F6B3C63839468B7),
            GoldilocksField(0xBBD9267B5B575941),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x94BE85F938B56FA7),
            GoldilocksField(0x139794D63C1AA09D),
            GoldilocksField(0x1DD665B970682C0C),
            GoldilocksField(0xCC7E05810839AFBD),
            GoldilocksField(0xBD5552D96F644DC1),
        ]),
    },
    AffinePoint {
        /* 16 */
        x: QuinticExtension([
            GoldilocksField(0xE4C0092E44A2CD56),
            GoldilocksField(0x8640177369C89FB1),
            GoldilocksField(0x6BB2EFB2D47BEF55),
            GoldilocksField(0xC3DAF4226CB841EE),
            GoldilocksField(0xC496CD80F48FC705),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE0D4A4D054567841),
            GoldilocksField(0xDAEECD5F3824B100),
            GoldilocksField(0x7921F0AC2E9068A8),
            GoldilocksField(0xC6FA0D80661C7931),
            GoldilocksField(0x5F583DEB723D2502),
        ]),
    },
];

pub(crate) const G160: [AffinePoint; 16] = [
    AffinePoint {
        /* 1 */
        x: QuinticExtension([
            GoldilocksField(0x3830EE0B997A7060),
            GoldilocksField(0x7497CABD07057DC0),
            GoldilocksField(0x75927E746D00183E),
            GoldilocksField(0x9C6DADCC11DEAEAC),
            GoldilocksField(0x9F66177B8921CD13),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x8A86D127D7936BEF),
            GoldilocksField(0xE5149E6285479398),
            GoldilocksField(0x87F259B34E5BD418),
            GoldilocksField(0xC9BEF27D34416090),
            GoldilocksField(0xE8F755D40D6865BA),
        ]),
    },
    AffinePoint {
        /* 2 */
        x: QuinticExtension([
            GoldilocksField(0xE5319BBC868A2D7B),
            GoldilocksField(0x2725C72D33F3817D),
            GoldilocksField(0xC9E0DE1B121D2EDF),
            GoldilocksField(0x6FF67E5D3EFD38A3),
            GoldilocksField(0x4CFD98FA2FB85BF2),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB696AED1DD180D3F),
            GoldilocksField(0xC3659521AAD5ECA1),
            GoldilocksField(0x32D71817413B6DC1),
            GoldilocksField(0x7295CB868CCFA40C),
            GoldilocksField(0x6F61DA7362EA8310),
        ]),
    },
    AffinePoint {
        /* 3 */
        x: QuinticExtension([
            GoldilocksField(0x63E82230DE1A29EE),
            GoldilocksField(0x5A4EDC24826D47C8),
            GoldilocksField(0x794F1E3CC2F9C121),
            GoldilocksField(0x45D55698BCEE11DC),
            GoldilocksField(0x146BD13724FBD24D),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xDFD1F5F4F742A5E5),
            GoldilocksField(0xF51E15E97A61F13F),
            GoldilocksField(0x4B4DD1444E59F27D),
            GoldilocksField(0xB0118CDBF0436FCB),
            GoldilocksField(0xA25FEEDA6DC43B5E),
        ]),
    },
    AffinePoint {
        /* 4 */
        x: QuinticExtension([
            GoldilocksField(0xD7ABE1664B48916D),
            GoldilocksField(0x52F58A2EC038E24B),
            GoldilocksField(0xF468611E6A5CD3D0),
            GoldilocksField(0x19F2CEDB6FD320E2),
            GoldilocksField(0x03A45C530E0E481E),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE7058EDFFC45E01C),
            GoldilocksField(0x229820847CA245FF),
            GoldilocksField(0xB4571CE63E1FE7CD),
            GoldilocksField(0xFAD16D9CCA297C82),
            GoldilocksField(0xE0936814B7A3FAA2),
        ]),
    },
    AffinePoint {
        /* 5 */
        x: QuinticExtension([
            GoldilocksField(0xC88E81BE7CB8BE13),
            GoldilocksField(0x253980DF689AA3B3),
            GoldilocksField(0xDB0EE9B47B609458),
            GoldilocksField(0x89937595BCF05ABF),
            GoldilocksField(0xED186AB8BCA3CFA8),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB4A0638B7F06669A),
            GoldilocksField(0xFBBF589D6FCEDCBE),
            GoldilocksField(0x473DDDC6DF96C85A),
            GoldilocksField(0x0811C5874613BCED),
            GoldilocksField(0x94C84D7D9D2209D2),
        ]),
    },
    AffinePoint {
        /* 6 */
        x: QuinticExtension([
            GoldilocksField(0x18191FA200FCF6AF),
            GoldilocksField(0x2302A5F56CDD8D5E),
            GoldilocksField(0x75D51CEE7508100A),
            GoldilocksField(0x2A7CB1BBCC4B6E82),
            GoldilocksField(0x68ED086B8EFE0952),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x238679B861A2B71E),
            GoldilocksField(0x72E5947423E98442),
            GoldilocksField(0x825E4FB253B3BEE7),
            GoldilocksField(0xD59828C79D39170D),
            GoldilocksField(0xD64BE0C2D3C4BDDF),
        ]),
    },
    AffinePoint {
        /* 7 */
        x: QuinticExtension([
            GoldilocksField(0x906BE97E5BC88259),
            GoldilocksField(0xA9D29DEB5BFA6301),
            GoldilocksField(0x5794A30DDA67C7AF),
            GoldilocksField(0x391573041251E9DA),
            GoldilocksField(0xE958FCA20E602C8E),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x2BF49C6D28B7CD14),
            GoldilocksField(0x2ED045F27AC1AC99),
            GoldilocksField(0xB7B07E480D6E4980),
            GoldilocksField(0xDB7DED6782E1AEF3),
            GoldilocksField(0x6F0C4FBE7C1C5FC0),
        ]),
    },
    AffinePoint {
        /* 8 */
        x: QuinticExtension([
            GoldilocksField(0xFC44982DA86BFA06),
            GoldilocksField(0x64099395CF0D88B6),
            GoldilocksField(0xCD1D2382A6C43A54),
            GoldilocksField(0x1BB3F627B1B99008),
            GoldilocksField(0xD0E9DDEA10328819),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xDA17D468F1DE5C01),
            GoldilocksField(0x78EF4B4D0ED0BE22),
            GoldilocksField(0xE11770286753C974),
            GoldilocksField(0x0368BF12362BF72C),
            GoldilocksField(0x404DEE08CFDDFD8A),
        ]),
    },
    AffinePoint {
        /* 9 */
        x: QuinticExtension([
            GoldilocksField(0x198E62045DB35A74),
            GoldilocksField(0xFC033F301DB930CC),
            GoldilocksField(0x628E7DBA48CDD07F),
            GoldilocksField(0xF1C019DF9777F7C9),
            GoldilocksField(0xDC4526D93675652D),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xCF5037DB2206A383),
            GoldilocksField(0x147AA2B70FFE8CA7),
            GoldilocksField(0xDEBE043528F23B85),
            GoldilocksField(0xCF27F1AC633AA891),
            GoldilocksField(0x9E1E0153D9C060A8),
        ]),
    },
    AffinePoint {
        /* 10 */
        x: QuinticExtension([
            GoldilocksField(0x98E0C0CCAB0764E4),
            GoldilocksField(0x8F97708426132A48),
            GoldilocksField(0xD5E74BDE6BEC4714),
            GoldilocksField(0x349746192FA1CB98),
            GoldilocksField(0x8F525A63D2A9223A),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x146AAB875512FE13),
            GoldilocksField(0x3DC2670A500DE2B6),
            GoldilocksField(0x1BBE14227A54E1FF),
            GoldilocksField(0x19D792E2356253AA),
            GoldilocksField(0x8882DD5B5969304D),
        ]),
    },
    AffinePoint {
        /* 11 */
        x: QuinticExtension([
            GoldilocksField(0x5D6C8679525E09F1),
            GoldilocksField(0x870A9337817CAC33),
            GoldilocksField(0xC882B79142B218C1),
            GoldilocksField(0xA9C42826B65CE8A2),
            GoldilocksField(0x477783199F3B842F),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB8A2C0757677C713),
            GoldilocksField(0x851ACDFFD11BB41A),
            GoldilocksField(0x62B97CD3A35DC135),
            GoldilocksField(0xA3B31A08DE1FBFAC),
            GoldilocksField(0xD752D76988DDD12D),
        ]),
    },
    AffinePoint {
        /* 12 */
        x: QuinticExtension([
            GoldilocksField(0xE0E90C6ED39B654C),
            GoldilocksField(0x67A39AF711058681),
            GoldilocksField(0x0D3DAEF3081904F3),
            GoldilocksField(0xD3C338BADA40006D),
            GoldilocksField(0xB33F7393564EC4D1),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x987708704EB02D95),
            GoldilocksField(0x0CC04AB0A501765E),
            GoldilocksField(0x943216BE72A6DFD7),
            GoldilocksField(0x7F51AD2A5D248721),
            GoldilocksField(0x61CC888BEBCFFB80),
        ]),
    },
    AffinePoint {
        /* 13 */
        x: QuinticExtension([
            GoldilocksField(0x4CAA65A18B975968),
            GoldilocksField(0xB495513833E42713),
            GoldilocksField(0x78C113F1379958F1),
            GoldilocksField(0x697CA2048D326DD5),
            GoldilocksField(0xE963A1404D90CB8C),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xF04BA0466834991C),
            GoldilocksField(0x7B912D4EB5F31927),
            GoldilocksField(0x99C6E4E89612F6E6),
            GoldilocksField(0xA91F4BC177C2176A),
            GoldilocksField(0x94A885B9B08B2D5F),
        ]),
    },
    AffinePoint {
        /* 14 */
        x: QuinticExtension([
            GoldilocksField(0x56F0D795C32CBE87),
            GoldilocksField(0x931F870DF82C1525),
            GoldilocksField(0xCB6ABB5DDBC8E3A5),
            GoldilocksField(0x3DBF89D464C2312F),
            GoldilocksField(0xD48BF8E7B291816E),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x276C69A01726373F),
            GoldilocksField(0x5107374917AB3272),
            GoldilocksField(0x773DD2A5CC82B6F8),
            GoldilocksField(0x991989E9601107EC),
            GoldilocksField(0x944E8035A63D7EAF),
        ]),
    },
    AffinePoint {
        /* 15 */
        x: QuinticExtension([
            GoldilocksField(0xFF673761A7450B49),
            GoldilocksField(0x4AE007FAF082E23F),
            GoldilocksField(0x3C8C3E05EEB3F6A5),
            GoldilocksField(0x35FF9C11CC2C3DED),
            GoldilocksField(0x62BC066A6E5C6720),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x372C34D7004A8EFD),
            GoldilocksField(0xEF289159A6DB7C6C),
            GoldilocksField(0x29FA45E3FA32C323),
            GoldilocksField(0xA6DE74555D6F4465),
            GoldilocksField(0xE68B116A5AA1BF61),
        ]),
    },
    AffinePoint {
        /* 16 */
        x: QuinticExtension([
            GoldilocksField(0xE9EE4BD0ECE528FC),
            GoldilocksField(0xCD31C6FDD181B556),
            GoldilocksField(0x25E8F951B5C13F88),
            GoldilocksField(0x4F1D865943228FB1),
            GoldilocksField(0xE526F967A8BDCCE8),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xD8FD6271DED8E77A),
            GoldilocksField(0x8675F320D3750CEB),
            GoldilocksField(0xBD5FE217A8A07A18),
            GoldilocksField(0xA40203C0B4FCCD19),
            GoldilocksField(0x4AE32049567A9B8B),
        ]),
    },
];

pub(crate) const G200: [AffinePoint; 16] = [
    AffinePoint {
        /* 1 */
        x: QuinticExtension([
            GoldilocksField(0xE616A89F4A4C3DDC),
            GoldilocksField(0x40965D57246A34FD),
            GoldilocksField(0x1A16EB3EFDFB7930),
            GoldilocksField(0x6371F59AB8442179),
            GoldilocksField(0x8B287EF89765E37C),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xCB9DE30898259CD1),
            GoldilocksField(0x70E50403C409DE58),
            GoldilocksField(0x175767980E9CB777),
            GoldilocksField(0x2AE405D1B885A713),
            GoldilocksField(0x46883205F6461F73),
        ]),
    },
    AffinePoint {
        /* 2 */
        x: QuinticExtension([
            GoldilocksField(0xB383FC6C08553929),
            GoldilocksField(0xBDF89C37968EF761),
            GoldilocksField(0x7D1F44EE38C66D8B),
            GoldilocksField(0xFAD5FEB55710565D),
            GoldilocksField(0x8BB6AC3E11119EC2),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x1A3FF0E69015297B),
            GoldilocksField(0x83118325E370E206),
            GoldilocksField(0x95A5C6611F30EC5F),
            GoldilocksField(0xD1B9AB1269DA3C74),
            GoldilocksField(0x75D202969B61D2FF),
        ]),
    },
    AffinePoint {
        /* 3 */
        x: QuinticExtension([
            GoldilocksField(0x371CF61D1B6BEE4F),
            GoldilocksField(0xA46896D9A0D6D340),
            GoldilocksField(0x64A4EC7ED6F98B69),
            GoldilocksField(0x27619FE45F8042C9),
            GoldilocksField(0x07B2BBA45D8B4DA0),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x1FB1647224994736),
            GoldilocksField(0x40EE9C769BD173F1),
            GoldilocksField(0x62091AE7A020F159),
            GoldilocksField(0x11F4F4AC42AAF6DB),
            GoldilocksField(0x3935E9FAF93174AB),
        ]),
    },
    AffinePoint {
        /* 4 */
        x: QuinticExtension([
            GoldilocksField(0xF0C24C0829D03884),
            GoldilocksField(0x77E44895D7601409),
            GoldilocksField(0xDB85941EDF473DFA),
            GoldilocksField(0xF91CDACADB1C6EA9),
            GoldilocksField(0xD3D067DC0ECCD4C0),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x46DFFBED29FBD34F),
            GoldilocksField(0x3794971420F3F342),
            GoldilocksField(0xBB34444ABECC2B7A),
            GoldilocksField(0x13FAB655F99C4328),
            GoldilocksField(0x287805E5B39C7A54),
        ]),
    },
    AffinePoint {
        /* 5 */
        x: QuinticExtension([
            GoldilocksField(0x600FE70A8B05B252),
            GoldilocksField(0x42D935D34D536CFF),
            GoldilocksField(0xC72035E7C95B3354),
            GoldilocksField(0xE57CC01BD27B29F6),
            GoldilocksField(0x732714762A107AA3),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x3144661D70681FA1),
            GoldilocksField(0x51E1542CFF2C0CE8),
            GoldilocksField(0xDC665EB7401E02BC),
            GoldilocksField(0x3E5EBA71BD571963),
            GoldilocksField(0x09B22663205E1FBA),
        ]),
    },
    AffinePoint {
        /* 6 */
        x: QuinticExtension([
            GoldilocksField(0xE163D75C513E0CCC),
            GoldilocksField(0x5518CABC28DDC4F7),
            GoldilocksField(0xE9961D5370BC5B65),
            GoldilocksField(0xEF742BAE495AC058),
            GoldilocksField(0x9B395F6A4F9B6A26),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x3BA66FFA8EAD89EB),
            GoldilocksField(0x63C5E48A245D7AA8),
            GoldilocksField(0x82B6ACEFEA7E4D76),
            GoldilocksField(0xC9A9359D54A4DD0F),
            GoldilocksField(0x768ACA943DDECEEC),
        ]),
    },
    AffinePoint {
        /* 7 */
        x: QuinticExtension([
            GoldilocksField(0xD661DAE6D3DD9F89),
            GoldilocksField(0x4C710B466E774311),
            GoldilocksField(0xE4C95E255E5683A8),
            GoldilocksField(0xB78137C990E22F80),
            GoldilocksField(0x2BDA022583DAFB0A),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x4838E9014DEA3D70),
            GoldilocksField(0xBAE8C676F3AFCA06),
            GoldilocksField(0xEE93BE162C1AF743),
            GoldilocksField(0xFD3883EA55C9ACF9),
            GoldilocksField(0xD4B0C1DAF01C9D23),
        ]),
    },
    AffinePoint {
        /* 8 */
        x: QuinticExtension([
            GoldilocksField(0x93657B3895954257),
            GoldilocksField(0xEDD022FFBE81D10E),
            GoldilocksField(0x8142C0945C90101C),
            GoldilocksField(0x82033657EB7353F8),
            GoldilocksField(0x76BC6646BB3716A6),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x6A5E0A4DB04ED6F9),
            GoldilocksField(0xE1429E1F0979A5DD),
            GoldilocksField(0x5B747016E1ED1833),
            GoldilocksField(0xCAE934C07525C37F),
            GoldilocksField(0x5E3424ADE845E49B),
        ]),
    },
    AffinePoint {
        /* 9 */
        x: QuinticExtension([
            GoldilocksField(0x09FF77563A30B446),
            GoldilocksField(0x07BF53470594DE2A),
            GoldilocksField(0x4CE72C200F8D6E9B),
            GoldilocksField(0xE52273BC7ED7CA35),
            GoldilocksField(0x9033849A19000AF4),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB933301371FF9291),
            GoldilocksField(0xBB910D3069AE5106),
            GoldilocksField(0x5E0D307FE9F0F779),
            GoldilocksField(0xCD9A0EB95DDDCEF1),
            GoldilocksField(0x19178A0F43AE2C1E),
        ]),
    },
    AffinePoint {
        /* 10 */
        x: QuinticExtension([
            GoldilocksField(0x7623C4BE201756E1),
            GoldilocksField(0xBD2F0633AEC30A94),
            GoldilocksField(0xD337C303193BB7AE),
            GoldilocksField(0x572CFBFED1A8ACF2),
            GoldilocksField(0xB01C34B6EA7E1EF3),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xD380C1FC9D68FF85),
            GoldilocksField(0xBBB8B9F9CF5182A2),
            GoldilocksField(0x0557D0E40BBA2FDF),
            GoldilocksField(0xECB278AD146404F7),
            GoldilocksField(0x87FFD86E47AF23AA),
        ]),
    },
    AffinePoint {
        /* 11 */
        x: QuinticExtension([
            GoldilocksField(0xE40EDFE66BA15A89),
            GoldilocksField(0x0E9E3E2F33526D04),
            GoldilocksField(0x4BA5C6033A38B887),
            GoldilocksField(0x3842C01D32AB77CC),
            GoldilocksField(0x28E5DE9E77245D22),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xF8CFA6BB96DD0801),
            GoldilocksField(0xF133E8AD176BCA29),
            GoldilocksField(0x9C1EAB583453E8F8),
            GoldilocksField(0xBEC6ADFB1A8D8E97),
            GoldilocksField(0xAE7635A4AF1C877C),
        ]),
    },
    AffinePoint {
        /* 12 */
        x: QuinticExtension([
            GoldilocksField(0xFF384ED70E2E92B1),
            GoldilocksField(0xC1B07C818FE7707F),
            GoldilocksField(0xCEB40739BD2E6AA8),
            GoldilocksField(0x4A03A62921D5C51C),
            GoldilocksField(0x8BD601A17D2F7CB4),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x727EF2E2653167B4),
            GoldilocksField(0x4FD64C260F9A4FD8),
            GoldilocksField(0x3297687634294C9C),
            GoldilocksField(0xB307544D393DC724),
            GoldilocksField(0xEA542C84CB669681),
        ]),
    },
    AffinePoint {
        /* 13 */
        x: QuinticExtension([
            GoldilocksField(0x156FC875BDCDFEBA),
            GoldilocksField(0x092FCFF076042FFE),
            GoldilocksField(0xFFAFCA9A76A95EB9),
            GoldilocksField(0x2D0C797173C306F9),
            GoldilocksField(0x485198EF86718F90),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x3DE83246EBA16461),
            GoldilocksField(0x2FE484942CB74FA7),
            GoldilocksField(0xEE8D2538886D8B97),
            GoldilocksField(0xE7B3CFE2D407E1A4),
            GoldilocksField(0x09E243332ADB265D),
        ]),
    },
    AffinePoint {
        /* 14 */
        x: QuinticExtension([
            GoldilocksField(0x57586A9E04A0DD6E),
            GoldilocksField(0x0EB1CCC8CD18F018),
            GoldilocksField(0x4A1D82A3B69C217D),
            GoldilocksField(0x4635FB578FC490AF),
            GoldilocksField(0x30F103C0D76E17B9),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x9AD7880E8116B808),
            GoldilocksField(0xB4388460DC70697C),
            GoldilocksField(0x3135215930D6569F),
            GoldilocksField(0xA38FB7775B6F9168),
            GoldilocksField(0x8C27888E47141129),
        ]),
    },
    AffinePoint {
        /* 15 */
        x: QuinticExtension([
            GoldilocksField(0xA36967535666430F),
            GoldilocksField(0x0B10F73BE3D3B04B),
            GoldilocksField(0x9C90A44FAD53D2D1),
            GoldilocksField(0xD1542D9374A67FEF),
            GoldilocksField(0x23F683062E9E65EB),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x98C6FBB81DC121C1),
            GoldilocksField(0x4C2C310848E48D1B),
            GoldilocksField(0xCF8082BCB5E7D69A),
            GoldilocksField(0x9B495103371A673C),
            GoldilocksField(0x4A5E8A2E23C4D340),
        ]),
    },
    AffinePoint {
        /* 16 */
        x: QuinticExtension([
            GoldilocksField(0x6AC4419C7277D1CA),
            GoldilocksField(0x2C5C1F5E44B298D7),
            GoldilocksField(0x18C662C232D514DD),
            GoldilocksField(0x44D950568161E181),
            GoldilocksField(0xE714CE4B73A7B072),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xA81D41AAAF4FA6CF),
            GoldilocksField(0x6E1DE35FDAD17243),
            GoldilocksField(0xC98288B163F43776),
            GoldilocksField(0x130AB99648D7BD44),
            GoldilocksField(0x9C417A9BAFAB5466),
        ]),
    },
];

pub(crate) const G240: [AffinePoint; 16] = [
    AffinePoint {
        /* 1 */
        x: QuinticExtension([
            GoldilocksField(0xA8A0EE7803A9262A),
            GoldilocksField(0x3A9E64856DDF4568),
            GoldilocksField(0xA8D15981C6B5E366),
            GoldilocksField(0x8C50DFC96818C3F6),
            GoldilocksField(0x67747DA91EA27AFA),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x721E3407EBD5F510),
            GoldilocksField(0xCD45A5FB06B5D837),
            GoldilocksField(0x5ADF593FEF90259D),
            GoldilocksField(0xC3B0FFB9AA1C972A),
            GoldilocksField(0x86EF37AC9F5CF331),
        ]),
    },
    AffinePoint {
        /* 2 */
        x: QuinticExtension([
            GoldilocksField(0x200E8B5C9C37E3AF),
            GoldilocksField(0x921C4CD1D43B733E),
            GoldilocksField(0x285F3CFA7B98BDEB),
            GoldilocksField(0xC43D0057A09411A4),
            GoldilocksField(0xCABD6E7DC2D3243A),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xB8C8962CB51C8455),
            GoldilocksField(0x27E98000626D107C),
            GoldilocksField(0x8D9E90EEA287F110),
            GoldilocksField(0x2315D937D3B14704),
            GoldilocksField(0xD855ED171240A04F),
        ]),
    },
    AffinePoint {
        /* 3 */
        x: QuinticExtension([
            GoldilocksField(0x8EBDF4139F2AC3BC),
            GoldilocksField(0x5EBD2FDBAE6EE6DC),
            GoldilocksField(0x329010A7F5F44D0B),
            GoldilocksField(0x30EB8FFFB8A0547E),
            GoldilocksField(0x8094D0CD6D63E13F),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xFFC1C1D1EDB1FD4D),
            GoldilocksField(0xE2B3A119FC8E2D33),
            GoldilocksField(0x140D049877E3268B),
            GoldilocksField(0x0763801996111CC8),
            GoldilocksField(0xE55EB94944D8FD3F),
        ]),
    },
    AffinePoint {
        /* 4 */
        x: QuinticExtension([
            GoldilocksField(0x4F2A667724E34E0C),
            GoldilocksField(0x66C0D5F79375414F),
            GoldilocksField(0x91398D556C18DE95),
            GoldilocksField(0x56B37C05A0068FDA),
            GoldilocksField(0x46FD0DA0D321A8D0),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xF3F7F65AADC697EE),
            GoldilocksField(0x2498FBE550D9998D),
            GoldilocksField(0x42D957D8D25DCC73),
            GoldilocksField(0x0371646CB0DB7F11),
            GoldilocksField(0x3EE41B5B478EFE53),
        ]),
    },
    AffinePoint {
        /* 5 */
        x: QuinticExtension([
            GoldilocksField(0x3DFFE1FB511B6C11),
            GoldilocksField(0x07BFD5AEE108720C),
            GoldilocksField(0x4140011508479DA9),
            GoldilocksField(0xE725984D11458168),
            GoldilocksField(0xCFD5C8AEBA6BEB4F),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x3F0B1B59FCFEB6F7),
            GoldilocksField(0xF78EECBE0E1A6029),
            GoldilocksField(0x358349B2F8B945C6),
            GoldilocksField(0x33490E5BBC1FD56A),
            GoldilocksField(0x932EE56AFE959D14),
        ]),
    },
    AffinePoint {
        /* 6 */
        x: QuinticExtension([
            GoldilocksField(0x853D24B74B77BE2D),
            GoldilocksField(0x28D86D666FF156DC),
            GoldilocksField(0x6824E07BDFD14641),
            GoldilocksField(0x3236F2172775AFA1),
            GoldilocksField(0x6EFB083EFC900F5B),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x11C1CA68D2B4998E),
            GoldilocksField(0x4206817DB4E66EB6),
            GoldilocksField(0xAC5EA5717A533864),
            GoldilocksField(0xECB4C2DEE8BA26A1),
            GoldilocksField(0xDEEB73F656D8AD27),
        ]),
    },
    AffinePoint {
        /* 7 */
        x: QuinticExtension([
            GoldilocksField(0x0089D625D4A275FE),
            GoldilocksField(0x4AEE1C1C159E64A8),
            GoldilocksField(0xCF02452BB6ABE083),
            GoldilocksField(0x097802619A3CD9E8),
            GoldilocksField(0xFC53D6AEDE0E9FE9),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x435279229BD3625C),
            GoldilocksField(0x1889895FE07C89E2),
            GoldilocksField(0xE056DDC0F402CE97),
            GoldilocksField(0xE804E68342F959D9),
            GoldilocksField(0x2F028095D09C7904),
        ]),
    },
    AffinePoint {
        /* 8 */
        x: QuinticExtension([
            GoldilocksField(0x1CD80828E1157548),
            GoldilocksField(0x7AB3312B81CBD770),
            GoldilocksField(0xA191B99E0F818F66),
            GoldilocksField(0x36FFEE5A4112C1F2),
            GoldilocksField(0xAD3F8AC8AD921E49),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xBF4F161F2614DCFB),
            GoldilocksField(0x7587B98E8A95AAF7),
            GoldilocksField(0x2C0EC2B7F5F7CBB8),
            GoldilocksField(0x95E505D4A14AF60A),
            GoldilocksField(0xB45D26196646C9AF),
        ]),
    },
    AffinePoint {
        /* 9 */
        x: QuinticExtension([
            GoldilocksField(0x8A9CE262C5329443),
            GoldilocksField(0x07A3B8C865423F4F),
            GoldilocksField(0xE018203AA3102625),
            GoldilocksField(0x9F596F7B6817C8C1),
            GoldilocksField(0xFCB769579909678B),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xEFD12FC70B32AAB2),
            GoldilocksField(0x47630ED573839DAE),
            GoldilocksField(0x8311D5C3758990F4),
            GoldilocksField(0x20A8765C516F9BCF),
            GoldilocksField(0x679A6D91FF5DD6D2),
        ]),
    },
    AffinePoint {
        /* 10 */
        x: QuinticExtension([
            GoldilocksField(0x5AF23895A8573E74),
            GoldilocksField(0xF6AC9DA9076A8710),
            GoldilocksField(0x7162587AE722EF71),
            GoldilocksField(0xE4B3BCCE6873A42D),
            GoldilocksField(0x7DA1F5CD972D59C1),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x461B13504D813650),
            GoldilocksField(0x70FF153C0B3881BE),
            GoldilocksField(0x5BE064C9652C1CF9),
            GoldilocksField(0xD64C780542D5C6D8),
            GoldilocksField(0xF783C2AD3E4E50B8),
        ]),
    },
    AffinePoint {
        /* 11 */
        x: QuinticExtension([
            GoldilocksField(0xB2AC7989C02502E9),
            GoldilocksField(0x46F0A5085FD69389),
            GoldilocksField(0x83AD6BE496C248D3),
            GoldilocksField(0x1FE71326A8F0A166),
            GoldilocksField(0xD75068C5D239ED41),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x89DE70D73D84658D),
            GoldilocksField(0x437C5714FE4F8B2C),
            GoldilocksField(0x63CD50470844C646),
            GoldilocksField(0xAEB39B4DC0637ED3),
            GoldilocksField(0xE96CE8D06024A136),
        ]),
    },
    AffinePoint {
        /* 12 */
        x: QuinticExtension([
            GoldilocksField(0x4C6B70FEF9176C74),
            GoldilocksField(0x6DD1A052CEB84B64),
            GoldilocksField(0xEC6B6236BC4CDB96),
            GoldilocksField(0xA995D28DA35CCBBA),
            GoldilocksField(0x413DA1C9356B35BB),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xD8E20D8054E422DD),
            GoldilocksField(0x8A5C4188E9409B51),
            GoldilocksField(0x3798827D3AAE1108),
            GoldilocksField(0x9E75FFAC84DABFEA),
            GoldilocksField(0x8455308253067827),
        ]),
    },
    AffinePoint {
        /* 13 */
        x: QuinticExtension([
            GoldilocksField(0x092BE4D5B779CE6E),
            GoldilocksField(0x205D65D92C7BBE5A),
            GoldilocksField(0x4410382623DCC479),
            GoldilocksField(0xF426801189B81EEB),
            GoldilocksField(0x18A49AEDBAD87946),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x72E31C8A73E2A5F2),
            GoldilocksField(0x038D4FCB2D0D45F9),
            GoldilocksField(0xFACE7BBC088C9465),
            GoldilocksField(0x4D8A29C235CE48DF),
            GoldilocksField(0x6DDE986C6F3DE573),
        ]),
    },
    AffinePoint {
        /* 14 */
        x: QuinticExtension([
            GoldilocksField(0xCE8281832197F5E7),
            GoldilocksField(0x02DF944FE891E27E),
            GoldilocksField(0x7C32B2AC7CA4DF96),
            GoldilocksField(0xDDDE4642C673E511),
            GoldilocksField(0x910884A15C6E9E90),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x927B31CAA1595D30),
            GoldilocksField(0x9AB94754B9B1C1C8),
            GoldilocksField(0xB0FFE7EF45AD9608),
            GoldilocksField(0x5617F7967DC07DD7),
            GoldilocksField(0x748B52FC2F538F06),
        ]),
    },
    AffinePoint {
        /* 15 */
        x: QuinticExtension([
            GoldilocksField(0x85987CE43845212C),
            GoldilocksField(0x0E13745D09A067D9),
            GoldilocksField(0xAE4D3C09C5D3B59C),
            GoldilocksField(0x17DCCE2F3BC41E08),
            GoldilocksField(0xA8DD0B19FE8DF324),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x8B52DBAC4316DF3C),
            GoldilocksField(0xE6DD1B0B096BA49B),
            GoldilocksField(0x45B50205A85D4930),
            GoldilocksField(0xBCA014CCF49A96CF),
            GoldilocksField(0x43EF77C96A70C64D),
        ]),
    },
    AffinePoint {
        /* 16 */
        x: QuinticExtension([
            GoldilocksField(0xD253F638008F64D9),
            GoldilocksField(0x300911F4CF750079),
            GoldilocksField(0xEBA4C0CCF84421F1),
            GoldilocksField(0xD9A8EC0A382CC42A),
            GoldilocksField(0xB2B4D66CAC8FDA35),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x8C725DFAE21259FB),
            GoldilocksField(0xA93424C7A5FC6938),
            GoldilocksField(0x97CE72FE5A59C0EF),
            GoldilocksField(0xA8EE2E81BCA93CBC),
            GoldilocksField(0x3B54AD70A6670624),
        ]),
    },
];

pub(crate) const G280: [AffinePoint; 16] = [
    AffinePoint {
        /* 1 */
        x: QuinticExtension([
            GoldilocksField(0xE101EFE0DD3D126E),
            GoldilocksField(0xEE415579E7763701),
            GoldilocksField(0x481DBDD5A30063D0),
            GoldilocksField(0x9A4A5A2CE9DC78D0),
            GoldilocksField(0x70DFF28028661446),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x9DE418BD4B624FA5),
            GoldilocksField(0x3E080A3E987B0F4D),
            GoldilocksField(0x7D171E8A81DAF782),
            GoldilocksField(0x13B661FEF2D14550),
            GoldilocksField(0xE193CBFC0EC5427A),
        ]),
    },
    AffinePoint {
        /* 2 */
        x: QuinticExtension([
            GoldilocksField(0x5DF5EC1ABF7F91F1),
            GoldilocksField(0xED85BEA6EACB5E1C),
            GoldilocksField(0x571DEA21506D257F),
            GoldilocksField(0x81FECBEFEE649A35),
            GoldilocksField(0x60C9A0D1FA6944FB),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x17B12A9CD0DA93BD),
            GoldilocksField(0xE34F65A1C5FE95E5),
            GoldilocksField(0x85402906EAB91693),
            GoldilocksField(0xFF1BF5F49AE493A4),
            GoldilocksField(0x8839E8685FAE3E48),
        ]),
    },
    AffinePoint {
        /* 3 */
        x: QuinticExtension([
            GoldilocksField(0x80FBEEF7CD2E13C8),
            GoldilocksField(0xEDBC56CCE4B4BDB9),
            GoldilocksField(0xF6D0C9C57A7D8082),
            GoldilocksField(0x6DAF1CFC656E1455),
            GoldilocksField(0xBA7E4182B917F1FD),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xCE0EFDA5F6B68E02),
            GoldilocksField(0x35CCA934993EDDEA),
            GoldilocksField(0xF41B71BDB24254F1),
            GoldilocksField(0xB7F6C4594149EE3D),
            GoldilocksField(0xE9A7895F548783E2),
        ]),
    },
    AffinePoint {
        /* 4 */
        x: QuinticExtension([
            GoldilocksField(0x59FE2AB81ABB02AE),
            GoldilocksField(0xCBADFCCB2F04B57A),
            GoldilocksField(0x0D1969F23952938D),
            GoldilocksField(0x831C8C1258CAE189),
            GoldilocksField(0xEB843136EA9A94FC),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x608228855343E93E),
            GoldilocksField(0x78091B902F3A0E56),
            GoldilocksField(0x27E3EC97B5F1D957),
            GoldilocksField(0x5E3FCAA84D86532D),
            GoldilocksField(0x40FD2108182D5785),
        ]),
    },
    AffinePoint {
        /* 5 */
        x: QuinticExtension([
            GoldilocksField(0xF7BEB6DBF433A526),
            GoldilocksField(0x0B2A53C27E29826C),
            GoldilocksField(0x88809F3B7ADFE4D6),
            GoldilocksField(0x269D7073C55388BA),
            GoldilocksField(0x99104832D8D4C84C),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xF10D0D94CD8A08E4),
            GoldilocksField(0x9FE8F86BF58A0740),
            GoldilocksField(0x922B67E3F366C40B),
            GoldilocksField(0xD15574F1FD92B0CF),
            GoldilocksField(0xAB3245EEA77D39A7),
        ]),
    },
    AffinePoint {
        /* 6 */
        x: QuinticExtension([
            GoldilocksField(0xF8CC69057F7C5225),
            GoldilocksField(0x0DB771295EA9B3F1),
            GoldilocksField(0xEDC9814F24289219),
            GoldilocksField(0x33FB7A518409DA5A),
            GoldilocksField(0xA7A4B6DEDF49F9B2),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xE82170DA6BE518E0),
            GoldilocksField(0xE7B7459B42536CEB),
            GoldilocksField(0x188951DE0336CD0E),
            GoldilocksField(0x47EC99CA8DFD40B5),
            GoldilocksField(0xB319A211D2EF9288),
        ]),
    },
    AffinePoint {
        /* 7 */
        x: QuinticExtension([
            GoldilocksField(0x48B43DB63C7204A7),
            GoldilocksField(0x69EC0CFEBE4F89D8),
            GoldilocksField(0xB700D394422584B0),
            GoldilocksField(0x4A91AFFADC8C6C71),
            GoldilocksField(0x42345336177B3C4C),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x0C92DF3450E3302A),
            GoldilocksField(0x230917E124B9227E),
            GoldilocksField(0x1846F380DFF4F6EC),
            GoldilocksField(0x7498858A1257454F),
            GoldilocksField(0xBFA3A9AF58CB8E48),
        ]),
    },
    AffinePoint {
        /* 8 */
        x: QuinticExtension([
            GoldilocksField(0x300FE3B1CBD6A543),
            GoldilocksField(0x82B53FA07658D890),
            GoldilocksField(0xE704A58CE42E61E5),
            GoldilocksField(0x6FB145C2E7EB6299),
            GoldilocksField(0x378DEF33925DEB86),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x07DAD4739750D586),
            GoldilocksField(0x4E42EDC099CEE5A1),
            GoldilocksField(0x30A64BA70DF477FC),
            GoldilocksField(0x0B7E53D8154398E7),
            GoldilocksField(0x27A84D1C4843D524),
        ]),
    },
    AffinePoint {
        /* 9 */
        x: QuinticExtension([
            GoldilocksField(0x92D2D5B7EF4EAB2A),
            GoldilocksField(0x13CA27355FD36907),
            GoldilocksField(0x3D9538F8C489F69F),
            GoldilocksField(0x7FC99FC81C60EC16),
            GoldilocksField(0x70E05CE274B2DDA7),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x4DD5F5E8AA5F8B6C),
            GoldilocksField(0xCB929B26407014DC),
            GoldilocksField(0x30F1214C5E00B284),
            GoldilocksField(0xE0BB4238490F1785),
            GoldilocksField(0xCD979B08C903ACCE),
        ]),
    },
    AffinePoint {
        /* 10 */
        x: QuinticExtension([
            GoldilocksField(0x5A7572DBFD2428DD),
            GoldilocksField(0x8E7B9FAFCDFBED39),
            GoldilocksField(0x62A60250AB74DDF1),
            GoldilocksField(0xCB2E17DA9E0BE40F),
            GoldilocksField(0x5BF6B50F358AD87F),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xC93002FECFBA924F),
            GoldilocksField(0x20BDDD4FA0C3967D),
            GoldilocksField(0x7AED8F011DA76FE7),
            GoldilocksField(0xEDCAC7D07BFCD27D),
            GoldilocksField(0x2926364889CE6286),
        ]),
    },
    AffinePoint {
        /* 11 */
        x: QuinticExtension([
            GoldilocksField(0x2CE1D8A8F8C936C9),
            GoldilocksField(0x2CC208D366E2AC94),
            GoldilocksField(0x6257C9ED583D473D),
            GoldilocksField(0xB2B56863B06A3EDA),
            GoldilocksField(0xA07329D48DB98857),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x27258DD37498FC8E),
            GoldilocksField(0xE1E1DCDA4FF7613D),
            GoldilocksField(0x755F5BF95A6AB233),
            GoldilocksField(0x032742AC5790144F),
            GoldilocksField(0x815E83A0EAAF5B08),
        ]),
    },
    AffinePoint {
        /* 12 */
        x: QuinticExtension([
            GoldilocksField(0x805FA56682DC7521),
            GoldilocksField(0x1884FA5CFFB9706B),
            GoldilocksField(0xBAD790C71F9FAA62),
            GoldilocksField(0xB96900BBA4D86E22),
            GoldilocksField(0x6F42C94A933A1D71),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x1F4BB93538BC8AD3),
            GoldilocksField(0x84225EC5AEFE8090),
            GoldilocksField(0x25C275174590A594),
            GoldilocksField(0x9FEEA6C1E2A5F099),
            GoldilocksField(0x19B8897561179347),
        ]),
    },
    AffinePoint {
        /* 13 */
        x: QuinticExtension([
            GoldilocksField(0x2955E592EB1EFE5A),
            GoldilocksField(0x7A282B0798A33C54),
            GoldilocksField(0x74BA891F8A832B8B),
            GoldilocksField(0x7C6D6A557F1625FD),
            GoldilocksField(0x019718B0DB5A0FB0),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x8E71B3C48DF30D11),
            GoldilocksField(0x82B0B16111166A18),
            GoldilocksField(0x5DA9B5BC441B0F5C),
            GoldilocksField(0x52C893875D4FC42A),
            GoldilocksField(0x4A94ADFBBCAB9093),
        ]),
    },
    AffinePoint {
        /* 14 */
        x: QuinticExtension([
            GoldilocksField(0x9C61B93A717AC45D),
            GoldilocksField(0x2CBCA52F2D36F2B3),
            GoldilocksField(0x4289E63D03B28D39),
            GoldilocksField(0x82919F9E399E7D80),
            GoldilocksField(0x8633A31B3179F23B),
        ]),
        u: QuinticExtension([
            GoldilocksField(0x071F2E41D3CC3E52),
            GoldilocksField(0x5CEF76B14B6F9011),
            GoldilocksField(0xD12158D2928C481B),
            GoldilocksField(0x76020B56EB95AFBE),
            GoldilocksField(0x81F94088FBD89C53),
        ]),
    },
    AffinePoint {
        /* 15 */
        x: QuinticExtension([
            GoldilocksField(0x7A657D2637EBB4E5),
            GoldilocksField(0x5D2ED7179B27BF7A),
            GoldilocksField(0xA7C592D6837D8E67),
            GoldilocksField(0xEDAEB459581E2034),
            GoldilocksField(0xD89552C3F92ACA28),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xFFA6C86C78A12912),
            GoldilocksField(0xCE9BD2556518E6AB),
            GoldilocksField(0xE8E07519D7635B1A),
            GoldilocksField(0x962EDA250276ACF0),
            GoldilocksField(0x5C941DC8238F8EED),
        ]),
    },
    AffinePoint {
        /* 16 */
        x: QuinticExtension([
            GoldilocksField(0x33E179A5F02EA96F),
            GoldilocksField(0x35486A003E0C192E),
            GoldilocksField(0x8A17FDB3A32F31A9),
            GoldilocksField(0xBCB9C322FE0B8DA8),
            GoldilocksField(0x7162143807AA558C),
        ]),
        u: QuinticExtension([
            GoldilocksField(0xF1848305499B4272),
            GoldilocksField(0xAA60A5F08C4015F2),
            GoldilocksField(0x01DFEB78A828F5BC),
            GoldilocksField(0x5E8F88CD8A52F258),
            GoldilocksField(0xDC499E276A245215),
        ]),
    },
];

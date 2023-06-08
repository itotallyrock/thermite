use crate::castles::CastleDirection;
use crate::pieces::{PieceType, PlacedPiece};
use crate::player_color::PlayerColor;
use crate::square::{EnPassantSquare, Square};
use core::hash::Hasher;
use derive_more::AsRef;
use enum_map::EnumMap;

/// The truncated Zobrist hash for a specific position to save memory when collisions are less likely
#[derive(Copy, Clone, Eq, PartialEq, Debug, AsRef)]
pub struct HistoryHash(u8);

/// The Zobrist hash for a specific position
#[derive(Copy, Clone, Eq, PartialEq, Debug, AsRef)]
pub struct ZobristHash(u64);

impl Hasher for ZobristHash {
    fn finish(&self) -> u64 {
        *self.as_ref()
    }

    fn write(&mut self, _: &[u8]) {
        unimplemented!()
    }

    fn write_u64(&mut self, i: u64) {
        *self = Self(*self.as_ref() ^ i);
    }
}

impl ZobristHash {
    fn toggle(&mut self, mask: u64) {
        self.write_u64(mask);
    }

    /// Toggle the placement of a piece on a given square for a given side.
    /// Adds the piece placement to the hash; otherwise, removes the piece if it is already included.
    pub fn toggle_piece_square(&mut self, placed_piece: PlacedPiece) {
        self.toggle(
            PIECE_SQUARES[placed_piece.owned_piece.player][placed_piece.owned_piece.piece]
                [placed_piece.square],
        );
    }

    /// Toggle the side to move between white and black.
    pub fn switch_sides(&mut self) {
        self.toggle(SIDE_KEY);
    }

    /// Toggle the side to move between white and black.
    pub fn toggle_en_passant_square(&mut self, en_passant_square: EnPassantSquare) {
        self.toggle(EN_PASSANT_KEYS[en_passant_square]);
    }

    /// Toggle the castle rights for a side to castle in one direction
    pub fn toggle_castle_ability(
        &mut self,
        player: PlayerColor,
        castle_direction: CastleDirection,
    ) {
        self.toggle(CASTLE_KEYS[castle_direction][player]);
    }
}

impl Default for ZobristHash {
    fn default() -> Self {
        Self(EMPTY_ZOBRIST_KEY)
    }
}

impl From<ZobristHash> for HistoryHash {
    fn from(value: ZobristHash) -> Self {
        // ALLOW: Intentional truncation for a smaller memory footprint with still enough bits to avoid a hash collision
        #[allow(clippy::cast_possible_truncation)]
        Self(*value.as_ref() as u8)
    }
}

impl PartialEq<HistoryHash> for ZobristHash {
    fn eq(&self, other: &HistoryHash) -> bool {
        HistoryHash::from(*self) == *other
    }
}

/// The base key for an empty position
const EMPTY_ZOBRIST_KEY: u64 = 0xF1DC_4349_4EA4_76CE;

/// Hash value used for black to move
const SIDE_KEY: u64 = 0xA92C_CEB8_91EA_45C2;

/// Hash value for all piece square possibilities (including some illegal positions like pawns on last rank)
#[rustfmt::skip]
const PIECE_SQUARES: EnumMap<PlayerColor, EnumMap<PieceType, EnumMap<Square, u64>>> = EnumMap::from_array([
    EnumMap::from_array([
        EnumMap::from_array([
            0xBCBD_2C2F_7DAB_FCBE, 0x8756_17FC_113F_9090, 0x314A_7DFE_25D7_39E3, 0x47F5_6D36_49FE_FA55, 0x2276_E9C2_6AD3_4276, 0x776F_E868_69DA_CEAD, 0x4CDA_34E6_051B_A0AC, 0x2580_0E89_C066_3865,
            0x5634_EEDA_8F6E_658B, 0x6947_845B_FD63_D7F7, 0xE85B_94FA_9808_12E5, 0x1F8D_1CF9_944F_D778, 0x63E8_2D59_AE66_8F3A, 0xFF82_9419_1FD1_2797, 0xD3AF_0509_C345_130E, 0x9018_4984_1E22_3B79,
            0x721D_2AE5_51AC_50E0, 0xE1D0_FC8B_9911_46B4, 0x578F_1785_DA5A_5360, 0x67E7_3D3A_61B0_801A, 0xA398_5019_2806_8508, 0x712B_1876_FFE5_8BEE, 0x8B87_8C1E_9300_3BC1, 0xA7C3_0663_60F2_08F7,
            0x227A_1A10_0D31_A3FF, 0x6861_CFA9_7E22_4B21, 0x08FA_31F0_3AC0_630D,  0x08F9_9842_E1A4_310F,  0x0760_639E_8B26_9F8D,  0x5B5A_4456_033E_62F5, 0x1045_92EA_1780_E7E9, 0x58E7_6D95_AD43_433E,
            0x989B_E44C_C2AB_1362, 0x5EA5_B4AB_668B_1AF1, 0x877B_9718_C138_9801, 0xB94D_BA7F_5E20_B729, 0xEECC_6ABC_ACD8_3CF3, 0x8888_14C0_CE90_BFA7, 0x5A46_EB53_DB79_AACD, 0xB53B_A4CB_C4B9_F2D2,
            0x903B_33F6_FA0B_BFE2, 0xB0EF_F01B_C78F_12D3, 0x75A0_158C_798B_7238, 0x085B_E5E2_2FFB_65A5,  0x83D7_FCBE_AAC0_9CC3, 0x1AAA_5FFE_8B57_47C3, 0x0CC0_2DCC_EB25_3F09,  0x97BD_CB46_41D4_3291,
            0xB6CC_2E1A_5F3B_CE32, 0x9FBF_DCE6_881F_DB26, 0x1F64_C684_79CC_21AD, 0xFAB6_AA78_32F9_5158, 0xBBCF_3EEA_8911_CC7E, 0x73D7_6DBD_6029_1258, 0x32A8_5E23_3EF8_57AE, 0x9D72_C739_0502_D7AC,
            0x6F17_39C3_4B67_B344, 0xDF96_9D84_A61A_57F1, 0x6C67_27DF_AE59_6F6F, 0x33B1_C277_5CDB_F746, 0x8F2D_FF56_5239_F7E9, 0x8694_066F_A7C5_7A33, 0x6E49_1ABA_0ADD_400E, 0x23BD_B00C_D3F3_6FF4,
        ]),
        EnumMap::from_array([
            0x1514_F499_4C36_AFCA, 0xF17B_6E98_6095_EF04, 0xCC25_705F_DD21_9642, 0x6223_1637_C9B0_14FD, 0x17FC_E4F9_A285_980B, 0x2C44_A0AB_2B3D_C6BC, 0x9C52_43E0_5375_A93F, 0xD28A_1875_5B0B_45C9,
            0x376E_7CB8_F521_0466, 0xEF78_22C5_C5C5_7251, 0x30EB_A2ED_5197_F1E0, 0xE9DA_2A85_1B3A_FD17, 0xBBF1_90E0_8C18_A139, 0x67E2_30BE_30A1_E31A, 0x86B6_7A41_102B_53BD, 0x0240_F02B_0206_484A,
            0xD51E_93EF_E131_50CC, 0x46D9_3E2E_9A4D_67B3, 0x7D2C_9F1D_5CB9_7808, 0x7E5D_DCDD_0C6D_1E81, 0x875D_8C04_BB95_FE81, 0x59E9_69E5_8561_F1D4, 0x6A49_C35B_DAF3_7FFB, 0xE725_C9DE_0D3D_C949,
            0x3FDF_4C33_F73C_58BF, 0x4463_7662_1908_01ED, 0x0F62_8079_F241_C485,  0x05C1_38BA_6622_1CB0,  0xF4DF_4141_9A33_F4D9, 0x6C1E_71E6_D318_6688, 0x2AE8_A5DB_D2D7_E72E, 0x07B1_B4F0_5DA7_AC4F,
            0x54B0_4DEA_70B8_3260, 0x4A14_E486_24E4_B1D3, 0x238E_20FC_A906_3B3B, 0x69C7_BE0C_FBD9_A4EB, 0x5BEB_3B9C_A51D_BA3F, 0xDC3D_B15E_515C_A7B8, 0x90F2_5265_8F1C_998D, 0x487D_1ADB_67D7_765B,
            0x8228_6F7B_C453_DABC, 0x908C_F828_C482_7A85, 0xE2B0_FD2E_530F_F5F4, 0xAB58_4F2B_9EC9_1B1B, 0xB646_2F20_FE42_A8C1, 0x4CF8_AB64_5B1E_BC8D, 0xE941_0F0E_A35F_01F8, 0xA5C1_8F5A_3416_B6B7,
            0x1AF2_394C_1E31_8DF4, 0x1B72_BFCD_43F5_0672, 0x7EE0_BDAA_8EB1_D943, 0x4694_6AC1_F54E_8A3D, 0x2999_BE75_4AEC_2106, 0x9169_DAD7_727F_9353, 0x3CAE_0325_0C89_1A9F, 0x233C_CFE4_AEE6_07DC,
            0xC95B_F444_6F11_13BB, 0x4186_8A63_B923_B9D8, 0x25C3_E1C7_33AD_BA90, 0x0CF5_A89C_5839_1100,  0x626A_394E_6F50_937E, 0xC225_A950_0E5C_41A8, 0x7014_FB42_774D_7625, 0x1DFF_3BFD_08C3_DC62,
        ]),
        EnumMap::from_array([
            0x9CAB_AF2E_3D29_C6B1, 0xCBC2_9465_4083_87AA, 0x14F5_9282_CF4F_79F3, 0x4E01_E81C_AEF6_D7D9, 0x0ACA_E411_ABFC_FE03,  0x6606_060B_1C24_97C1, 0x5B69_B0AC_EBD4_3F0A, 0xCF26_1D70_CA4A_0DA5,
            0xBC1C_A022_3DCB_E770, 0x3D64_73BB_40F7_0284, 0x4CAE_C1A7_4F7A_E101, 0xD098_0C5B_21DB_5C65, 0x6588_2919_7777_BBF1, 0xF5FC_7798_C7A1_2A85, 0x9C90_BE84_9F89_5139, 0xA5CF_3ABD_D085_2F7F,
            0xCE94_767C_46F4_9493, 0x54AB_7A92_3785_E18D, 0x0A66_90F0_2456_613D,  0xBE01_954A_B856_BD10, 0x5662_D4D7_322C_10FC, 0x0BED_135C_AF11_B2BF,  0xF9EA_2691_AB21_79F1, 0xEA16_2C63_FA90_3D09,
            0x35FC_7815_B102_C450, 0xB094_CDCF_B627_3E93, 0x7FBA_CB0D_BDA5_0F22, 0x29F1_5721_CAD6_6B98, 0xA6C8_0364_91B5_ED7B, 0xBB36_485F_E1E8_E3F6, 0xF4A4_88E2_4571_BE79, 0x18E9_B92E_1E98_A598,
            0xF42B_6E3D_9B58_D82B, 0x1D93_3E4C_6EC9_9FE3, 0xEC59_07A8_4E07_3A9B, 0xC4F8_717A_2FEA_61A9, 0xF2F3_5A56_3DDE_9F45, 0x28C5_E9B1_76AD_0627, 0xDF12_7719_92EC_1246, 0x3AA4_E5D9_E581_0670,
            0x41D1_D69A_7EB7_4078, 0xFE4A_37FB_4CAB_3560, 0x5F05_5BBF_6967_87CA, 0x01E6_6CA0_E48F_A991,  0x83F5_05C7_B109_B558, 0xE770_7B2B_46B4_A6DB, 0x572C_5857_3D47_0CEA, 0x5989_B37D_75E9_20AF,
            0x65D8_CCEB_7364_9B51, 0xA2BC_8290_D775_A331, 0x659F_A1E3_CDE3_B250, 0xAD80_EF9D_9324_DC33, 0x6E18_1F30_AF26_DF11, 0x740D_E1F0_0856_4A09, 0x9A52_DEF8_2DE4_800E, 0xD602_600B_84B9_6526,
            0x6AFB_479E_AF5C_9F24, 0x5C11_BA26_3F8A_BB5A, 0x8265_CA26_3800_A341, 0xAFE2_E41E_9307_0218, 0x7010_2A9F_07E4_8FA4, 0x3B9F_024D_82B1_504C, 0x50D3_E1B5_912B_9061, 0x0A64_5200_E06D_B27A,
        ]),
        EnumMap::from_array([
            0x7E17_D334_A661_F5EC, 0x16F0_477A_6FDE_45FE, 0x6616_DBB4_D84F_9C0A, 0xC28B_41D4_5FCA_5C88, 0xA562_DED7_B543_5FC9, 0x5625_B142_6858_F0B1, 0x88E0_079B_BDAB_5DAC, 0x125B_9523_1E84_0685,
            0x5A19_11E1_60AB_8469, 0x42D5_17E5_21C3_1BCF, 0x0977_E75D_83BB_4D89,  0x1986_A8BF_8656_281B, 0x34CF_E1DB_B6C2_36B4, 0xC874_8C60_099B_CF7A, 0xF7E1_A6B1_7FD5_7F61, 0x6AF3_18ED_4054_92F6,
            0xB511_4640_CD06_0FBA, 0x9C6D_60AF_BB06_B84B, 0x8256_E205_E135_D5F2, 0xE348_4662_F424_E4AA, 0x145B_B338_E633_D6A6, 0x91BB_96ED_BE93_36C3, 0x05F5_9C93_E860_69DC,  0x1EAF_7C4E_FB58_FB54,
            0x2E59_7905_7267_2463, 0xD760_7910_63D6_B119, 0x0155_4AB5_B18F_4A1B,  0xEA91_F7E1_3ABA_AD1F, 0x82F2_F019_31AB_599D, 0xC9CE_496B_E9CC_E5A1, 0xCDE5_3EB2_C5CD_0310, 0x0067_150A_ACE0_3F17,
            0x81C4_DB41_8899_F7D6, 0xBA5E_E5C7_5E10_1CD1, 0x3C4C_547E_E278_D196, 0xE058_7058_C0A8_BA76, 0x2E38_8061_2120_DC4C, 0x06C2_304C_812B_FC05,  0xA108_8566_5D53_5BA3, 0xB12A_3BE3_3B76_4C04,
            0x1346_39DD_81FF_56C1, 0x8ADF_7DE2_316C_B82C, 0xA85A_61A4_9575_7D75, 0x0BF7_A381_11B7_4C60, 0xEA1C_F905_0866_7BD4, 0x2E22_A6BD_3E5D_7447, 0xFA20_9918_A4D0_7A0E, 0x0DAF_A7BC_A298_7D49,
            0x52B0_3DF5_9163_F7BC, 0x058A_FE80_CCB6_6165,  0xCB47_F9BB_89C0_4A28, 0x72A1_8D89_A6C1_DF14, 0xA491_64C7_75F1_7A4E, 0xEC0C_F520_C945_483C, 0x0F10_18CD_36D4_16FD,  0xCE03_76C0_906F_E2DE,
            0xF9B0_8CE8_88A7_97BA, 0xFFC6_4519_B3B4_BD4D, 0x286A_B0A0_A8A7_8A54, 0x8661_5029_F5BD_0B12, 0x6E6F_33B3_80F9_BADD, 0xC9AE_6B05_AA1A_E10A, 0x49A8_0235_5C11_589F, 0x61C0_5FA2_E8A6_7086,
        ]),
        EnumMap::from_array([
            0x2187_6737_12C2_D996, 0xA71F_B671_3031_131C, 0x2A4B_A1F7_F524_6A3B, 0x5F5E_F8AE_50DE_0C2C, 0x482F_AAD0_642B_702A, 0x472F_9AFE_BF41_EA3D, 0xCC2D_E305_D44C_11A7, 0xD32F_608A_9DA2_4B51,
            0xB2F4_753E_2C8B_075B, 0x5C07_ACE8_0BE7_786C, 0x4878_1B3A_4084_4D2E, 0x4D38_4A30_660D_AD80, 0xB817_6B02_6745_46C5, 0xCDAE_B3E1_13C2_E36D, 0x36F2_CB8B_9EF8_4873, 0x0218_A5F8_923C_F32E,
            0x5239_D4EC_8163_1654, 0x0A3D_64CC_A8F1_B330,  0xF39A_12F8_AD70_1F4F, 0x92CF_215D_9BD0_FF66, 0x8065_8684_3C10_916C, 0xBE45_69D0_328D_4C99, 0x2080_FA79_1874_A042, 0x9CDA_2665_110C_62CD,
            0x83F8_B50F_8158_34F3, 0xE8A7_552D_2598_2CA3, 0x5015_662C_8B95_BA30, 0xABD2_FB71_E88C_B231, 0x0E9E_33DD_7C9C_776E,  0x458E_46D3_BD9C_8F06, 0x7CAF_FBC6_BB73_0740, 0xCF62_C0D3_6077_B3C5,
            0x523A_F458_205D_B0C3, 0x96C7_7518_1324_9FB5, 0xA9F1_83A0_B67D_4FE0, 0x16FB_EE72_86CA_5EA6, 0xB92B_DFB9_07F3_CE3A, 0xA112_9D39_5FE8_665D, 0x0B2D_3F65_574A_96BA,  0xEFC8_45A0_96C3_0D92,
            0xFC4B_C21C_7ABF_53B5, 0xB17B_1910_5881_FEEB, 0xDA67_CF1D_1E5F_3452, 0xF956_2F4B_15A3_9114, 0x7C56_7D5F_19E3_E79F, 0xD639_5FE9_1058_4064, 0xF780_E698_FD29_A6C0, 0x14A4_A20E_276A_65C0,
            0xB66D_4634_0547_AA99, 0xCAF7_4A80_C4D5_9AFC, 0x5442_ADB5_C57E_45B7, 0x0DCD_C971_BC05_A9CE, 0x88CE_1F9D_A024_6F23, 0xF5BA_8DD4_BE2B_6E4A, 0xBD54_8B36_DC35_B932, 0x679B_EFC4_98D0_0552,
            0xEB0E_C16F_AA13_402F, 0x44FE_AC66_7E4E_D6D8, 0x2631_2BAA_0AA0_069E, 0xCB8F_12B3_79AA_DB29, 0xF797_984C_9058_280E, 0xAD63_70BA_86E9_41E9, 0xD131_0EC1_D930_507A, 0x1DB3_A5A8_4E13_48F9,
        ]),
        EnumMap::from_array([
            0xD1A0_1829_0F41_C4CE, 0xC7AE_F088_55AA_3294, 0xD9BD_3B50_46D2_4BE9, 0x678E_EBFB_0888_3333, 0x390A_03D5_BC93_D102, 0x41D0_61DB_2E27_7408, 0x3805_92E5_FCB7_A829, 0x9771_D70B_5A28_DA50,
            0x955E_F1F4_6EB3_01D0, 0x2674_677F_CAE8_D0AB, 0x2D4F_8BCA_3E1B_1C6D, 0xC85F_CE8A_E762_5CF3, 0x4A00_9C4A_9573_C3F3, 0xDF6E_D771_9FBF_FECC, 0xEA79_71A0_0359_3238, 0x88FB_1E06_E64D_F966,
            0x14B6_AD74_A1BE_6FD8, 0x325A_F886_FB5E_8EAA, 0xE88C_E416_DC2F_90BE, 0xF690_7D18_78DA_C859, 0xC299_ECDA_44B4_5620, 0x27D7_0608_556E_0D54, 0xF24F_22FF_AD58_853C, 0xEA84_C586_D601_F5C0,
            0xBD2C_883F_1228_5AEA, 0x4BE8_0C6D_DC7C_BC82, 0x875E_BFC2_38C7_A9E3, 0xD809_DE1B_3531_D667, 0xED04_7201_D2C9_0C7E, 0x9384_7895_7AFD_9A31, 0xF034_5F91_14EA_EEAE, 0x0AAD_3EC8_27C7_C58E,
            0xCF40_363F_52FD_5D36, 0x664C_8C56_019F_6A5E, 0x73F0_E4E9_55E5_7E8F, 0x6A19_6634_884C_58F2, 0xB2A9_137F_2928_6040, 0x0D85_4485_EDBD_7838,  0x36BD_2FC2_BF56_283F, 0xF74E_AAFD_F30F_F098,
            0xCE42_B02E_0184_248A, 0xECFA_4F13_FF06_EC74, 0xA7D1_2C2F_92DC_CE67, 0xB2F5_27BB_5865_9F9C, 0xE51B_6ECA_ADC2_B71B, 0xA954_CAED_C0E0_0A87, 0x983C_86D4_4987_9ED7, 0xE6A1_E9B3_5595_BEE8,
            0x62B1_084C_F684_3C2F, 0x7CE6_D096_BE39_FE97, 0xC2E9_91E9_A577_8BCC, 0x6712_13C1_5333_557E, 0xCEE7_D8B4_1319_1C9C, 0x4192_220F_BE7D_653E, 0x05B6_0B48_31F5_58E8,  0xCA1D_6D63_E6BF_A363,
            0x93BE_14C8_EC83_DE40, 0x583A_C4D4_4E26_A599, 0x512D_8219_B155_5967, 0xA780_9EF2_4AA6_0A5F, 0x18A1_1322_AF27_7C22, 0xD156_A4CB_B705_7B6A, 0x04CB_C6A5_5E03_A50C,  0x06A0_4602_EA6D_FA31,
        ]),
    ]),
    EnumMap::from_array([
        EnumMap::from_array([
            0x7834_6D76_0A10_BC4E, 0x3895_E347_EBB4_0051, 0x95A5_5214_9BE3_13B1, 0x5914_96B9_3C30_45D3, 0x01DA_809F_1834_D8AE,  0x2393_2F68_2045_7419, 0xD612_1C28_EFDD_CECE, 0x5330_4B54_9791_1AB0,
            0x9D7C_CEB7_AC2C_EE2E, 0x769E_B8F9_A2E7_E6DF, 0x76A8_01DD_90DA_E5C2, 0x79C2_BDA0_17D2_5D01, 0xC31D_87E2_B43E_BCAB, 0x4A4B_22BA_DED0_3607, 0xC05B_E87F_2289_50AB, 0xBF31_B18C_6102_D07E,
            0x6E7A_E04F_8928_263B, 0xD54C_5EE4_E67E_8FC0, 0x8DDA_288D_F4D7_0602, 0x3A5E_B0C1_40A2_0CCF, 0xA81F_54D7_753E_6039, 0x7DA4_0C26_8A63_20C7, 0x43C4_0947_39E7_7401, 0x8A37_A80F_44FB_AD2F,
            0x5D86_996D_CAB9_3567, 0xDD58_9D02_24C2_1CDE, 0xFBA2_FEE2_0984_DF4D, 0x1300_EEBE_F87F_53DC, 0x4004_B893_6021_123F, 0x230E_5D31_0794_F9A7, 0xACD2_BF83_B9E8_2824, 0xAC6C_F35E_80AC_6777,
            0x4222_D94D_8523_CEDD, 0x3C6D_8BC0_D824_A470, 0x94F4_B4CF_DD11_73D4, 0x3684_556E_5D65_9741, 0x323E_36EC_4020_AA5A, 0x11B0_A242_2727_EE94, 0x3A5C_EFA1_C0E1_4107, 0x10A0_AACF_AAC1_E480,
            0x85DB_245D_5323_6C19, 0x864E_DFFF_CDDC_02BE, 0x6299_9802_342C_E505, 0xD68E_FE4C_FBAD_0D4D, 0xB4A5_6053_3C05_9643, 0x83CC_C85B_A03E_2F50, 0x4CF2_A3FF_1CC7_02B1, 0xF284_C374_E152_FA7F,
            0xCCF5_36B9_BA8F_3875, 0x30C1_98F0_E1AC_0037, 0x508B_26CA_94DD_9F6C, 0x0F0F_CB6D_4B75_6550,  0x7230_DA68_0273_1234, 0x914A_977E_1E89_A3B9, 0x3064_F933_4DBA_5FEA, 0xCCA2_D477_C271_892D,
            0x7588_4CCF_07B2_D822, 0xC4CA_1406_A035_D5ED, 0x0B74_43CF_4787_6D98,  0x6464_F6FA_0F7B_E46A, 0xAD96_BA6D_3C4E_B7C0, 0x338F_07A6_8AB8_29CF, 0x31DE_32D9_D2F1_F8AF, 0x3455_FCE9_5157_184F,
        ]),
        EnumMap::from_array([
            0xABFB_8C41_9D44_127B, 0xCE65_235B_E7B6_C7B4, 0x3DD3_8B3A_F071_8F2D, 0x7BCE_C4BA_6660_AE8C, 0xC8A9_5ADF_1F80_92D2, 0xBFD0_EF84_D73E_97A3, 0x8A77_419F_C6DB_8920, 0xF38E_58DC_C867_A9C3,
            0x000F_89F3_1F29_0478,    0x1636_1025_5CE8_1668, 0xEDF4_06F6_7488_B74F, 0x48BC_0B72_7001_A743, 0x02BA_3279_FE28_DBB4,  0x53D7_73A1_2D11_35E5, 0xA4D3_19C9_A10E_2BD2, 0x116D_F7D4_C3CE_F5B4,
            0xB7F3_58DE_059A_F0FF, 0x0579_74A0_67BA_BA7F,  0x6DFB_53E7_8060_5A43, 0xEE96_9CD2_1A24_7DBF, 0x037A_4C3E_506B_0747,  0xF31F_EEBE_BF54_2D3A, 0xB9C4_CA70_A6DC_0E07, 0x51E0_18EB_A299_0EC3,
            0x1B93_8049_BC25_552C, 0xE352_2E26_165C_B203, 0xF961_E702_54C2_042A, 0x86F5_5EF8_890B_6576, 0x70E3_83D8_8999_81AC, 0x6F7F_3201_6215_E827, 0x9955_21ED_FD61_C81E, 0x926A_22F0_72FF_BF6F,
            0x1442_9708_E518_66B6, 0xC0F0_8A80_6F66_4097, 0x7E87_B6B0_4B0C_FFD3, 0x61C2_CE4C_3261_E5C4, 0xB950_F6F9_92F4_6DA0, 0xB177_3170_C97D_B00D, 0xA363_35D5_BB91_6AF2, 0xA78A_C0CF_0EB5_FE04,
            0x6537_2331_914E_2443, 0xA84B_5DAE_C0A7_917B, 0x44EF_1028_4494_33C1, 0x0563_3486_F05C_9DEB, 0xB6D0_CB18_C7B2_78FB, 0x6AEB_6FCF_905D_F5A1, 0x7A7A_31A2_D8A8_717F, 0x17E8_8FD5_FD4D_5733,
            0x00D1_AFBD_E4E6_250E,   0xB9CA_AAAD_7EEC_4194, 0x7A81_04D2_0815_8879, 0x4AB3_658D_5916_4186, 0x9007_CF0F_A317_375E, 0xC61C_6A77_EA39_E39C, 0x3BF3_C4D1_F7F4_BA08, 0xBB98_B2EB_6307_C405,
            0xA589_6291_BB42_F549, 0xD0CB_FF48_CDCB_C572, 0xA7EE_204D_FAD5_BC6A, 0x9AC1_A7EC_1B9C_5F1A, 0x8294_F675_E957_7764, 0x1D67_1DE4_D33F_4F70, 0x6651_7572_2167_26FB, 0x9FEC_8FFF_B076_A76A,
        ]),
        EnumMap::from_array([
            0x1503_9581_F573_8DA8, 0xE1EC_A975_7797_4D32, 0xE4C5_F6CB_1B93_AF6D, 0x4D12_9A70_7B0A_C8EC, 0x6189_396A_F01F_8495, 0x2EE8_EDE2_18D6_5FB9, 0xA4F7_7E2D_4715_39C9, 0x191F_C93D_E6D9_AD16,
            0x69D2_5DD5_2E69_3695, 0x2A53_440B_0EEC_4935, 0x7026_E2BF_FDAF_C546, 0x0CB6_2B3A_5808_E791, 0xC7DF_6CD6_6043_8A4B, 0x5A9F_19B3_73AD_7D49, 0x23E6_B1A1_09DD_FFA1, 0xDA55_6E2B_8E65_F704,
            0xFD5C_5E53_F18D_D3A6, 0xE15B_BCD8_100B_11B1, 0xFF5C_5F0E_56CA_0491, 0xA607_1D39_07A6_A447, 0xA1EA_457B_BE87_35E3, 0x3869_59B4_1264_6EB2, 0x5B17_FC21_2441_5ED2, 0xC068_FE94_FB71_7CA3,
            0x9575_ED44_719D_D229, 0x3667_EB64_FC53_3A85, 0xBAE5_7836_41DB_7334, 0xBB02_4D73_1DC2_4D0C, 0x1FDF_3067_BEA1_49C1, 0xDFCE_291E_2FE4_91A7, 0xF501_C818_8895_3C66, 0x5FA4_07EF_7EAC_FC53,
            0x527F_872F_BF8E_EA62, 0x0D8B_DD1C_137D_1C36,  0xB7F3_717E_01D1_2C16, 0x1110_ECB7_4D1D_F3F1, 0xB0F7_EDBD_2554_7561, 0x17F5_422B_EB08_E3F2, 0x2887_6E94_E5EE_892D, 0x198F_58DC_39AB_9973,
            0x42A9_9B48_6008_A903, 0x46A5_2C5E_EF27_3F4F, 0x7FEF_872B_4763_B969, 0xCBA4_4115_6AEF_1D7F, 0xC92B_5749_4E6C_2B65, 0xC953_5333_2605_4C0F, 0xCE8B_9136_84C9_0592, 0x146A_6115_84AA_05B4,
            0x1B95_6878_40B7_44F4, 0x9182_7031_CAF1_159E, 0x890D_616A_D4DA_3428, 0x6ABD_1665_1732_022D, 0xCE44_0958_5A3A_4AF1, 0x380D_311B_73FE_18E7, 0xBEFF_988B_FB32_9DCC, 0x9ED8_E9E7_A99C_82F8,
            0xE7BE_2B87_C553_C114, 0x2AC9_D1B0_ADA8_EC61, 0xF1F7_A568_8A8D_9ED6, 0x0F59_B11D_FCC4_2A5F, 0x415E_9D16_6814_2011, 0x21C4_DA1F_059F_FB47, 0x9CC2_92EA_CDBC_DE84, 0x14EB_650B_AAC9_A27F,
        ]),
        EnumMap::from_array([
            0x6D58_F6EA_A5B9_EEDC, 0xBEBC_9E6C_7236_45C3, 0xC430_AF2D_D2A1_02B8, 0xBD56_A676_6F1A_13FB, 0x524A_B2D1_A364_5815, 0x972E_080B_F0EA_58D5, 0x380C_E998_EEF6_C953, 0x55EF_FF35_6AAA_144A,
            0xB2A5_2047_18E7_C9D3, 0x8D5A_6B9C_ECB7_8D61, 0xAD4E_8768_18E0_0384, 0x76D7_FDF7_F7A6_3137, 0x20F2_DD28_5D85_3862, 0x7EEE_812A_CE88_0B78, 0x7F45_2607_DF1D_3895, 0xBAD8_A1B3_77FF_EA94,
            0x273A_62F5_3EEC_D97C, 0x0BE2_D9D5_D680_43C0,  0xAAEF_99E0_C586_D735, 0x9261_604C_3863_D6D6, 0xD705_782E_C05F_86D1, 0xEB73_621E_967C_7A81, 0x6264_D27D_E7FB_8B35, 0x3A99_6463_97C3_A420,
            0x1709_0289_C965_1196, 0x5F05_572B_AE72_F03E, 0x86B1_7CB9_04D2_5277, 0x985E_D20B_2BAD_68A8, 0x2C15_E0DD_60CA_AE57, 0xF0DD_9415_5C15_0492, 0x784D_7B1B_5DD3_372C, 0x9263_9C50_B8F8_643D,
            0x6FDF_404F_35FF_0652, 0x9C5A_6633_2D10_4C67, 0x9E6E_618A_6A85_4BE4, 0x2E9A_88E3_C297_F808, 0xD653_298F_4CFC_8335, 0xA3D9_7654_902E_EB13, 0x2058_8237_EE5E_7AA7, 0x451B_C480_8ED3_0A8C,
            0xA3FD_384F_5CF2_15B7, 0x07CF_A762_E9F2_14E6,  0xB26F_C261_A7B1_B896, 0x7D5A_B840_DBC5_CDDA, 0x4E1E_BF72_D755_57AA, 0x637E_964A_EE82_DC0A, 0x31F4_061D_76F3_C95E, 0xE587_5613_B74E_7129,
            0x1358_4884_79D1_6766, 0x39FA_8819_BE20_C9E5, 0xB958_068D_7604_C5CF, 0x0449_B39A_E842_3E2E,  0x0685_D9FE_1C54_30A5,  0x3FE2_F97E_DE44_8C65, 0xF083_A0B0_C120_80AD, 0x3A0E_2692_8948_4BD7,
            0x26E6_E5AF_DBD0_E935, 0xC900_8DFB_B2F3_8391, 0xEB0F_8971_3FB8_F596, 0x5D7B_3390_4D2F_478D, 0xF010_E350_C1D2_3084, 0xB115_42FF_DB5B_488D, 0xFA1B_BB96_AD88_5511, 0x0D70_0106_4A53_E328,
        ]),
        EnumMap::from_array([
            0x6B6A_5C53_B375_A3BA, 0xA0AA_B8AC_AF62_B833, 0x9D65_EB0A_9708_270C, 0x830E_B351_6664_E837, 0x4066_FE75_6CAB_A452, 0x59DA_43F5_64BD_C62D, 0xA93E_04AE_6F0B_A79C, 0xBC78_9908_52AC_78C6,
            0xDBB0_7092_97A8_4DD0, 0xCA48_0BDC_DDDA_C43B, 0x5923_E0D5_BC2A_615C, 0xF59C_4241_97EC_BF9B, 0x01CD_3C81_7563_7B31,  0x0AD0_81B3_2E7C_CDB8,  0xF4A2_E611_1C6C_9E65, 0x9B0C_F6E2_2402_7BCF,
            0xAB2C_CB16_ACD5_5BCB, 0xEF68_D690_FBEA_D92D, 0xBC5A_6EC8_2316_6637, 0x180E_FBC1_AA48_B07B, 0x7237_79BA_8876_CE58, 0x8D5E_D10E_F4EB_58EB, 0xEE4B_7AC3_8CA7_CBC6, 0x71EC_7244_BA68_FE2D,
            0x94A9_0CD0_BA08_70C6, 0x7527_B416_AB3F_6853, 0x0158_F0D1_5C73_D1E4,  0x0CD2_A0AC_FC17_AC2B, 0x4898_30D3_F6C0_A185, 0xEC1E_E3EF_620D_8EA1, 0x4E1C_0FC2_88CA_1D7A, 0xD13A_6A3E_FB2A_F639,
            0xF149_A4CF_E4B4_752D, 0x6969_6C4F_337D_5E4D, 0xD72E_A09A_2A70_2537, 0x18CD_F62D_0F67_AFBF, 0x45A6_52DA_8B91_3E2C, 0xA0DB_8B23_7CA4_1AC5, 0x1D53_90EB_2A60_AECC, 0x32B0_003E_A95A_5D33,
            0xB229_967C_6474_7999, 0x812E_3A67_2F21_6CA4, 0xF15F_FC2B_073B_6BE3, 0x8ACC_8A96_5963_C08D, 0x39B1_B5AC_DC7B_A46E, 0xEAE4_AD67_C17F_FCEE, 0xCBF8_D576_38A6_87E6, 0xA25D_2D7E_284F_8B70,
            0x38FD_4D90_3E9F_90C9, 0x3083_C4F2_4240_9C1D, 0x94DB_7E90_D919_934C, 0x5DD7_518C_34D2_FC8E, 0x4AA1_4F72_D82F_CCA9, 0x7E1A_0532_18FE_E895, 0xC83E_A32B_9C3A_EB25, 0x90F9_E63E_74AA_D712,
            0x7354_824E_5683_24F1, 0xC2E7_18C3_AF0D_2CD7, 0x2690_6260_5662_E95C, 0x9CFB_4CA6_1E25_6E84, 0x9AD3_59DE_F2A8_A96D, 0xB8F3_9386_20A9_3C42, 0xE2B7_DF81_01CB_B76F, 0x1FC6_DD4C_EF2B_520D,
        ]),
        EnumMap::from_array([
            0xE123_37D7_9D74_E9FE, 0xC759_B24F_504A_A46F, 0xFF52_78C0_8C30_80F6, 0xF5C1_3EE8_E322_0BEF, 0xAAB5_5DFC_683F_07A0, 0xBB56_9B79_CA7C_B8A0, 0xB439_FFC4_E11E_FDBC, 0xD34D_DB89_AD3A_CBCE,
            0x5F35_45C1_847D_6076, 0xFBB7_ECF9_9B6D_2207, 0x9F3B_39C0_801F_8822, 0xD9C5_B7BA_823E_6224, 0x6A29_FDC0_9E58_ADC1, 0xF787_46E1_2C71_B173, 0xB601_C5D3_D3D1_EA3B, 0x41E3_5433_E9D6_C06C,
            0x4549_9F5B_7C4B_D910, 0x5BD3_8607_851E_4382, 0xB121_2FF7_35A2_2AD5, 0x8C29_187C_DE77_9BFF, 0x8710_0C21_8235_56D5, 0xF20B_591C_E223_B985, 0x7B76_EB6E_B2A4_2BFF, 0x4537_5884_E6AE_87C5,
            0x6F54_4707_3B22_74ED, 0xFDD4_387B_C354_0BC2, 0x20D7_D0AD_866F_5D45, 0x666D_B2E3_263D_5B33, 0x94E0_2142_A277_918C, 0x9E12_1E59_03DF_80A9, 0x33F5_52A6_96A6_D0BA, 0xC91E_23B4_BE4C_6249,
            0x1665_2D46_A6AC_34D5, 0xD2E9_69A1_1F32_D406, 0xF8DB_E1B7_07F6_B273, 0x2187_DD7B_4005_D9BB, 0x90B7_00E5_1DC3_B6D1, 0x950D_BDB3_FF94_2002, 0x29B9_F48C_343C_4A73, 0x2487_AAA2_7DA1_CC21,
            0xB8F6_F70E_BDBC_5630, 0x9D54_09AF_DF3A_7D06, 0x3BBF_4E7D_5198_773E, 0x330C_FB76_7521_E96F, 0xB3F1_8DA1_EC7C_8992, 0x2188_6C66_32DE_4BDB, 0xFD5E_B5AA_3EEC_05B9, 0xE152_DCCA_823B_842B,
            0x5B90_585C_75E7_639B, 0x37C1_C6D0_77F1_6BAE, 0xFE6C_AACE_4255_DEA5, 0x1330_5AF7_8CA6_9D7A, 0x46A5_3280_7059_2955, 0x1E87_ECCB_BD86_209E, 0x5735_6095_1B7B_39DC, 0xF266_286D_9FDF_94F4,
            0x3A54_202E_B691_F2B8, 0xF678_6A6B_966B_C518, 0x25E3_B72C_BA43_4484, 0x04B8_FDA0_911F_CBE6, 0x3DFA_ED88_E0F6_83C5, 0xDE46_9EB5_D8A6_64ED, 0x51E5_8412_6142_6823, 0x29DB_D31C_802A_5F77,
        ]),
    ]),
]);

/// Hash value of one of the 16 possible en-passant squares A3-H3, A6-H6
const EN_PASSANT_KEYS: EnumMap<EnPassantSquare, u64> = EnumMap::from_array([
    0xCC5E_EF11_3797_E347,
    0xAA90_BC6F_508F_C0AE,
    0x735D_A197_A644_D75E,
    0x3774_4D11_E638_E6DA,
    0x0197_A767_F276_8F84,
    0x2051_D4EE_0123_676B,
    0x2B9A_D8C0_0CFF_B700,
    0x9C54_065D_4D23_E231,
    0x3D93_FE65_2786_B4DF,
    0x946E_EEB8_1F3B_174D,
    0x4C2E_C39C_EE8B_9A0A,
    0x276F_22C3_BA40_D7E9,
    0x2529_97C6_9EB7_4C9C,
    0xEC92_6AE5_50EE_73E0,
    0x3CE2_A3E8_8BC5_6598,
    0xB6CF_5D2A_80FB_EBD7,
]);

/// Hash for one of 4 castle rights, white/black king-side/queen-side
const CASTLE_KEYS: EnumMap<CastleDirection, EnumMap<PlayerColor, u64>> = EnumMap::from_array([
    EnumMap::from_array([
        // White King
        0x8499_3F26_2ABB_0E4A,
        // Black King
        0x49A6_17EA_01D9_B291,
    ]),
    EnumMap::from_array([
        // White Queen
        0xA0AC_B86F_0695_F023,
        // Black Queen
        0xE111_D878_8EEF_CFDE,
    ]),
]);

#[cfg(test)]
mod test {
    use crate::castles::CastleDirection;
    use crate::pieces::{Piece, PieceType};
    use crate::player_color::PlayerColor;
    use crate::square::Square;
    use crate::zobrist::ZobristHash;

    use test_case::test_case;

    #[test_case(PieceType::Pawn, PlayerColor::White, Square::E2)]
    #[test_case(PieceType::Pawn, PlayerColor::Black, Square::E2)]
    #[test_case(PieceType::Queen, PlayerColor::White, Square::A3)]
    #[test_case(PieceType::King, PlayerColor::Black, Square::B8)]
    #[test_case(PieceType::Rook, PlayerColor::Black, Square::C2)]
    #[test_case(PieceType::Knight, PlayerColor::Black, Square::C2)]
    #[test_case(PieceType::Rook, PlayerColor::White, Square::D4)]
    #[test_case(PieceType::King, PlayerColor::White, Square::H1)]
    #[test_case(PieceType::Rook, PlayerColor::Black, Square::F5)]
    fn toggle_piece_square_symmetric(piece: PieceType, player: PlayerColor, square: Square) {
        let mut hasher = ZobristHash::default();
        let hasher_original = hasher;
        hasher.toggle_piece_square(piece.owned_by(player).placed_on(square));
        assert_ne!(hasher_original, hasher);
        hasher.toggle_piece_square(piece.owned_by(player).placed_on(square));
        assert_eq!(hasher_original, hasher);
    }

    #[test]
    fn switch_sides_symmetric() {
        let mut hasher = ZobristHash::default();
        let hasher_original = hasher;
        hasher.switch_sides();
        assert_ne!(hasher_original, hasher);
        hasher.switch_sides();
        assert_eq!(hasher_original, hasher);
    }

    #[test_case(Square::A3)]
    #[test_case(Square::B3)]
    #[test_case(Square::C3)]
    #[test_case(Square::D3)]
    #[test_case(Square::E3)]
    #[test_case(Square::F3)]
    #[test_case(Square::G3)]
    #[test_case(Square::H3)]
    #[test_case(Square::A6)]
    #[test_case(Square::B6)]
    #[test_case(Square::C6)]
    #[test_case(Square::D6)]
    #[test_case(Square::E6)]
    #[test_case(Square::F6)]
    #[test_case(Square::G6)]
    #[test_case(Square::H6)]
    fn toggle_en_passant_square_symmetric(input: Square) {
        let input = input.try_into().expect("invalid test setup");
        let mut hasher = ZobristHash::default();
        let hasher_original = hasher;
        hasher.toggle_en_passant_square(input);
        assert_ne!(hasher_original, hasher);
        hasher.toggle_en_passant_square(input);
        assert_eq!(hasher_original, hasher);
    }

    #[test_case(PlayerColor::White, CastleDirection::KingSide)]
    #[test_case(PlayerColor::White, CastleDirection::QueenSide)]
    #[test_case(PlayerColor::Black, CastleDirection::KingSide)]
    #[test_case(PlayerColor::Black, CastleDirection::QueenSide)]
    fn toggle_castle_ability_symmetric(player: PlayerColor, castle_direction: CastleDirection) {
        let mut hasher = ZobristHash::default();
        let hasher_original = hasher;
        hasher.toggle_castle_ability(player, castle_direction);
        assert_ne!(hasher_original, hasher);
        hasher.toggle_castle_ability(player, castle_direction);
        assert_eq!(hasher_original, hasher);
    }
}

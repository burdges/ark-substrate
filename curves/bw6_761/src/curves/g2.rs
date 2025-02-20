use ark_ff::{Field, MontFp};
use ark_std::{marker::PhantomData, vec::Vec};
use sp_ark_models::{
    bw6,
    short_weierstrass::{Affine, Projective},
    {short_weierstrass::SWCurveConfig, CurveConfig},
};
use sp_ark_utils::{deserialize_result, serialize_argument};

use crate::{Fq, Fr, HostFunctions};

pub type G2Affine<H> = bw6::G2Affine<crate::Config<H>>;
pub type G2Projective<H> = bw6::G2Projective<crate::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]

pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

impl<H: HostFunctions> CurveConfig for Config<H> {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 26642435879335816683987677701488073867751118270052650655942102502312977592501693353047140953112195348280268661194869
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x3de5800000000075,
        0x832ba4061000003b,
        0xc61c554757551c0c,
        0xc856a0853c9db94c,
        0x2c77d5ac34cb12ef,
        0xad1972339049ce76,
    ];

    /// COFACTOR^(-1) mod r =
    /// 214911522365886453591244899095480747723790054550866810551297776298664428889000553861210287833206024638187939842124
    const COFACTOR_INV: Fr = MontFp!("214911522365886453591244899095480747723790054550866810551297776298664428889000553861210287833206024638187939842124");
}

impl<H: HostFunctions> SWCurveConfig for Config<H> {
    /// COEFF_A = 0
    const COEFF_A: Fq = Fq::ZERO;

    /// COEFF_B = 4
    const COEFF_B: Fq = MontFp!("4");

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine<H> = G2Affine::<H>::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_elem: Self::BaseField) -> Self::BaseField {
        use ark_ff::Zero;
        Self::BaseField::zero()
    }

    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: Vec<Vec<u8>> = bases.iter().map(|elem| serialize_argument(*elem)).collect();
        let scalars: Vec<Vec<u8>> = scalars
            .iter()
            .map(|elem| serialize_argument(*elem))
            .collect();

        let result = H::bw6_761_msm_g2(bases, scalars);

        let result = deserialize_result::<Affine<Self>>(&result);
        Ok(result.into())
    }

    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::bw6_761_mul_projective_g1(serialized_base, serialized_scalar);

        let result = deserialize_result::<Affine<Self>>(&result);
        result.into()
    }

    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::bw6_761_mul_affine_g1(serialized_base, serialized_scalar);

        let result = deserialize_result::<Affine<Self>>(&result);
        result.into()
    }
}

/// G2_GENERATOR_X =
///  6445332910596979336035888152774071626898886139774101364933948236926875073754470830732273879639675437155036544153105017729592600560631678554299562762294743927912429096636156401171909259073181112518725201388196280039960074422214428
pub const G2_GENERATOR_X: Fq = MontFp!("6445332910596979336035888152774071626898886139774101364933948236926875073754470830732273879639675437155036544153105017729592600560631678554299562762294743927912429096636156401171909259073181112518725201388196280039960074422214428");

/// G2_GENERATOR_Y =
/// 562923658089539719386922163444547387757586534741080263946953401595155211934630598999300396317104182598044793758153214972605680357108252243146746187917218885078195819486220416605630144001533548163105316661692978285266378674355041
pub const G2_GENERATOR_Y: Fq = MontFp!("562923658089539719386922163444547387757586534741080263946953401595155211934630598999300396317104182598044793758153214972605680357108252243146746187917218885078195819486220416605630144001533548163105316661692978285266378674355041");

use crate::EcError;
use tc_asym_cipher::AsymmetricKeyParams;
use tc_ec_core::algo::import_point;
use tc_ec_core::{Curve, Point};

/// BC's `ECPublicKeyParameters`: the point Q, with the generator G standing for the
/// domain.
#[derive(Clone)]
pub struct EcPublicKey<C: Curve> {
    g: C::Point,
    q: C::Point,
}

impl<C: Curve> EcPublicKey<C> {
    /// BC's `ValidatePublicPoint`: Q is moved onto G's curve and normalized, and refused when
    /// it is infinity or fails `is_valid`, which checks the subgroup too. Variable time; Q is
    /// public.
    ///
    /// # Errors
    ///
    /// [`EcError::InvalidPublicPoint`] when Q cannot be moved onto G's curve, is infinity
    /// or is not a valid point of the subgroup.
    pub fn new(g: C::Point, q: &C::Point) -> Result<Self, EcError> {
        let q = import_point(g.curve(), q).map_err(|_| EcError::InvalidPublicPoint)?;
        if q.is_infinity() || !q.is_valid() {
            return Err(EcError::InvalidPublicPoint);
        }
        Ok(Self { g, q })
    }

    /// The generator G, which stands for the domain. Constant time.
    pub fn g(&self) -> &C::Point {
        &self.g
    }

    /// The public point Q. Constant time.
    pub fn q(&self) -> &C::Point {
        &self.q
    }
}

impl<C: Curve> AsymmetricKeyParams for EcPublicKey<C> {
    /// BC's `IsPrivate`: never. Constant time.
    fn is_private_key(&self) -> bool {
        false
    }
}

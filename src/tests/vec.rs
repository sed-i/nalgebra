#[test]
use std::vec;
#[test]
use std::iterator::IteratorUtil;
#[test]
use std::num::{Zero, One};
#[test]
use std::rand::{random};
#[test]
use std::cmp::ApproxEq;
#[test]
use dim3::vec3::Vec3;
#[test]
use dim2::vec2::Vec2;
#[test]
use dim1::vec1::Vec1;
#[test]
use ndim::nvec::NVec;
#[test]
use traits::dim::d7;
#[test]
use traits::basis::Basis;
#[test]
use traits::cross::Cross;
#[test]
use traits::dot::Dot;
#[test]
use traits::norm::Norm;
#[test]
use traits::flatten::Flatten;

macro_rules! test_commut_dot_impl(
  ($t: ty) => (
    for 10000.times
    {
      let v1 : $t = random();
      let v2 : $t = random();
    
      assert!(v1.dot(&v2).approx_eq(&v2.dot(&v1)));
    }
  );
)

macro_rules! test_basis_impl(
  ($t: ty) => (
    for 10000.times
    {
      let basis = Basis::canonical_basis::<$t>();

      // check vectors form an ortogonal basis
      assert!(
        do basis.iter().zip(basis.iter()).all
        |(e1, e2)| { e1 == e2 || e1.dot(e2).approx_eq(&Zero::zero()) }
      );
      // check vectors form an orthonormal basis
      assert!(basis.all(|e| e.norm().approx_eq(&One::one())));
    }
  );
)

macro_rules! test_subspace_basis_impl(
  ($t: ty) => (
    for 10000.times
    {
      let v : Vec3<f64> = random();
      let v1            = v.normalized();
      let subbasis      = v1.orthogonal_subspace_basis();

      // check vectors are orthogonal to v1
      assert!(subbasis.all(|e| v1.dot(e).approx_eq(&Zero::zero())));
      // check vectors form an ortogonal basis
      assert!(
        do subbasis.iter().zip(subbasis.iter()).all
           |(e1, e2)| { e1 == e2 || e1.dot(e2).approx_eq(&Zero::zero()) }
      );
      // check vectors form an orthonormal basis
      assert!(subbasis.all(|e| e.norm().approx_eq(&One::one())));
    }
  );
)

macro_rules! test_flatten_impl(
  ($t: ty, $n: ty) => (
    for 10000.times
    {
      let v:     $t    = random();
      let mut l: ~[$n] = vec::from_elem(42 + Flatten::flat_size::<$n, $t>(), Zero::zero::<$n>());

      v.to_flattened_inplace(l, 42);

      assert!(Flatten::from_flattened::<$n, $t>(v.to_flattened(), 0) == v);
      assert!(Flatten::from_flattened::<$n, $t>(l, 42) == v);
    }
  )
)

#[test]
fn test_cross_vec3()
{
  for 10000.times
  {
    let v1 : Vec3<f64> = random();
    let v2 : Vec3<f64> = random();
    let v3 : Vec3<f64> = v1.cross(&v2);

    assert!(v3.dot(&v2).approx_eq(&Zero::zero()));
    assert!(v3.dot(&v1).approx_eq(&Zero::zero()));
  }
}

#[test]
fn test_commut_dot_nvec()
{ test_commut_dot_impl!(NVec<d7, f64>); }

#[test]
fn test_commut_dot_vec3()
{ test_commut_dot_impl!(Vec3<f64>); }

#[test]
fn test_commut_dot_vec2()
{ test_commut_dot_impl!(Vec2<f64>); }

#[test]
fn test_commut_dot_vec1()
{ test_commut_dot_impl!(Vec1<f64>); }

#[test]
fn test_basis_vec1()
{ test_basis_impl!(Vec1<f64>); }

#[test]
fn test_basis_vec2()
{ test_basis_impl!(Vec2<f64>); }

#[test]
fn test_basis_vec3()
{ test_basis_impl!(Vec3<f64>); }

#[test]
fn test_basis_nvec()
{ test_basis_impl!(NVec<d7, f64>); }

#[test]
fn test_subspace_basis_vec1()
{ test_subspace_basis_impl!(Vec1<f64>); }

#[test]
fn test_subspace_basis_vec2()
{ test_subspace_basis_impl!(Vec2<f64>); }

#[test]
fn test_subspace_basis_vec3()
{ test_subspace_basis_impl!(Vec3<f64>); }

#[test]
fn test_subspace_basis_nvec()
{ test_subspace_basis_impl!(NVec<d7, f64>); }

#[test]
fn test_flatten_vec1()
{ test_flatten_impl!(Vec1<f64>, f64); }

#[test]
fn test_flatten_vec2()
{ test_flatten_impl!(Vec2<f64>, f64); }

#[test]
fn test_flatten_vec3()
{ test_flatten_impl!(Vec3<f64>, f64); }

#[test]
fn test_flatten_nvec()
{ test_flatten_impl!(NVec<d7, f64>, f64); }
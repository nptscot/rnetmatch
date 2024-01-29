use arrow::{
    array::{ArrayData, make_array},
    datatypes::Field,
    error::ArrowError
};
use arrow_extendr::from::FromArrowRobj;
use geoarrow::{array::LineStringArray, GeometryArrayTrait};
use extendr_api::prelude::*;
use itertools::Itertools;
use std::result::Result;

pub type ErrGeoArrowRobj = ArrowError;

// wrapper functions around R functions to make converting from
// nanoarrow-geoarrow easier
pub fn infer_geoarrow_schema(robj: &Robj) -> Result<Robj, Error> {
    R!("geoarrow::infer_geoarrow_schema")
        .expect("`geoarrow` must be installed")
        .as_function()
        .expect("`infer_geoarrow_schema()` must be available")
        .call(pairlist!(robj))
}

pub fn as_data_type(robj: &Robj) -> Result<Robj, Error> {
    R!("arrow::as_data_type")
    .expect("`arrow` must be installed")
    .as_function()
    .expect("`as_data_type()` must be available")
    .call(pairlist!(robj))
}

pub fn new_field(robj: &Robj, name: &str) -> Result<Robj, Error> {
    R!("arrow::field")
        .expect("`arrow` must be installed")
        .as_function()
        .expect("`new_field()` must be available")
        .call(pairlist!(name, robj))
}

fn read_geoarrow_r(robj: Robj) -> Result<
    std::sync::Arc<dyn GeometryArrayTrait>, 
    geoarrow::error::GeoArrowError
>  {

    // extract datatype from R object
    let narrow_data_type = infer_geoarrow_schema(&robj).unwrap();
    let arrow_dt = as_data_type(&narrow_data_type).unwrap();

    // create and extract field 
    let field = new_field(&arrow_dt, "geometry").unwrap();
    let field = Field::from_arrow_robj(&field).unwrap();

    // extract array data
    let x = make_array(ArrayData::from_arrow_robj(&robj).unwrap());

    // create geoarrow array
    geoarrow::array::from_arrow_array(&x, &field)
    // ?.unwrap();
    // l?et ga = ga.as_any().downcast_ref::<LineStringArray<i32>>().unwrap().clone();
}

#[extendr]  
fn rnet_match_two_trees(x: Robj, y: Robj, distance_tolerance: f64, slope_tolerance: f64, is_projected: bool) -> Robj {

    let crs_type = match is_projected {
        true => rnetmatch::CrsType::Projected,
        false => unimplemented!("Geographic CRS not yet supported.")
    };

    let x = read_geoarrow_r(x).unwrap();
    let y = read_geoarrow_r(y).unwrap();

    let x = x.as_any().downcast_ref::<LineStringArray<i32>>().unwrap().clone();
    let y = y.as_any().downcast_ref::<LineStringArray<i32>>().unwrap().clone();

    let res = rnetmatch::find_candidates(
        x.iter_geo_values(), 
        y.iter_geo_values(), 
        distance_tolerance, 
        slope_tolerance, 
        crs_type
    );

    let (ks, js, shared_lens): (Vec<_>, Vec<_>, Vec<_>) = res
        .into_iter()
        .flat_map(|(k, v)| {
            let (j, shared_len): (Vec<_>, Vec<_>) = v.into_iter().unzip();
            let ks = vec![k; j.len()];
            ks.into_iter()
                .zip(j.into_iter())
                .zip(shared_len.into_iter())
                .map(|((k, j), shared_len)| (k, j, shared_len))
        })
        .multiunzip();

    data_frame!(i = ks, j = js, shared_len = shared_lens)

}


#[extendr]  
fn rnet_match_one_tree(x: Robj, y: Robj, distance_tolerance: f64, slope_tolerance: f64, is_projected: bool) -> Robj {

    let crs_type = match is_projected {
        true => rnetmatch::CrsType::Projected,
        false => unimplemented!("Geographic CRS not yet supported.")
    };

    let x = read_geoarrow_r(x).unwrap();
    let y = read_geoarrow_r(y).unwrap();

    let x = x.as_any().downcast_ref::<LineStringArray<i32>>().unwrap().clone();
    let y = y.as_any().downcast_ref::<LineStringArray<i32>>().unwrap().clone();

    let res = rnetmatch::find_candidates_one_tree(
        x.iter_geo_values(), 
        y.iter_geo_values(), 
        distance_tolerance, 
        slope_tolerance,
        crs_type
    );

    let (ks, js, shared_lens): (Vec<_>, Vec<_>, Vec<_>) = res
        .into_iter()
        .flat_map(|(k, v)| {
            let (j, shared_len): (Vec<_>, Vec<_>) = v.into_iter().unzip();
            let ks = vec![k; j.len()];
            ks.into_iter()
                .zip(j.into_iter())
                .zip(shared_len.into_iter())
                .map(|((k, j), shared_len)| (k, j, shared_len))
        })
        .multiunzip();

    data_frame!(i = ks, j = js, shared_len = shared_lens)

}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rnetmatch;
    fn rnet_match_one_tree;
    fn rnet_match_two_trees;
}



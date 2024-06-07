pub fn e_log(inp: f64) -> f64 {
    unsafe { fdlibm_rs::__ieee754_log(inp.into()) }
}

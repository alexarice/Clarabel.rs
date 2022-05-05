use clarabel_algebra::*;
use derive_builder::Builder;

#[derive(Builder,Debug,Clone)]
pub struct Settings<T: FloatT> {

    #[builder(default = "50")]
    pub max_iter: u32,

    #[builder(default = "T::zero()")]
    pub time_limit: T,

    #[builder(default = "true")]
    pub verbose: bool,

    #[builder(default = "T::from(1e-8).unwrap()")]
    pub tol_gap_abs: T,

    #[builder(default = "T::from(1e-8).unwrap()")]
    pub tol_gap_rel: T,

    #[builder(default = "T::from(1e-5).unwrap()")]
    pub tol_feas: T,

    #[builder(default = "T::from(1e-8).unwrap()")]
    pub tol_infeas_abs: T,

    #[builder(default = "T::from(1e-8).unwrap()")]
    pub tol_infeas_rel: T,

    #[builder(default = "T::from(0.99).unwrap()")]
    pub max_step_fraction: T,
    //
    // data equilibration
    #[builder(default = "true")]
    pub equilibrate_enable: bool,

    #[builder(default = "10")]
    pub equilibrate_max_iter: u32,

    #[builder(default = "T::from(1e-4).unwrap()")]
    pub equilibrate_min_scaling: T,

    #[builder(default = "T::from(1e+4).unwrap()")]
    pub equilibrate_max_scaling: T,

    // can be :qdldl or :mkl
    #[builder(default = "true")]
    pub direct_kkt_solver: bool,

    // //pub direct_solve_method: Symbol,   PJG:Add this later

    // static regularization parameters
    #[builder(default = "true")]
    pub static_regularization_enable: bool,

    #[builder(default = "T::from(1e-8).unwrap()")]
    pub static_regularization_eps: T,

    // dynamic regularization parameters
    #[builder(default = "true")]
    pub dynamic_regularization_enable: bool,

    #[builder(default = "T::from(1e-13).unwrap()")]
    pub dynamic_regularization_eps: T,

    #[builder(default = "T::from(2e-7).unwrap()")]
    pub dynamic_regularization_delta: T,

    // iterative refinement (for QDLDL)
    #[builder(default = "true")]
    pub iterative_refinement_enable: bool,

    #[builder(default = "T::from(1e-10).unwrap()")]
    pub iterative_refinement_reltol: T,

    #[builder(default = "T::from(1e-10).unwrap()")]
    pub iterative_refinement_abstol: T,

    #[builder(default = "10")]
    pub iterative_refinement_max_iter: u32,

    #[builder(default = "T::from(2.).unwrap()")]
    pub iterative_refinement_stop_ratio: T,
}


impl<T:FloatT> Default for Settings<T>{
    fn default() -> Settings<T>{
        SettingsBuilder::<T>::default().build().unwrap()
    }
}
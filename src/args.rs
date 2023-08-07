use crate::Jxx;
use structopt::StructOpt;

#[allow(non_snake_case)]
#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short = "J", long = "J")]
    /// J_{i,j} of x_i, x_j
    pub J: Option<f64>,
    #[structopt(short = "G", long = "gamma")]
    /// Gamma of the Hamiltonian
    pub Gamma: Option<f64>,
    #[structopt(short = "L", long = "length")]
    /// Side length of the triangular Lattice
    pub L: Option<i32>,
    #[structopt(short = "H", long = "height")]
    /// Height of the triangular Lattice
    pub H: Option<i32>,
    #[structopt(short = "T", long = "time")]
    /// Time limit of the Fujitsu request
    pub T: Option<i32>,
    #[structopt(short = "u", long = "use-random")]
    /// Use random strength for each node
    pub use_random: bool,
    #[structopt(short = "d", long = "debug-output")]
    /// Output debug information
    pub debug_output: bool,
    #[structopt(short = "w", long = "without-cycle")]
    /// Without cycle
    pub without_cycle: bool,
    #[structopt(short = "g", long = "gamma-analysis")]
    /// Gamma analysis target file
    pub file_path: Option<String>,
}

pub fn get_options() -> Options {
    Options::from_args()
}

pub fn get_jxx(jxx: &mut Jxx, options: Options) {
    const TEN_DECIMAL_PLACES: f64 = 100000.0 * 100000.0; // Ten decimal places
    match &options.J {
        Some(j) => jxx.j = *j,
        None => (),
    }
    match &options.Gamma {
        Some(gamma) => {
            if *gamma == 0.0 {
                jxx.jl = 0.0;
            } else {
                let jl: f64 = -(0.5) * gamma.tanh().ln();
                jxx.jl = (jl * TEN_DECIMAL_PLACES).round() / TEN_DECIMAL_PLACES;
            }
            jxx.gamma = *gamma;
        }
        None => (),
    }
    match &options.L {
        Some(l) => {
            if (*l % 3 != 0) || (*l <= 0) {
                panic!("L must be a multiple of 3 and greater than 0.");
            }
            jxx.l = *l
        }
        None => (),
    }
    match &options.H {
        Some(h) => {
            if *h <= 0 {
                panic!("H must be greater than or equal to 0.");
            }
            jxx.h = *h
        }
        None => (),
    }
}

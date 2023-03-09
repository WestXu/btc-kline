use itertools::Itertools;
use ndarray::Array1;
use polyfit_residuals::try_fit_poly;
use polynomen::Poly;
use web_sys::console;

#[derive(Debug, Clone)]
pub struct PolyFit {
    pub fitted: Vec<f64>,
    pub max_extreams: Vec<(f64, f64)>,
    pub min_extreams: Vec<(f64, f64)>,
}

fn fit(ys: &Vec<f64>, degree: usize) -> Option<PolyFit> {
    console::log_1(
        &format!(
            "test: {:?}",
            Poly::new_from_coeffs(&[-1f64, 0.0, 1.0]).real_roots()
        )
        .into(),
    );

    let xs = (0..ys.len()).map(|x| x as f64).collect::<Array1<f64>>();

    let poly = try_fit_poly(xs.view(), Array1::from_vec(ys.clone()).view(), degree)
        .expect("Failed to fit polynomial to data");

    let poly = Poly::new_from_coeffs(&poly.into_raw().0);
    let derive = poly.derive();

    let Some(extream_pos) = derive
        .real_roots()
        else { return None; };
    let extream_pos: Vec<f64> = extream_pos
        .into_iter()
        .filter(|x| *x > 0.0 && *x < ys.len() as f64)
        .collect();

    console::log_1(&format!("extream_pos: {extream_pos:?}").into());

    let mut max_extreams = vec![];
    let mut min_extreams = vec![];
    let second_derive = derive.derive();
    for pos in extream_pos {
        if second_derive.eval_by_val(pos) < 0.0 {
            max_extreams.push((pos, poly.eval_by_val(pos)));
        } else {
            min_extreams.push((pos, poly.eval_by_val(pos)));
        }
    }

    Some(PolyFit {
        fitted: xs
            .iter()
            .map(|x| poly.eval_by_val(*x))
            .collect::<Vec<f64>>(),
        max_extreams,
        min_extreams,
    })
}

// by ChatGPT
fn rolling_mean(data: &[f64], window_size: usize) -> Vec<f64> {
    let mut sum = 0.0;
    let mut result = Vec::new();

    for i in 0..data.len() {
        sum += data[i];

        if i >= window_size {
            sum -= data[i - window_size];
            result.push(sum / window_size as f64);
        } else {
            result.push(sum / (i + 1) as f64);
        }
    }

    result
}

pub fn best_fit(ys: &Vec<f64>) -> PolyFit {
    let y_roll_mean = rolling_mean(&ys, ys.len() / 30);

    let rolling_error = y_roll_mean
        .iter()
        .zip(ys.iter())
        .map(|(y, f)| (y - f).powi(2))
        .sum::<f64>()
        / ys.len() as f64;

    console::log_1(&format!("rolling_error: {rolling_error}").into());

    (ys.len() / 10..100)
        .filter_map(|degree| {
            let Some(polyfit) = fit(ys, degree) else { return None; } ;

            let fitted_error = ys
                .iter()
                .zip(polyfit.fitted.iter())
                .map(|(y, f)| (y - f).powi(2))
                .sum::<f64>()
                / ys.len() as f64;

            console::log_1(&format!("degree: {degree}, fitted_error: {fitted_error}").into());
            Some((degree, polyfit, fitted_error))
        })
        .tuple_windows()
        .find_map(
            |((degree, fitted, fitted_error), (next_degree, next_fitted, next_fitted_error))| {
                if fitted_error < next_fitted_error {
                    console::log_1(&format!("best_degree: {degree}").into());
                    Some(fitted)
                } else if next_fitted_error < 0.6 * rolling_error || next_degree >= 100 {
                    console::log_1(&format!("best_degree: {next_degree}").into());
                    Some(next_fitted)
                } else {
                    None
                }
            },
        )
        .unwrap()
}

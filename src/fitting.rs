use ndarray::Array1;
use polyfit_residuals::try_fit_poly;
use web_sys::console;

fn fit(ys: &Vec<f64>, degree: usize) -> Vec<f64> {
    let xs = (0..ys.len()).map(|x| x as f64).collect::<Array1<f64>>();

    let poly = try_fit_poly(xs.view(), Array1::from_vec(ys.clone()).view(), degree)
        .expect("Failed to fit polynomial to data");

    xs.iter().map(|x| poly.left_eval(*x)).collect::<Vec<f64>>()
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

pub fn best_fit(ys: &Vec<f64>) -> Vec<f64> {
    let mut degree = ys.len() / 10;

    let y_roll_mean = rolling_mean(&ys, ys.len() / 30);

    let rolling_error = y_roll_mean
        .iter()
        .zip(ys.iter())
        .map(|(y, f)| (y - f).powi(2))
        .sum::<f64>()
        / ys.len() as f64;

    console::log_1(&format!("rolling_error: {rolling_error}").into());

    let mut record_fitted_error = f64::MAX;
    let mut record_fitted = Vec::new();
    loop {
        let fitted = fit(ys, degree);

        let fitted_error = ys
            .iter()
            .zip(fitted.iter())
            .map(|(y, f)| (y - f).powi(2))
            .sum::<f64>()
            / ys.len() as f64;

        console::log_1(&format!("degree: {degree}, fitted_error: {fitted_error}").into());
        if fitted_error > record_fitted_error {
            return record_fitted;
        }
        if fitted_error < 0.6 * rolling_error || degree >= 100 {
            return fitted;
        }
        degree += 1;
        record_fitted_error = fitted_error;
        record_fitted = fitted;
    }
}

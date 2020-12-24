extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::f64::consts::PI;

static STOP_CONDITION: f64 = 1e-6; //停止条件

#[no_mangle]
pub fn add(a : i32, b : i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn norm(a : &[f64], b : &[f64]) -> f64 {

    let mut sum : f64 = 0.0;
    for i in 0..a.len() {
        let sub = a[i] - b[i];
        let pow = sub * sub;
        sum += pow;
    }
    let ans = sum.sqrt();
    return ans;
}

#[wasm_bindgen]
pub fn substruct(a : &mut[f64], b: &mut [f64]) {

    let offset = vec![0.1, 0.1, 0.1];
    for i in 0..a.len() {
        a[i] -= b[i] + offset[i];
    }
}

#[wasm_bindgen]
pub fn wrap_remez(order : usize, f_pass : f64, f_stop : f64, coefficients : &mut [f64], axis_freq : &mut[f64], magnitude_response : &mut [f64]) -> f64 {
    let mut error : f64 = 0.0;
    let coef = remez(order, f_pass, f_stop, &mut error);
    let delta = 0.5 / (magnitude_response.len() - 1) as f64;
    
    //magnitude response
    for i in 0..magnitude_response.len() {
        let f = delta * i as f64;
        axis_freq[i] = f;
        magnitude_response[i] = magnitude(&coef, f).abs();
        magnitude_response[i] = 20.0 * magnitude_response[i].log10()
    }

    //copy
    for i in 0..(coef.len() - 1) {
        coefficients[i] = coef[(coef.len() - 1) - i] / 2.0;
        coefficients[(coefficients.len() - 1) - i] = coefficients[i];
    }
    coefficients[coef.len() - 1] = coef[0];
    return error;
}

fn magnitude(coefficients: &Vec<f64>, f: f64) -> f64 {
    let mut ans: f64 = 0.0;

    for i in 0..coefficients.len() {
        let omega_n = 2.0 * PI * f * i as f64;
        ans += coefficients[i] * omega_n.cos();
    }
    ans
}

fn grad_magnitude(coefficients: &Vec<f64>, f: f64) -> f64 {
    let mut ans: f64 = 0.0;

    for i in 1..coefficients.len() {
        let omega_n = 2.0 * PI * f * i as f64;
        ans -= coefficients[i] * omega_n.sin() * 2.0 * PI * i as f64;
    }
    ans
}

fn hess_magnitude(coefficients: &Vec<f64>, f: f64) -> f64 {
    let mut ans: f64 = 0.0;

    for i in 0..coefficients.len() {
        let omega_n = 2.0 * PI * f * i as f64;
        ans -= coefficients[i] * omega_n.cos() 
        * i as f64 * i as f64 * 4.0 * PI * PI;
    }
    ans
}

fn newton_method(coefficients: &Vec<f64>, x_n: f64) -> f64 {
    let mut old_point = x_n + 1.0;
    let mut new_point = x_n;
    while (new_point - old_point).abs() > STOP_CONDITION {
        old_point = new_point;
        new_point = -grad_magnitude(coefficients, old_point)
            / hess_magnitude(coefficients, old_point)
            + old_point;
    }
    new_point
}

fn divide_approximation_band(
    length: usize,
    f_pass: f64,
    f_stop: f64,
    index_edge: &mut usize,
) -> Vec<f64> {
    let mut frequency: Vec<f64> = Vec::with_capacity(length);
    let pass_band_division: usize =
        (f_pass / (0.5 - (f_stop - f_pass)) * length as f64) as usize + 1;
    let stop_band_division: usize = length - pass_band_division;
    let delta_pass_band: f64 = f_pass / (pass_band_division as f64 - 1.0);
    let delta_stop_band: f64 = 
    (0.5 - f_stop) / (stop_band_division as f64 - 1.0);

    for i in 0..pass_band_division {
        frequency.push(i as f64 * delta_pass_band);
    }

    for i in 0..stop_band_division {
        frequency.push(i as f64 * delta_stop_band + f_stop);
    }
    *index_edge = pass_band_division;
    frequency
}

fn divide_equally(length: usize) -> Vec<f64> {
    let mut frequency: Vec<f64> = Vec::with_capacity(length);
    let delta: f64 = 0.5 / (length - 1) as f64;
    for i in 0..length {
        frequency.push(delta * i as f64);
    }
    frequency
}

fn judge_convergence
(extremal_frequency: &Vec<f64>, old_extremal_frequency: &Vec<f64>) -> f64 {
    let mut max = 0.0;

    for i in 0..extremal_frequency.len() {
        let buffer = (extremal_frequency[i] - old_extremal_frequency[i]).abs();
        if max < buffer {
            max = buffer;
        }
    }
    max
}

fn desired_resonse
(response: &mut Vec<f64>, extremal_frequency: &Vec<f64>, f_pass: f64) {
    for i in 0..response.len() {
        if extremal_frequency[i] <= f_pass {
            response[i] = 1.0;
        } else {
            response[i] = 0.0;
        }
    }
}

fn judge_sign(a: f64, b: f64) -> isize {
    let sign_a: isize = if a >= 0.0 { 1 } else { -1 };
    let sign_b: isize = if b >= 0.0 { 1 } else { -1 };

    return if sign_a == sign_b { 1 } else { -1 };
}

fn adjust_candidate
(candidates: &mut Vec<f64>, length: usize, f_pass: f64, f_stop: f64) {
    if length - 1 == candidates.len() {
        let mut pass_min = std::f64::MAX;
        let mut pass_index: usize = 0;
        for i in 0..candidates.len() {
            let buff = (candidates[i] - f_pass).abs();
            if pass_min > buff {
                pass_min = buff;
                pass_index = i;
            }
        }
        let mut stop_min = std::f64::MAX;
        let mut stop_index: usize = 0;
        for i in 0..candidates.len() {
            let buff = (candidates[i] - f_stop).abs();
            if stop_min > buff {
                stop_min = buff;
                stop_index = i;
            }
        }

        if stop_min > pass_min {
            candidates[pass_index] = f_pass;
            candidates.insert(pass_index + 1, f_stop)
        } else {
            candidates[stop_index] = f_stop;
            candidates.insert(stop_index, f_pass);
        }
    } else if length - 2 == candidates.len() {
        let mut pass_min = std::f64::MAX;
        let mut pass_index: usize = 0;
        for i in 0..candidates.len() {
            let buff = (candidates[i] - f_pass).abs();
            if pass_min > buff {
                pass_min = buff;
                pass_index = i;
            }
        }

        if candidates[pass_index] > f_pass {
            candidates.insert(pass_index, f_pass);
        } else {
            candidates.insert(pass_index + 1, f_pass);
        }
        let mut stop_min = std::f64::MAX;
        let mut stop_index: usize = 0;
        for i in 0..candidates.len() {
            let buff = (candidates[i] - f_stop).abs();
            if stop_min > buff {
                stop_min = buff;
                stop_index = i;
            }
        }

        if candidates[stop_index] > f_stop {
            candidates.insert(stop_index, f_stop);
        } else {
            candidates.insert(stop_index + 1, f_stop);
        }
    }
}

fn gauss_method(matrix: &mut Vec<Vec<f64>>, vector: &mut Vec<f64>) -> i32 {
    let dimention = vector.len();
    for i in 0..(dimention - 1) {
        let mut pivot = i;
        let mut pivot_max = matrix[i][i].abs();
        for j in (i + 1)..vector.len() {
            let fact_abs = matrix[j][i].abs();

            if fact_abs > pivot_max {
                pivot_max = fact_abs;
                pivot = j;
            }
        }

        if pivot_max == 0.0 {
            return -1;
        }

        if i != pivot {
            for j in 0..dimention {
                let buffer = matrix[i][j];
                matrix[i][j] = matrix[pivot][j];
                matrix[pivot][j] = buffer;
            }

            let buffer = vector[i];
            vector[i] = vector[pivot];
            vector[pivot] = buffer;
        }

        let coefficient = 1.0 / matrix[i][i];
        matrix[i][i] = 1.0;
        for j in (i + 1)..dimention {
            matrix[i][j] *= coefficient;
        }
        vector[i] *= coefficient;

        for j in (i + 1)..dimention {
            let coeff = matrix[j][i];
            for k in (i + 1)..dimention {
                matrix[j][k] -= coeff * matrix[i][k];
            }
            matrix[j][i] = 0.0;
            vector[j] -= coeff * vector[i];
        }
    }

    vector[dimention - 1] /= matrix[dimention - 1][dimention - 1];
    for i in (0..dimention).rev() {
        for j in (i + 1)..dimention {
            vector[i] -= matrix[i][j] * vector[j];
        }
    }
    return 0;
}

fn remez(order: usize, f_pass: f64, f_stop: f64, error : &mut f64) -> Vec<f64> {
    let length = order + 2; 
    let mut index_edge: usize = 0;
    let mut cnt : usize = 0; 
    let mut old_extremal_frequency: Vec<f64> = vec![0.0; length];
    let mut matrix: Vec<Vec<f64>> = Vec::with_capacity(length);
    let mut desired_vector: Vec<f64> = vec![0.0; length];
    let mut extremal_frequency: Vec<f64> =
        divide_approximation_band(length, f_pass, f_stop, &mut index_edge);
    let divided_frequency: Vec<f64> = divide_equally(length * 10);
    let mut coef: Vec<f64> = vec![0.0; length - 1];
    let mut grad = vec![0.0; 2];

    for i in 0..length {
        let vector = Vec::with_capacity(length);
        matrix.push(vector);
        for _j in 0..length {
            matrix[i].push(0.0);
        }
    }

    while judge_convergence
    (&extremal_frequency, &old_extremal_frequency) > STOP_CONDITION {

        desired_resonse(&mut desired_vector, &extremal_frequency, f_pass);

        for i in 0..length {
            matrix[i][0] = 1.0;
            for j in 1..(length - 1) {
                let omega_n = 2.0 * PI * j as f64 * extremal_frequency[i];
                matrix[i][j] = omega_n.cos();
            }
            matrix[i][length - 1] = if i % 2 == 1 { -1.0 } else { 1.0 };
        }

        gauss_method(&mut matrix, &mut desired_vector);

        for i in 0..length {
            old_extremal_frequency[i] = extremal_frequency[i];
        }

        for i in 0..coef.len() {
            coef[i] = desired_vector[i];
        }

        let mut candidate: Vec<f64> = Vec::new();
        grad[1] = grad_magnitude(&coef, divided_frequency[1]); //勾配
        candidate.push(0.0);
        for i in 2..(divided_frequency.len() - 1) {
            grad[0] = grad_magnitude(&coef, divided_frequency[i]);
            if judge_sign(grad[0], grad[1]) == -1 {
                candidate.push(
                    newton_method(&coef, divided_frequency[i - 1]));
            }
            grad[1] = grad[0];
        }
        candidate.push(0.5);

        adjust_candidate(&mut candidate, length, f_pass, f_stop);
        //println!("ite:{}", iteration);
        //println!("{:?}", candidate);
        //println!("len:{}", candidate.len());

        for i in 0..length {
            extremal_frequency[i] = candidate[i];
        }
        cnt += 1;
        if cnt >= 100 {
            break;
        }
    }

    for i in 0..(length - 1) {
        coef[i] = desired_vector[i];
    }
    *error = desired_vector[coef.len()].abs(); 
    //println!("coefficient : {:?}", coef);
    //println!("Error : {}", desired_vector[coef.len()].abs());
    return coef;
}
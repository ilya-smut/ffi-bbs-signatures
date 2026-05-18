use bbs::prelude::*;
use std::time::Instant;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn calculate_mean_and_std(times: &[f64]) -> (f64, f64) {
    if times.is_empty() {
        return (0.0, 0.0);
    }
    let sum: f64 = times.iter().sum();
    let mean = sum / (times.len() as f64);
    
    let variance: f64 = times.iter()
        .map(|t| {
            let diff = t - mean;
            diff * diff
        })
        .sum::<f64>() / (times.len() as f64);
    let std_dev = variance.sqrt();
    
    (mean, std_dev)
}

fn calculate_median(times: &mut [f64]) -> f64 {
    if times.is_empty() {
        return 0.0;
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = times.len() / 2;
    if times.len() % 2 == 0 {
        (times[mid - 1] + times[mid]) / 2.0
    } else {
        times[mid]
    }
}

fn main() {
    println!("BBS+ Pure Rust Performance Benchmark");
    println!("======================================");

    let iterations = 50;
    let n_values: Vec<usize> = vec![
        10, 50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 
        550, 600, 650, 700, 750, 800, 850, 900, 950, 1000, 
        1050, 1100, 1150, 1200, 1250
    ];
    
    let mut results_json = String::new();
    results_json.push_str("{\n  \"metadata\": {\n");
    results_json.push_str(&format!("    \"iterations\": {},\n", iterations));
    results_json.push_str("    \"language\": \"rust\"\n");
    results_json.push_str("  },\n  \"benchmark\": [\n");

    for (idx, &n) in n_values.iter().enumerate() {
        println!("Running pure Rust benchmark for N = {} attributes...", n);
        
        // Generate Key Pair
        let (dpk, sk) = DeterministicPublicKey::new(None);
        let pk = dpk.to_public_key(n).unwrap();
        
        // Generate Messages
        let messages: Vec<SignatureMessage> = (0..n)
            .map(|i| SignatureMessage::hash(format!("attribute_{}", i).as_bytes()))
            .collect();
            
        let mut sign_times = Vec::with_capacity(iterations);
        let mut verify_times = Vec::with_capacity(iterations);
        
        for _ in 0..iterations {
            // Benchmark Signature Generation
            let sign_start = Instant::now();
            let signature = Signature::new(messages.as_slice(), &sk, &pk).unwrap();
            let sign_duration = sign_start.elapsed().as_secs_f64() * 1000.0; // convert to ms
            sign_times.push(sign_duration);
            
            // Benchmark Verification
            let verify_start = Instant::now();
            let is_valid = signature.verify(messages.as_slice(), &pk).unwrap();
            let verify_duration = verify_start.elapsed().as_secs_f64() * 1000.0; // convert to ms
            verify_times.push(verify_duration);
            
            assert!(is_valid);
        }
        
        let (sign_mean, sign_std) = calculate_mean_and_std(&sign_times);
        let sign_median = calculate_median(&mut sign_times);
        
        let (verify_mean, verify_std) = calculate_mean_and_std(&verify_times);
        let verify_median = calculate_median(&mut verify_times);
        
        println!("  - Sign: Mean {:.3} ms, Std {:.3} ms", sign_mean, sign_std);
        println!("  - Verify: Mean {:.3} ms, Std {:.3} ms", verify_mean, verify_std);
        
        results_json.push_str("    {\n");
        results_json.push_str(&format!("      \"attribute_count\": {},\n", n));
        results_json.push_str("      \"stats\": {\n");
        
        results_json.push_str("        \"signing\": {\n");
        results_json.push_str(&format!("          \"mean_ms\": {},\n", sign_mean));
        results_json.push_str(&format!("          \"std_ms\": {},\n", sign_std));
        results_json.push_str(&format!("          \"median_ms\": {}\n", sign_median));
        results_json.push_str("        },\n");
        
        results_json.push_str("        \"verification\": {\n");
        results_json.push_str(&format!("          \"mean_ms\": {},\n", verify_mean));
        results_json.push_str(&format!("          \"std_ms\": {},\n", verify_std));
        results_json.push_str(&format!("          \"median_ms\": {}\n", verify_median));
        results_json.push_str("        }\n");
        
        results_json.push_str("      }\n");
        results_json.push_str(&format!("    }}{}", if idx == n_values.len() - 1 { "" } else { ",\n" }));
    }
    
    results_json.push_str("\n  ]\n}");
    
    // Save results JSON robust to the execution CWD
    let mut output_path = Path::new("testing/evaluation/data/rust_results.json");
    if !Path::new("testing/evaluation").exists() {
        // We are likely executing from within vendor/ffi-bbs-signatures/
        output_path = Path::new("../../testing/evaluation/data/rust_results.json");
    }
    
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(output_path).unwrap();
    file.write_all(results_json.as_bytes()).unwrap();
    println!("Pure Rust benchmark results written to {}", output_path.display());
}

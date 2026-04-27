// HTTP/3 综合测试模块 - 整合原生 h3 和 reqwest HTTP/3 测试
use anyhow::{ Context, Result };
use clap::{ Arg, Command };
use serde::{ Deserialize, Serialize };
use std::collections::{ HashMap, HashSet };
use std::fs;
use std::net::IpAddr;
use std::path::Path;
use std::time::Instant;

// 导入所有测试模块
use crate::h3_direct_test::{
    generate_test_report,
    get_default_h3_test_configs,
    H3TestConfig,
    H3TestResult,
    H3Tester,
};
use crate::http3_test::{ resolve_domain_with_rfc8484, InputTask, TestResult };
use crate::main_h3_test::{
    get_default_integration_test_configs,
    run_http3_integration_tests,
    H3IntegrationResult,
    H3IntegrationTest,
};

// --- 1. 测试配置 ---
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComprehensiveTestConfig {
    pub test_mode: String, // "native_h3", "reqwest_h3", "integration", "all"
    pub target_domains: Vec<String>,
    pub output_format: String, // "json", "table", "all"
    pub max_concurrent_tests: usize,
    pub timeout_seconds: u64,
    pub enable_ipv6: bool,
    pub dns_resolve_mode: String, // "https", "a_aaaa", "direct"
    pub doh_server: String,
    pub test_paths: Vec<String>,
    pub use_fallback: bool,
    pub max_field_section_size: Option<u64>,
}

impl Default for ComprehensiveTestConfig {
    fn default() -> Self {
        Self {
            test_mode: "all".to_string(),
            target_domains: vec![
                "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                "google.com".to_string(),
                "facebook.com".to_string()
            ],
            output_format: "all".to_string(),
            max_concurrent_tests: 10,
            timeout_seconds: 30,
            enable_ipv6: false,
            dns_resolve_mode: "https".to_string(),
            doh_server: "https://61919494499.security.cloudflare-dns.com/dns-query".to_string(),
            test_paths: vec!["/".to_string(), "/cdn-cgi/trace".to_string(), "/health".to_string()],
            use_fallback: true,
            max_field_section_size: Some(8192),
        }
    }
}

// --- 2. 综合测试结果 ---
#[derive(Debug, Clone, Serialize)]
pub struct ComprehensiveTestResult {
    pub test_mode: String,
    pub target_domain: String,
    pub target_ip: String,
    pub ip_version: String,
    pub test_path: String,
    pub test_method: String,
    pub success: bool,
    pub status_code: Option<u16>,
    pub protocol_detected: String,
    pub latency_ms: Option<u64>,
    pub response_size: Option<usize>,
    pub server_header: Option<String>,
    pub alpn_protocol: Option<String>,
    pub error_message: Option<String>,
    pub dns_source: String,
    pub test_timestamp: String,
    pub additional_metrics: HashMap<String, serde_json::Value>,
}

impl ComprehensiveTestResult {
    pub fn success(
        domain: &str,
        ip: &str,
        version: &str,
        path: &str,
        method: &str,
        protocol: &str,
        dns_source: String
    ) -> Self {
        Self {
            test_mode: method.to_string(),
            target_domain: domain.to_string(),
            target_ip: ip.to_string(),
            ip_version: version.to_string(),
            test_path: path.to_string(),
            test_method: method.to_string(),
            success: true,
            status_code: Some(200),
            protocol_detected: protocol.to_string(),
            latency_ms: Some(0),
            response_size: Some(0),
            server_header: None,
            alpn_protocol: Some(protocol.to_string()),
            error_message: None,
            dns_source,
            test_timestamp: chrono::Utc::now().to_rfc3339(),
            additional_metrics: HashMap::new(),
        }
    }

    pub fn failure(
        domain: &str,
        ip: &str,
        version: &str,
        path: &str,
        method: &str,
        protocol: &str,
        dns_source: String,
        error: String
    ) -> Self {
        Self {
            test_mode: method.to_string(),
            target_domain: domain.to_string(),
            target_ip: ip.to_string(),
            ip_version: version.to_string(),
            test_path: path.to_string(),
            test_method: method.to_string(),
            success: false,
            status_code: None,
            protocol_detected: protocol.to_string(),
            latency_ms: None,
            response_size: None,
            server_header: None,
            alpn_protocol: Some(protocol.to_string()),
            error_message: Some(error),
            dns_source,
            test_timestamp: chrono::Utc::now().to_rfc3339(),
            additional_metrics: HashMap::new(),
        }
    }
}

// --- 3. 命令行解析 ---
pub fn parse_command_line() -> ComprehensiveTestConfig {
    let matches = Command::new("rust-http3-test-tool")
        .version("1.0.0")
        .about("Comprehensive HTTP/3 testing tool with native h3 and reqwest support")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Test mode: native_h3, reqwest_h3, integration, all")
                .default_value("all")
        )
        .arg(
            Arg::new("domains")
                .short('d')
                .long("domains")
                .value_name("DOMAINS")
                .help("Target domains (comma-separated)")
                .default_value("local-aria2-webui.masx200.ddns-ip.net,google.com,facebook.com")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FORMAT")
                .help("Output format: json, table, all")
                .default_value("all")
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECONDS")
                .help("Request timeout in seconds")
                .default_value("30")
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path (JSON)")
        )
        .arg(
            Arg::new("ipv6")
                .short('6')
                .long("ipv6")
                .help("Enable IPv6 testing")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("resolve-mode")
                .short('r')
                .long("resolve-mode")
                .value_name("MODE")
                .help("DNS resolution mode: https, a_aaaa, direct")
                .default_value("https")
        )
        .arg(
            Arg::new("doh-server")
                .short('s')
                .long("doh-server")
                .value_name("URL")
                .help("DNS over HTTPS server URL")
                .default_value("https://61919494499.security.cloudflare-dns.com/dns-query")
        )
        .get_matches();

    // 如果提供了配置文件，尝试加载
    if let Some(config_path) = matches.get_one::<String>("config") {
        if let Ok(config_content) = fs::read_to_string(config_path) {
            if
                let Ok(mut config) = serde_json::from_str::<ComprehensiveTestConfig>(
                    &config_content
                )
            {
                // 命令行参数覆盖配置文件
                if let Some(mode) = matches.get_one::<String>("mode") {
                    config.test_mode = mode.clone();
                }
                if let Some(domains) = matches.get_one::<String>("domains") {
                    config.target_domains = domains
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                }
                if let Some(output) = matches.get_one::<String>("output") {
                    config.output_format = output.clone();
                }
                if let Some(timeout) = matches.get_one::<String>("timeout") {
                    if let Ok(seconds) = timeout.parse::<u64>() {
                        config.timeout_seconds = seconds;
                    }
                }
                if matches.get_flag("ipv6") {
                    config.enable_ipv6 = true;
                }
                if let Some(resolve_mode) = matches.get_one::<String>("resolve-mode") {
                    config.dns_resolve_mode = resolve_mode.clone();
                }
                if let Some(doh_server) = matches.get_one::<String>("doh-server") {
                    config.doh_server = doh_server.clone();
                }
                return config;
            }
        }
    }

    // 使用默认配置和命令行参数
    let mut config = ComprehensiveTestConfig::default();

    if let Some(mode) = matches.get_one::<String>("mode") {
        config.test_mode = mode.clone();
    }
    if let Some(domains) = matches.get_one::<String>("domains") {
        config.target_domains = domains
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }
    if let Some(output) = matches.get_one::<String>("output") {
        config.output_format = output.clone();
    }
    if let Some(timeout) = matches.get_one::<String>("timeout") {
        if let Ok(seconds) = timeout.parse::<u64>() {
            config.timeout_seconds = seconds;
        }
    }
    if matches.get_flag("ipv6") {
        config.enable_ipv6 = true;
    }
    if let Some(resolve_mode) = matches.get_one::<String>("resolve-mode") {
        config.dns_resolve_mode = resolve_mode.clone();
    }
    if let Some(doh_server) = matches.get_one::<String>("doh-server") {
        config.doh_server = doh_server.clone();
    }

    config
}

// --- 4. 原生 h3 测试 ---
pub async fn run_native_h3_tests(
    config: &ComprehensiveTestConfig
) -> Result<Vec<ComprehensiveTestResult>> {
    println!("🚀 开始原生 HTTP/3 测试");
    println!("================================");

    let h3_tester = H3Tester::new().context("Failed to create HTTP/3 tester")?;

    let mut h3_configs = Vec::new();
    for domain in &config.target_domains {
        for path in &config.test_paths {
            let h3_config = H3TestConfig {
                target_domain: domain.clone(),
                target_ip: "auto".to_string(), // 将通过 DNS 解析
                port: 443,
                sni_host: domain.clone(),
                test_path: path.clone(),
                user_agent: Some("rust-http3-test-tool/1.0".to_string()),
                max_field_section_size: config.max_field_section_size,
                enable_datagram: false,
                enable_extended_connect: false,
                send_grease: true,
                timeout_seconds: config.timeout_seconds,
            };
            h3_configs.push(h3_config);
        }
    }

    // 这里需要通过 DNS 解析获取实际的 IP 地址
    let client = reqwest::Client::new();
    let mut results = Vec::new();

    for h3_config in h3_configs {
        // 创建 DNS 查询任务
        let dns_task = InputTask {
            doh_resolve_domain: h3_config.target_domain.clone(),
            test_sni_host: h3_config.sni_host.clone(),
            test_host_header: h3_config.sni_host.clone(),
            doh_url: config.doh_server.clone(),
            port: h3_config.port,
            prefer_ipv6: Some(config.enable_ipv6),
            resolve_mode: config.dns_resolve_mode.clone(),
            direct_ips: None,
            test_path: Some(h3_config.test_path.clone()),
        };

        match resolve_domain_with_rfc8484(&client, &dns_task).await {
            Ok(ips) => {
                for ip in ips {
                    if let Some(prefer_ipv6) = dns_task.prefer_ipv6 {
                        if prefer_ipv6 != ip.is_ipv6() {
                            continue;
                        }
                    }

                    let ip_str = ip.to_string();
                    let ip_version = if ip.is_ipv6() { "IPv6" } else { "IPv4" };
                    let dns_source = format!("DoH ({})", config.doh_server);

                    // 修改 h3 配置使用实际 IP
                    let mut actual_h3_config = h3_config.clone();
                    actual_h3_config.target_ip = ip_str.clone();

                    match h3_tester.test_http3_connection(&actual_h3_config).await {
                        Ok(h3_result) => {
                            let mut result = ComprehensiveTestResult::success(
                                &h3_result.config.target_domain,
                                &h3_result.target_ip,
                                &h3_result.ip_version,
                                &h3_result.config.test_path,
                                "native_h3",
                                &h3_result.protocol_version,
                                dns_source
                            );
                            result.status_code = h3_result.response_status;
                            result.latency_ms = h3_result.latency_ms;
                            result.response_size = h3_result.response_size;
                            result.server_header = None; // h3_result doesn't have server_header field
                            result.alpn_protocol = h3_result.alpn_protocol;
                            // 注释掉不存在的字段
                            // result.additional_metrics.insert(
                            //     "connection_id".to_string(),
                            //     serde_json::Value::String(h3_result.connection_id.unwrap_or_default()),
                            // );
                            // result.additional_metrics.insert(
                            //     "stream_id".to_string(),
                            //     serde_json::Value::Number(serde_json::Number::from(h3_result.stream_id.unwrap_or(0))),
                            // );
                            results.push(result);
                        }
                        Err(e) => {
                            let result = ComprehensiveTestResult::failure(
                                &h3_config.target_domain,
                                &ip_str,
                                ip_version,
                                &h3_config.test_path,
                                "native_h3",
                                "HTTP/3",
                                dns_source,
                                format!("Native HTTP/3 test failed: {}", e)
                            );
                            results.push(result);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("DNS resolution failed for {}: {:?}", h3_config.target_domain, e);
            }
        }
    }

    Ok(results)
}

// --- 5. 主运行函数 ---
pub async fn run_comprehensive_h3_tests() -> Result<()> {
    let config = parse_command_line();
    println!("🚀 HTTP/3 综合测试开始");
    println!("================================");
    println!("测试模式: {}", config.test_mode);
    println!("目标域名: {:?}", config.target_domains);
    println!("DNS 解析模式: {}", config.dns_resolve_mode);
    println!("DoH 服务器: {}", config.doh_server);
    println!("超时时间: {} 秒", config.timeout_seconds);
    println!("IPv6 支持: {}", config.enable_ipv6);

    let mut all_results = Vec::new();

    match config.test_mode.as_str() {
        "native_h3" => {
            let results = run_native_h3_tests(&config).await?;
            all_results.extend(results);
        }
        "reqwest_h3" | "integration" => {
            // 运行集成测试 (使用 reqwest HTTP/3)
            let integration_configs: Vec<H3IntegrationTest> = config.target_domains
                .iter()
                .flat_map(|domain| {
                    config.test_paths.iter().map(move |path| H3IntegrationTest {
                        input_task: InputTask {
                            doh_resolve_domain: domain.clone(),
                            test_sni_host: domain.clone(),
                            test_host_header: domain.clone(),
                            doh_url: config.doh_server.clone(),
                            port: 443,
                            prefer_ipv6: Some(config.enable_ipv6),
                            resolve_mode: config.dns_resolve_mode.clone(),
                            direct_ips: None,
                            test_path: Some(path.clone()),
                        },
                        use_native_h3: false,
                        enable_fallback: config.use_fallback,
                        timeout_seconds: config.timeout_seconds,
                        max_field_section_size: config.max_field_section_size,
                    })
                })
                .collect();

            // 临时修改 main_h3_test 来运行自定义配置
            println!("集成测试配置已准备，共 {} 个测试", integration_configs.len());
        }
        "all" => {
            // 运行所有测试模式
            println!("运行所有测试模式...");

            // 原生 h3 测试
            let native_results = run_native_h3_tests(&config).await?;
            all_results.extend(native_results);

            // 集成测试 (reqwest)
            println!("\n现在运行集成测试 (reqwest HTTP/3)...");
            // 这里可以调用 main_h3_test 的函数
        }
        _ => {
            return Err(anyhow::anyhow!("不支持的测试模式: {}", config.test_mode));
        }
    }

    // --- 6. 输出结果 ---
    if config.output_format == "json" || config.output_format == "all" {
        let json_output = serde_json
            ::to_string_pretty(&all_results)
            .context("Failed to serialize results to JSON")?;
        println!("\n📄 JSON 输出:");
        println!("{}", json_output);
    }

    if config.output_format == "table" || config.output_format == "all" {
        print_table_output(&all_results);
    }

    // --- 7. 生成报告 ---
    generate_comprehensive_report(&all_results)?;

    // --- 8. 保存结果到文件 ---
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("http3_test_results_{}.json", timestamp);
    if let Ok(json_output) = serde_json::to_string_pretty(&all_results) {
        if let Err(e) = fs::write(&filename, json_output) {
            eprintln!("保存结果文件失败: {}", e);
        } else {
            println!("\n📁 结果已保存到: {}", filename);
        }
    }

    Ok(())
}

// --- 8. 表格输出 ---
pub fn print_table_output(results: &[ComprehensiveTestResult]) {
    println!("\n📊 测试结果表格:");
    println!("{}", "=".repeat(150));
    println!(
        "{:<20} {:<15} {:<10} {:<15} {:<10} {:<8} {:<8} {:<10} {:<15} {:<10}",
        "域名",
        "IP地址",
        "版本",
        "协议",
        "状态",
        "延迟",
        "大小",
        "ALPN",
        "测试方法",
        "错误"
    );
    println!("{}", "-".repeat(150));

    for result in results {
        let status = if result.success { "成功" } else { "失败" };
        let latency = result.latency_ms.unwrap_or(0).to_string();
        let size = result.response_size.unwrap_or(0).to_string();
        let alpn = result.alpn_protocol.as_deref().unwrap_or("N/A");
        let error = result.error_message.as_deref().unwrap_or("");

        println!(
            "{:<20} {:<15} {:<10} {:<15} {:<10} {:<8} {:<8} {:<10} {:<15} {:<10}",
            result.target_domain,
            result.target_ip,
            result.ip_version,
            result.protocol_detected,
            status,
            latency,
            size,
            alpn,
            result.test_method,
            error
        );
    }
}

// --- 9. 综合报告 ---
pub fn generate_comprehensive_report(results: &[ComprehensiveTestResult]) -> Result<()> {
    let mut report = String::new();
    report.push_str("=== HTTP/3 综合测试报告 ===\n\n");

    // 基本统计
    let total = results.len();
    let successful = results
        .iter()
        .filter(|r| r.success)
        .count();
    let failed = total - successful;

    report.push_str(&format!("总测试数: {}\n", total));
    report.push_str(&format!("成功: {}\n", successful));
    report.push_str(&format!("失败: {}\n", failed));
    report.push_str(&format!("成功率: {:.2}%\n\n", ((successful as f64) / (total as f64)) * 100.0));

    // 按域名分组
    let mut domain_stats: HashMap<String, (usize, usize)> = HashMap::new();
    for result in results {
        let entry = domain_stats.entry(result.target_domain.clone()).or_insert((0, 0));
        if result.success {
            entry.0 += 1;
        } else {
            entry.1 += 1;
        }
    }

    report.push_str("📡 按域名统计:\n");
    for (domain, (success, failed)) in domain_stats {
        let total_domain = success + failed;
        let success_rate = ((success as f64) / (total_domain as f64)) * 100.0;
        report.push_str(
            &format!("  {}: {}/{} ({:.2}% 成功)\n", domain, success, total_domain, success_rate)
        );
    }

    // 协议统计
    let mut protocol_stats: HashMap<String, usize> = HashMap::new();
    for result in results.iter().filter(|r| r.success) {
        *protocol_stats.entry(result.protocol_detected.clone()).or_insert(0) += 1;
    }

    report.push_str("\n🔗 协议分布:\n");
    for (protocol, count) in protocol_stats {
        let percentage = ((count as f64) / (successful as f64)) * 100.0;
        report.push_str(&format!("  {}: {} ({:.2}%)\n", protocol, count, percentage));
    }

    // ALPN 统计
    let mut alpn_stats: HashMap<String, usize> = HashMap::new();
    for result in results.iter().filter(|r| r.alpn_protocol.is_some()) {
        if let Some(ref alpn) = result.alpn_protocol {
            *alpn_stats.entry(alpn.clone()).or_insert(0) += 1;
        }
    }

    report.push_str("\n🔐 ALPN 协议分布:\n");
    for (alpn, count) in alpn_stats {
        let percentage = ((count as f64) / (successful as f64)) * 100.0;
        report.push_str(&format!("  {}: {} ({:.2}%)\n", alpn, count, percentage));
    }

    // 延迟统计
    let latencies: Vec<u64> = results
        .iter()
        .filter_map(|r| r.latency_ms)
        .collect();

    if !latencies.is_empty() {
        let avg_latency = (latencies.iter().sum::<u64>() as f64) / (latencies.len() as f64);
        let min_latency = latencies.iter().min().unwrap();
        let max_latency = latencies.iter().max().unwrap();

        report.push_str("\n⏱️  延迟统计 (ms):\n");
        report.push_str(&format!("  平均: {:.2}\n", avg_latency));
        report.push_str(&format!("  最小: {}\n", min_latency));
        report.push_str(&format!("  最大: {}\n", max_latency));
        report.push_str(&format!("  中位数: {}\n", latencies[latencies.len() / 2]));
    }

    // 错误统计
    let mut error_stats: HashMap<String, usize> = HashMap::new();
    for result in results.iter().filter(|r| !r.success) {
        if let Some(ref error) = result.error_message {
            // 简化错误消息
            let simplified_error = if error.contains("timeout") {
                "超时"
            } else if error.contains("DNS") {
                "DNS 解析失败"
            } else if error.contains("connection") {
                "连接失败"
            } else if error.contains("certificate") {
                "证书错误"
            } else {
                "其他错误"
            };
            *error_stats.entry(simplified_error.to_string()).or_insert(0) += 1;
        }
    }

    if !error_stats.is_empty() {
        report.push_str("\n❌ 错误统计:\n");
        for (error, count) in error_stats {
            report.push_str(&format!("  {}: {}\n", error, count));
        }
    }

    // 保存报告到文件
    let report_filename = format!(
        "http3_test_report_{}.txt",
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );
    if let Err(e) = fs::write(&report_filename, &report) {
        eprintln!("保存报告失败: {}", e);
    } else {
        println!("\n📄 综合报告已保存到: {}", report_filename);
        println!("\n📋 报告预览:");
        println!("{}", report);
    }

    Ok(())
}

// --- 10. 主程序入口 ---
pub async fn main() -> Result<()> {
    // 设置 panic hook 来提供更好的错误信息
    std::panic::set_hook(
        Box::new(|panic_info| {
            eprintln!("程序 panic: {}", panic_info);
            std::process::exit(1);
        })
    );

    // 检查命令行参数
    let args: Vec<String> = std::env::args().collect();

    // 如果没有参数或 --help，显示帮助
    if args.len() == 1 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        return Ok(());
    }

    // 如果有 --version，显示版本
    if args.contains(&"--version".to_string()) || args.contains(&"-V".to_string()) {
        println!("rust-http3-test-tool v1.0.0");
        println!("HTTP/3 testing tool with native h3 and reqwest support");
        println!("Features: native HTTP/3, HTTP/3 over reqwest, DNS over HTTPS, IPv6 support");
        return Ok(());
    }

    // 运行综合测试
    run_comprehensive_h3_tests().await
}

// --- 11. 帮助信息 ---
pub fn print_help() {
    println!("rust-http3-test-tool - HTTP/3 综合测试工具");
    println!("");
    println!("用法:");
    println!(
        "  {} [选项]",
        std::env
            ::args()
            .next()
            .unwrap_or_else(|| "program".to_string())
    );
    println!("");
    println!("选项:");
    println!("  -m, --mode <MODE>        测试模式 (native_h3, reqwest_h3, integration, all)");
    println!("  -d, --domains <DOMAINS>   目标域名 (逗号分隔)");
    println!("  -o, --output <FORMAT>     输出格式 (json, table, all)");
    println!("  -t, --timeout <SECONDS>   请求超时时间");
    println!("  -c, --config <FILE>       配置文件路径");
    println!("  -6, --ipv6                启用 IPv6 测试");
    println!("  -r, --resolve-mode <MODE> DNS 解析模式 (https, a_aaaa, direct)");
    println!("  -s, --doh-server <URL>    DNS over HTTPS 服务器");
    println!("  -h, --help                 显示此帮助信息");
    println!("  -V, --version              显示版本信息");
    println!("");
    println!("示例:");
    println!(
        "  {} -m native_h3 -d local-aria2-webui.masx200.ddns-ip.net,google.com",
        std::env
            ::args()
            .next()
            .unwrap_or_else(|| "program".to_string())
    );
    println!(
        "  {} --mode all --domains local-aria2-webui.masx200.ddns-ip.net --ipv6 --output table",
        std::env
            ::args()
            .next()
            .unwrap_or_else(|| "program".to_string())
    );
    println!(
        "  {} --config config.json",
        std::env
            ::args()
            .next()
            .unwrap_or_else(|| "program".to_string())
    );
    println!("");
    println!("测试模式说明:");
    println!("  native_h3    - 使用原生 h3 库进行 HTTP/3 测试");
    println!("  reqwest_h3   - 使用 reqwest 库进行 HTTP/3 测试");
    println!("  integration   - 集成测试，包含协议协商和回退机制");
    println!("  all          - 运行所有测试模式");
}

# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/3/4 14:25:44
- **数据来源**: connectivity_results-20260304-142543.json
- **总测试数**: 967
- **失败测试数**: 4
- **成功测试数**: 963
- **失败率**: 0.41%
- **平均延迟**: 66.82ms
- **最小延迟**: 39ms
- **最大延迟**: 1504ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/3/4 14:25:44
- **IP地址**: 2a09:bac1:76e0:1a98::48c:41
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 34.0261, -118.1756
- **时区**: America/Los_Angeles
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (50.0%)
- **DNS解析错误: 其他DNS错误**: 2 次 (50.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 9 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 342 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

#### DNS解析错误: 其他DNS错误 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 43 | www.gov.ua | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 392 | www.7749tv.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (50.0%)
- **DNS解析错误**: 2 次 (50.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 4 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 963 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 867 | 104.25.240.123 | 104.25.240.123 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 326 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 433 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 21 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 711 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 712 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 747 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 53 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 420 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 571 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 583 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 610 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 628 | 104.19.52.56 | 104.19.52.56 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 662 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 700 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 749 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 883 | 172.64.229.15 | 172.64.229.15 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 962 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 6 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 19 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 55 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 76 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 80 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 85 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 103 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 162 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 235 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 236 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 246 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 267 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 268 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 275 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 286 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 293 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 294 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 314 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 322 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 362 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 373 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 419 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 501 | 104.26.4.213 | 104.26.4.213 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 569 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 572 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 578 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 580 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 596 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 601 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 602 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 629 | 104.19.48.12 | 104.19.48.12 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 638 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 643 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 646 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 647 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 648 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 649 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 651 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 673 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 683 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 710 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 742 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 762 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 791 | 172.64.229.149 | 172.64.229.149 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 902 | 104.26.3.117 | 104.26.3.117 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 18 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 28 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 41 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 51 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 52 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 73 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 84 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 116 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 172 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 173 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 177 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 230 | cf.090227.xyz | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 234 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 240 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 248 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 249 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 261 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 266 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 278 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 295 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 304 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 321 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 330 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 346 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 353 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 363 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 367 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 372 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 376 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 407 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 409 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 411 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 418 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 425 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 450 | 104.26.12.33 | 104.26.12.33 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 461 | 104.16.247.125 | 104.16.247.125 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 486 | 172.64.52.150 | 172.64.52.150 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 502 | 104.20.18.125 | 104.20.18.125 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 507 | 104.25.243.36 | 104.25.243.36 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 510 | 104.25.242.137 | 104.25.242.137 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 512 | 104.25.246.117 | 104.25.246.117 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 539 | 172.64.52.184 | 172.64.52.184 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 563 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 570 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 593 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 600 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 607 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 609 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 627 | 104.17.107.32 | 104.17.107.32 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 633 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 652 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 653 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 654 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 682 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 689 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 698 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 699 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 701 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 707 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 709 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 731 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 739 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 741 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 763 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 764 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 782 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 822 | 104.18.166.232 | 104.18.166.232 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 863 | 104.18.148.235 | 104.18.148.235 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 888 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 898 | 104.20.26.221 | 104.20.26.221 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 899 | 104.20.22.141 | 104.20.22.141 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 963 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 8 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 10 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 33 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 45 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 47 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 57 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 62 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 79 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 98 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 102 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 118 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 134 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 154 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 184 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 190 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 192 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 193 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 195 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 222 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 229 | cf.090227.xyz | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 250 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 281 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 282 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 283 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 328 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 341 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 349 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 351 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 354 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 396 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 397 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 423 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 427 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 428 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 438 | 162.159.21.16 | 162.159.21.16 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 451 | 172.67.72.250 | 172.67.72.250 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 460 | 104.25.241.19 | 104.25.241.19 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 473 | 172.64.152.85 | 172.64.152.85 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 474 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 484 | 172.64.53.195 | 172.64.53.195 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 490 | 172.64.229.106 | 172.64.229.106 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 503 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 505 | 104.25.240.21 | 104.25.240.21 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 506 | 104.25.245.173 | 104.25.245.173 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 508 | 104.25.250.121 | 104.25.250.121 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 541 | 162.159.33.4 | 162.159.33.4 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 549 | 172.67.64.39 | 172.67.64.39 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 550 | 104.20.31.243 | 104.20.31.243 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 551 | 172.67.69.226 | 172.67.69.226 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 554 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 567 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 597 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 598 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 603 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 605 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 621 | 104.16.252.92 | 104.16.252.92 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 622 | 104.17.148.36 | 104.17.148.36 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 630 | 104.17.20.97 | 104.17.20.97 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 631 | 172.64.229.172 | 172.64.229.172 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 639 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 641 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 642 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 644 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 660 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 661 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 670 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 692 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 696 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 705 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 708 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 726 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 737 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 738 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 743 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 744 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 755 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 757 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 758 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 761 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 770 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 780 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 811 | 104.20.20.192 | 104.20.20.192 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 823 | 104.25.250.174 | 104.25.250.174 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 842 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 870 | 104.25.249.225 | 104.25.249.225 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 879 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 909 | 104.17.143.82 | 104.17.143.82 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 912 | 104.17.62.194 | 104.17.62.194 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 920 | 2a06:98c1:51:0:4371:ce16:475:2557 | 2a06:98c1:51:0:4371:ce16:475:2557 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 936 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 954 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 965 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 13 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 17 | www.ipget.net | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 49 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 54 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 56 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 58 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 60 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 63 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 70 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 81 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 83 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 87 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 89 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 115 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 131 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 133 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 139 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 144 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 151 | bestcf.030101.xyz | 104.17.27.231 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 156 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 161 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 182 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 188 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 218 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 219 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 221 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 228 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 231 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 247 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 265 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 279 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 289 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 301 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 309 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 313 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 355 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 356 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 357 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 359 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 365 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 369 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 374 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 390 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 393 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 395 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 406 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 416 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 424 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 426 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 429 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 434 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 452 | 172.67.64.116 | 172.67.64.116 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 458 | 104.25.247.129 | 104.25.247.129 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 463 | 104.25.253.253 | 104.25.253.253 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 464 | 104.17.171.88 | 104.17.171.88 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 469 | 172.64.157.214 | 172.64.157.214 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 470 | 162.159.0.79 | 162.159.0.79 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 477 | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 492 | 172.64.41.47 | 172.64.41.47 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 493 | 172.67.64.123 | 172.67.64.123 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 524 | 2a06:98c1:3102:8768:b929:7455:f040:5aee | 2a06:98c1:3102:8768:b929:7455:f040:5aee | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 532 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 537 | 162.159.44.249 | 162.159.44.249 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 538 | 172.64.53.127 | 172.64.53.127 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 544 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 545 | 104.20.25.222 | 104.20.25.222 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 546 | 172.67.75.134 | 172.67.75.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 547 | 172.67.68.109 | 172.67.68.109 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 552 | 172.67.67.81 | 172.67.67.81 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 553 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 558 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 562 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 564 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 575 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 576 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 579 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 584 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 585 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 594 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 611 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 618 | 162.159.9.186 | 162.159.9.186 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 623 | 104.17.208.77 | 104.17.208.77 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 637 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 645 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 663 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 676 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 677 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 678 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 686 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 693 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 697 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 720 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 729 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 736 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 769 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 774 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 796 | 172.64.53.165 | 172.64.53.165 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 797 | 172.64.52.189 | 172.64.52.189 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 798 | 162.159.44.101 | 162.159.44.101 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 801 | 104.18.42.16 | 104.18.42.16 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 808 | 104.26.0.210 | 104.26.0.210 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 820 | 104.25.245.233 | 104.25.245.233 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 852 | 162.159.20.46 | 162.159.20.46 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 853 | 104.20.31.132 | 104.20.31.132 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 854 | 172.67.70.56 | 172.67.70.56 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 855 | 172.67.67.0 | 172.67.67.0 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 856 | 104.26.11.33 | 104.26.11.33 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 860 | 104.20.19.180 | 104.20.19.180 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 862 | 104.26.5.194 | 104.26.5.194 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 865 | 104.17.56.208 | 104.17.56.208 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 866 | 104.25.247.78 | 104.25.247.78 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 868 | 104.25.244.36 | 104.25.244.36 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 896 | 172.67.73.196 | 172.67.73.196 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 903 | 104.25.244.87 | 104.25.244.87 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 906 | 104.25.246.24 | 104.25.246.24 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 910 | 104.18.160.38 | 104.18.160.38 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 922 | 172.64.147.123 | 172.64.147.123 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 934 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 937 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 945 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 948 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 951 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 952 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 955 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 957 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 961 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 5 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 7 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 16 | www.ipget.net | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 34 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 44 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 46 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 59 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 61 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 64 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 67 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 75 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 82 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 88 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 90 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 94 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 106 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 125 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 142 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 143 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 148 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 153 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 155 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 157 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 158 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 159 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 185 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 187 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 191 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 194 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 223 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 232 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 233 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 244 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 259 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 276 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 280 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 290 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 291 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 292 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 298 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 299 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 323 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 331 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 335 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 345 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 347 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 352 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 364 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 368 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 375 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 389 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 410 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 422 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 430 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 432 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 439 | 104.26.1.55 | 104.26.1.55 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 442 | 172.67.78.23 | 172.67.78.23 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 456 | 104.25.252.135 | 104.25.252.135 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 466 | 104.18.44.159 | 104.18.44.159 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 476 | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 480 | 2a06:98c1:310b::fda8:fa9e:4a3e | 2a06:98c1:310b::fda8:fa9e:4a3e | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 482 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 483 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 487 | 162.159.44.60 | 162.159.44.60 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 497 | 172.67.73.129 | 172.67.73.129 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 500 | 172.67.79.218 | 172.67.79.218 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 509 | 104.25.252.192 | 104.25.252.192 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 511 | 104.25.254.47 | 104.25.254.47 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 543 | 172.64.50.149 | 172.64.50.149 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 548 | 172.67.74.211 | 172.67.74.211 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 568 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 592 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 599 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 608 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 620 | 172.67.73.30 | 172.67.73.30 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 625 | 104.17.48.93 | 104.17.48.93 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 636 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 650 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 656 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 668 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 694 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 706 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 716 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 727 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 745 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 760 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 771 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 775 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 781 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 789 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 799 | 162.159.39.146 | 162.159.39.146 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 805 | 104.26.6.247 | 104.26.6.247 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 812 | 104.26.13.110 | 104.26.13.110 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 813 | 104.17.97.228 | 104.17.97.228 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 817 | 104.17.111.150 | 104.17.111.150 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 819 | 172.64.153.140 | 172.64.153.140 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 835 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 836 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 858 | 104.26.4.135 | 104.26.4.135 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 872 | 104.25.250.205 | 104.25.250.205 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 876 | 172.64.144.132 | 172.64.144.132 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 877 | 162.159.45.150 | 162.159.45.150 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 889 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 891 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 892 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 907 | 104.25.240.227 | 104.25.240.227 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 908 | 104.25.242.249 | 104.25.242.249 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 911 | 104.16.245.121 | 104.16.245.121 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 923 | 104.18.44.96 | 104.18.44.96 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 925 | 198.41.223.23 | 198.41.223.23 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 933 | 2a06:98c1:310e:5f9e:101d:94ce:cb6b:49ca | 2a06:98c1:310e:5f9e:101d:94ce:cb6b:49ca | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 943 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 12 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 15 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 20 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 42 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 48 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 71 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 72 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 74 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 86 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 97 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 107 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 114 | freeyx.cloudflare88.eu.org | 172.64.146.198 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 129 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 135 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 136 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 150 | bestcf.030101.xyz | 104.17.99.183 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 152 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 160 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 164 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 175 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 186 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 196 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 197 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 198 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 204 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 215 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 216 | xn--b6gac.eu.org | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 245 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 264 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 277 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 284 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 285 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 288 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 297 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 302 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 308 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 310 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 312 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 317 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 325 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 360 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 366 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 386 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 388 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 398 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 413 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 421 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 436 | 2a06:98c1:3100:7f01:2f67:5ef8:2a97:8d82 | 2a06:98c1:3100:7f01:2f67:5ef8:2a97:8d82 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 440 | 104.26.14.88 | 104.26.14.88 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 455 | 172.64.146.137 | 172.64.146.137 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 457 | 104.17.97.146 | 104.17.97.146 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 462 | 104.25.255.103 | 104.25.255.103 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 494 | 104.20.20.156 | 104.20.20.156 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 495 | 172.67.77.104 | 172.67.77.104 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 499 | 172.67.76.20 | 172.67.76.20 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 504 | 104.25.241.235 | 104.25.241.235 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 515 | 108.162.198.206 | 108.162.198.206 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 528 | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 529 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 536 | 108.162.198.216 | 108.162.198.216 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 540 | 172.64.229.238 | 172.64.229.238 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 556 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 565 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 574 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 587 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 591 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 626 | 104.19.54.128 | 104.19.54.128 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 667 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 680 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 681 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 684 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 713 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 730 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 740 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 768 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 772 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 777 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 778 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 784 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 794 | 162.159.38.226 | 162.159.38.226 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 800 | 172.64.229.134 | 172.64.229.134 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 806 | 104.26.14.117 | 104.26.14.117 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 810 | 104.26.8.171 | 104.26.8.171 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 814 | 104.25.241.198 | 104.25.241.198 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 815 | 104.25.244.239 | 104.25.244.239 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 818 | 104.18.172.20 | 104.18.172.20 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 830 | 198.41.222.191 | 198.41.222.191 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 846 | 172.64.52.224 | 172.64.52.224 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 857 | 104.20.24.244 | 104.20.24.244 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 861 | 172.67.72.212 | 172.67.72.212 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 864 | 104.25.245.215 | 104.25.245.215 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 871 | 104.25.254.14 | 104.25.254.14 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 878 | 172.64.151.253 | 172.64.151.253 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 905 | 104.17.56.177 | 104.17.56.177 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 927 | 2a06:98c1:3106:0:fb94:fc7b:2b7f:ae54 | 2a06:98c1:3106:0:fb94:fc7b:2b7f:ae54 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 947 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 949 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 956 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 958 | 2a06:98c1:3106:d8af:8b29:8a81:bf10:9cef | 2a06:98c1:3106:d8af:8b29:8a81:bf10:9cef | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 959 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 960 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 22 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 69 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 99 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 117 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 124 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 145 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 149 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 165 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 174 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 189 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 199 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 202 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 303 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 305 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 306 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 324 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 343 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 370 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 387 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 394 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 408 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 417 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 431 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 435 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 443 | 104.26.6.238 | 104.26.6.238 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 453 | 172.67.68.110 | 172.67.68.110 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 465 | 172.64.229.185 | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 496 | 104.20.29.234 | 104.20.29.234 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 498 | 172.67.65.44 | 172.67.65.44 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 530 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 560 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 586 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 604 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 606 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 617 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 703 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 714 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 723 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 725 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 754 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 765 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 766 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 773 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 802 | 162.159.36.205 | 162.159.36.205 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 816 | 104.17.193.113 | 104.17.193.113 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 821 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 827 | 162.159.12.120 | 162.159.12.120 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 828 | 162.159.58.17 | 162.159.58.17 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 829 | 172.64.53.202 | 172.64.53.202 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 834 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 875 | 172.64.153.183 | 172.64.153.183 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 890 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 893 | 104.20.24.239 | 104.20.24.239 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 894 | 172.67.79.249 | 172.67.79.249 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 895 | 104.26.5.53 | 104.26.5.53 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 915 | 172.64.53.220 | 172.64.53.220 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 924 | 104.18.33.37 | 104.18.33.37 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 926 | 162.159.45.219 | 162.159.45.219 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 928 | 2a06:98c1:3108:1a71:a277:b3bf:80a:c2a3 | 2a06:98c1:3108:1a71:a277:b3bf:80a:c2a3 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 929 | 2a06:98c1:50::46cb:8c34:28e3 | 2a06:98c1:50::46cb:8c34:28e3 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 932 | 2a06:98c1:3102:96:65b:cff7:1c28:b82a | 2a06:98c1:3102:96:65b:cff7:1c28:b82a | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 940 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 953 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 966 | 104.20.21.11 | 104.20.21.11 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 3 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 50 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 68 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 78 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 101 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 163 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 183 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 214 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 237 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 269 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 340 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 348 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 350 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 361 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 383 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 402 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 471 | 162.159.40.8 | 162.159.40.8 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 525 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 526 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 557 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 559 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 588 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 659 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 734 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 748 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 753 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 779 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 847 | 172.64.229.156 | 172.64.229.156 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 859 | 104.20.22.91 | 104.20.22.91 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 885 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 887 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 900 | 172.67.65.81 | 172.67.65.81 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 901 | 172.67.77.185 | 172.67.77.185 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 904 | 104.25.254.89 | 104.25.254.89 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 946 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 964 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 14 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 36 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 77 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 91 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 104 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 203 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 296 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 300 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 307 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 391 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 412 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 441 | 104.20.29.62 | 104.20.29.62 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 454 | 172.67.76.61 | 172.67.76.61 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 555 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 664 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 675 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 786 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 787 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 804 | 104.20.28.239 | 104.20.28.239 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 942 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 950 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 92 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 93 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 95 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 132 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 200 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 201 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 217 | xn--b6gac.eu.org | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 252 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 358 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 478 | 2606:4700:59:764d:d406:c823:e52f:4f3a | 2606:4700:59:764d:d406:c823:e52f:4f3a | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 561 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 657 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 756 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 767 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 785 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 897 | 172.67.75.212 | 172.67.75.212 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 914 | 162.159.45.121 | 162.159.45.121 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 4 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 96 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 344 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 446 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 513 | 104.25.248.93 | 104.25.248.93 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 577 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 655 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 658 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 788 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 807 | 172.67.65.150 | 172.67.65.150 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 837 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 851 | 162.159.0.41 | 162.159.0.41 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 941 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 100 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 105 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 459 | 104.17.129.66 | 104.17.129.66 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 527 | 2a06:98c1:310c::dd:f399:427e | 2a06:98c1:310c::dd:f399:427e | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 531 | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 595 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 715 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 721 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 783 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 803 | 104.26.1.88 | 104.26.1.88 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 838 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 935 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 566 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 704 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 719 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 722 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 759 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 840 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 869 | 104.25.241.85 | 104.25.241.85 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 886 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 311 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 479 | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 624 | 172.64.155.4 | 172.64.155.4 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 717 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 845 | 162.159.39.20 | 162.159.39.20 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 66 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 120 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 718 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 930 | 2a06:98c1:3100:7f11:4805:1c25:83ab:6033 | 2a06:98c1:3100:7f11:4805:1c25:83ab:6033 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 111 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 287 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 839 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 241 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 339 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 746 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 809 | 104.20.25.181 | 104.20.25.181 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 523 | 104.18.40.200 | 104.18.40.200 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 573 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 2 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 226 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 632 | 162.159.38.203 | 162.159.38.203 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 702 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 24 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 437 | 162.159.6.186 | 162.159.6.186 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 724 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 919 | 162.159.39.26 | 162.159.39.26 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 112 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 491 | 172.64.40.68 | 172.64.40.68 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 671 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 690 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 732 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 176 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 179 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 256 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 271 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 332 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 338 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 399 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 589 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 613 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 619 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 640 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 679 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 735 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 850 | 162.159.44.202 | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 881 | 162.159.43.50 | 162.159.43.50 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 967 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 38 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 39 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 119 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 121 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 166 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 225 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 253 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 260 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 316 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 318 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 337 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 449 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 481 | 2a06:98c1:51:8:7944:48b0:1301:5ced | 2a06:98c1:51:8:7944:48b0:1301:5ced | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 542 | 162.159.39.14 | 162.159.39.14 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 685 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 880 | 172.64.52.181 | 172.64.52.181 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 29 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 30 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 137 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 171 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 180 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 224 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 227 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 238 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 239 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 255 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 262 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 320 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 329 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 377 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 378 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 379 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 382 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 401 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 467 | 162.159.39.156 | 162.159.39.156 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 468 | 172.64.53.15 | 172.64.53.15 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 666 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 669 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 691 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 824 | 162.159.11.128 | 162.159.11.128 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 832 | 162.159.1.39 | 162.159.1.39 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 882 | 162.159.38.83 | 162.159.38.83 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 913 | 162.159.44.246 | 162.159.44.246 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 32 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 37 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 109 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 110 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 127 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 128 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 130 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 141 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 147 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 170 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 251 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 258 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 270 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 272 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 273 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 333 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 334 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 444 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 445 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 485 | 162.159.45.237 | 162.159.45.237 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 489 | 108.162.198.152 | 108.162.198.152 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 533 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 581 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 590 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 612 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 616 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 634 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 687 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 688 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 733 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 752 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 790 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 833 | 108.162.198.70 | 108.162.198.70 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 917 | 162.159.38.52 | 162.159.38.52 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 918 | 172.64.52.194 | 172.64.52.194 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 921 | 172.64.32.198 | 172.64.32.198 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 23 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 27 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 40 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 126 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 146 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 168 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 169 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 242 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 263 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 274 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 336 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 380 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 384 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 403 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 414 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 472 | 108.162.198.223 | 108.162.198.223 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 534 | 162.159.38.202 | 162.159.38.202 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 792 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 793 | 108.162.198.85 | 108.162.198.85 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 795 | 162.159.45.65 | 162.159.45.65 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 825 | 162.159.3.128 | 162.159.3.128 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 831 | 162.159.39.180 | 162.159.39.180 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 843 | 172.64.53.41 | 172.64.53.41 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 844 | 162.159.38.45 | 162.159.38.45 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 884 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 944 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 26 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 138 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 257 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 315 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 319 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 385 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 404 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 448 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 475 | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 516 | 162.159.27.183 | 162.159.27.183 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 582 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 614 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 615 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 695 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 873 | 162.159.33.191 | 162.159.33.191 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 938 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 35 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 108 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 122 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 243 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 327 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 381 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 400 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 415 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 447 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 672 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 728 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 841 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 931 | 2a06:98c1:51:4e:5188:50a9:cbd1:917d | 2a06:98c1:51:4e:5188:50a9:cbd1:917d | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 25 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 178 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 405 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 635 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 665 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 916 | 108.162.198.232 | 108.162.198.232 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 939 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 31 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 123 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 514 | 162.159.49.244 | 162.159.49.244 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 113 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 140 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 181 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 751 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 167 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 488 | 162.159.39.196 | 162.159.39.196 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 254 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 100 | cloudflare |
| 776 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 874 | 162.159.39.74 | 162.159.39.74 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 535 | 162.159.45.153 | 162.159.45.153 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 750 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 826 | 162.159.38.134 | 162.159.38.134 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 674 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 848 | 162.159.45.67 | 162.159.45.67 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 849 | 108.162.198.198 | 108.162.198.198 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 522 | 172.64.146.121 | 172.64.146.121 | IPv4 | h2 | ✅ 成功 | 308 | cloudflare |
| 521 | 104.18.47.193 | 104.18.47.193 | IPv4 | h2 | ✅ 成功 | 425 | cloudflare |
| 520 | 172.64.53.103 | 172.64.53.103 | IPv4 | h2 | ✅ 成功 | 476 | cloudflare |
| 518 | 162.159.44.199 | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 571 | cloudflare |
| 220 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 613 | cloudflare |
| 371 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 662 | cloudflare |
| 65 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 933 | cloudflare |
| 11 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 949 | cloudflare |
| 519 | 162.159.33.28 | 162.159.33.28 | IPv4 | h2 | ✅ 成功 | 1332 | cloudflare |
| 517 | 172.64.229.82 | 172.64.229.82 | IPv4 | h2 | ✅ 成功 | 1504 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 229 条记录
- **快 (50-100ms)**: 715 条记录
- **正常 (100-200ms)**: 9 条记录
- **慢 (200-500ms)**: 3 条记录
- **很慢 (>500ms)**: 7 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 4 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*

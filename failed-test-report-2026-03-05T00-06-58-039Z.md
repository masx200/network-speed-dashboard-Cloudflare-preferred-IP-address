# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/3/5 00:06:58
- **数据来源**: connectivity_results-20260305-000656.json
- **总测试数**: 988
- **失败测试数**: 10
- **成功测试数**: 978
- **失败率**: 1.01%
- **平均延迟**: 80.88ms
- **最小延迟**: 57ms
- **最大延迟**: 916ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/3/5 00:06:58
- **IP地址**: 2a09:bac6:d6e1:2632::3ce:41
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 37.751, -97.822
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **DNS解析错误: 其他DNS错误**: 8 次 (80.0%)
- **连接超时: I/O超时**: 2 次 (20.0%)

### 🔍 按错误类型分类的失败测试详情

#### DNS解析错误: 其他DNS错误 (8 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 9 | cmcc.877774.xyz | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 10 | shopify.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 17 | cloudflare.182682.xyz | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 210 | asia.877774.xyz | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 225 | www.4chan.org | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 229 | cu.877774.xyz | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 235 | www.visa.cn | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 241 | www.visa.com.sg | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 495 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 974 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **DNS解析错误**: 8 次 (80.0%)
- **连接超时**: 2 次 (20.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 10 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 978 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 532 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 767 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 285 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 760 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 490 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 743 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 99 | 172.64.146.121 | 172.64.146.121 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 200 | 162.159.61.56 | 162.159.61.56 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 74 | 104.20.29.234 | 104.20.29.234 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 132 | 172.67.73.94 | 172.67.73.94 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 159 | 2a06:98c1:3108:1a71:a277:b3bf:80a:c2a3 | 2a06:98c1:3108:1a71:a277:b3bf:80a:c2a3 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 245 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 293 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 456 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 485 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 539 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 591 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 632 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 757 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 954 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 964 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 21 | 104.26.12.33 | 104.26.12.33 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 28 | 172.67.68.110 | 172.67.68.110 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 83 | 104.25.245.173 | 104.25.245.173 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 89 | 104.25.246.117 | 104.25.246.117 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 97 | 172.64.53.103 | 172.64.53.103 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 101 | 2a06:98c1:3102:8768:b929:7455:f040:5aee | 2a06:98c1:3102:8768:b929:7455:f040:5aee | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 108 | 2a06:98c1:310c::dd:f399:427e | 2a06:98c1:310c::dd:f399:427e | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 123 | 104.18.32.161 | 104.18.32.161 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 127 | 104.26.15.142 | 104.26.15.142 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 134 | 172.67.79.166 | 172.67.79.166 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 137 | 104.16.148.143 | 104.16.148.143 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 139 | 104.17.118.227 | 104.17.118.227 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 152 | 162.159.34.205 | 162.159.34.205 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 186 | 104.17.184.153 | 104.17.184.153 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 237 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 304 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 341 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 343 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 383 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 396 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 399 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 416 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 467 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 476 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 484 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 492 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 499 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 526 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 554 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 675 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 682 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 728 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 736 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 764 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 897 | 162.159.38.83 | 162.159.38.83 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 917 | 172.67.75.212 | 172.67.75.212 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 923 | 104.25.240.227 | 104.25.240.227 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 933 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 25 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 38 | 104.25.241.19 | 104.25.241.19 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 116 | 162.159.44.36 | 162.159.44.36 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 128 | 104.20.21.202 | 104.20.21.202 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 130 | 104.20.19.37 | 104.20.19.37 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 142 | 104.19.144.110 | 104.19.144.110 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 143 | 172.64.154.86 | 172.64.154.86 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 181 | 104.20.30.69 | 104.20.30.69 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 190 | 104.19.144.174 | 104.19.144.174 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 195 | 104.19.54.183 | 104.19.54.183 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 203 | 172.64.229.243 | 172.64.229.243 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 217 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 218 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 233 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 244 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 254 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 284 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 309 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 319 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 336 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 339 | cf.090227.xyz | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 392 | ip.gs | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 402 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 415 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 418 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 450 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 458 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 464 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 471 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 489 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 498 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 514 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 523 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 533 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 541 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 546 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 580 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 590 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 597 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 614 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 630 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 637 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 639 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 683 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 687 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 700 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 710 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 738 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 740 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 745 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 747 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 750 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 782 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 790 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 791 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 819 | 104.17.97.228 | 104.17.97.228 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 820 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 830 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 837 | 104.18.172.20 | 104.18.172.20 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 886 | 172.64.144.132 | 172.64.144.132 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 894 | 104.25.241.85 | 104.25.241.85 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 913 | 172.67.79.249 | 172.67.79.249 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 932 | 104.16.245.121 | 104.16.245.121 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 934 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 940 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 941 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 956 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 16 | 104.26.14.88 | 104.26.14.88 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 32 | 104.17.97.146 | 104.17.97.146 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 53 | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 65 | 162.159.44.60 | 162.159.44.60 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 67 | 108.162.198.152 | 108.162.198.152 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 71 | 172.67.64.123 | 172.67.64.123 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 72 | 104.20.20.156 | 104.20.20.156 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 73 | 172.67.77.104 | 172.67.77.104 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 75 | 172.67.73.129 | 172.67.73.129 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 77 | 172.67.76.20 | 172.67.76.20 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 81 | 104.25.241.235 | 104.25.241.235 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 88 | 104.25.254.47 | 104.25.254.47 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 91 | 108.162.198.206 | 108.162.198.206 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 107 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 125 | 104.26.2.242 | 104.26.2.242 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 140 | 172.64.145.253 | 172.64.145.253 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 144 | 104.17.119.130 | 104.17.119.130 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 145 | 162.159.39.165 | 162.159.39.165 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 149 | 162.159.3.222 | 162.159.3.222 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 150 | 162.159.9.224 | 162.159.9.224 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 163 | 2a06:98c1:3100:7f01:2f67:5ef8:2a97:8d82 | 2a06:98c1:3100:7f01:2f67:5ef8:2a97:8d82 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 173 | 172.64.229.111 | 172.64.229.111 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 175 | 172.64.157.174 | 172.64.157.174 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 176 | 104.20.18.49 | 104.20.18.49 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 177 | 104.20.21.83 | 104.20.21.83 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 184 | 104.20.27.57 | 104.20.27.57 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 191 | 104.17.150.224 | 104.17.150.224 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 192 | 104.19.37.149 | 104.19.37.149 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 212 | 2400:cb00:2049:ddcd:7011:1125:9f3:6a4f | 2400:cb00:2049:ddcd:7011:1125:9f3:6a4f | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 227 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 232 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 248 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 251 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 253 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 258 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 262 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 271 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 277 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 287 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 305 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 306 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 316 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 321 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 340 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 344 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 377 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 382 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 385 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 386 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 391 | ip.gs | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 403 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 406 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 429 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 433 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 437 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 438 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 439 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 446 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 469 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 486 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 487 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 488 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 491 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 494 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 496 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 501 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 502 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 504 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 509 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 550 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 555 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 557 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 560 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 571 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 572 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 574 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 576 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 593 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 607 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 619 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 620 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 623 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 629 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 634 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 663 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 669 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 673 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 678 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 684 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 690 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 691 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 694 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 697 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 729 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 733 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 746 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 749 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 751 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 753 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 765 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 771 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 792 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 798 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 800 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 805 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 813 | 104.20.20.192 | 104.20.20.192 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 823 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 839 | 172.64.153.140 | 172.64.153.140 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 843 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 868 | 172.64.229.156 | 172.64.229.156 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 880 | 104.25.254.14 | 104.25.254.14 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 885 | 172.64.153.183 | 172.64.153.183 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 892 | 104.25.247.78 | 104.25.247.78 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 911 | 104.20.26.221 | 104.20.26.221 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 916 | 172.67.73.196 | 172.67.73.196 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 919 | 172.67.65.81 | 172.67.65.81 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 936 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 944 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 947 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 948 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 955 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 968 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 969 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 978 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 22 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 29 | 172.64.146.137 | 172.64.146.137 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 57 | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 70 | 172.64.41.47 | 172.64.41.47 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 76 | 172.67.65.44 | 172.67.65.44 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 78 | 172.67.79.218 | 172.67.79.218 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 80 | 104.20.18.125 | 104.20.18.125 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 82 | 104.25.240.21 | 104.25.240.21 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 85 | 104.25.250.121 | 104.25.250.121 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 103 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 106 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 110 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 111 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 122 | 172.64.229.158 | 172.64.229.158 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 138 | 104.18.42.129 | 104.18.42.129 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 141 | 104.17.165.38 | 104.17.165.38 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 146 | 172.64.229.172 | 172.64.229.172 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 147 | 104.18.33.253 | 104.18.33.253 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 174 | 162.159.3.82 | 162.159.3.82 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 179 | 104.20.25.245 | 104.20.25.245 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 180 | 104.20.28.118 | 104.20.28.118 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 188 | 104.17.178.163 | 104.17.178.163 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 189 | 104.17.163.110 | 104.17.163.110 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 201 | 172.64.52.114 | 172.64.52.114 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 219 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 220 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 224 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 226 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 230 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 234 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 260 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 315 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 334 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 337 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 338 | cf.090227.xyz | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 342 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 366 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 367 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 381 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 384 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 387 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 388 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 411 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 417 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 422 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 434 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 442 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 452 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 457 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 461 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 468 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 472 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 474 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 481 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 493 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 507 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 519 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 528 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 534 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 549 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 552 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 556 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 559 | www.7749tv.com | 104.16.1.142 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 565 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 566 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 570 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 577 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 585 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 592 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 598 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 618 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 622 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 624 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 627 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 635 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 643 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 665 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 674 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 686 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 693 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 695 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 701 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 714 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 732 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 734 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 742 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 748 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 752 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 754 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 775 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 778 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 779 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 780 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 799 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 806 | 104.26.6.247 | 104.26.6.247 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 807 | 104.26.14.117 | 104.26.14.117 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 815 | 104.26.13.110 | 104.26.13.110 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 816 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 817 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 836 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 838 | 104.17.111.150 | 104.17.111.150 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 847 | 162.159.38.134 | 162.159.38.134 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 857 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 858 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 863 | 162.159.39.20 | 162.159.39.20 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 866 | 108.162.198.198 | 108.162.198.198 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 873 | 104.20.24.244 | 104.20.24.244 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 875 | 172.67.70.56 | 172.67.70.56 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 878 | 172.67.72.212 | 172.67.72.212 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 898 | 172.64.229.15 | 172.64.229.15 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 910 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 914 | 104.26.5.53 | 104.26.5.53 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 915 | 104.20.22.141 | 104.20.22.141 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 920 | 172.67.77.185 | 172.67.77.185 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 924 | 104.25.242.249 | 104.25.242.249 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 929 | 162.159.44.246 | 162.159.44.246 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 931 | 104.18.160.38 | 104.18.160.38 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 937 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 938 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 942 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 945 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 946 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 951 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 966 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 970 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 981 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 11 | 172.64.229.149 | 172.64.229.149 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 24 | 172.67.72.250 | 172.67.72.250 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 26 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 27 | 172.67.64.116 | 172.67.64.116 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 30 | 172.67.76.61 | 172.67.76.61 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 31 | 104.25.252.135 | 104.25.252.135 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 40 | 104.25.255.103 | 104.25.255.103 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 44 | 104.18.44.159 | 104.18.44.159 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 63 | 172.64.52.150 | 172.64.52.150 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 84 | 104.25.243.36 | 104.25.243.36 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 86 | 104.25.252.192 | 104.25.252.192 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 87 | 104.25.242.137 | 104.25.242.137 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 94 | 172.64.229.82 | 172.64.229.82 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 98 | 104.18.47.193 | 104.18.47.193 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 115 | 162.159.45.0 | 162.159.45.0 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 126 | 104.26.5.121 | 104.26.5.121 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 131 | 172.67.74.78 | 172.67.74.78 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 165 | 172.64.52.133 | 172.64.52.133 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 172 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 183 | 104.20.29.75 | 104.20.29.75 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 187 | 104.17.109.128 | 104.17.109.128 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 193 | 104.17.161.112 | 104.17.161.112 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 194 | 104.19.44.187 | 104.19.44.187 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 196 | 104.18.39.162 | 104.18.39.162 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 238 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 239 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 250 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 255 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 270 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 276 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 283 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 288 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 297 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 318 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 320 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 322 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 326 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 329 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 335 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 351 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 365 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 368 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 369 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 373 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 393 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 394 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 397 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 412 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 419 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 435 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 436 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 475 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 478 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 497 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 506 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 515 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 521 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 529 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 530 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 531 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 538 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 543 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 587 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 589 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 602 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 604 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 628 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 633 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 641 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 642 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 644 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 658 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 660 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 662 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 689 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 692 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 704 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 718 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 726 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 730 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 731 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 737 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 744 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 768 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 777 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 795 | 172.64.229.134 | 172.64.229.134 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 810 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 828 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 840 | 104.25.245.233 | 104.25.245.233 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 841 | 104.18.166.232 | 104.18.166.232 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 850 | 162.159.39.180 | 162.159.39.180 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 852 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 854 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 870 | 162.159.20.46 | 162.159.20.46 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 879 | 104.20.19.180 | 104.20.19.180 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 883 | 104.26.5.194 | 104.26.5.194 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 887 | 162.159.45.150 | 162.159.45.150 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 908 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 909 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 912 | 104.20.24.239 | 104.20.24.239 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 918 | 104.25.244.87 | 104.25.244.87 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 925 | 104.17.143.82 | 104.17.143.82 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 927 | 104.25.246.24 | 104.25.246.24 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 943 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 952 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 965 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 976 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 977 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 13 | 162.159.6.186 | 162.159.6.186 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 15 | 104.26.1.55 | 104.26.1.55 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 33 | 104.25.247.129 | 104.25.247.129 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 39 | 104.16.247.125 | 104.16.247.125 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 47 | 172.64.157.214 | 172.64.157.214 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 51 | 172.64.152.85 | 172.64.152.85 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 59 | 2a06:98c1:310b::fda8:fa9e:4a3e | 2a06:98c1:310b::fda8:fa9e:4a3e | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 60 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 61 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 68 | 172.64.229.106 | 172.64.229.106 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 69 | 172.64.40.68 | 172.64.40.68 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 79 | 104.26.4.213 | 104.26.4.213 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 90 | 104.25.248.93 | 104.25.248.93 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 100 | 104.18.40.200 | 104.18.40.200 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 102 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 104 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 105 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 133 | 104.26.0.124 | 104.26.0.124 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 136 | 104.17.104.208 | 104.17.104.208 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 158 | 2a06:98c1:3100:7f11:4805:1c25:83ab:6033 | 2a06:98c1:3100:7f11:4805:1c25:83ab:6033 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 228 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 231 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 236 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 256 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 267 | freeyx.cloudflare88.eu.org | 172.64.147.79 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 274 | bestcf.030101.xyz | 104.18.87.57 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 275 | bestcf.030101.xyz | 104.17.220.38 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 286 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 291 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 303 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 307 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 314 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 317 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 332 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 333 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 348 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 356 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 370 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 389 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 401 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 404 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 407 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 408 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 409 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 424 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 448 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 455 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 459 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 463 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 473 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 520 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 542 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 553 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 561 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 562 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 568 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 569 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 575 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 578 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 583 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 586 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 606 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 616 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 617 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 625 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 631 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 679 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 685 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 699 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 703 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 709 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 723 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 762 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 793 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 809 | 172.67.65.150 | 172.67.65.150 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 811 | 104.26.0.210 | 104.26.0.210 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 812 | 104.26.8.171 | 104.26.8.171 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 829 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 834 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 859 | 162.159.1.39 | 162.159.1.39 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 867 | 162.159.44.202 | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 872 | 104.26.11.33 | 104.26.11.33 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 874 | 104.20.22.91 | 104.20.22.91 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 877 | 172.67.67.0 | 172.67.67.0 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 888 | 104.18.148.235 | 104.18.148.235 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 895 | 172.64.151.253 | 172.64.151.253 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 896 | 162.159.43.50 | 162.159.43.50 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 899 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 902 | 104.25.249.225 | 104.25.249.225 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 904 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 922 | 104.25.254.89 | 104.25.254.89 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 935 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 953 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 975 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 980 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 986 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 7 | 162.159.38.52 | 162.159.38.52 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 8 | 172.64.52.194 | 172.64.52.194 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 12 | 162.159.39.26 | 162.159.39.26 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 18 | 104.26.6.238 | 104.26.6.238 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 41 | 172.64.229.185 | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 113 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 129 | 172.67.75.231 | 172.67.75.231 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 135 | 104.18.44.187 | 104.18.44.187 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 154 | 2a06:98c1:3106:0:fb94:fc7b:2b7f:ae54 | 2a06:98c1:3106:0:fb94:fc7b:2b7f:ae54 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 161 | 2a06:98c1:3102:96:65b:cff7:1c28:b82a | 2a06:98c1:3102:96:65b:cff7:1c28:b82a | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 162 | 2a06:98c1:3106:d8af:8b29:8a81:bf10:9cef | 2a06:98c1:3106:d8af:8b29:8a81:bf10:9cef | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 170 | 162.159.44.37 | 162.159.44.37 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 198 | 162.159.39.12 | 162.159.39.12 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 207 | 2a06:98c1:310e:e69c:a40c:913f:b22d:5951 | 2a06:98c1:310e:e69c:a40c:913f:b22d:5951 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 208 | 2400:cb00:2049:ddcd:c532:69fa:251a:af7a | 2400:cb00:2049:ddcd:c532:69fa:251a:af7a | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 209 | 2a06:98c1:3103:f1:e422:c175:1c8e:df1a | 2a06:98c1:3103:f1:e422:c175:1c8e:df1a | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 240 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 268 | freeyx.cloudflare88.eu.org | 172.64.146.43 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 324 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 352 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 380 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 398 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 453 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 460 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 462 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 466 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 508 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 564 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 600 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 601 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 605 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 608 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 615 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 621 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 626 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 653 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 656 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 657 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 664 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 696 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 713 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 716 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 720 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 722 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 724 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 727 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 739 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 741 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 755 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 774 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 789 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 801 | 104.26.1.88 | 104.26.1.88 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 804 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 808 | 104.20.25.181 | 104.20.25.181 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 814 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 833 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 835 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 842 | 104.25.250.174 | 104.25.250.174 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 851 | 172.64.53.202 | 172.64.53.202 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 903 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 905 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 926 | 104.17.56.177 | 104.17.56.177 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 939 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 949 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 959 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 972 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 5 | 172.64.53.220 | 172.64.53.220 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 19 | 104.20.29.62 | 104.20.29.62 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 42 | 104.17.171.88 | 104.17.171.88 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 112 | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 160 | 2a06:98c1:310e:5f9e:101d:94ce:cb6b:49ca | 2a06:98c1:310e:5f9e:101d:94ce:cb6b:49ca | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 178 | 104.20.31.51 | 104.20.31.51 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 182 | 104.20.16.14 | 104.20.16.14 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 185 | 104.20.26.30 | 104.20.26.30 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 204 | 172.64.147.253 | 172.64.147.253 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 216 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 249 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 252 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 289 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 308 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 327 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 361 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 374 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 405 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 420 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 423 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 441 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 477 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 483 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 522 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 545 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 636 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 640 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 702 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 711 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 735 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 769 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 770 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 776 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 796 | 104.18.42.16 | 104.18.42.16 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 802 | 104.20.28.239 | 104.20.28.239 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 826 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 856 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 889 | 104.25.245.215 | 104.25.245.215 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 890 | 104.17.56.208 | 104.17.56.208 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 906 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 928 | 104.17.62.194 | 104.17.62.194 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 967 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 973 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 20 | 172.67.78.23 | 172.67.78.23 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 34 | 104.17.129.66 | 104.17.129.66 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 35 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 43 | 104.25.253.253 | 104.25.253.253 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 55 | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 121 | 162.159.39.189 | 162.159.39.189 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 156 | 2a06:98c1:50::46cb:8c34:28e3 | 2a06:98c1:50::46cb:8c34:28e3 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 223 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 257 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 269 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 272 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 278 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 451 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 470 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 503 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 579 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 582 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 599 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 666 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 698 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 763 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 881 | 104.25.250.205 | 104.25.250.205 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 893 | 104.25.244.36 | 104.25.244.36 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 901 | 104.25.240.123 | 104.25.240.123 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 6 | 108.162.198.232 | 108.162.198.232 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 52 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 109 | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 390 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 518 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 527 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 645 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 688 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 803 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 921 | 104.26.3.117 | 104.26.3.117 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 23 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 119 | 162.159.38.68 | 162.159.38.68 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 273 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 302 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 395 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 400 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 410 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 573 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 661 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 822 | 104.25.244.239 | 104.25.244.239 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 950 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 982 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 323 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 596 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 215 | 2a06:98c1:3107:2f:54ce:e76b:a1c9:7cd | 2a06:98c1:3107:2f:54ce:e76b:a1c9:7cd | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 454 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 588 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 613 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 638 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 971 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 171 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 213 | 2a06:98c1:3109:4ed3:5a24:20f9:dac0:1f5e | 2a06:98c1:3109:4ed3:5a24:20f9:dac0:1f5e | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 222 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 465 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 871 | 104.20.31.132 | 104.20.31.132 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 214 | 2a06:98c1:51:403a:9791:47cd:2ae7:2 | 2a06:98c1:51:403a:9791:47cd:2ae7:2 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 432 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 206 | 2a06:98c1:3109:4e23:df4:a0d:70:bb41 | 2a06:98c1:3109:4e23:df4:a0d:70:bb41 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 371 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 603 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 821 | 104.25.241.198 | 104.25.241.198 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 831 | 104.17.193.113 | 104.17.193.113 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 876 | 104.26.4.135 | 104.26.4.135 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 979 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 594 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 960 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 414 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 500 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 440 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 655 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 962 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 62 | 172.64.53.195 | 172.64.53.195 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 372 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 547 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 783 | 108.162.198.85 | 108.162.198.85 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 988 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 331 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 345 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 358 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 362 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 443 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 447 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 672 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 677 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 681 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 900 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 92 | 162.159.49.244 | 162.159.49.244 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 168 | 162.159.38.137 | 162.159.38.137 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 199 | 162.159.10.81 | 162.159.10.81 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 243 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 264 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 313 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 328 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 376 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 449 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 537 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 548 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 609 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 646 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 680 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 797 | 162.159.36.205 | 162.159.36.205 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 46 | 172.64.53.15 | 172.64.53.15 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 49 | 162.159.40.8 | 162.159.40.8 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 50 | 108.162.198.223 | 108.162.198.223 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 54 | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 120 | 172.64.53.101 | 172.64.53.101 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 124 | 162.159.58.251 | 162.159.58.251 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 148 | 162.159.45.8 | 162.159.45.8 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 151 | 162.159.44.58 | 162.159.44.58 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 153 | 162.159.18.22 | 162.159.18.22 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 157 | 2a06:98c1:51:4e:5188:50a9:cbd1:917d | 2a06:98c1:51:4e:5188:50a9:cbd1:917d | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 202 | 162.159.45.82 | 162.159.45.82 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 265 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 266 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 357 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 375 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 480 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 540 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 544 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 667 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 670 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 825 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 845 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 862 | 162.159.38.45 | 162.159.38.45 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 930 | 162.159.46.38 | 162.159.46.38 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 987 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 14 | 162.159.21.16 | 162.159.21.16 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 48 | 162.159.0.79 | 162.159.0.79 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 66 | 162.159.39.196 | 162.159.39.196 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 164 | 162.159.45.194 | 162.159.45.194 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 211 | 2a06:98c1:51:403a:971b:3a6b:65cb:bf8a | 2a06:98c1:51:403a:971b:3a6b:65cb:bf8a | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 242 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 247 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 295 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 298 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 311 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 312 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 330 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 347 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 355 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 360 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 363 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 364 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 379 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 425 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 444 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 516 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 517 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 536 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 563 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 584 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 659 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 671 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 725 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 766 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 781 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 787 | 172.64.52.189 | 172.64.52.189 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 844 | 162.159.11.128 | 162.159.11.128 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 855 | 198.41.222.191 | 198.41.222.191 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 864 | 172.64.52.224 | 172.64.52.224 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 865 | 162.159.45.67 | 162.159.45.67 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 984 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 56 | 2606:4700:59:764d:d406:c823:e52f:4f3a | 2606:4700:59:764d:d406:c823:e52f:4f3a | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 64 | 162.159.45.237 | 162.159.45.237 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 117 | 172.64.52.168 | 172.64.52.168 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 118 | 108.162.198.168 | 108.162.198.168 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 281 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 346 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 359 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 430 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 558 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 647 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 648 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 719 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 761 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 773 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 794 | 162.159.39.146 | 162.159.39.146 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 848 | 162.159.12.120 | 162.159.12.120 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 860 | 108.162.198.70 | 108.162.198.70 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 869 | 162.159.0.41 | 162.159.0.41 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 884 | 162.159.39.74 | 162.159.39.74 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 891 | 172.64.52.181 | 172.64.52.181 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 907 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 58 | 2a06:98c1:51:8:7944:48b0:1301:5ced | 2a06:98c1:51:8:7944:48b0:1301:5ced | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 95 | 162.159.33.28 | 162.159.33.28 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 114 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 166 | 108.162.198.146 | 108.162.198.146 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 167 | 162.159.39.90 | 162.159.39.90 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 259 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 280 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 301 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 310 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 353 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 378 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 421 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 426 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 427 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 428 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 445 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 511 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 525 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 567 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 649 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 654 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 705 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 712 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 758 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 824 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 846 | 162.159.3.128 | 162.159.3.128 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 849 | 162.159.58.17 | 162.159.58.17 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 882 | 162.159.33.191 | 162.159.33.191 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 958 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 1 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 2 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 36 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 37 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 45 | 162.159.39.156 | 162.159.39.156 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 205 | 162.159.60.188 | 162.159.60.188 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 261 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 279 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 282 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 290 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 294 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 296 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 350 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 479 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 510 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 610 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 668 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 676 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 707 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 715 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 717 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 721 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 756 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 786 | 172.64.53.165 | 172.64.53.165 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 788 | 162.159.44.101 | 162.159.44.101 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 961 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 197 | 162.159.38.100 | 162.159.38.100 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 349 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 113 | cloudflare |
| 431 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 535 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 611 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 650 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 759 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 832 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 853 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | h2 | ✅ 成功 | 113 | cloudflare |
| 983 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 93 | 162.159.27.183 | 162.159.27.183 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 169 | 172.64.53.216 | 172.64.53.216 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 299 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 114 | cloudflare |
| 505 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 512 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 524 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 652 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 706 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 708 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 772 | 162.159.45.121 | 162.159.45.121 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 963 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 114 | cloudflare |
| 155 | 2a06:98c1:51:0:4371:ce16:475:2557 | 2a06:98c1:51:0:4371:ce16:475:2557 | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 482 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 818 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 827 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 861 | 172.64.53.41 | 172.64.53.41 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 651 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 985 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 3 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 4 | 2a06:98c1:50::b9:30bc:de63 | 2a06:98c1:50::b9:30bc:de63 | IPv6 | h2 | ✅ 成功 | 117 | cloudflare |
| 96 | 162.159.44.199 | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 263 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 117 | cloudflare |
| 413 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 246 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 119 | cloudflare |
| 292 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 300 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 119 | cloudflare |
| 513 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 784 | 162.159.38.226 | 162.159.38.226 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 785 | 162.159.45.65 | 162.159.45.65 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 354 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 128 | cloudflare |
| 595 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 168 | cloudflare |
| 581 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 196 | cloudflare |
| 612 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 242 | cloudflare |
| 551 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 488 | cloudflare |
| 325 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 672 | cloudflare |
| 221 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 738 | cloudflare |
| 957 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 916 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 762 条记录
- **正常 (100-200ms)**: 211 条记录
- **慢 (200-500ms)**: 2 条记录
- **很慢 (>500ms)**: 3 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 10 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*

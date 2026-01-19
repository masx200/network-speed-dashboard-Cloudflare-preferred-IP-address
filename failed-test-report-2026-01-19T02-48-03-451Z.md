# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/19 02:48:03
- **数据来源**: connectivity_results-20260119-024802.json
- **总测试数**: 774
- **失败测试数**: 2
- **成功测试数**: 772
- **失败率**: 0.26%
- **平均延迟**: 73.56ms
- **最小延迟**: 45ms
- **最大延迟**: 932ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/19 02:48:03
- **IP地址**: 2a09:bac5:c852:188c::272:93
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 38.6877, -77.8369
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 35 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 372 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 772 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 516 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 762 | 162.159.13.208 | 162.159.13.208 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 152 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 362 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 536 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 262 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 294 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 355 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 446 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 340 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 381 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 453 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 571 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 643 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 680 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 173 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 361 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 450 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 499 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 505 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 598 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 684 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 743 | 104.20.28.71 | 104.20.28.71 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 8 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 98 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 455 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 468 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 528 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 534 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 624 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 627 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 694 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 15 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 72 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 260 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 282 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 452 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 576 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 630 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 646 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 757 | 162.159.18.2 | 162.159.18.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 61 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 71 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 84 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 117 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 198 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 234 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 244 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 245 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 254 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 283 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 309 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 422 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 426 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 498 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 667 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 737 | 104.20.23.184 | 104.20.23.184 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 14 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 55 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 105 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 130 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 167 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 194 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 200 | bestcf.030101.xyz | 104.17.120.229 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 261 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 302 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 316 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 346 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 454 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 486 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 488 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 568 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 569 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 579 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 588 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 629 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 633 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 638 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 697 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 34 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 56 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 67 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 87 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 110 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 137 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 202 | bestcf.030101.xyz | 2606:4700:0:b21c:66ba:fef:2ef4:2658 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 280 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 281 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 369 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 425 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 465 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 514 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 555 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 608 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 66 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 122 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 187 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 233 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 238 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 268 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 307 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 341 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 382 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 390 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 424 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 463 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 464 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 480 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 519 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 563 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 651 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 689 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 696 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 707 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 716 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 755 | 104.19.153.238 | 104.19.153.238 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 6 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 44 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 45 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 47 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 49 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 57 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 106 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 164 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 195 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 273 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 284 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 293 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 303 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 371 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 375 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 427 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 552 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 575 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 589 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 591 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 631 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 690 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 708 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 733 | 172.67.70.163 | 172.67.70.163 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 734 | 172.64.154.12 | 172.64.154.12 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 749 | 104.16.240.165 | 104.16.240.165 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 3 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 29 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 59 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 115 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 128 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 163 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 201 | bestcf.030101.xyz | 104.16.254.88 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 203 | bestcf.030101.xyz | 2606:4700:0:26e6:9bb0:8293:779:dc27 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 256 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 279 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 305 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 322 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 335 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 365 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 421 | www.7749tv.com | 104.16.0.141 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 459 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 479 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 517 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 560 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 567 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 584 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 636 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 648 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 653 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 675 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 732 | 162.159.26.213 | 162.159.26.213 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 759 | 172.64.157.187 | 172.64.157.187 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 767 | 2a06:98c1:3106:0:54b2:2751:f330:de0a | 2a06:98c1:3106:0:54b2:2751:f330:de0a | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 7 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 10 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 16 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 25 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 39 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 51 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 53 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 63 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 73 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 76 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 108 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 121 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 183 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 266 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 275 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 300 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 317 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 324 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 373 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 391 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 409 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 439 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 443 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 461 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 467 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 485 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 526 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 535 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 553 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 554 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 647 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 649 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 669 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 682 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 698 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 744 | 172.67.64.153 | 172.67.64.153 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 772 | 2a06:98c1:3105:6e:f984:95fe:9b26:f194 | 2a06:98c1:3105:6e:f984:95fe:9b26:f194 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 60 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 144 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 156 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 235 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 347 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 384 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 411 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 435 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 445 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 476 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 478 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 489 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 492 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 496 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 508 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 614 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 639 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 652 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 655 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 671 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 692 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 705 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 722 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 731 | 104.18.42.35 | 104.18.42.35 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 763 | 172.64.48.175 | 172.64.48.175 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 768 | 2a06:98c1:310a:1a6a:bc43:5b7:9d0f:62f3 | 2a06:98c1:310a:1a6a:bc43:5b7:9d0f:62f3 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 2 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 11 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 70 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 204 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 232 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 236 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 241 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 251 | cf.090227.xyz | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 253 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 306 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 338 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 398 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 418 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 434 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 462 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 484 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 503 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 509 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 546 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 688 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 699 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 709 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 724 | 104.18.41.10 | 104.18.41.10 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 740 | 104.26.7.153 | 104.26.7.153 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 748 | 104.19.149.143 | 104.19.149.143 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 48 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 95 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 103 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 131 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 134 | freeyx.cloudflare88.eu.org | 2606:4700:3010:bf:5dba:14ae:9501:48f7 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 136 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 141 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 162 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 165 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 184 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 196 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 205 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 250 | cf.090227.xyz | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 344 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 368 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 376 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 444 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 451 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 458 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 474 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 487 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 491 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 515 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 551 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 586 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 596 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 601 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 632 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 634 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 662 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 683 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 693 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 700 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 727 | 162.159.42.32 | 162.159.42.32 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 745 | 104.26.1.44 | 104.26.1.44 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 751 | 104.17.126.38 | 104.17.126.38 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 774 | 2a06:98c1:3108:0:7a27:7d2:b20f:dd0d | 2a06:98c1:3108:0:7a27:7d2:b20f:dd0d | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 54 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 64 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 88 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 92 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 126 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 191 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 192 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 240 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 247 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 271 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 292 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 318 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 321 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 493 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 507 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 512 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 521 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 525 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 544 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 547 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 559 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 562 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 577 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 587 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 600 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 607 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 613 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 665 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 666 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 678 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 702 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 703 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 704 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 728 | 162.159.40.159 | 162.159.40.159 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 738 | 172.67.69.156 | 172.67.69.156 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 42 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 80 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 86 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 94 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 109 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 116 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 124 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 142 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 189 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 193 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 246 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 299 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 319 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 356 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 377 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 383 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 423 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 477 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 538 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 556 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 642 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 663 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 679 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 681 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 710 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 711 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 713 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 24 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 40 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 79 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 90 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 100 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 149 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 248 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 270 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 337 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 366 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 367 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 394 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 441 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 481 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 522 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 549 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 564 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 578 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 595 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 641 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 644 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 654 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 730 | 162.159.49.39 | 162.159.49.39 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 753 | 104.19.155.152 | 104.19.155.152 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 756 | 172.64.144.130 | 172.64.144.130 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 9 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 19 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 50 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 107 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 120 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 133 | freeyx.cloudflare88.eu.org | 141.101.121.23 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 190 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 243 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 312 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 323 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 333 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 352 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 413 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 414 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 432 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 483 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 490 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 520 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 531 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 585 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 594 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 677 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 725 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 18 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 74 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 114 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 252 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 272 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 301 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 313 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 348 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 351 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 385 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 397 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 405 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 415 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 419 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 436 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 460 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 511 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 524 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 625 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 635 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 637 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 640 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 668 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 674 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 736 | 104.20.29.127 | 104.20.29.127 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 13 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 26 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 101 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 159 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 161 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 181 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 242 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 286 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 311 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 345 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 350 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 370 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 389 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 449 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 466 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 532 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 541 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 542 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 548 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 561 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 583 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 673 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 686 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 706 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 714 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 720 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 12 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 30 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 52 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 62 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 112 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 138 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 276 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 410 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 456 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 457 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 510 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 539 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 550 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 618 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 664 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 691 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 701 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 4 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 132 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 143 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 320 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 325 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 329 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 353 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 378 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 400 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 420 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 527 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 657 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 658 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 723 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 741 | 172.67.73.236 | 172.67.73.236 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 773 | 2a06:98c1:3108:2538:ff37:9d72:2b2e:a61f | 2a06:98c1:3108:2538:ff37:9d72:2b2e:a61f | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 33 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 36 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 157 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 170 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 257 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 277 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 334 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 387 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 399 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 495 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 523 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 573 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 616 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 628 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 750 | 104.19.51.31 | 104.19.51.31 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 58 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 69 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 127 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 176 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 380 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 448 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 494 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 687 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 719 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 760 | 172.64.42.141 | 172.64.42.141 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 93 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 104 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 111 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 513 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 558 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 566 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 580 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 592 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 604 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 611 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 645 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 695 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 715 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 718 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 769 | 2a06:98c1:310b:e48e:b8b6:72b6:a637:5a5d | 2a06:98c1:310b:e48e:b8b6:72b6:a637:5a5d | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 65 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 82 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 129 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 308 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 314 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 332 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 386 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 429 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 438 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 471 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 582 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 615 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 659 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 717 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 747 | 104.17.31.153 | 104.17.31.153 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 89 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 113 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 199 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 255 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 269 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 326 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 530 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 20 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 102 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 379 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 402 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 557 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 593 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 660 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 754 | 172.64.155.187 | 172.64.155.187 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 46 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 96 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 239 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 328 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 416 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 609 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 758 | 172.64.156.31 | 172.64.156.31 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 31 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 83 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 91 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 158 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 330 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 392 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 393 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 396 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 404 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 752 | 104.16.145.125 | 104.16.145.125 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 37 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 433 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 437 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 597 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 5 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 43 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 442 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 622 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 721 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 739 | 172.67.79.160 | 172.67.79.160 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 77 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 119 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 197 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 336 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 360 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 518 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 537 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 565 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 543 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 770 | 2a06:98c1:3100:61:d3df:bc30:67f7:c5a | 2a06:98c1:3100:61:d3df:bc30:67f7:c5a | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 75 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 160 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 440 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 621 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 327 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 374 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 22 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 28 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 41 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 171 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 179 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 185 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 395 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 735 | 162.159.34.78 | 162.159.34.78 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 349 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 388 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 85 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 172 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 304 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 482 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 506 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 570 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 650 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 742 | 162.159.40.85 | 162.159.40.85 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 135 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 166 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 175 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 178 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 290 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 417 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 125 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 363 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 771 | 2a06:98c1:310f:398a:fb7c:46a:71b3:4cd0 | 2a06:98c1:310f:398a:fb7c:46a:71b3:4cd0 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 32 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 180 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 746 | 104.17.30.63 | 104.17.30.63 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 447 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 502 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 764 | 162.159.24.205 | 162.159.24.205 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 78 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 186 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 430 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 572 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 99 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 146 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 237 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 265 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 285 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 475 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 174 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 339 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 118 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 287 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 359 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 504 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 529 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 533 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 403 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 497 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 726 | 162.159.3.132 | 162.159.3.132 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 364 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 99 | cloudflare |
| 500 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 581 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 729 | 172.64.48.205 | 172.64.48.205 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 574 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 670 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 100 | cloudflare |
| 97 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 263 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 278 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 101 | cloudflare |
| 470 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 605 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 145 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 177 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 408 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 431 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 472 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 761 | 162.159.58.15 | 162.159.58.15 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 331 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 407 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 606 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 766 | 2803:f800:51:b8:746f:a352:43e8:3c2a | 2803:f800:51:b8:746f:a352:43e8:3c2a | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 139 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 296 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 148 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 545 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 590 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 155 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 182 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 626 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 123 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 169 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 258 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 617 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 661 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 23 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 259 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 274 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 310 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 428 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 473 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 603 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 610 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 17 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 153 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 188 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 656 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 154 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 264 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 267 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 342 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 358 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 599 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 619 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 168 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 620 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 623 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 612 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 114 | cloudflare |
| 765 | 2803:f800:51::717b:266f:2bc | 2803:f800:51::717b:266f:2bc | IPv6 | h2 | ✅ 成功 | 114 | cloudflare |
| 21 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 295 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 297 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 540 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 672 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 150 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 412 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 116 | cloudflare |
| 140 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 117 | cloudflare |
| 712 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 147 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 118 | cloudflare |
| 676 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 685 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 27 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 122 | cloudflare |
| 602 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 81 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 151 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 343 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 125 | cloudflare |
| 354 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 298 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 291 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 128 | cloudflare |
| 357 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 128 | cloudflare |
| 469 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 131 | cloudflare |
| 406 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 138 | cloudflare |
| 288 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 144 | cloudflare |
| 315 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 153 | cloudflare |
| 289 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 164 | cloudflare |
| 501 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 188 | cloudflare |
| 249 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 446 | cloudflare |
| 68 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 672 | cloudflare |
| 38 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 877 | cloudflare |
| 401 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 932 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 23 条记录
- **快 (50-100ms)**: 663 条记录
- **正常 (100-200ms)**: 82 条记录
- **慢 (200-500ms)**: 1 条记录
- **很慢 (>500ms)**: 3 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 2 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*

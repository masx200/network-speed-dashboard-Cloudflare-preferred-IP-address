# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/23 02:45:37
- **数据来源**: connectivity_results-20260123-024536.json
- **总测试数**: 770
- **失败测试数**: 2
- **成功测试数**: 768
- **失败率**: 0.26%
- **平均延迟**: 67.14ms
- **最小延迟**: 34ms
- **最大延迟**: 3962ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/23 02:45:37
- **IP地址**: 2a09:bac5:7971:25a5::3c0:68
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8874, -87.6318
- **时区**: America/Chicago
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
| 272 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 600 | 162.159.41.141 | 162.159.41.141 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.41.141:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 768 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 684 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 314 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 501 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 453 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 462 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 56 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 132 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 289 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 335 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 12 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 61 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 98 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 105 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 183 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 237 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 276 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 338 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 343 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 496 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 612 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 11 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 49 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 94 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 130 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 134 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 147 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 170 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 184 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 258 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 270 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 293 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 300 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 333 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 334 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 336 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 358 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 448 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 484 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 633 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 693 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 7 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 9 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 13 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 145 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 193 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 241 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 267 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 285 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 292 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 312 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 319 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 339 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 369 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 404 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 456 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 466 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 468 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 549 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 593 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 615 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 643 | bestcf.030101.xyz | 2606:4700:0:67b9:b065:a90a:a0a2:2889 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 652 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 667 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 673 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 738 | 104.26.13.215 | 104.26.13.215 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 740 | 104.26.15.207 | 104.26.15.207 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 741 | 172.67.73.198 | 172.67.73.198 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 747 | 104.17.177.91 | 104.17.177.91 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 750 | 104.17.57.66 | 104.17.57.66 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 4 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 35 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 36 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 51 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 57 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 63 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 92 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 174 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 176 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 179 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 182 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 191 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 240 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 287 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 318 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 365 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 378 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 413 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 416 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 529 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 545 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 595 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 602 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 651 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 662 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 690 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 703 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 708 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 734 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 745 | 104.18.94.137 | 104.18.94.137 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 10 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 15 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 27 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 29 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 30 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 42 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 44 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 53 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 73 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 104 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 128 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 165 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 168 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 181 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 188 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 189 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 190 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 251 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 268 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 299 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 323 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 361 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 372 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 384 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 403 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 449 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 452 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 454 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 458 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 460 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 474 | freeyx.cloudflare88.eu.org | 141.101.121.211 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 537 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 546 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 548 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 594 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 607 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 623 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 632 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 707 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 733 | 172.67.71.177 | 172.67.71.177 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 735 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 737 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 764 | 2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837 | 2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 17 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 31 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 37 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 60 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 78 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 80 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 85 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 97 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 148 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 151 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 162 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 187 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 197 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 228 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 283 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 286 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 310 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 348 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 388 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 395 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 414 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 431 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 445 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 446 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 478 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 490 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 518 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 533 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 567 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 569 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 572 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 603 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 609 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 630 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 637 | zread.ai | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 646 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 659 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 687 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 702 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 718 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 729 | 104.20.23.55 | 104.20.23.55 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 732 | 104.26.3.200 | 104.26.3.200 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 742 | 104.16.155.40 | 104.16.155.40 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 743 | 104.19.38.110 | 104.19.38.110 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 748 | 104.17.58.98 | 104.17.58.98 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 16 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 18 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 19 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 32 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 43 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 50 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 67 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 87 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 149 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 185 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 192 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 203 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 244 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 255 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 274 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 309 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 324 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 380 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 383 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 387 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 398 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 410 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 447 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 500 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 517 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 550 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 596 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 597 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 616 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 617 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 631 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 634 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 678 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 697 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 699 | 2a06:98c1:3103:335e:fabe:8b11:3107:3685 | 2a06:98c1:3103:335e:fabe:8b11:3107:3685 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 700 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 717 | 172.64.229.233 | 172.64.229.233 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 746 | 172.64.158.41 | 172.64.158.41 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 755 | 172.64.229.3 | 172.64.229.3 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 6 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 23 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 28 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 38 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 40 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 45 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 54 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 59 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 66 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 74 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 93 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 99 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 100 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 129 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 143 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 146 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 177 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 233 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 242 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 269 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 277 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 280 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 281 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 284 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 301 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 320 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 347 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 351 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 363 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 386 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 405 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 407 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 411 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 429 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 439 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 469 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 472 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 476 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 482 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 507 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 508 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 544 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 574 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 599 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 601 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 610 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 624 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 636 | zread.ai | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 638 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 639 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 649 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 654 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 676 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 704 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 726 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 751 | 104.17.221.233 | 104.17.221.233 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 763 | 2a06:98c1:50:37c9:e79d:f57f:5979:246b | 2a06:98c1:50:37c9:e79d:f57f:5979:246b | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 8 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 21 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 48 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 58 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 64 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 65 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 79 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 103 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 131 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 167 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 171 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 186 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 196 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 201 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 204 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 264 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 316 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 364 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 373 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 381 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 385 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 400 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 409 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 444 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 457 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 465 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 503 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 506 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 575 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 583 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 591 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 641 | bestcf.030101.xyz | 104.18.40.176 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 656 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 668 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 669 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 677 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 701 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 709 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 731 | 104.26.10.159 | 104.26.10.159 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 739 | 104.26.8.60 | 104.26.8.60 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 34 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 41 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 70 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 88 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 95 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 102 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 140 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 141 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 142 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 150 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 155 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 166 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 173 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 178 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 198 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 239 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 265 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 294 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 362 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 368 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 379 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 397 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 406 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 408 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 438 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 470 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 473 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 481 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 491 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 515 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 608 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 642 | bestcf.030101.xyz | 2606:4700:0:6704:6c5a:ab4f:1bd5:b002 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 666 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 671 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 674 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 682 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 696 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 20 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 24 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 62 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 71 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 157 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 194 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 232 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 238 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 263 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 282 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 290 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 315 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 332 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 352 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 374 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 382 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 493 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 528 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 536 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 588 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 604 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 660 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 698 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 744 | 104.17.181.90 | 104.17.181.90 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 749 | 104.18.92.250 | 104.18.92.250 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 762 | 2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d | 2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 2 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 22 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 81 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 96 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 202 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 307 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 337 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 350 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 354 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 371 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 402 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 426 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 432 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 436 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 442 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 450 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 451 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 471 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 480 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 492 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 502 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 516 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 534 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 650 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 658 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 672 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 52 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 109 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 122 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 175 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 353 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 375 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 422 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 434 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 483 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 547 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 592 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 640 | bestcf.030101.xyz | 104.17.182.228 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 691 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 710 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 728 | 104.26.5.25 | 104.26.5.25 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 14 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 75 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 76 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 77 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 110 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 144 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 180 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 243 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 291 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 376 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 418 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 424 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 606 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 655 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 663 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 681 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 689 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 730 | 104.20.27.188 | 104.20.27.188 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 736 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 5 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 46 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 47 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 199 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 259 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 261 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 367 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 440 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 573 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 584 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 626 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 647 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 679 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 705 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 766 | 2a06:98c1:3105:0:1f7b:99bd:a6f2:b984 | 2a06:98c1:3105:0:1f7b:99bd:a6f2:b984 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 26 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 72 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 236 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 342 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 366 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 401 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 412 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 441 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 527 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 556 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 570 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 644 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 768 | 2a06:98c1:3101:0:b1cc:b849:730:3d20 | 2a06:98c1:3101:0:b1cc:b849:730:3d20 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 3 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 89 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 91 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 172 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 200 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 235 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 260 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 279 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 346 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 370 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 505 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 538 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 622 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 645 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 648 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 664 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 675 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 90 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 430 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 443 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 494 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 598 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 614 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 714 | 162.159.39.95 | 162.159.39.95 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 719 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 724 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 271 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 357 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 419 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 495 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 568 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 613 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 685 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 713 | 162.159.45.138 | 162.159.45.138 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 101 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 108 | cf.090227.xyz | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 349 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 423 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 425 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 526 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 692 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 69 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 133 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 278 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 344 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 415 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 504 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 571 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 587 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 629 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 715 | 162.159.38.179 | 162.159.38.179 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 195 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 479 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 670 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 273 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 539 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 625 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 695 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 25 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 249 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 275 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 525 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 722 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 327 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 390 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 106 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 262 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 520 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 769 | 2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b | 2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 169 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 356 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 435 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 688 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 68 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 83 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 757 | 172.64.53.110 | 172.64.53.110 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 297 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 396 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 487 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 530 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 605 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 627 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 694 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 311 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 341 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 657 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 758 | 162.159.45.140 | 162.159.45.140 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 107 | cf.090227.xyz | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 325 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 420 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 509 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 680 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 313 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 464 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 252 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 377 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 394 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 477 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 510 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 540 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 554 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 578 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 321 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 551 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 560 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 620 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 137 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 322 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 355 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 586 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 727 | 162.159.36.94 | 162.159.36.94 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 112 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 391 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 519 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 628 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 754 | 162.159.48.112 | 162.159.48.112 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 229 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 455 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 711 | 108.162.198.183 | 108.162.198.183 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 159 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 253 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 463 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 160 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 359 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 541 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 665 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 712 | 162.159.44.17 | 162.159.44.17 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 753 | 162.159.39.194 | 162.159.39.194 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 39 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 138 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 250 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 433 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 720 | 162.159.40.162 | 162.159.40.162 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 760 | 172.64.52.204 | 172.64.52.204 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 139 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 256 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 317 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 330 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 360 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 392 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 524 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 552 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 555 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 557 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 156 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 164 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 328 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 488 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 512 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 522 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 248 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 305 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 306 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 513 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 127 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 329 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 489 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 581 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 721 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 163 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 257 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 298 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 542 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 115 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 117 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 437 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 475 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cd47:f6f7:aaa7 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 514 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 635 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 543 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 116 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 123 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 621 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 706 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 86 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 561 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 765 | 2a06:98c1:51:0:9437:4b55:9f2c:c3a | 2a06:98c1:51:0:9437:4b55:9f2c:c3a | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 767 | 2803:f800:51:c7:ea87:3a0c:9a73:a94a | 2803:f800:51:c7:ea87:3a0c:9a73:a94a | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 770 | 2400:cb00:f00e:0:9fb9:570f:6b2a:983f | 2400:cb00:f00e:0:9fb9:570f:6b2a:983f | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 562 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 427 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 428 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 523 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 535 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 564 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 725 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 120 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 121 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 99 | cloudflare |
| 126 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 421 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 266 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 566 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 101 | cloudflare |
| 135 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 136 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 686 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 417 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 531 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 511 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 590 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 113 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 303 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 304 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 577 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 661 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 124 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 461 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 619 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 114 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 558 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 308 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 683 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 246 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 296 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 340 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 497 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 499 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 234 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 113 | cloudflare |
| 331 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 498 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 559 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 589 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 84 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 467 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 565 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 761 | 198.41.223.250 | 198.41.223.250 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 752 | 108.162.198.146 | 108.162.198.146 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 295 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 759 | 162.159.44.249 | 162.159.44.249 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 521 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 326 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 618 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 582 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 579 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 716 | 172.64.52.80 | 172.64.52.80 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 345 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 393 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 580 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 82 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 154 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 389 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 653 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 247 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 553 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 723 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 153 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 133 | cloudflare |
| 756 | 162.159.38.113 | 162.159.38.113 | IPv4 | h2 | ✅ 成功 | 135 | cloudflare |
| 230 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 486 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 576 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 611 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 485 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare |
| 125 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 142 | cloudflare |
| 532 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare |
| 585 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare |
| 563 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 150 | cloudflare |
| 161 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 151 | cloudflare |
| 119 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 159 | cloudflare |
| 158 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 166 | cloudflare |
| 245 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 167 | cloudflare |
| 459 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 170 | cloudflare |
| 231 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 171 | cloudflare |
| 302 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 171 | cloudflare |
| 118 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 174 | cloudflare |
| 254 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 176 | cloudflare |
| 399 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 293 | cloudflare |
| 152 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 354 | cloudflare |
| 111 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 407 | cloudflare |
| 33 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 553 | cloudflare |
| 288 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 858 | cloudflare |
| 55 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 3962 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 433 条记录
- **快 (50-100ms)**: 254 条记录
- **正常 (100-200ms)**: 75 条记录
- **慢 (200-500ms)**: 3 条记录
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

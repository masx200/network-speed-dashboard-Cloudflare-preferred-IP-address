# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 04:45:14
- **数据来源**: connectivity_results-20260225-044513.json
- **总测试数**: 836
- **失败测试数**: 10
- **成功测试数**: 826
- **失败率**: 1.20%
- **平均延迟**: 52.69ms
- **最小延迟**: 30ms
- **最大延迟**: 1206ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 04:45:14
- **IP地址**: 2a09:bac5:7973:8c::e:367
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

- **连接超时: I/O超时**: 10 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (10 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 118 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 252 | 172.64.53.202 | 172.64.53.202 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.53.202:443: i/o timeout |
| 395 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.44.188:443: i/o timeout |
| 423 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 439 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.35.192:443: i/o timeout |
| 491 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.44.155:443: i/o timeout |
| 658 | 162.159.6.115 | 162.159.6.115 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.6.115:443: i/o timeout |
| 688 | 172.64.53.0 | 172.64.53.0 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.53.0:443: i/o timeout |
| 735 | 162.159.19.37 | 162.159.19.37 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.19.37:443: i/o timeout |
| 757 | 162.159.45.176 | 162.159.45.176 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.45.176:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 10 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 10 次超时，主要集中在IP段 162.159（5 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 10 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 826 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 529 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 662 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 22 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 381 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 560 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 829 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 17 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 46 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 65 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 75 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 290 | 172.64.144.132 | 172.64.144.132 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 424 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 500 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 519 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 547 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 549 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 553 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 554 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 696 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 48 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 77 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 92 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 94 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 95 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 126 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 187 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 209 | 104.26.6.247 | 104.26.6.247 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 263 | 104.20.22.91 | 104.20.22.91 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 266 | 172.67.72.212 | 172.67.72.212 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 268 | 104.26.5.194 | 104.26.5.194 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 271 | 104.17.56.208 | 104.17.56.208 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 283 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 289 | 172.64.153.183 | 172.64.153.183 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 314 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 318 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 323 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 326 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 330 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 341 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 345 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 350 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 376 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 387 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 388 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 389 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 390 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 392 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 393 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 413 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 461 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 497 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 501 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 503 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 504 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 528 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 537 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 542 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 552 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 558 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 559 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 563 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 564 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 595 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 601 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 632 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 634 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 676 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 685 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 733 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 784 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 786 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 787 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 788 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 792 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 832 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 8 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 11 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 16 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 19 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 21 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 30 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 31 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 32 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 44 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 55 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 62 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 68 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 71 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 74 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 93 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 96 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 98 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 101 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 102 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 130 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 131 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 135 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 143 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 153 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 158 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 159 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 178 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 188 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 207 | 104.26.1.88 | 104.26.1.88 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 210 | 104.26.14.117 | 104.26.14.117 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 212 | 172.67.65.150 | 172.67.65.150 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 217 | 104.17.97.228 | 104.17.97.228 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 218 | 104.25.241.198 | 104.25.241.198 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 219 | 104.25.244.239 | 104.25.244.239 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 270 | 104.25.245.215 | 104.25.245.215 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 274 | 104.25.241.85 | 104.25.241.85 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 287 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 315 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 319 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 320 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 325 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 332 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 338 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 339 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 358 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 384 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 400 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 405 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 427 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 436 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 464 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 468 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 479 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 483 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 496 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 498 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 507 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 509 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 513 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 520 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 523 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 524 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 525 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 533 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 534 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 540 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 541 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 546 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 548 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 550 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 555 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 567 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 581 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 598 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 599 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 623 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 626 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 629 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 644 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 648 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 653 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 655 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 666 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 690 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 693 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 695 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 707 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 721 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 724 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 748 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 760 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 771 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 775 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 779 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 780 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 9 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 12 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 13 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 14 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 20 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 28 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 29 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 58 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 60 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 67 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 69 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 81 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 82 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 83 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 84 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 85 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 99 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 108 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 114 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 119 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 122 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 123 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 134 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 140 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 142 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 146 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 150 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 152 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 154 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 156 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 176 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 180 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 189 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 211 | 104.20.25.181 | 104.20.25.181 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 214 | 104.26.8.171 | 104.26.8.171 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 216 | 104.26.13.110 | 104.26.13.110 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 221 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 241 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 243 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 245 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 255 | 172.64.229.156 | 172.64.229.156 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 259 | 172.67.70.56 | 172.67.70.56 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 264 | 104.26.4.135 | 104.26.4.135 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 272 | 104.25.247.78 | 104.25.247.78 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 275 | 104.25.240.123 | 104.25.240.123 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 277 | 104.25.254.14 | 104.25.254.14 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 278 | 104.25.250.205 | 104.25.250.205 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 281 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 284 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 286 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 288 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 300 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 321 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 324 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 329 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 331 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 334 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 336 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 337 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 340 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 344 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 352 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 355 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 364 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 370 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 372 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 378 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 380 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 382 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 383 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 386 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 391 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 406 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 415 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 429 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 430 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 433 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 444 | freeyx.cloudflare88.eu.org | 141.101.121.66 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 445 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:51:d1e4:65f0:5ed1 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 459 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 477 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 478 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 481 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 485 | zread.ai | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 487 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 489 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 502 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 505 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 512 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 517 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 521 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 531 | bestcf.030101.xyz | 104.19.156.188 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 535 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 536 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 539 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 551 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 561 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 569 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 573 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 578 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 580 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 586 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 590 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 596 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 597 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 603 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 606 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 607 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 608 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 615 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 617 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 621 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 624 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 627 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 628 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 630 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 631 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 633 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 636 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 645 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 646 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 656 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 659 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 660 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 661 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 663 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 664 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 687 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 699 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 703 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 708 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 714 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 722 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 727 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 734 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 752 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 753 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 767 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 768 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 772 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 776 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 777 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 785 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 789 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 794 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 817 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 831 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 10 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 15 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 18 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 34 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 43 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 45 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 56 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 57 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 59 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 61 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 63 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 90 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 103 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 107 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 113 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 116 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 120 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 121 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 125 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 144 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 145 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 147 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 151 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 182 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 204 | 172.64.229.134 | 172.64.229.134 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 205 | 104.18.42.16 | 104.18.42.16 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 208 | 104.20.28.239 | 104.20.28.239 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 213 | 104.26.0.210 | 104.26.0.210 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 220 | 104.17.193.113 | 104.17.193.113 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 223 | 104.17.111.150 | 104.17.111.150 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 224 | 172.64.153.140 | 172.64.153.140 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 226 | 104.18.166.232 | 104.18.166.232 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 238 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 265 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 267 | 104.20.19.180 | 104.20.19.180 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 269 | 104.18.148.235 | 104.18.148.235 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 276 | 104.25.249.225 | 104.25.249.225 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 282 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 291 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 293 | 172.64.151.253 | 172.64.151.253 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 303 | 172.64.229.15 | 172.64.229.15 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 304 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 313 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 316 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 322 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 328 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 342 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 343 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 351 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 354 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 357 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 362 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 365 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 374 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 379 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 402 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 403 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 404 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 414 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 416 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 417 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 418 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 421 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 425 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 426 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 432 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 440 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 441 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 462 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 469 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 475 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 480 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 506 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 508 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 518 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 527 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 532 | bestcf.030101.xyz | 104.17.206.188 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 543 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 544 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 565 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 568 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 588 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 594 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 600 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 604 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 614 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 616 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 620 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 635 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 642 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 647 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 649 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 650 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 652 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 665 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 683 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 684 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 686 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 689 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 694 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 697 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 698 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 710 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 713 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 716 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 718 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 719 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 723 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 729 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 736 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 751 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 756 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 778 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 781 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 782 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 790 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 793 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 795 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 797 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 813 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 814 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 824 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 825 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 834 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 33 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 41 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 47 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 73 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 76 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 115 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 117 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 127 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 128 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 132 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 133 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 138 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 141 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 148 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 184 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 186 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 222 | 104.18.172.20 | 104.18.172.20 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 227 | 104.25.250.174 | 104.25.250.174 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 234 | 198.41.222.191 | 198.41.222.191 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 260 | 172.67.67.0 | 172.67.67.0 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 262 | 104.20.24.244 | 104.20.24.244 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 312 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 333 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 335 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 359 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 361 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 367 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 368 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 369 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 371 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 375 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 377 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 434 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 443 | freeyx.cloudflare88.eu.org | 141.101.120.232 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 460 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 463 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 482 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 488 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 510 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 511 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 522 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 530 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 566 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 651 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 657 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 680 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 691 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 701 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 702 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 706 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 725 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 726 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 728 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 730 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 731 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 737 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 750 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 754 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 755 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 783 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 791 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 809 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 810 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 819 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 821 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 823 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 830 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 835 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 836 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 66 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 70 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 72 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 80 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 124 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 129 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 136 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 137 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 139 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 155 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 181 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 215 | 104.20.20.192 | 104.20.20.192 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 239 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 273 | 104.25.244.36 | 104.25.244.36 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 317 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 353 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 360 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 399 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 411 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 419 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 422 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 428 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 435 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 446 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cdf7:f50d:f763 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 476 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 484 | zread.ai | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 490 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 495 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 516 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 545 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 562 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 592 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 682 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 717 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 720 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 732 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 738 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 815 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 828 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 78 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 97 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 100 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 167 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 183 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 225 | 104.25.245.233 | 104.25.245.233 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 237 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 261 | 104.26.11.33 | 104.26.11.33 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 311 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 366 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 420 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 526 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 556 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 654 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 681 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 692 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 700 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 705 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 744 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 749 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 796 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 816 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 833 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 202 | 162.159.44.101 | 162.159.44.101 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 258 | 104.20.31.132 | 104.20.31.132 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 401 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 486 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 577 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 591 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 618 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 619 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 677 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 812 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 820 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 64 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 179 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 285 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 492 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 587 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 625 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 807 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 109 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 149 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 230 | 162.159.38.134 | 162.159.38.134 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 327 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 602 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 668 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 747 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 811 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 157 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 356 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 514 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 622 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 639 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 233 | 162.159.39.180 | 162.159.39.180 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 437 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 474 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 709 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 49 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 86 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 160 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 199 | 162.159.45.65 | 162.159.45.65 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 246 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 431 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 746 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 24 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 35 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 79 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 105 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 236 | 108.162.198.70 | 108.162.198.70 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 305 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 373 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 385 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 803 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 456 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 669 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 826 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 740 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 774 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 802 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 805 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 818 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 88 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 104 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 106 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 161 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 168 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 169 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 201 | 172.64.52.189 | 172.64.52.189 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 306 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 471 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 472 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 571 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 674 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 759 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 765 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 447 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 466 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 579 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 667 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 762 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 822 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 248 | 162.159.38.45 | 162.159.38.45 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 455 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 557 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 593 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 1 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 87 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 228 | 162.159.11.128 | 162.159.11.128 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 229 | 162.159.3.128 | 162.159.3.128 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 231 | 162.159.12.120 | 162.159.12.120 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 254 | 162.159.44.202 | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 473 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 515 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 582 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 641 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 799 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 2 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 4 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 394 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 605 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 711 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 712 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 761 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 800 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 3 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 110 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 251 | 162.159.45.67 | 162.159.45.67 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 448 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 643 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 798 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 806 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 170 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 232 | 162.159.58.17 | 162.159.58.17 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 253 | 108.162.198.198 | 108.162.198.198 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 295 | 162.159.43.50 | 162.159.43.50 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 640 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 743 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 299 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 408 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 704 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 764 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 5 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 42 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 279 | 162.159.33.191 | 162.159.33.191 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 739 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 770 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 637 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 6 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 206 | 162.159.36.205 | 162.159.36.205 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 250 | 172.64.52.224 | 172.64.52.224 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 741 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 576 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 280 | 162.159.39.74 | 162.159.39.74 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 449 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 610 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 801 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 589 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 763 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 257 | 162.159.20.46 | 162.159.20.46 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 609 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 758 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 804 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 52 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 499 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 91 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 585 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 165 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 174 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 242 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 301 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 310 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 494 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 678 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 679 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 27 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 166 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 173 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 308 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 397 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 398 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 457 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 493 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 39 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 40 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 89 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 112 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 172 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 240 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 302 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 309 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 410 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 412 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 442 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 451 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 452 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 575 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 808 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 54 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 196 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 197 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 244 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 574 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 26 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 164 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 190 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 195 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 612 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 53 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 185 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 396 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 407 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 450 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 613 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 36 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 38 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 177 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 192 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 292 | 162.159.45.150 | 162.159.45.150 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 470 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 50 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 235 | 162.159.1.39 | 162.159.1.39 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 458 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 638 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 672 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 193 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 769 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 200 | 172.64.53.165 | 172.64.53.165 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 247 | 172.64.53.41 | 172.64.53.41 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 409 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 572 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 611 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 673 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 111 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 163 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 454 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 467 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 671 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 51 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 175 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 296 | 162.159.38.83 | 162.159.38.83 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 766 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 203 | 162.159.39.146 | 162.159.39.146 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 570 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 583 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 715 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 23 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 256 | 162.159.0.41 | 162.159.0.41 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 453 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 438 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 465 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 773 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 294 | 172.64.52.181 | 172.64.52.181 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 745 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 171 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 191 | 108.162.198.85 | 108.162.198.85 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 670 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 675 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 827 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 37 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 162 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 194 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 742 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 249 | 162.159.39.20 | 162.159.39.20 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 584 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 198 | 162.159.38.226 | 162.159.38.226 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 25 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 128 | cloudflare |
| 297 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 135 | cloudflare |
| 298 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 138 | cloudflare |
| 307 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 180 | cloudflare |
| 349 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 312 | cloudflare |
| 346 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 510 | cloudflare |
| 7 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 581 | cloudflare |
| 363 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 780 | cloudflare |
| 538 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 1012 | cloudflare |
| 348 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 1065 | cloudflare |
| 347 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 1206 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 626 条记录
- **快 (50-100ms)**: 182 条记录
- **正常 (100-200ms)**: 11 条记录
- **慢 (200-500ms)**: 1 条记录
- **很慢 (>500ms)**: 6 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 10 次
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

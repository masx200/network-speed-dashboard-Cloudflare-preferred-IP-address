# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:24:52
- **数据来源**: connectivity_results-20251229-172451.json
- **总测试数**: 494
- **失败测试数**: 3
- **成功测试数**: 491
- **失败率**: 0.61%
- **平均延迟**: 79.84ms
- **最小延迟**: 35ms
- **最大延迟**: 849ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:24:52
- **IP地址**: 2a09:bac5:c853:2828::400:a
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

- **连接超时: I/O超时**: 3 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 112 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 380 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 482 | 198.41.194.162 | 198.41.194.162 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.194.162:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 313 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 224 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 160 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 173 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 281 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 341 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 210 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 261 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 65 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 66 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 175 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 374 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 450 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 476 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 317 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 463 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 475 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 493 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 44 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 70 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 79 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 221 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 328 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 384 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 440 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 448 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 480 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 111 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 114 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 191 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 264 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 269 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 297 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 342 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 362 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 396 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 410 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 87 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 89 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 162 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 256 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 260 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 276 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 284 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 405 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 451 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 464 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 120 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 225 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 296 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 343 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 354 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 404 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 438 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 449 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 452 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 453 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 459 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 472 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 19 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 86 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 97 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 107 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 135 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 141 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 145 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 272 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 294 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 298 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 355 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 368 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 386 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 393 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 402 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 417 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 468 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 491 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 30 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 147 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 157 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 178 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 232 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 356 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 382 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 447 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 465 | www.7749tv.com | 104.17.196.161 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 473 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 488 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 156 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 172 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 179 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 194 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 195 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 223 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 243 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 249 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 262 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 398 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 425 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 426 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 433 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 467 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 471 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 478 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 47 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 68 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 138 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 171 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 254 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 255 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 266 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 280 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 289 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 300 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 327 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 373 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 403 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 444 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 469 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 479 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 46 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 117 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 121 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 132 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 159 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 204 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 267 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 293 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 295 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 352 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 408 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 424 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 446 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 460 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 474 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 28 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 31 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 32 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 48 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 72 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 80 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 84 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 99 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 108 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 123 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 124 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 140 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 184 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 250 | bestcf.030101.xyz | 104.19.147.41 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 251 | bestcf.030101.xyz | 198.41.209.230 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 253 | bestcf.030101.xyz | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 285 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 288 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 329 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 331 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 370 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 407 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 41 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 129 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 130 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 139 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 142 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 149 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 155 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 246 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 247 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 265 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 291 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 345 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 349 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 366 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 371 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 376 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 33 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 36 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 42 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 76 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 94 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 133 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 209 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 287 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 322 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 383 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 458 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 14 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 146 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 169 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 174 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 235 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 248 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 252 | bestcf.030101.xyz | 2606:4700:0:c5:4803:8845:8bde:1897 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 279 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 282 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 325 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 330 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 392 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 395 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 483 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 494 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 20 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 88 条记录
- **快 (50-100ms)**: 112 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 3 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*

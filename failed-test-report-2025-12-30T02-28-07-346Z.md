# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:28:07
- **数据来源**: connectivity_results-20251230-022806.json
- **总测试数**: 583
- **失败测试数**: 3
- **成功测试数**: 580
- **失败率**: 0.51%
- **平均延迟**: 79.46ms
- **最小延迟**: 54ms
- **最大延迟**: 672ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:28:07
- **IP地址**: 2a09:bac5:6192:183c::26a:e0
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 33.4475, -112.0866
- **时区**: America/Phoenix
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
| 98 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 383 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 462 | 198.41.194.162 | 198.41.194.162 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.194.162:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 580 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 192 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 361 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 408 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 60 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 250 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 258 | cf.090227.xyz | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 259 | cf.090227.xyz | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 431 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 469 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 31 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 40 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 45 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 69 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 70 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 103 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 283 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 36 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 72 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 327 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 403 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 432 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 530 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 15 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 29 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 48 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 79 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 84 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 104 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 200 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 255 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 300 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 313 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 330 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 360 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 549 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 22 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 41 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 63 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 71 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 94 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 118 | cf.877771.xyz | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 122 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 128 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 236 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 238 | bestcf.030101.xyz | 104.17.60.95 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 253 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 261 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 291 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 309 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 315 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 318 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 366 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 387 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 407 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 429 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 445 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 452 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 512 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 515 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 532 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 581 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 14 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 32 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 37 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 52 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 80 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 96 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 114 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 119 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 141 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 166 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 170 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 191 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 195 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 201 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 243 | saas.sin.fan | 104.26.1.111 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 245 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 278 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 332 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 355 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 368 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 401 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 485 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 490 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 524 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 568 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 10 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 43 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 44 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 47 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 49 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 58 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 75 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 83 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 88 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 112 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 121 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 206 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 246 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 251 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 289 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 369 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 396 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 402 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 412 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 430 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 442 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 460 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 470 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 489 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 525 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 16 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 53 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 61 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 74 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 82 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 93 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 95 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 124 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 167 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 193 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 198 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 208 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 244 | saas.sin.fan | 172.67.68.216 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 260 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 290 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 314 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 323 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 334 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 340 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 357 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 381 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 382 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 393 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 398 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 399 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 406 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 410 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 416 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 446 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 448 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 449 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 480 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 487 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 488 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 518 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 520 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 521 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 558 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 583 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 13 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 62 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 64 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 66 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 68 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 105 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 117 | cf.877771.xyz | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 120 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 136 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 182 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 194 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 203 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 240 | bestcf.030101.xyz | 2606:4700:0:2244:5578:2ade:2c48:85 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 248 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 252 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 299 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 312 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 317 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 325 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 338 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 376 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 391 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 397 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 447 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 454 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 458 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 494 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 501 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 502 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 528 | 162.159.62.214 | 162.159.62.214 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 547 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 571 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 5 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 7 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 11 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 30 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 35 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 39 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 51 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 56 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 73 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 81 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 89 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 123 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 140 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 146 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 235 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 266 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 303 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 310 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 320 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 322 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 333 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 343 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 353 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 405 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 411 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 417 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 450 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 468 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 472 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 479 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 498 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 500 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 514 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 523 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 529 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 552 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 553 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 554 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 12 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 54 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 57 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 65 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 92 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 135 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 169 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 176 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 199 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 249 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 275 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 286 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 288 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 311 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 354 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 358 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 377 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 384 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 395 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 428 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 434 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 451 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 453 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 463 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 467 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 481 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 483 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 491 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 495 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 503 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 572 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 578 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 59 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 86 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 87 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 97 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 113 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 126 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 139 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 145 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 183 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 204 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 237 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 277 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 296 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 328 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 329 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 375 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 379 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 418 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 477 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 482 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 504 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 516 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 522 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 535 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 556 | 162.159.40.228 | 162.159.40.228 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 563 | 162.159.48.95 | 162.159.48.95 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 569 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 570 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 576 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 8 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 46 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 127 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 138 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 188 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 189 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 247 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 272 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 308 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 319 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 336 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 344 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 352 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 356 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 386 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 441 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 444 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 464 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 475 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 499 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 546 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 565 | 162.159.58.5 | 162.159.58.5 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 3 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 91 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 100 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 101 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 111 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 130 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 177 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 207 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 239 | bestcf.030101.xyz | 104.17.55.198 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 276 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 302 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 339 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 351 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 400 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 419 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 427 | www.7749tv.com | 104.16.115.36 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 439 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 519 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 526 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 551 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 575 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 67 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 78 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 110 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 147 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 241 | bestcf.030101.xyz | 2606:4700:0:69:bc02:15e0:153e:f2cf | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 256 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 269 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 279 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 321 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 337 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 350 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 388 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 390 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 476 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 484 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 493 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 55 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 148 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 175 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 190 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 295 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 341 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 342 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 378 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 466 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 471 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 496 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 505 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 555 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 19 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 115 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 151 | freeyx.cloudflare88.eu.org | 141.101.120.95 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 152 | freeyx.cloudflare88.eu.org | 141.101.120.232 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 157 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 202 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 274 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 345 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 433 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 455 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 533 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 537 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 559 | 173.245.59.46 | 173.245.59.46 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 9 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 42 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 50 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 254 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 280 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 316 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 365 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 371 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 380 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 435 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 562 | 108.162.193.41 | 108.162.193.41 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 567 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 34 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 99 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 102 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 116 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 165 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 282 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 284 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 287 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 335 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 389 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 404 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 459 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 129 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 149 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 443 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 465 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 534 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 564 | 108.162.196.137 | 108.162.196.137 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 38 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 125 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 171 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 179 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 184 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 264 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 297 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 324 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 531 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 579 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 24 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 185 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 301 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 331 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 457 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 577 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 196 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 197 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 205 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 370 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 372 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 492 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 18 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 109 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 150 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 326 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 359 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 461 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 153 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cdf7:3783:cfe9 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 513 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 527 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 560 | 172.64.49.198 | 172.64.49.198 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 582 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 174 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 545 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 580 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 85 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 90 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 159 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 168 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 186 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 392 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 497 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 539 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 574 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 347 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 348 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 374 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 517 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 25 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 507 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 142 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 421 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 143 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 172 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 440 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 543 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 566 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 162 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 304 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 77 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 160 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 281 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 436 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 557 | 162.159.19.204 | 162.159.19.204 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 161 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 178 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 473 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 536 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 306 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 573 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 4 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 346 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 394 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 486 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 510 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 548 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 242 | saas.sin.fan | 104.26.0.111 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 305 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 478 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 163 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 511 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 158 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 363 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 540 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 28 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 187 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 285 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 426 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 550 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 2 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 265 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 273 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 181 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 425 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 438 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 27 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 164 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 367 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 423 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 263 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 307 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 17 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 21 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 113 | cloudflare |
| 106 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 180 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 349 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 113 | cloudflare |
| 506 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 542 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 137 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 114 | cloudflare |
| 385 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 23 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 270 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 131 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 117 | cloudflare |
| 456 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 133 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 293 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 413 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 108 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 262 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 26 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 120 | cloudflare |
| 267 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 120 | cloudflare |
| 6 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 271 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 121 | cloudflare |
| 132 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 422 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 474 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 294 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 292 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 414 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 420 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 20 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 130 | cloudflare |
| 155 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 130 | cloudflare |
| 415 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 131 | cloudflare |
| 424 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 131 | cloudflare |
| 538 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 131 | cloudflare |
| 541 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 131 | cloudflare |
| 362 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 134 | cloudflare |
| 508 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 154 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 138 | cloudflare |
| 134 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 139 | cloudflare |
| 373 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare |
| 544 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 141 | cloudflare |
| 364 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 142 | cloudflare |
| 298 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare |
| 173 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 146 | cloudflare |
| 437 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 146 | cloudflare |
| 509 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 150 | cloudflare |
| 144 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 164 | cloudflare |
| 107 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 173 | cloudflare |
| 156 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 178 | cloudflare |
| 268 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 225 | cloudflare |
| 561 | 104.18.45.32 | 104.18.45.32 | IPv4 | h2 | ✅ 成功 | 306 | cloudflare |
| 257 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 327 | cloudflare |
| 33 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 449 | cloudflare |
| 409 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 472 | cloudflare |
| 76 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 672 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 495 条记录
- **正常 (100-200ms)**: 79 条记录
- **慢 (200-500ms)**: 5 条记录
- **很慢 (>500ms)**: 1 条记录


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

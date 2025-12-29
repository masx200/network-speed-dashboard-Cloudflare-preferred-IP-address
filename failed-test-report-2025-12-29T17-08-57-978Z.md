# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:08:57
- **数据来源**: connectivity_results-20251229-170856.json
- **总测试数**: 489
- **失败测试数**: 2
- **成功测试数**: 487
- **失败率**: 0.41%
- **平均延迟**: 91.59ms
- **最小延迟**: 66ms
- **最大延迟**: 1796ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:08:58
- **IP地址**: 2a09:bac5:d2a5:2632::3ce:38
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.1446, -104.8116
- **时区**: America/Denver
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
| 37 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 166 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 19 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 333 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 18 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 305 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 313 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 405 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 441 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 442 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 14 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 38 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 79 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 142 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 294 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 314 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 332 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 336 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 395 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 408 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 420 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 5 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 6 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 7 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 11 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 34 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 35 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 43 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 45 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 47 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 49 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 52 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 56 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 110 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 209 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 243 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 292 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 301 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 310 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 315 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 418 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 425 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 430 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 443 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 480 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 484 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 42 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 61 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 67 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 75 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 88 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 93 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 172 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 205 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 295 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 296 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 312 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 327 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 335 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 337 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 357 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 373 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 383 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 384 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 401 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 423 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 444 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 461 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 470 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 476 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 478 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 488 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 23 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 27 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 28 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 32 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 46 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 48 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 51 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 60 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 62 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 63 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 68 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 70 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 74 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 77 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 80 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 92 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 97 | icook.hk | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 109 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 117 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 118 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 123 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 125 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 126 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 141 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 143 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 198 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 199 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 245 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 254 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 265 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 267 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 268 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 325 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 355 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 372 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 385 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 398 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 400 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 407 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 409 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 434 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 436 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 438 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 473 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 477 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 487 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 9 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 13 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 30 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 44 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 57 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 64 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 66 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 73 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 91 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 95 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 115 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 119 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 154 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 173 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 174 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 176 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 182 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 185 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 202 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 208 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 238 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 248 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 249 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 253 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 256 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 262 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 266 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 269 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 273 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 276 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 293 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 318 | bestcf.030101.xyz | 2606:4700::f61:72ba:362:704d | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 320 | xn--b6gac.eu.org | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 330 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 340 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 350 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 353 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 375 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 399 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 403 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 406 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 422 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 426 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 435 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 445 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 454 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 463 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 465 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 469 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 483 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 15 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 31 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 53 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 55 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 111 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 113 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 122 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 140 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 156 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 175 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 203 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 244 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 274 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 321 | xn--b6gac.eu.org | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 334 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 341 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 351 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 376 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 410 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 411 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 414 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 417 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 419 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 200 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


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

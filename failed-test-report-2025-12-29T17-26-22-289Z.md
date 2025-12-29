# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:26:22
- **数据来源**: connectivity_results-20251229-172621.json
- **总测试数**: 492
- **失败测试数**: 2
- **成功测试数**: 490
- **失败率**: 0.41%
- **平均延迟**: 70.70ms
- **最小延迟**: 36ms
- **最大延迟**: 1656ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:26:22
- **IP地址**: 2a09:bac1:76c1:6298::400:3a
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
| 321 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 442 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 78 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 192 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 236 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 415 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 18 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 272 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 372 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 26 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 150 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 293 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 336 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 429 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 441 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 20 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 67 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 93 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 124 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 127 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 132 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 136 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 193 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 253 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 266 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 333 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 339 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 403 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 427 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 10 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 25 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 77 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 92 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 176 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 235 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 261 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 273 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 275 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 286 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 297 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 379 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 431 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 433 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 447 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 475 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 14 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 66 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 82 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 95 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 143 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 178 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 213 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 218 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 224 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 296 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 315 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 325 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 341 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 12 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 74 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 112 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 133 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 135 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 137 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 141 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 156 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 168 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 184 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 190 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 198 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 265 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 274 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 278 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 279 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 311 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 319 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 320 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 328 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 374 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 418 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 449 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 461 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 9 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 45 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 48 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 54 | icook.hk | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 80 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 90 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 91 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 94 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 97 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 151 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 166 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 172 | bestcf.030101.xyz | 104.19.147.41 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 180 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 222 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 243 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 257 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 263 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 287 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 314 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 332 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 337 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 344 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 405 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 411 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 489 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 7 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 52 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 83 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 87 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 139 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 171 | bestcf.030101.xyz | 198.41.209.230 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 219 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 252 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 255 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 258 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 277 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 288 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 290 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 294 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 324 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 373 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 381 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 424 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 463 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 469 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 471 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 477 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 491 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 17 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 28 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 29 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 44 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 73 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 183 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 217 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 233 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 234 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 249 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 316 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 378 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 414 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 421 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 450 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 485 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 488 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 42 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 88 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 128 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 157 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 188 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 194 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 223 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 247 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 310 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 330 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 342 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 362 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 396 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 443 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 451 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 458 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 459 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 460 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 478 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 483 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 16 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 27 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 55 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 72 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 105 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4f66:caf5 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 142 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 264 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 323 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 338 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 340 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 359 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 392 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 413 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 430 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 435 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 446 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 51 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 85 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 104 | freeyx.cloudflare88.eu.org | 141.101.120.109 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 114 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 129 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 130 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 161 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 186 | xn--b6gac.eu.org | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 189 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 225 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 254 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 260 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 262 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 289 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 295 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 303 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 377 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 391 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 420 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 145 条记录
- **快 (50-100ms)**: 55 条记录
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

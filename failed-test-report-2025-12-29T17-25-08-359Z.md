# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:25:08
- **数据来源**: connectivity_results-20251229-172507.json
- **总测试数**: 498
- **失败测试数**: 5
- **成功测试数**: 493
- **失败率**: 1.00%
- **平均延迟**: 79.44ms
- **最小延迟**: 45ms
- **最大延迟**: 628ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:25:08
- **IP地址**: 2a09:bac5:9f24:2828::400:1b
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 36.4766, -78.1847
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 5 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (5 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 75 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 143 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.35.161:443: i/o timeout |
| 380 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 421 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | none | N/A | 0 | N/A | dial tcp 108.162.195.135:443: i/o timeout |
| 428 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.44.168:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 5 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 5 次超时，主要集中在IP段 172.64（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 5 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 12 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 37 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 264 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 340 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 44 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 119 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 385 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 42 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 90 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 321 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 177 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 377 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 475 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 67 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 92 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 125 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 184 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 308 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 442 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 35 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 49 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 88 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 252 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 335 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 415 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 52 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 126 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 246 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 305 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 318 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 337 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 342 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 397 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 448 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 6 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 15 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 46 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 48 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 60 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 65 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 120 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 139 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 164 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 197 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 241 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 250 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 294 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 325 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 339 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 387 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 443 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 453 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 492 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 87 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 168 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 281 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 293 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 297 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 304 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 354 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 460 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 490 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 14 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 22 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 39 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 53 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 70 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 77 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 133 | freeyx.cloudflare88.eu.org | 2606:4700:3010:bf:5dba:fabf:8068:e072 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 206 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 262 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 392 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 414 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 17 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 41 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 54 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 58 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 76 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 94 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 95 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 99 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 101 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 124 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 163 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 271 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 272 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 283 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 285 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 315 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 320 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 322 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 353 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 373 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 406 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 418 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 419 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 433 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 445 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 472 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 493 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 494 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 38 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 45 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 84 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 113 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 131 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 138 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 186 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 201 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 203 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 237 | bestcf.030101.xyz | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 242 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 307 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 309 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 324 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 329 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 383 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 399 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 458 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 471 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 485 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 486 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 98 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 127 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 132 | freeyx.cloudflare88.eu.org | 141.101.120.176 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 166 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 187 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 200 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 205 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 234 | bestcf.030101.xyz | 104.19.147.41 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 254 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 261 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 341 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 359 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 362 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 396 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 410 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 450 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 455 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 457 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 473 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 495 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 16 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 18 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 57 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 69 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 79 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 93 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 141 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 142 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 160 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 161 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 191 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 204 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 238 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 247 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 249 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 282 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 312 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 327 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 334 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 348 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 379 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 400 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 409 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 417 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 30 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 83 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 137 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 239 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 245 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 268 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 311 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 314 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 333 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 361 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 446 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 36 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 59 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 80 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 91 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 153 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 159 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 165 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 240 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 306 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 360 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 371 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 390 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 391 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 441 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 14 条记录
- **快 (50-100ms)**: 186 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 5 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 5 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*

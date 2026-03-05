# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/3/5 00:07:09
- **数据来源**: connectivity_results-20260305-000707.json
- **总测试数**: 960
- **失败测试数**: 17
- **成功测试数**: 943
- **失败率**: 1.77%
- **平均延迟**: 43.94ms
- **最小延迟**: 22ms
- **最大延迟**: 670ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/3/5 00:07:09
- **IP地址**: 2a09:bac6:d739:1232::1d0:bc
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

- **DNS解析错误: 其他DNS错误**: 14 次 (82.4%)
- **连接超时: I/O超时**: 3 次 (17.6%)

### 🔍 按错误类型分类的失败测试详情

#### DNS解析错误: 其他DNS错误 (14 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 3 | ipv4.ip.sb | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 71 | www.okcupid.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 76 | www.visa.com.sg | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 77 | cf.877771.xyz | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 78 | craig.ns.cloudflare.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 79 | www.visa.cn | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 83 | pranab.ns.cloudflare.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 86 | freeyx.cloudflare88.eu.org | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 90 | toy-people.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 91 | kyree.ns.cloudflare.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 92 | decker.ns.cloudflare.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 93 | cris.ns.cloudflare.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 96 | www.whatismyip.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |
| 169 | www.glassdoor.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: 未查询到任何IP记录 |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 63 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 266 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 344 | 198.41.194.162 | 198.41.194.162 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.194.162:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **DNS解析错误**: 14 次 (82.4%)
- **连接超时**: 3 次 (17.6%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 17 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 943 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 64 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 22 | cloudflare |
| 810 | 104.17.111.150 | 104.17.111.150 | IPv4 | h2 | ✅ 成功 | 23 | cloudflare |
| 454 | 162.159.39.26 | 162.159.39.26 | IPv4 | h2 | ✅ 成功 | 26 | cloudflare |
| 122 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 27 | cloudflare |
| 10 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 29 | cloudflare |
| 119 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 120 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 143 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 182 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 29 | cloudflare |
| 314 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 315 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 29 | cloudflare |
| 470 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 498 | 172.64.229.185 | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 631 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 743 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 29 | cloudflare |
| 927 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 29 | cloudflare |
| 929 | 104.20.26.221 | 104.20.26.221 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 8 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 11 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 30 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 128 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 132 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 205 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 210 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 218 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 226 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 230 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 231 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 289 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 304 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 312 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 321 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 323 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 339 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 341 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 347 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 349 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 351 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 356 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 401 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 403 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 412 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 418 | 104.25.254.89 | 104.25.254.89 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 524 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 534 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 538 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 553 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 617 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 619 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 691 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 709 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 717 | 104.20.21.202 | 104.20.21.202 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 718 | 172.67.75.231 | 172.67.75.231 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 723 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 729 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 821 | 162.159.39.12 | 162.159.39.12 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 822 | 162.159.61.56 | 162.159.61.56 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 867 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 888 | 104.26.11.33 | 104.26.11.33 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 910 | 104.25.249.225 | 104.25.249.225 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 916 | 172.64.144.132 | 172.64.144.132 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 949 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 6 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 9 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 25 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 33 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 34 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 46 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 47 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 50 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 65 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 72 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 74 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 82 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 87 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 98 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 111 | bestcf.030101.xyz | 104.18.87.57 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 116 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 118 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 127 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 130 | cf.090227.xyz | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 133 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 136 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 144 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 147 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 151 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 161 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 166 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 180 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 185 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 203 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 212 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 213 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 236 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 238 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 239 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 241 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 243 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 257 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 264 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 269 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 282 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 297 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 316 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 317 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 319 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 342 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 348 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 362 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 363 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 369 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 371 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 372 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 384 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 387 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 402 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 413 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 417 | 104.25.244.87 | 104.25.244.87 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 423 | 104.17.143.82 | 104.17.143.82 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 443 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 447 | 104.17.62.194 | 104.17.62.194 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 458 | 104.26.1.55 | 104.26.1.55 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 459 | 104.26.14.88 | 104.26.14.88 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 461 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 469 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 472 | 172.67.72.250 | 172.67.72.250 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 480 | 104.17.129.66 | 104.17.129.66 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 482 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 499 | 104.18.44.159 | 104.18.44.159 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 515 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 531 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 571 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 575 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 588 | 104.26.4.213 | 104.26.4.213 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 597 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 602 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 603 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 618 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 622 | 108.162.198.206 | 108.162.198.206 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 629 | 104.18.47.193 | 104.18.47.193 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 697 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 719 | 104.20.19.37 | 104.20.19.37 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 721 | 172.67.73.94 | 172.67.73.94 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 726 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 727 | 104.26.3.117 | 104.26.3.117 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 731 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 734 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 738 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 739 | 104.26.0.124 | 104.26.0.124 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 742 | 104.16.148.143 | 104.16.148.143 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 785 | 172.64.229.111 | 172.64.229.111 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 797 | 104.20.30.69 | 104.20.30.69 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 798 | 104.20.16.14 | 104.20.16.14 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 807 | 104.17.109.128 | 104.17.109.128 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 809 | 104.26.1.88 | 104.26.1.88 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 813 | 104.17.150.224 | 104.17.150.224 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 814 | 104.19.37.149 | 104.19.37.149 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 817 | 104.19.54.183 | 104.19.54.183 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 824 | 162.159.45.82 | 162.159.45.82 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 841 | 172.67.65.150 | 172.67.65.150 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 847 | 104.25.241.198 | 104.25.241.198 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 848 | 104.25.244.239 | 104.25.244.239 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 849 | 172.64.153.140 | 172.64.153.140 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 850 | 104.17.193.113 | 104.17.193.113 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 894 | 104.26.5.194 | 104.26.5.194 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 926 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 954 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 7 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 12 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 13 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 21 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 24 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 26 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 27 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 28 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 39 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 40 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 41 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 49 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 51 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 81 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 85 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 106 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 108 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 123 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 131 | cf.090227.xyz | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 135 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 142 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 145 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 148 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 154 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 156 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 162 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 163 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 171 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 172 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 181 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 184 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 199 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 204 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 207 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 208 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 211 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 217 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 219 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 220 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 221 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 222 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 224 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 242 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 252 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 265 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 267 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 278 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 285 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 288 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 296 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 318 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 320 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 335 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 340 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 345 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 346 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 350 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 360 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 365 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 366 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 367 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 373 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 396 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 398 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 400 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 404 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 427 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 428 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 437 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 440 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 441 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 446 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 451 | 108.162.198.232 | 108.162.198.232 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 462 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 464 | 104.20.29.62 | 104.20.29.62 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 473 | 172.67.64.116 | 172.67.64.116 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 481 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 483 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 486 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 487 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 489 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 490 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 491 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 501 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 503 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 505 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 509 | 172.64.157.214 | 172.64.157.214 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 523 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 530 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 543 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 554 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 561 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 565 | 172.64.229.106 | 172.64.229.106 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 568 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 583 | 104.20.29.234 | 104.20.29.234 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 585 | 172.67.65.44 | 172.67.65.44 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 610 | 104.25.243.36 | 104.25.243.36 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 613 | 104.25.242.137 | 104.25.242.137 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 620 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 621 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 630 | 172.64.146.121 | 172.64.146.121 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 634 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 638 | 104.18.40.200 | 104.18.40.200 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 656 | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 661 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 679 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 683 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 688 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 689 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 693 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 701 | 172.64.229.158 | 172.64.229.158 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 702 | 104.18.32.161 | 104.18.32.161 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 712 | 104.26.15.142 | 104.26.15.142 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 713 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 716 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 722 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 749 | 104.17.165.38 | 104.17.165.38 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 754 | 172.64.229.172 | 172.64.229.172 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 756 | 104.18.33.253 | 104.18.33.253 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 765 | 162.159.44.58 | 162.159.44.58 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 772 | 2a06:98c1:3100:7f11:4805:1c25:83ab:6033 | 2a06:98c1:3100:7f11:4805:1c25:83ab:6033 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 773 | 2a06:98c1:3108:1a71:a277:b3bf:80a:c2a3 | 2a06:98c1:3108:1a71:a277:b3bf:80a:c2a3 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 777 | 2a06:98c1:3100:7f01:2f67:5ef8:2a97:8d82 | 2a06:98c1:3100:7f01:2f67:5ef8:2a97:8d82 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 787 | 172.64.157.174 | 172.64.157.174 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 788 | 104.20.18.49 | 104.20.18.49 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 796 | 104.20.28.118 | 104.20.28.118 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 808 | 104.17.178.163 | 104.17.178.163 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 811 | 104.17.163.110 | 104.17.163.110 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 812 | 104.19.144.174 | 104.19.144.174 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 842 | 104.26.0.210 | 104.26.0.210 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 852 | 104.25.245.233 | 104.25.245.233 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 882 | 172.64.229.156 | 172.64.229.156 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 887 | 172.67.67.0 | 172.67.67.0 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 890 | 104.20.22.91 | 104.20.22.91 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 891 | 104.26.4.135 | 104.26.4.135 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 896 | 104.25.245.215 | 104.25.245.215 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 898 | 104.25.247.78 | 104.25.247.78 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 900 | 104.25.241.85 | 104.25.241.85 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 902 | 172.64.151.253 | 172.64.151.253 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 915 | 172.64.153.183 | 172.64.153.183 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 925 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 928 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 933 | 104.18.44.187 | 104.18.44.187 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 937 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 945 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 951 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 5 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 14 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 16 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 18 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 19 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 23 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 31 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 36 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 38 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 42 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 43 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 44 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 52 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 53 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 59 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 67 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 73 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 84 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 105 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 107 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 109 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 110 | bestcf.030101.xyz | 104.17.220.38 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 112 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 114 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 115 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 117 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 124 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 150 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 152 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 153 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 158 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 159 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 168 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 193 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 201 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 206 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 214 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 223 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 244 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 245 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 247 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 248 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 253 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 258 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 268 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 270 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 281 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 283 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 286 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 309 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 310 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 311 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 313 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 322 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 332 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 334 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 336 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 338 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 359 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 361 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 364 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 377 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 390 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 395 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 408 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 410 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 411 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 420 | 104.25.246.24 | 104.25.246.24 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 424 | 104.18.160.38 | 104.18.160.38 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 426 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 429 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 431 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 445 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 452 | 162.159.38.52 | 162.159.38.52 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 460 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 463 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 465 | 172.67.78.23 | 172.67.78.23 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 467 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 468 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 471 | 104.26.12.33 | 104.26.12.33 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 474 | 172.67.68.110 | 172.67.68.110 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 479 | 104.25.247.129 | 104.25.247.129 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 492 | 104.25.241.19 | 104.25.241.19 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 494 | 104.16.247.125 | 104.16.247.125 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 497 | 104.17.171.88 | 104.17.171.88 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 507 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 511 | 108.162.198.223 | 108.162.198.223 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 512 | 172.64.152.85 | 172.64.152.85 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 520 | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 522 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 528 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 529 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 532 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 536 | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 541 | 2a06:98c1:310b::fda8:fa9e:4a3e | 2a06:98c1:310b::fda8:fa9e:4a3e | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 584 | 172.67.73.129 | 172.67.73.129 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 589 | 104.20.18.125 | 104.20.18.125 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 596 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 598 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 601 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 604 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 612 | 104.25.252.192 | 104.25.252.192 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 624 | 162.159.27.183 | 162.159.27.183 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 633 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 635 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 637 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 641 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 645 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 653 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 659 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 660 | 162.159.45.0 | 162.159.45.0 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 664 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 666 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 692 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 700 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 704 | 104.26.2.242 | 104.26.2.242 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 705 | 104.26.5.121 | 104.26.5.121 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 708 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 710 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 720 | 172.67.74.78 | 172.67.74.78 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 724 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 733 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 741 | 104.17.104.208 | 104.17.104.208 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 744 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 746 | 104.18.42.129 | 104.18.42.129 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 747 | 104.17.118.227 | 104.17.118.227 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 761 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 764 | 162.159.38.226 | 162.159.38.226 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 774 | 2a06:98c1:310e:5f9e:101d:94ce:cb6b:49ca | 2a06:98c1:310e:5f9e:101d:94ce:cb6b:49ca | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 789 | 104.20.21.83 | 104.20.21.83 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 790 | 104.20.31.51 | 104.20.31.51 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 804 | 104.20.27.57 | 104.20.27.57 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 805 | 104.20.26.30 | 104.20.26.30 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 834 | 2a06:98c1:3109:4ed3:5a24:20f9:dac0:1f5e | 2a06:98c1:3109:4ed3:5a24:20f9:dac0:1f5e | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 837 | 2a06:98c1:3107:2f:54ce:e76b:a1c9:7cd | 2a06:98c1:3107:2f:54ce:e76b:a1c9:7cd | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 839 | 104.26.14.117 | 104.26.14.117 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 845 | 104.26.13.110 | 104.26.13.110 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 853 | 104.18.166.232 | 104.18.166.232 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 864 | 108.162.198.70 | 108.162.198.70 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 885 | 104.20.31.132 | 104.20.31.132 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 886 | 172.67.70.56 | 172.67.70.56 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 889 | 104.20.24.244 | 104.20.24.244 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 893 | 104.20.19.180 | 104.20.19.180 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 897 | 104.17.56.208 | 104.17.56.208 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 906 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 909 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 912 | 104.25.250.205 | 104.25.250.205 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 923 | 172.67.73.196 | 172.67.73.196 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 924 | 172.67.75.212 | 172.67.75.212 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 932 | 172.67.77.185 | 172.67.77.185 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 947 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 953 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 4 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 35 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 48 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 66 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 69 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 75 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 88 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 89 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 97 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 102 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 121 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 129 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 146 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 157 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 160 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 165 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 167 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 173 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 209 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 215 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 225 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 233 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 249 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 279 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 303 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 306 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 307 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 325 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 326 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 337 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 358 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 370 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 376 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 381 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 399 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 405 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 406 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 407 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 409 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 419 | 104.17.56.177 | 104.17.56.177 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 421 | 104.25.240.227 | 104.25.240.227 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 422 | 104.25.242.249 | 104.25.242.249 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 444 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 477 | 104.25.252.135 | 104.25.252.135 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 478 | 104.17.97.146 | 104.17.97.146 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 485 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 496 | 104.25.253.253 | 104.25.253.253 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 518 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 519 | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 521 | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 525 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 526 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 535 | 2606:4700:59:764d:d406:c823:e52f:4f3a | 2606:4700:59:764d:d406:c823:e52f:4f3a | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 537 | 2a06:98c1:51:8:7944:48b0:1301:5ced | 2a06:98c1:51:8:7944:48b0:1301:5ced | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 547 | 162.159.45.237 | 162.159.45.237 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 551 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 552 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 556 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 572 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 577 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 582 | 172.67.77.104 | 172.67.77.104 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 590 | 104.25.241.235 | 104.25.241.235 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 599 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 611 | 104.25.250.121 | 104.25.250.121 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 614 | 104.25.254.47 | 104.25.254.47 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 627 | 162.159.44.199 | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 636 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 643 | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 647 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 651 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 652 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 654 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 678 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 684 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 686 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 687 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 690 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 698 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 706 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 714 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 715 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 737 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 740 | 172.67.79.166 | 172.67.79.166 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 745 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 750 | 104.19.144.110 | 104.19.144.110 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 768 | 2a06:98c1:3106:0:fb94:fc7b:2b7f:ae54 | 2a06:98c1:3106:0:fb94:fc7b:2b7f:ae54 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 770 | 2a06:98c1:50::46cb:8c34:28e3 | 2a06:98c1:50::46cb:8c34:28e3 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 780 | 108.162.198.146 | 108.162.198.146 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 786 | 162.159.3.82 | 162.159.3.82 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 794 | 162.159.44.101 | 162.159.44.101 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 795 | 104.20.25.245 | 104.20.25.245 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 799 | 104.20.29.75 | 104.20.29.75 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 815 | 104.17.161.112 | 104.17.161.112 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 816 | 104.19.44.187 | 104.19.44.187 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 818 | 104.18.39.162 | 104.18.39.162 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 826 | 172.64.147.253 | 172.64.147.253 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 836 | 104.20.28.239 | 104.20.28.239 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 838 | 104.26.6.247 | 104.26.6.247 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 843 | 104.26.8.171 | 104.26.8.171 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 844 | 104.20.20.192 | 104.20.20.192 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 846 | 104.17.97.228 | 104.17.97.228 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 854 | 104.25.250.174 | 104.25.250.174 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 856 | 162.159.3.128 | 162.159.3.128 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 865 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 869 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 873 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 874 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 892 | 172.67.72.212 | 172.67.72.212 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 895 | 104.18.148.235 | 104.18.148.235 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 901 | 104.25.240.123 | 104.25.240.123 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 905 | 172.64.229.15 | 172.64.229.15 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 919 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 921 | 172.67.79.249 | 172.67.79.249 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 922 | 104.26.5.53 | 104.26.5.53 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 930 | 104.20.22.141 | 104.20.22.141 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 931 | 172.67.65.81 | 172.67.65.81 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 936 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 948 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 950 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 952 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 20 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 45 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 55 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 68 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 70 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 94 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 100 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 113 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 126 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 149 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 164 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 179 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 183 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 186 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 187 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 190 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 232 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 240 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 246 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 250 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 271 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 272 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 275 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 280 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 284 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 324 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 333 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 368 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 382 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 415 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 416 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 436 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 475 | 172.67.76.61 | 172.67.76.61 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 488 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 495 | 104.25.255.103 | 104.25.255.103 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 500 | 162.159.39.156 | 162.159.39.156 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 533 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 569 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 570 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 579 | 172.64.41.47 | 172.64.41.47 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 580 | 172.67.64.123 | 172.67.64.123 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 581 | 104.20.20.156 | 104.20.20.156 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 586 | 172.67.76.20 | 172.67.76.20 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 606 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 607 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 608 | 104.25.240.21 | 104.25.240.21 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 615 | 104.25.246.117 | 104.25.246.117 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 616 | 104.25.248.93 | 104.25.248.93 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 625 | 172.64.229.82 | 172.64.229.82 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 632 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 648 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 669 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 676 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 681 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 694 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 695 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 711 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 736 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 748 | 172.64.145.253 | 172.64.145.253 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 776 | 2a06:98c1:3106:d8af:8b29:8a81:bf10:9cef | 2a06:98c1:3106:d8af:8b29:8a81:bf10:9cef | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 778 | 162.159.45.194 | 162.159.45.194 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 801 | 172.64.229.134 | 172.64.229.134 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 802 | 104.18.42.16 | 104.18.42.16 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 806 | 104.17.184.153 | 104.17.184.153 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 825 | 172.64.229.243 | 172.64.229.243 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 828 | 2a06:98c1:3109:4e23:df4:a0d:70:bb41 | 2a06:98c1:3109:4e23:df4:a0d:70:bb41 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 871 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 875 | 172.64.53.41 | 172.64.53.41 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 940 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 946 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 960 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 15 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 17 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 196 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 200 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 216 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 227 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 228 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 229 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 273 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 277 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 343 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 378 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 380 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 439 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 456 | 162.159.6.186 | 162.159.6.186 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 466 | 104.26.6.238 | 104.26.6.238 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 516 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 527 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 539 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 550 | 162.159.39.196 | 162.159.39.196 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 587 | 172.67.79.218 | 172.67.79.218 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 605 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 640 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 644 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 646 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 650 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 675 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 696 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 735 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 755 | 162.159.46.38 | 162.159.46.38 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 840 | 104.20.25.181 | 104.20.25.181 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 851 | 104.18.172.20 | 104.18.172.20 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 877 | 162.159.39.20 | 162.159.39.20 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 899 | 104.25.244.36 | 104.25.244.36 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 907 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 911 | 104.25.254.14 | 104.25.254.14 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 920 | 104.20.24.239 | 104.20.24.239 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 935 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 938 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 29 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 37 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 103 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 274 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 276 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 305 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 308 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 379 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 385 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 388 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 425 | 104.16.245.121 | 104.16.245.121 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 484 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 540 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 542 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 576 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 609 | 104.25.245.173 | 104.25.245.173 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 639 | 2a06:98c1:3102:8768:b929:7455:f040:5aee | 2a06:98c1:3102:8768:b929:7455:f040:5aee | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 649 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 760 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 775 | 2a06:98c1:3102:96:65b:cff7:1c28:b82a | 2a06:98c1:3102:96:65b:cff7:1c28:b82a | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 829 | 2a06:98c1:310e:e69c:a40c:913f:b22d:5951 | 2a06:98c1:310e:e69c:a40c:913f:b22d:5951 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 866 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 54 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 62 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 95 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 155 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 170 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 197 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 202 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 287 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 290 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 291 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 375 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 383 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 414 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 455 | 172.64.229.149 | 172.64.229.149 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 476 | 172.64.146.137 | 172.64.146.137 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 544 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 557 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 558 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 594 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 880 | 108.162.198.198 | 108.162.198.198 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 959 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 374 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 386 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 555 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 560 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 831 | 2a06:98c1:3103:f1:e422:c175:1c8e:df1a | 2a06:98c1:3103:f1:e422:c175:1c8e:df1a | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 934 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 299 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 642 | 2a06:98c1:310c::dd:f399:427e | 2a06:98c1:310c::dd:f399:427e | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 699 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 517 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 104 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 138 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 189 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 235 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 254 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 331 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 355 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 434 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 672 | 162.159.38.68 | 162.159.38.68 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 677 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 791 | 162.159.45.65 | 162.159.45.65 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 913 | 162.159.33.191 | 162.159.33.191 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 56 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 237 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 295 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 328 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 448 | 162.159.44.246 | 162.159.44.246 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 453 | 172.64.52.194 | 172.64.52.194 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 493 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 545 | 172.64.53.195 | 172.64.53.195 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 549 | 162.159.44.60 | 162.159.44.60 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 563 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 573 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 662 | 162.159.44.36 | 162.159.44.36 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 707 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 732 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 763 | 108.162.198.85 | 108.162.198.85 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 878 | 172.64.52.224 | 172.64.52.224 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 903 | 162.159.43.50 | 162.159.43.50 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 58 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 134 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 137 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 141 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 174 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 178 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 192 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 194 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 198 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 251 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 353 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 357 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 433 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 442 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 508 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 593 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 658 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 671 | 108.162.198.168 | 108.162.198.168 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 682 | 162.159.39.189 | 162.159.39.189 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 728 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 757 | 162.159.45.8 | 162.159.45.8 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 759 | 162.159.9.224 | 162.159.9.224 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 781 | 162.159.39.90 | 162.159.39.90 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 782 | 172.64.53.216 | 172.64.53.216 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 803 | 162.159.36.205 | 162.159.36.205 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 819 | 162.159.38.100 | 162.159.38.100 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 820 | 162.159.10.81 | 162.159.10.81 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 835 | 2a06:98c1:51:403a:9791:47cd:2ae7:2 | 2a06:98c1:51:403a:9791:47cd:2ae7:2 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 857 | 162.159.38.134 | 162.159.38.134 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 858 | 162.159.12.120 | 162.159.12.120 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 862 | 198.41.222.191 | 198.41.222.191 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 879 | 162.159.45.67 | 162.159.45.67 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 908 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 918 | 172.64.52.181 | 172.64.52.181 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 944 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 60 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 61 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 101 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 176 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 195 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 234 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 255 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 256 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 260 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 263 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 298 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 300 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 329 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 354 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 397 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 432 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 435 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 449 | 162.159.45.121 | 162.159.45.121 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 548 | 172.64.52.150 | 172.64.52.150 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 564 | 108.162.198.152 | 108.162.198.152 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 591 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 623 | 162.159.49.244 | 162.159.49.244 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 626 | 162.159.33.28 | 162.159.33.28 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 628 | 172.64.53.103 | 172.64.53.103 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 673 | 172.64.53.101 | 172.64.53.101 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 703 | 162.159.58.251 | 162.159.58.251 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 758 | 162.159.3.222 | 162.159.3.222 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 767 | 162.159.18.22 | 162.159.18.22 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 859 | 162.159.58.17 | 162.159.58.17 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 863 | 162.159.1.39 | 162.159.1.39 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 904 | 162.159.38.83 | 162.159.38.83 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 942 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 955 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 956 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 191 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 262 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 293 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 330 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 352 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 457 | 162.159.21.16 | 162.159.21.16 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 559 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 562 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 574 | 172.64.40.68 | 172.64.40.68 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 665 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 668 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 680 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 685 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 730 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 762 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 766 | 162.159.34.205 | 162.159.34.205 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 769 | 2a06:98c1:51:0:4371:ce16:475:2557 | 2a06:98c1:51:0:4371:ce16:475:2557 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 771 | 2a06:98c1:51:4e:5188:50a9:cbd1:917d | 2a06:98c1:51:4e:5188:50a9:cbd1:917d | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 792 | 172.64.53.165 | 172.64.53.165 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 800 | 162.159.39.146 | 162.159.39.146 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 870 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 881 | 162.159.44.202 | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 883 | 162.159.0.41 | 162.159.0.41 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 884 | 162.159.20.46 | 162.159.20.46 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 939 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 943 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 2 | 2a06:98c1:50::b9:30bc:de63 | 2a06:98c1:50::b9:30bc:de63 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 99 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 140 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 175 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 177 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 261 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 294 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 327 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 389 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 393 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 430 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 438 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 450 | 172.64.53.220 | 172.64.53.220 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 504 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 513 | 162.159.40.8 | 162.159.40.8 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 514 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 567 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 595 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 600 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 657 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 663 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 667 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 783 | 162.159.38.137 | 162.159.38.137 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 793 | 172.64.52.189 | 172.64.52.189 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 832 | 2a06:98c1:51:403a:971b:3a6b:65cb:bf8a | 2a06:98c1:51:403a:971b:3a6b:65cb:bf8a | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 833 | 2400:cb00:2049:ddcd:7011:1125:9f3:6a4f | 2400:cb00:2049:ddcd:7011:1125:9f3:6a4f | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 855 | 162.159.11.128 | 162.159.11.128 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 860 | 162.159.39.180 | 162.159.39.180 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 861 | 172.64.53.202 | 172.64.53.202 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 868 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 941 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 958 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 80 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 139 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 188 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 301 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 392 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 510 | 162.159.0.79 | 162.159.0.79 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 546 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 566 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 674 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 823 | 172.64.52.114 | 172.64.52.114 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 830 | 2400:cb00:2049:ddcd:c532:69fa:251a:af7a | 2400:cb00:2049:ddcd:c532:69fa:251a:af7a | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 876 | 162.159.38.45 | 162.159.38.45 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 957 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 57 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 391 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 394 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 502 | 172.64.53.15 | 172.64.53.15 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 655 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 779 | 172.64.52.133 | 172.64.52.133 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 827 | 162.159.60.188 | 162.159.60.188 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 872 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 914 | 162.159.39.74 | 162.159.39.74 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 506 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 917 | 162.159.45.150 | 162.159.45.150 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 259 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 578 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 592 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 302 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 784 | 162.159.44.37 | 162.159.44.37 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 670 | 172.64.52.168 | 172.64.52.168 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 753 | 162.159.39.165 | 162.159.39.165 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 752 | 104.17.119.130 | 104.17.119.130 | IPv4 | h2 | ✅ 成功 | 168 | cloudflare |
| 751 | 172.64.154.86 | 172.64.154.86 | IPv4 | h2 | ✅ 成功 | 181 | cloudflare |
| 725 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 242 | cloudflare |
| 22 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 400 | cloudflare |
| 292 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 448 | cloudflare |
| 125 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 558 | cloudflare |
| 32 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 670 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 749 条记录
- **快 (50-100ms)**: 186 条记录
- **正常 (100-200ms)**: 3 条记录
- **慢 (200-500ms)**: 3 条记录
- **很慢 (>500ms)**: 2 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 17 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*

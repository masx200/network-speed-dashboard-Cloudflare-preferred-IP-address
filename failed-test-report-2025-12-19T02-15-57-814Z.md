# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:15:57
- **数据来源**: connectivity_results-20251219-021556.json
- **总测试数**: 453
- **失败测试数**: 3
- **成功测试数**: 450
- **失败率**: 0.66%
- **平均延迟**: 79.35ms
- **最小延迟**: 43ms
- **最大延迟**: 609ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:15:57
- **IP地址**: 2a09:bac5:9f24:2832::401:1a
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

- **连接超时: I/O超时**: 3 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 6 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 37 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 207 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.44.155:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 66 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 114 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 338 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 147 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 442 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 88 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 201 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 339 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 378 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 82 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 111 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 133 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 332 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 10 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 62 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 92 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 98 | icook.hk | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 186 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 278 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 315 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 404 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 419 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 441 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 87 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 121 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 168 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 257 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 290 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 302 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 379 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 392 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 43 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 142 | freeyx.cloudflare88.eu.org | 141.101.121.193 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 153 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 174 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 261 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 291 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 303 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 316 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 357 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 385 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 406 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 436 | ifconfig.co | 104.21.54.91 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 17 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 24 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 49 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 118 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 150 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 154 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 236 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 276 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 284 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 285 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 324 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 384 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 5 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 16 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 35 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 50 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 54 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 61 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 108 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 132 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 171 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 202 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 204 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 253 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 355 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 390 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 391 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 422 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 428 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 53 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 64 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 84 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 113 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 115 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 172 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 188 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 247 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 251 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 255 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 310 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 351 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 356 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 395 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 427 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 14 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 52 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 60 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 69 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 74 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 85 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 110 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 117 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 146 | cu.877774.xyz | 104.26.4.115 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 189 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 198 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 205 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 260 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 262 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 292 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 321 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 323 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 337 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 353 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 375 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 388 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 421 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 434 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 8 | www.ipget.net | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 33 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 80 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 106 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 109 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 240 | bestcf.030101.xyz | 104.19.146.144 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 246 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 256 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 289 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 304 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 318 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 322 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 344 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 374 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 405 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 410 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 424 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 15 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 30 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 34 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 44 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 48 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 59 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 77 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 90 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 116 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 148 | cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 200 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 249 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 389 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 416 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 51 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 67 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 72 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 107 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 123 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 151 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 190 | cloudflare-ip.mofashi.ltd | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 196 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 241 | bestcf.030101.xyz | 2606:4700:0:3701:6875:ca63:41c9:a311 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 254 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 330 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 362 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 408 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 426 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 443 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 9 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 78 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 124 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 135 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 149 | cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 195 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 243 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 288 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 317 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 320 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 340 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 371 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 381 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 383 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 393 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 403 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 415 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 418 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 433 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 13 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 31 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 181 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 203 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 235 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 244 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 269 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 277 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 300 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 309 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 312 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 314 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 335 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 9 条记录
- **快 (50-100ms)**: 191 条记录
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

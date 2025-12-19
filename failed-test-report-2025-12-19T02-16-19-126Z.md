# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:16:19
- **数据来源**: connectivity_results-20251219-021618.json
- **总测试数**: 459
- **失败测试数**: 2
- **成功测试数**: 457
- **失败率**: 0.44%
- **平均延迟**: 73.83ms
- **最小延迟**: 42ms
- **最大延迟**: 621ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:16:19
- **IP地址**: 2a09:bac5:9f23:2446::39d:1d
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

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 278 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 428 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

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
| 35 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 130 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 338 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 437 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 438 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 453 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 78 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 286 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 93 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 124 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 266 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 7 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 115 | bestcf.030101.xyz | 104.19.146.144 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 141 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 441 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 129 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 184 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 190 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 350 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 9 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 17 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 20 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 23 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 110 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 165 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 189 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 263 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 297 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 413 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 426 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 444 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 6 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 19 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 22 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 30 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 34 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 41 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 136 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 194 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 262 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 279 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 280 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 290 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 291 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 342 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 357 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 411 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 12 | www.ipget.net | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 46 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 73 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 131 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 195 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 197 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 237 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 252 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 260 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 294 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 298 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 301 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 321 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 339 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 360 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 388 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 13 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 27 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 89 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 98 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 114 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 191 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 244 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 307 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 352 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 412 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 183 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 198 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 238 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 264 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 270 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 317 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 330 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 341 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 351 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 366 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 32 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 36 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 65 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 66 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 92 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 106 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 121 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 123 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 153 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 186 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 188 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 267 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 283 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 296 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 300 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 313 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 326 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 408 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 415 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 431 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 456 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 16 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 33 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 96 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 103 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 118 | bestcf.030101.xyz | 2606:4700:0:57:503f:ebaa:486e:53f7 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 134 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 176 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 177 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 241 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 268 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 302 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 337 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 364 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 414 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 451 | cu.877774.xyz | 104.26.4.114 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 457 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 4 | www.7749tv.com | 104.19.104.16 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 5 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 15 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 26 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 84 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 105 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 116 | bestcf.030101.xyz | 104.17.96.48 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 122 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 126 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 140 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 192 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 201 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 257 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 277 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 304 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 305 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 325 | ifconfig.co | 104.21.54.91 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 378 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 393 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 409 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 421 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 452 | cu.877774.xyz | 104.26.4.115 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 72 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 94 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 100 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 111 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 164 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 200 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 234 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 236 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 269 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 287 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 344 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 345 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 347 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 349 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 386 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 398 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 418 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 425 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 435 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 440 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 11 | www.ipget.net | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 31 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 95 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 125 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 135 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 162 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 169 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 172 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 182 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 193 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 231 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 239 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 243 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 245 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 285 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 306 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 309 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 319 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 346 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 389 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 424 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 454 | cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 455 | cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 24 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 29 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 55 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 82 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 15 条记录
- **快 (50-100ms)**: 185 条记录
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

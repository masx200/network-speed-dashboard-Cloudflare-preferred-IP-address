# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:22:21
- **数据来源**: connectivity_results-20251229-172221.json
- **总测试数**: 507
- **失败测试数**: 2
- **成功测试数**: 505
- **失败率**: 0.39%
- **平均延迟**: 64.05ms
- **最小延迟**: 42ms
- **最大延迟**: 748ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:22:22
- **IP地址**: 2a09:bac5:6214:28::4:323
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 37.1835, -121.7714
- **时区**: America/Los_Angeles
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
| 311 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 357 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

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
| 431 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 463 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 347 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 72 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 98 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 210 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 222 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 228 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 411 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 250 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 284 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 285 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 484 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 490 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 44 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 50 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 66 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 85 | cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 143 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 164 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 190 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 221 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 240 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 345 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 423 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 456 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 464 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 506 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 77 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 78 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 166 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 241 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 251 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 275 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 302 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 317 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 325 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 339 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 362 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 396 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 409 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 426 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 460 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 466 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 486 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 488 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 17 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 19 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 32 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 33 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 45 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 49 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 81 | cu.877774.xyz | 104.26.4.113 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 94 | freeyx.cloudflare88.eu.org | 2606:4700:3009::83da:e4f:b8a5 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 148 | bestcf.030101.xyz | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 157 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 182 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 216 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 232 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 244 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 273 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 301 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 308 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 400 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 402 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 404 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 413 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 432 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 446 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 455 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 459 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 61 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 100 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 124 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 140 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 144 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 181 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 204 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 215 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 243 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 268 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 281 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 283 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 288 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 295 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 307 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 320 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 374 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 387 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 392 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 406 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 407 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 458 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 470 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 475 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 10 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 54 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 76 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 87 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 104 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 105 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 110 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 112 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 120 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 145 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 162 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 167 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 170 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 171 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 174 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 179 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 194 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 196 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 197 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 198 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 200 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 234 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 253 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 271 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 272 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 294 | icook.hk | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 299 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 303 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 319 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 351 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 358 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 361 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 393 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 418 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 437 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 447 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 477 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 48 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 52 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 73 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 137 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 146 | bestcf.030101.xyz | 198.41.209.230 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 154 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 156 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 159 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 184 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 186 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 225 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 229 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 247 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 255 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 257 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 261 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 269 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 316 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 330 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 348 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 356 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 359 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 366 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 379 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 394 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 401 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 414 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 425 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 478 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 489 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 493 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 494 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 505 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 507 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 8 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 58 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 82 | cu.877774.xyz | 104.26.4.114 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 88 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 90 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 103 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 139 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 142 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 168 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 172 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 202 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 226 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 238 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 287 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 291 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 292 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 296 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 305 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 336 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 388 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 429 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 433 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 443 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 449 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 469 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 482 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 491 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 496 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 498 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 499 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 504 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 21 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 35 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 38 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 95 条记录
- **快 (50-100ms)**: 105 条记录
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

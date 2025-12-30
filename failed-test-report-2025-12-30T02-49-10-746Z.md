# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:49:10
- **数据来源**: connectivity_results-20251230-024910.json
- **总测试数**: 622
- **失败测试数**: 2
- **成功测试数**: 620
- **失败率**: 0.32%
- **平均延迟**: 67.91ms
- **最小延迟**: 36ms
- **最大延迟**: 3504ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:49:10
- **IP地址**: 2a09:bac1:7680:28::1d2:3
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
| 89 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 289 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 620 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 296 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 431 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 550 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 71 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 197 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 278 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 293 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 361 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 410 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 51 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 95 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 196 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 204 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 277 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 288 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 350 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 409 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 6 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 10 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 54 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 55 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 56 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 73 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 87 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 138 | www.ipget.net | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 173 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 188 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 198 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 208 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 279 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 356 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 390 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 403 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 425 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 445 | bestcf.030101.xyz | 104.17.55.198 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 448 | bestcf.030101.xyz | 2606:4700:0:72:e1e6:5546:888c:67d1 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 457 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 472 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 591 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 619 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 9 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 62 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 63 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 93 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 103 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 111 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 143 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 159 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 194 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 203 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 276 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 285 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 287 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 306 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 355 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 391 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 413 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 443 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 459 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 460 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 466 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 471 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 482 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 593 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 14 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 37 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 66 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 97 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 101 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 106 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 135 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 142 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 158 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 168 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 174 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 187 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 193 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 207 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 254 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 266 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 280 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 282 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 284 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 297 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 302 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 336 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 341 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 370 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 379 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 388 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 402 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 422 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 428 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 436 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 442 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 446 | bestcf.030101.xyz | 104.17.60.95 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 449 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 452 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 458 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 500 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 504 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 521 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 541 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 542 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 555 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 573 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 581 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 4 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 35 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 43 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 49 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 70 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 94 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 110 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 239 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 268 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 328 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 330 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 335 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 353 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 366 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 395 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 432 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 433 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 441 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 444 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 481 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 502 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 531 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 535 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 549 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 553 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 554 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 597 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 622 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 5 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 11 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 15 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 53 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 65 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 107 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 112 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 114 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 115 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 155 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 162 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 170 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 171 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 182 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 184 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 185 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 186 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 237 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 243 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 257 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 275 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 299 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 300 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 307 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 312 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 354 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 397 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 401 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 414 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 430 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 434 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 435 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 440 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 453 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 468 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 477 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 511 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 539 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 543 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 571 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 601 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 612 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 17 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 22 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 39 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 50 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 57 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 59 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 72 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 85 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 104 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 116 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 117 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 130 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 132 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 140 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 160 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 167 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 199 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 248 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 253 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 256 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 281 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 311 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 333 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 347 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 357 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 373 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 384 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 418 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 419 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 421 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 462 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 467 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 469 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 475 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 498 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 501 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 507 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 510 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 527 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 551 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 588 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 607 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 7 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 23 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 27 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 38 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 40 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 44 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 45 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 74 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 86 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 88 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 109 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 131 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 157 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 175 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 191 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 192 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 202 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 206 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 294 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 345 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 364 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 367 | freeyx.cloudflare88.eu.org | 141.101.121.116 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 385 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 387 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 404 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 429 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 438 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 455 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 461 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 491 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 496 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 525 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 557 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 582 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 584 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 590 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 596 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 13 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 41 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 64 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 90 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 102 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 201 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 241 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 246 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 283 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 303 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 334 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 456 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 473 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 480 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 495 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 512 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 533 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 540 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 545 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 552 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 562 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 564 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 585 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 592 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 595 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 615 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 618 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 620 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 18 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 47 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 52 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 67 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 79 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 136 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 190 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 236 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 238 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 290 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 295 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 337 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 338 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 348 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 375 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 386 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 447 | bestcf.030101.xyz | 2606:4700:19:e3ed:311e:da65:ac7a:3d52 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 505 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 508 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 526 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 530 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 534 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 537 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 538 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 544 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 547 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 548 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 565 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 572 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 589 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 602 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 609 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 621 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 8 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 16 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 60 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 69 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 76 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 81 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 113 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 169 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 200 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 240 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 244 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 250 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 252 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 258 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 260 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 329 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 368 | freeyx.cloudflare88.eu.org | 2606:4700:3009:0:72:9d2c:ac0d:3727 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 376 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 424 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 463 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 464 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 465 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 474 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 499 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 583 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 594 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 603 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 100 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 125 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 133 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 137 | www.ipget.net | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 139 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 141 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 156 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 235 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 301 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 317 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 326 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 360 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 372 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 378 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 451 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 476 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 506 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 523 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 528 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 559 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 561 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 605 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 48 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 68 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 84 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 164 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 313 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 325 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 352 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 439 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 454 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 515 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 566 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 578 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 604 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 83 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 98 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 124 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 147 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 152 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 270 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 298 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 321 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 351 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 380 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 416 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 493 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 519 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 529 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 546 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 3 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 29 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 99 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 151 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 245 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 305 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 308 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 309 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 365 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 427 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 497 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 517 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 556 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 586 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 19 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 32 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 161 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 267 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 322 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 331 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 405 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 415 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 426 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 569 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 606 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 608 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 610 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 82 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 255 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 346 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 396 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 399 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 489 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 516 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 576 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 61 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 91 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 123 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 165 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 166 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 292 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 327 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 411 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 492 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 518 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 154 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 369 | freeyx.cloudflare88.eu.org | 2606:4700:3010:bf:5dba:fa1d:5993:9cf8 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 450 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 509 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 12 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 24 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 26 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 163 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 183 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 205 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 242 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 406 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 417 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 96 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 108 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 195 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 304 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 381 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 412 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 575 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 344 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 392 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 470 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 488 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 580 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 599 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 172 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 291 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 374 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 389 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 574 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 587 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 92 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 176 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 408 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 423 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 478 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 503 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 536 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 105 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 118 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 265 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 520 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 286 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 31 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 522 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 269 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 398 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 400 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 524 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 371 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 494 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 320 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 358 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 484 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 490 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 579 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 42 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 611 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 600 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 2 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 189 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 247 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 479 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 249 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 486 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 274 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 181 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 78 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 377 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 394 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 532 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 28 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 318 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 437 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 568 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 613 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 617 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 180 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 272 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 314 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 323 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 343 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 616 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 75 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 179 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 261 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 362 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 119 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 122 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 177 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 262 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 319 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 570 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 121 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 264 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 146 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 324 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 342 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 383 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 407 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 36 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 80 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 153 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 251 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 1 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 126 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 129 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 514 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 33 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 148 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 363 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 560 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 34 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 99 | cloudflare |
| 120 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 145 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 577 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 127 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 100 | cloudflare |
| 340 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 30 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 77 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 259 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 359 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 273 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 393 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 487 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 567 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 25 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 315 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 483 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 149 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 263 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 271 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 128 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 116 | cloudflare |
| 150 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 316 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 332 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 563 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 485 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 614 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 20 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 513 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 558 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 144 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 138 | cloudflare |
| 598 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 139 | cloudflare |
| 178 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare |
| 382 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 142 | cloudflare |
| 349 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 157 | cloudflare |
| 420 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 162 | cloudflare |
| 339 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 164 | cloudflare |
| 46 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 166 | cloudflare |
| 134 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 300 | cloudflare |
| 21 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 472 | cloudflare |
| 310 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 698 | cloudflare |
| 58 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 3504 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 191 条记录
- **快 (50-100ms)**: 391 条记录
- **正常 (100-200ms)**: 34 条记录
- **慢 (200-500ms)**: 2 条记录
- **很慢 (>500ms)**: 2 条记录


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

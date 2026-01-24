# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/24 00:15:20
- **数据来源**: connectivity_results-20260124-001520.json
- **总测试数**: 781
- **失败测试数**: 3
- **成功测试数**: 778
- **失败率**: 0.38%
- **平均延迟**: 61.21ms
- **最小延迟**: 34ms
- **最大延迟**: 576ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/24 00:15:20
- **IP地址**: 2a09:bac5:7977:1cd2::2df:fb
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8874, -87.6318
- **时区**: America/Chicago
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
| 333 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 695 | 172.64.49.146 | 172.64.49.146 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.49.146:443: i/o timeout |
| 768 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 778 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 10 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 528 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 16 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 28 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 384 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 389 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 513 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 307 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 358 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 732 | 104.17.109.58 | 104.17.109.58 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 29 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 45 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 61 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 62 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 69 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 120 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 259 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 285 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 336 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 361 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 390 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 413 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 473 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 478 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 487 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 524 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 660 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 709 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 761 | 2a06:98c1:3107:0:f6:2182:d4a8:339f | 2a06:98c1:3107:0:f6:2182:d4a8:339f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 9 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 31 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 82 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 91 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 102 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 152 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 179 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 252 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 265 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 277 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 279 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 287 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 289 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 453 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 531 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 588 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 680 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 714 | 104.18.37.234 | 104.18.37.234 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 733 | 104.16.249.149 | 104.16.249.149 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 12 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 15 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 41 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 55 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 103 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 242 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 267 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 275 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 299 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 317 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 356 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 378 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 383 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 387 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 394 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 419 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 450 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 541 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 556 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 574 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 595 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 597 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 600 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 601 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 642 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 676 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 679 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 731 | 104.16.156.40 | 104.16.156.40 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 738 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 741 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 744 | 172.64.229.167 | 172.64.229.167 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 766 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 771 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 58 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 77 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 78 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 142 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 176 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 178 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 186 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 268 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 269 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 273 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 280 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 303 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 316 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 329 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 352 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 353 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 406 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 437 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 462 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 479 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 501 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 508 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 512 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 589 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 609 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 648 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 665 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 717 | 104.26.2.45 | 104.26.2.45 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 739 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 772 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 775 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 6 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 22 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 36 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 38 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 73 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 79 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 90 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 100 | freeyx.cloudflare88.eu.org | 141.101.120.161 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 154 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 157 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 168 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 174 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 183 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 239 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 241 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 246 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 251 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 263 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 304 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 315 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 326 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 331 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 397 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 423 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 474 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 480 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 521 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 537 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 538 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 550 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 567 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 605 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 607 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 613 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 639 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 664 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 678 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 688 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 698 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 708 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 710 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 737 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 757 | 2803:f800:50:41ba:5536:115f:5191:1805 | 2803:f800:50:41ba:5536:115f:5191:1805 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 764 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 11 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 54 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 66 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 92 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 109 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 116 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 171 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 233 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 237 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 238 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 270 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 271 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 306 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 321 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 349 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 388 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 407 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 432 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 448 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 469 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 476 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 502 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 504 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 514 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 518 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 532 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 544 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 547 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 549 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 606 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 635 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 638 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 687 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 718 | 104.26.9.186 | 104.26.9.186 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 719 | 104.26.15.22 | 104.26.15.22 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 767 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 769 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 770 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 43 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 60 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 72 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 75 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 83 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 104 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 121 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 130 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 134 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 143 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 173 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 180 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 274 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 286 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 302 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 330 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 401 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 402 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 405 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 456 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 465 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 466 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 467 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 475 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 503 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 509 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 517 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 526 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 540 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 543 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 554 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 569 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 573 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 608 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 625 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 632 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 658 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 663 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 668 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 681 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 689 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 697 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 760 | 2a06:98c1:3105::4c1a:e9f2:d41e | 2a06:98c1:3105::4c1a:e9f2:d41e | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 777 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 20 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 59 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 76 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 126 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 135 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 151 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 156 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 165 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 166 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 191 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 240 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 260 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 278 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 282 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 293 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 301 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 350 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 369 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 385 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 410 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 422 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 438 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 441 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 482 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 486 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 555 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 572 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 599 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 610 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 645 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 655 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 657 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 662 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 675 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 684 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 685 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 723 | 104.26.3.109 | 104.26.3.109 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 729 | 104.17.153.22 | 104.17.153.22 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 759 | 2a06:98c1:310d:70:d7a5:3f5:bb52:dfbf | 2a06:98c1:310d:70:d7a5:3f5:bb52:dfbf | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 774 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 30 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 42 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 64 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 119 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 136 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 175 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 272 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 312 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 314 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 318 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 319 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 327 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 332 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 351 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 379 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 428 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 433 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 439 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 451 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 472 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 499 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 594 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 614 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 640 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 674 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 694 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 721 | 104.26.11.221 | 104.26.11.221 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 735 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 763 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 765 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 773 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 25 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 56 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 71 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 81 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 101 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b79:282c:48ab:b061 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 108 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 110 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 132 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 163 | bestcf.030101.xyz | 2606:4700::7e99:e849:a432:54e0 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 203 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 204 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 253 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 281 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 311 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 335 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 362 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 416 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 477 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 520 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 523 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 534 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 536 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 641 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 670 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 690 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 713 | 172.64.229.129 | 172.64.229.129 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 742 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 743 | 104.17.106.109 | 104.17.106.109 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 780 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 5 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 40 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 93 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 155 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 162 | bestcf.030101.xyz | 104.17.126.234 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 170 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 182 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 185 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 190 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 196 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 202 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 418 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 440 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 481 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 539 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 545 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 552 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 568 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 585 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 615 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 634 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 656 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 693 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 706 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 722 | 104.26.7.8 | 104.26.7.8 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 734 | 104.19.60.36 | 104.19.60.36 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 65 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 70 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 127 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 129 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 159 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 169 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 195 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 290 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 328 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 344 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 347 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 360 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 386 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 411 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 445 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 447 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 498 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 505 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 506 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 522 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 584 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 604 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 659 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 661 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 705 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 716 | 104.20.16.97 | 104.20.16.97 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 725 | 104.26.1.119 | 104.26.1.119 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 779 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 27 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 67 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 68 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 128 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 133 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 177 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 184 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 261 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 294 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 341 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 366 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 431 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 444 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 471 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 525 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 598 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 669 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 701 | 108.162.198.97 | 108.162.198.97 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 712 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 726 | 104.16.154.33 | 104.16.154.33 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 17 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 47 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 153 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 249 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 264 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 300 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 346 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 382 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 403 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 404 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 420 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 429 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 484 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 497 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 519 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 624 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 703 | 162.159.45.11 | 162.159.45.11 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 724 | 104.26.6.10 | 104.26.6.10 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 776 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 14 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 74 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 131 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 138 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 194 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 338 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 343 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 357 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 400 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 417 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 421 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 430 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 483 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 511 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 548 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 671 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 672 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 692 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 727 | 104.17.168.76 | 104.17.168.76 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 13 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 19 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 26 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 137 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 197 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 199 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 291 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 320 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 370 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 446 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 571 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 611 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 666 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 700 | 172.64.53.182 | 172.64.53.182 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 707 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 728 | 104.18.80.38 | 104.18.80.38 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 118 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 122 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 125 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 257 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 284 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 334 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 339 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 340 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 398 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 454 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 530 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 654 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 673 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 691 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 696 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 736 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 21 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 39 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 80 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 99 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 167 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 172 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 193 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 234 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 381 | www.7749tv.com | 104.25.211.15 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 515 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 516 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 553 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 740 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 755 | 2803:f800:51:b883:8fe2:9a40:3016:f74 | 2803:f800:51:b883:8fe2:9a40:3016:f74 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 7 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 164 | bestcf.030101.xyz | 2606:4700::28:3815 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 348 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 373 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 442 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 449 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 198 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 266 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 305 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 529 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 551 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 631 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 653 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 720 | 104.26.5.55 | 104.26.5.55 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 24 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 158 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 161 | bestcf.030101.xyz | 104.16.147.120 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 288 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 354 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 458 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 460 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 535 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 612 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 683 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 1 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 44 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 98 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 140 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 337 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 355 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 455 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 468 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 510 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 749 | 162.159.44.235 | 162.159.44.235 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 2 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 124 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 148 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 201 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 452 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 542 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 592 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 596 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 637 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 644 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 758 | 2400:cb00:2049:b1aa:a619:c233:939c:74 | 2400:cb00:2049:b1aa:a619:c233:939c:74 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 781 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 345 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 704 | 162.159.38.32 | 162.159.38.32 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 32 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 63 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 112 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 146 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 380 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 443 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 577 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 646 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 57 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 117 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 200 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 243 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 276 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 492 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 493 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 546 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 559 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 630 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 50 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 95 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 111 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 250 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 295 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 377 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 602 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 629 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 699 | 162.159.39.75 | 162.159.39.75 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 756 | 2a06:98c1:3103:ca:fa78:84d3:2596:f679 | 2a06:98c1:3103:ca:fa78:84d3:2596:f679 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 84 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 113 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 141 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 160 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 192 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 590 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 622 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 652 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 745 | 162.159.34.8 | 162.159.34.8 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 149 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 150 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 232 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 283 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 647 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 711 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 730 | 104.17.145.189 | 104.17.145.189 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 18 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 254 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 748 | 172.64.53.175 | 172.64.53.175 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 296 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 412 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 507 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 667 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 298 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 391 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 485 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 527 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 575 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 587 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 3 | 2a06:98c1:310a:c1:ab96:14d6:b31a:6d03 | 2a06:98c1:310a:c1:ab96:14d6:b31a:6d03 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 4 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 363 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 426 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 434 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 491 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 752 | 172.64.52.145 | 172.64.52.145 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 123 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 255 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 309 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 628 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 105 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 376 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 415 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 560 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 564 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 576 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 582 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 46 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 145 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 409 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 297 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 490 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 34 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 96 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 106 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 558 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 593 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 616 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 408 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 470 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 578 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 464 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 649 | 172.64.52.45 | 172.64.52.45 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 754 | 2a06:98c1:3109:ecc2:5371:912b:e506:cd47 | 2a06:98c1:3109:ecc2:5371:912b:e506:cd47 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 292 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 427 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 591 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 618 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 750 | 172.64.48.160 | 172.64.48.160 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 37 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 579 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 626 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 399 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 650 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 651 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 435 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 581 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 633 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 682 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 762 | 2a06:98c1:310f:6020:3bcb:4a0f:8160:4ca2 | 2a06:98c1:310f:6020:3bcb:4a0f:8160:4ca2 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 88 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 368 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 375 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 396 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 500 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 87 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 89 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 325 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 561 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 566 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 617 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 621 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 623 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 627 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 247 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 393 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 436 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 677 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 114 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 235 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 533 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 557 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 753 | 162.159.39.5 | 162.159.39.5 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 144 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 463 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 636 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 702 | 162.159.44.197 | 162.159.44.197 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 258 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 495 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 188 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 248 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 262 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 51 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 236 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 374 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 746 | 162.159.38.160 | 162.159.38.160 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 414 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 580 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 115 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 324 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 367 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 715 | 172.64.48.78 | 172.64.48.78 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 189 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 365 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 424 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 603 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 686 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 53 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 313 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 323 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 395 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 459 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 49 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 97 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 583 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 778 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 494 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 461 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 496 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 372 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 85 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 457 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 565 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 751 | 108.162.198.221 | 108.162.198.221 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 139 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 371 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 425 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 52 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 245 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 747 | 162.159.45.118 | 162.159.45.118 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 33 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 570 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 114 | cloudflare |
| 231 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 392 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 562 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 107 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 187 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 35 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 230 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 147 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 48 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 123 | cloudflare |
| 563 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 86 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 620 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 643 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 133 | cloudflare |
| 322 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 135 | cloudflare |
| 256 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 364 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 308 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 138 | cloudflare |
| 586 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare |
| 619 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 144 | cloudflare |
| 94 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 147 | cloudflare |
| 310 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 149 | cloudflare |
| 489 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 152 | cloudflare |
| 488 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 154 | cloudflare |
| 244 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 165 | cloudflare |
| 342 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 198 | cloudflare |
| 181 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 344 | cloudflare |
| 359 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 567 | cloudflare |
| 23 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 572 | cloudflare |
| 8 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 576 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 352 条记录
- **快 (50-100ms)**: 378 条记录
- **正常 (100-200ms)**: 44 条记录
- **慢 (200-500ms)**: 1 条记录
- **很慢 (>500ms)**: 3 条记录


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

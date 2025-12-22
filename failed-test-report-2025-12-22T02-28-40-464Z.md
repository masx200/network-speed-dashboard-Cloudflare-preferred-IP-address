# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/22 02:28:40
- **数据来源**: connectivity_results-20251222-022839.json
- **总测试数**: 443
- **失败测试数**: 2
- **成功测试数**: 441
- **失败率**: 0.45%
- **平均延迟**: 60.54ms
- **最小延迟**: 34ms
- **最大延迟**: 845ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/22 02:28:40
- **IP地址**: 2a09:bac5:c857:a0::10:4ea
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
| 32 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 373 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

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
| 72 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 117 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 132 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 241 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 200 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 239 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 247 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 263 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 324 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 350 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 371 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 18 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 40 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 70 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 79 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 183 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 253 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 370 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 88 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 100 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 141 | freeyx.cloudflare88.eu.org | 141.101.120.20 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 190 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 346 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 386 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 25 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 33 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 61 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 94 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 136 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 144 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 162 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 265 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 293 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 298 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 349 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 365 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 11 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 19 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 59 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 112 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 149 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 152 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 194 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 249 | cf.090227.xyz | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 257 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 267 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 280 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 308 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 321 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 322 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 335 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 348 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 364 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 424 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 436 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 14 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 20 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 87 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 93 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 120 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 123 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 128 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 179 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 188 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 268 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 311 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 330 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 339 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 47 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 60 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 64 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 71 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 81 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 97 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 110 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 119 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 133 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 160 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 161 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 168 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 196 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 202 | bestcf.030101.xyz | 172.67.191.116 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 243 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 260 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 269 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 304 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 306 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 319 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 347 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 363 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 374 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 400 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 430 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 10 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 41 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 45 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 50 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 69 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 75 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 95 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 113 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 115 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 140 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 143 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 146 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 169 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 170 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 172 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 176 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 187 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 195 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 233 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 237 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 276 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 281 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 283 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 305 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 320 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 325 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 362 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 387 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 395 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 409 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 413 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 425 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 13 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 30 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 39 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 51 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 53 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 89 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 91 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 92 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 101 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 137 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 159 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 164 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 184 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 193 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 242 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 252 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 273 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 285 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 314 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 327 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 328 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 353 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 357 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 372 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 384 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 389 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 394 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 398 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 12 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 43 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 55 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 108 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 122 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 155 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 158 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 180 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 182 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 186 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 205 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 290 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 309 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 317 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 341 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 368 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 441 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 15 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 65 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 96 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 105 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 107 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 199 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 204 | bestcf.030101.xyz | 2606:4700::fffd:819d:acda | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 232 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 255 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 286 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 344 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 352 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 354 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 423 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 5 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 58 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 62 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 200 条记录
- **快 (50-100ms)**: 0 条记录
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

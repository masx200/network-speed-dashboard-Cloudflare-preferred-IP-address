# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 04:43:21
- **数据来源**: connectivity_results-20260225-044320.json
- **总测试数**: 835
- **失败测试数**: 3
- **成功测试数**: 832
- **失败率**: 0.36%
- **平均延迟**: 80.08ms
- **最小延迟**: 58ms
- **最大延迟**: 750ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 04:43:21
- **IP地址**: 2a09:bac1:76c0:8628::3ce:33
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

- **连接超时: I/O超时**: 3 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 107 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 360 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 415 | 198.41.194.162 | 198.41.194.162 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.194.162:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 832 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 212 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 763 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 686 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 50 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 163 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 739 | 104.20.20.192 | 104.20.20.192 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 5 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 10 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 16 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 49 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 63 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 67 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 94 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 116 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 131 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 177 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 187 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 244 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 269 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 271 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 332 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 431 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 450 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 493 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 559 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 583 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 638 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 698 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 737 | 104.26.0.210 | 104.26.0.210 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 8 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 9 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 12 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 15 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 19 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 22 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 23 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 39 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 45 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 51 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 53 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 56 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 95 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 97 | freeyx.cloudflare88.eu.org | 141.101.120.121 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 129 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 167 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 176 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 178 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 192 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 267 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 277 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 280 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 287 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 296 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 318 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 343 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 364 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 367 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 372 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 374 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 382 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 423 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 424 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 425 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 440 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 465 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 466 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 472 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 483 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 484 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 495 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 512 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 518 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 532 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 536 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 588 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 593 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 621 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 635 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 660 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 662 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 668 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 670 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 713 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 728 | 172.64.229.134 | 172.64.229.134 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 729 | 104.18.42.16 | 104.18.42.16 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 734 | 104.26.14.117 | 104.26.14.117 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 735 | 104.20.25.181 | 104.20.25.181 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 746 | 104.17.111.150 | 104.17.111.150 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 747 | 172.64.153.140 | 172.64.153.140 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 749 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 781 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 797 | 172.67.67.0 | 172.67.67.0 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 835 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 4 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 11 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 17 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 24 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 40 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 42 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 46 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 55 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 57 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 66 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 69 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 71 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 75 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 76 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 82 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 84 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 91 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 106 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 108 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 115 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 138 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 155 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 158 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 160 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 165 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 173 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 180 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 182 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 188 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 200 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 201 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 203 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 246 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 275 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 276 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 286 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 288 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 293 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 301 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 333 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 341 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 355 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 358 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 379 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 385 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 399 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 414 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 422 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 435 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 438 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 439 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 449 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 452 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 453 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 454 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 474 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 482 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 494 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 497 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 499 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 503 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 510 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 516 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 525 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 535 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 550 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 552 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 564 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 576 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 580 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 615 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 617 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 637 | 104.25.241.198 | 104.25.241.198 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 639 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 645 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 653 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 655 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 659 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 669 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 717 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 718 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 738 | 104.26.8.171 | 104.26.8.171 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 740 | 104.26.13.110 | 104.26.13.110 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 764 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 766 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 771 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 784 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 791 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 792 | 172.64.229.156 | 172.64.229.156 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 808 | 104.26.5.194 | 104.26.5.194 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 818 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 820 | 172.64.151.253 | 172.64.151.253 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 831 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 6 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 14 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 18 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 21 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 41 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 47 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 48 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 52 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 58 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 59 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 62 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 64 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 68 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 70 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 72 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 73 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 81 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 83 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 85 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 96 | freeyx.cloudflare88.eu.org | 141.101.121.241 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 105 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 114 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 117 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 128 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 137 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 159 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 166 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 170 | bestcf.030101.xyz | 104.19.156.188 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 181 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 183 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 194 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 196 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 197 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 198 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 226 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 233 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 236 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 237 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 245 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 258 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 272 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 274 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 298 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 303 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 317 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 327 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 328 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 329 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 339 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 352 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 354 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 363 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 365 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 376 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 378 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 383 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 384 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 389 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 413 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 418 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 433 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 444 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 456 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 459 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 469 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 491 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 506 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 517 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 522 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 528 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 533 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 560 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 573 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 578 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 579 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 582 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 587 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 589 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 594 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 595 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 626 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 632 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 648 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 649 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 652 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 654 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 661 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 666 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 667 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 671 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 692 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 696 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 702 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 707 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 720 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 731 | 104.26.1.88 | 104.26.1.88 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 732 | 104.20.28.239 | 104.20.28.239 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 733 | 104.26.6.247 | 104.26.6.247 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 751 | 104.25.250.174 | 104.25.250.174 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 765 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 773 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 786 | 104.18.148.235 | 104.18.148.235 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 796 | 172.67.70.56 | 172.67.70.56 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 812 | 172.64.153.183 | 172.64.153.183 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 816 | 104.25.249.225 | 104.25.249.225 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 817 | 172.64.144.132 | 172.64.144.132 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 828 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 832 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 3 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 7 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 13 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 43 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 54 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 74 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 98 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:f5e8:7af2:1225:455c | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 102 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 103 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 112 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 125 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 127 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 130 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 139 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 169 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 179 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 191 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 195 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 202 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 228 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 229 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 241 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 249 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 264 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 268 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 270 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 284 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 289 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 290 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 291 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 292 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 294 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 295 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 297 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 304 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 315 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 336 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 348 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 362 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 366 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 373 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 375 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 380 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 386 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 388 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 400 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 403 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 421 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 441 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 455 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 470 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 473 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 476 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 477 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 478 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 480 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 485 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 498 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 500 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 502 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 505 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 507 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 509 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 519 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 520 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 529 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 534 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 553 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 572 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 585 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 590 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 597 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 598 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 605 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 613 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 614 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 618 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 619 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 631 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 646 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 664 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 665 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 689 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 694 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 697 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 699 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 706 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 710 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 712 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 744 | 104.17.193.113 | 104.17.193.113 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 770 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 775 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 783 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 785 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 804 | 104.25.250.205 | 104.25.250.205 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 806 | 104.17.56.208 | 104.17.56.208 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 827 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 44 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 110 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 111 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 113 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 123 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 126 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 140 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 142 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 148 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 149 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 153 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 156 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 174 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 186 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 193 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 199 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 227 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 248 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 250 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 263 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 265 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 279 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 302 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 306 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 320 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 335 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 337 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 345 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 346 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 351 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 353 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 356 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 359 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 368 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 369 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 370 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 371 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 405 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 430 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 434 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 443 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 448 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 457 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 464 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 492 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 496 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 501 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 504 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 513 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 514 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 521 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 526 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 531 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 561 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 577 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 584 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 586 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 591 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 596 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 604 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 644 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 647 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 650 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 651 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 663 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 674 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 688 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 708 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 716 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 719 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 743 | 104.25.244.239 | 104.25.244.239 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 745 | 104.18.172.20 | 104.18.172.20 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 768 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 795 | 104.20.31.132 | 104.20.31.132 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 803 | 104.20.19.180 | 104.20.19.180 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 814 | 104.25.241.85 | 104.25.241.85 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 833 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 20 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 65 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 86 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 93 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 109 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 141 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 172 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 175 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 247 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 266 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 285 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 331 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 338 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 342 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 344 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 349 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 390 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 391 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 402 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 404 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 451 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 458 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 554 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 557 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 570 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 575 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 620 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 642 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 657 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 672 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 704 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 736 | 172.67.65.150 | 172.67.65.150 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 762 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 798 | 104.26.11.33 | 104.26.11.33 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 799 | 104.20.24.244 | 104.20.24.244 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 805 | 104.25.245.215 | 104.25.245.215 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 825 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 37 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 61 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 134 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 162 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 164 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 171 | bestcf.030101.xyz | 104.17.206.188 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 273 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 311 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 319 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 330 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 340 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 347 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 357 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 377 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 420 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 445 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 558 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 574 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 600 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 636 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 643 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 684 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 741 | 104.17.97.228 | 104.17.97.228 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 774 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 778 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 802 | 172.67.72.212 | 172.67.72.212 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 813 | 104.25.244.36 | 104.25.244.36 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 826 | 172.64.229.15 | 172.64.229.15 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 829 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 834 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 118 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 119 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 152 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 157 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 168 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 184 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 204 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 231 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 307 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 397 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 446 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 479 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 508 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 563 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 592 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 599 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 658 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 673 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 691 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 700 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 709 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 714 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 750 | 104.18.166.232 | 104.18.166.232 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 780 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 782 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 801 | 104.26.4.135 | 104.26.4.135 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 38 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 60 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 161 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 189 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 239 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 278 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 361 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 511 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 551 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 567 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 569 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 602 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 742 | 162.159.39.20 | 162.159.39.20 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 767 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 815 | 104.25.240.123 | 104.25.240.123 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 819 | 104.25.254.14 | 104.25.254.14 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 822 | 162.159.38.83 | 162.159.38.83 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 824 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 99 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 100 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 251 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 252 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 299 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 300 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 305 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 316 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 419 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 548 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 566 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 675 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 810 | 162.159.33.191 | 162.159.33.191 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 185 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 256 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 447 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 656 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 678 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 144 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 350 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 426 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 471 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 581 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 308 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 701 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 830 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 463 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 150 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 794 | 162.159.20.46 | 162.159.20.46 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 562 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 633 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 417 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 475 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 711 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 240 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 607 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 79 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 133 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 143 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 437 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 715 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 723 | 162.159.38.226 | 162.159.38.226 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 88 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 555 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 730 | 162.159.36.205 | 162.159.36.205 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 257 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 401 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 540 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 538 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 641 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 232 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 821 | 172.64.52.181 | 172.64.52.181 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 32 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 748 | 104.25.245.233 | 104.25.245.233 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 755 | 162.159.12.120 | 162.159.12.120 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 253 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 394 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 524 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 546 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 754 | 162.159.38.134 | 162.159.38.134 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 527 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 676 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 788 | 162.159.45.67 | 162.159.45.67 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 26 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 238 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 680 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 259 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 727 | 162.159.39.146 | 162.159.39.146 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 800 | 104.20.22.91 | 104.20.22.91 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 630 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 568 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 725 | 172.64.52.189 | 172.64.52.189 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 261 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 488 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 556 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 787 | 172.64.52.224 | 172.64.52.224 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 2 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 77 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 609 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 779 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 462 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 541 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 101 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 407 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 408 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 461 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 542 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 601 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 634 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 793 | 162.159.0.41 | 162.159.0.41 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 222 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 321 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 427 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 623 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 683 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 726 | 162.159.44.101 | 162.159.44.101 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 25 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 78 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 416 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 611 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 622 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 685 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 230 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 487 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 616 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 624 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 721 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 310 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 687 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 772 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 36 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 135 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 146 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 410 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 460 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 28 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 80 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 224 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 243 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 313 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 325 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 490 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 515 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 722 | 108.162.198.85 | 108.162.198.85 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 27 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 34 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 104 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 124 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 136 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 147 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 225 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 396 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 432 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 442 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 530 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 545 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 612 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 703 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 92 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 154 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 235 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 309 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 312 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 326 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 412 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 537 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 549 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 690 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 705 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 769 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 121 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 234 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 262 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 489 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 543 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 544 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 682 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 29 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 31 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 242 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 398 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 547 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 677 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 693 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 35 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 87 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 90 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 255 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 314 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 411 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 324 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 113 | cloudflare |
| 428 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 30 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 114 | cloudflare |
| 395 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 724 | 172.64.53.165 | 172.64.53.165 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 122 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 254 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 118 | cloudflare |
| 625 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 809 | 162.159.45.150 | 162.159.45.150 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 151 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 610 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 629 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 640 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 695 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 608 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 132 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 123 | cloudflare |
| 571 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 123 | cloudflare |
| 436 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 467 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 283 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 33 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 260 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 323 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 523 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 789 | 108.162.198.198 | 108.162.198.198 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 565 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 128 | cloudflare |
| 760 | 162.159.1.39 | 162.159.1.39 | IPv4 | h2 | ✅ 成功 | 128 | cloudflare |
| 823 | 162.159.43.50 | 162.159.43.50 | IPv4 | h2 | ✅ 成功 | 128 | cloudflare |
| 223 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 757 | 162.159.39.180 | 162.159.39.180 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 759 | 198.41.222.191 | 198.41.222.191 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 282 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 130 | cloudflare |
| 603 | 162.159.45.65 | 162.159.45.65 | IPv4 | h2 | ✅ 成功 | 130 | cloudflare |
| 752 | 162.159.11.128 | 162.159.11.128 | IPv4 | h2 | ✅ 成功 | 130 | cloudflare |
| 120 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 131 | cloudflare |
| 679 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 131 | cloudflare |
| 322 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 758 | 172.64.53.202 | 172.64.53.202 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 776 | 172.64.53.41 | 172.64.53.41 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 145 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 133 | cloudflare |
| 393 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 133 | cloudflare |
| 753 | 162.159.3.128 | 162.159.3.128 | IPv4 | h2 | ✅ 成功 | 134 | cloudflare |
| 381 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 135 | cloudflare |
| 221 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 392 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 811 | 162.159.39.74 | 162.159.39.74 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 790 | 162.159.44.202 | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 281 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 142 | cloudflare |
| 539 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 142 | cloudflare |
| 409 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 143 | cloudflare |
| 486 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 143 | cloudflare |
| 606 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare |
| 627 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 147 | cloudflare |
| 761 | 108.162.198.70 | 108.162.198.70 | IPv4 | h2 | ✅ 成功 | 147 | cloudflare |
| 756 | 162.159.58.17 | 162.159.58.17 | IPv4 | h2 | ✅ 成功 | 149 | cloudflare |
| 628 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 151 | cloudflare |
| 468 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 152 | cloudflare |
| 777 | 162.159.38.45 | 162.159.38.45 | IPv4 | h2 | ✅ 成功 | 152 | cloudflare |
| 406 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 155 | cloudflare |
| 89 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 156 | cloudflare |
| 681 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 160 | cloudflare |
| 429 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 168 | cloudflare |
| 807 | 104.25.247.78 | 104.25.247.78 | IPv4 | h2 | ✅ 成功 | 252 | cloudflare |
| 190 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 370 | cloudflare |
| 334 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 498 | cloudflare |
| 387 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 716 | cloudflare |
| 481 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 750 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 685 条记录
- **正常 (100-200ms)**: 142 条记录
- **慢 (200-500ms)**: 3 条记录
- **很慢 (>500ms)**: 2 条记录


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

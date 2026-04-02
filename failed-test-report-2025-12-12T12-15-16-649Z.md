# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:15:16
- **数据来源**: connectivity_results-20251212-121516.json
- **总测试数**: 444
- **失败测试数**: 2
- **成功测试数**: 442
- **失败率**: 0.45%
- **平均延迟**: 51.50ms
- **最小延迟**: 28ms
- **最大延迟**: 581ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:15:16
- **IP地址**: 2a09:bac5:6213:28::4:38d
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

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 39   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 371  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 221  | cmcc.877774.xyz                       | 104.16.148.7              | IPv4   | h2   | ✅ 成功 | 28       | cloudflare |
| 159  | toy-people.com                        | 2606:4700:20::681a:324    | IPv6   | h2   | ✅ 成功 | 29       | cloudflare |
| 323  | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:8a0    | IPv6   | h2   | ✅ 成功 | 30       | cloudflare |
| 44   | ipv4.ip.sb                            | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 103  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf   | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 120  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90 | IPv6   | h2   | ✅ 成功 | 33       | cloudflare |
| 246  | cf.090227.xyz                         | 104.18.35.15              | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 247  | cf.090227.xyz                         | 172.64.152.241            | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 259  | www.glassdoor.com                     | 104.17.64.70              | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 420  | www.wto.org                           | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 15   | comicabc.com                          | 104.21.64.10              | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 64   | www.gov.ua                            | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 75   | iplocation.io                         | 104.26.11.222             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 110  | www.visa.com.sg                       | 104.18.12.229             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 121  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70 | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 242  | fbi.gov                               | 104.16.149.244            | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 271  | palera.in                             | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 301  | ip.sb                                 | 2606:4700:20::ac43:4bac   | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 309  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 314  | singapore.com                         | 2606:4700:20::ac43:4bc2   | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 326  | whatismyipaddress.com                 | 104.19.222.79             | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 421  | www.wto.org                           | 2606:4700:4406::ac40:9242 | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 9    | www.ipget.net                         | 2606:4700:3036::6815:fd4  | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 18   | comicabc.com                          | 2606:4700:3036::6815:400a | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 56   | 104.18.37.13                          | 104.18.37.13              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 65   | 104.26.6.112                          | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 71   | cf.877774.xyz                         | cf.877774.xyz             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 92   | www.4444.cloudflare.182682.xyz                           | 162.159.153.2             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 140  | na.877774.xyz                         | 104.18.187.25             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 154  | toy-people.com                        | 104.26.2.36               | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 155  | toy-people.com                        | 104.26.3.36               | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 158  | toy-people.com                        | 2606:4700:20::681a:224    | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 186  | www.whatismyip.com                    | 104.26.12.23              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 191  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 204  | cf.zhetengsha.eu.org                  | 172.64.152.241            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 210  | saas.sin.fan                          | 172.67.68.216             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 211  | cmcc.877774.xyz                       | 104.16.149.10             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 241  | fbi.gov                               | 104.16.148.244            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 243  | fbi.gov                               | 2606:4700::6810:94f4      | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 244  | fbi.gov                               | 2606:4700::6810:95f4      | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 249  | cf.090227.xyz                         | 2a06:98c1:3101::ac40:919e | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 257  | www.ipchicken.com                     | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 298  | ip.sb                                 | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 318  | 456.cloudflare.182682.xyz             | 172.67.75.208             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 383  | icook.tw                              | 2606:4700:10::6814:1c4a   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 386  | 104.26.13.31                          | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 425  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 438  | japan.com                             | 2606:4700:20::ac43:465c   | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 17   | comicabc.com                          | 2606:4700:3030::ac43:ae15 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 20   | ipinfo.in                             | 172.67.198.203            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 28   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 51   | ct.877774.xyz                         | 172.64.229.174            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 58   | steamdb.info                          | 172.66.175.250            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 70   | 104.18.254.88                         | 104.18.254.88             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 72   | www.4chan.org                         | 104.16.229.229            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 76   | iplocation.io                         | 172.67.70.100             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 79   | iplocation.io                         | 2606:4700:20::ac43:4664   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 85   | 172.67.120.0                          | 172.67.120.0              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 97   | www.okcupid.com                       | 104.18.160.63             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 108  | 4444.cloudflare.182682.xyz                         | 2606:4700:3033::6815:50b4 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 111  | 172.67.243.218                        | 172.67.243.218            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 115  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 157  | toy-people.com                        | 2606:4700:20::ac43:4812   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 163  | zread.ai                              | 2606:4700:3032::ac43:ca4e | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 182  | asia.877774.xyz                       | 104.17.142.146            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 205  | cf.zhetengsha.eu.org                  | 104.18.35.15              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 209  | saas.sin.fan                          | 104.26.1.111              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 215  | cmcc.877774.xyz                       | 104.16.148.1              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 217  | cmcc.877774.xyz                       | 104.16.148.3              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 260  | www.glassdoor.com                     | 104.16.25.46              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 300  | ip.sb                                 | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 303  | ip.sb                                 | 2606:4700:20::681a:d1f    | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 306  | ip.gs                                 | 2606:4700:3036::6815:eb0  | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 312  | singapore.com                         | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 315  | singapore.com                         | 2606:4700:20::681a:c8c    | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 324  | 104.18.42.26                          | 104.18.42.26              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 327  | whatismyipaddress.com                 | 104.19.223.79             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 342  | dnschecker.org                        | 2606:4700:20::ac43:49d8   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 348  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 349  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad   | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 367  | tasteatlas.com                        | 104.17.37.105             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 368  | tasteatlas.com                        | 104.17.36.105             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 375  | www.udemy.com                         | 2606:4700::6810:8eed      | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 388  | www.digitalocean.com                  | 104.19.174.68             | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 391  | www.digitalocean.com                  | 2606:4700::6813:ae44      | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 411  | 104.19.175.123                        | 104.19.175.123            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 416  | ifconfig.co                           | 2606:4700:3030::ac43:a86a | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 434  | japan.com                             | 172.67.70.92              | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 22   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 42   | 104.16.45.84                          | 104.16.45.84              | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 47   | 172.67.75.172                         | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 52   | ct.877774.xyz                         | 172.64.229.185            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 61   | www.gov.ua                            | 172.67.209.127            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 62   | www.gov.ua                            | 104.21.23.72              | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 68   | 103.160.204.59                        | 103.160.204.59            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 77   | iplocation.io                         | 2606:4700:20::681a:bde    | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 84   | icook.hk                              | 2606:4700:3037::ac43:a168 | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 100  | www.hugedomains.com                   | 172.67.70.191             | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 105  | 4444.cloudflare.182682.xyz                         | 172.67.152.183            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 119  | yx-auto.pages.dev                     | 172.66.47.112             | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 100 条记录
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

_此报告由 HTTP/3 连接测试报告生成器自动生成_

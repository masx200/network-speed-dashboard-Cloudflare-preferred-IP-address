# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 15:19:38
- **数据来源**: connectivity_results-20251212-151938.json
- **总测试数**: 453
- **失败测试数**: 2
- **成功测试数**: 451
- **失败率**: 0.44%
- **平均延迟**: 64.02ms
- **最小延迟**: 41ms
- **最大延迟**: 1157ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 15:19:38
- **IP地址**: 2a09:bac5:6212:1250::1d3:43
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
| 141  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 270  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                               | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------------------ | ------ | ---- | ------- | -------- | ---------- |
| 199  | 104.26.13.31                          | 104.26.13.31                         | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 286  | zread.ai                              | 2606:4700:3032::ac43:ca4e            | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 140  | 172.67.79.211                         | 172.67.79.211                        | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 148  | icook.tw                              | 2606:4700:10::6814:1c4a              | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 152  | www.digitalocean.com                  | 104.19.174.68                        | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 195  | steamdb.info                          | 2606:4700:10::6814:22d4              | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 202  | www.gov.ua                            | 2606:4700:3031::6815:1748            | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 211  | ct.877774.xyz                         | 172.64.229.185                       | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 242  | www.wto.org                           | 104.18.41.190                        | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 432  | cf.zhetengsha.eu.org                  | 104.18.42.98                         | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 444  | fbi.gov                               | 104.16.149.244                       | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 53   | 104.17.68.85                          | 104.17.68.85                         | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 67   | ip.sb                                 | 2606:4700:20::ac43:4bac              | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 130  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2              | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 157  | eur.877774.xyz                        | 104.21.29.164                        | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 181  | ipinfo.in                             | 2606:4700:3031::6815:1581            | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 250  | japan.com                             | 2606:4700:20::681a:53c               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 287  | zread.ai                              | 2606:4700:3033::6815:4cf0            | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 290  | www.hugedomains.com                   | 172.67.70.191                        | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 291  | www.hugedomains.com                   | 2606:4700:20::681a:725               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 296  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7            | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 382  | toy-people.com                        | 2606:4700:20::ac43:4812              | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 65   | ip.sb                                 | 104.26.12.31                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 87   | singapore.com                         | 2606:4700:20::ac43:4bc2              | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 93   | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0              | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 94   | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:8a0               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 151  | www.digitalocean.com                  | 104.19.173.68                        | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 154  | www.digitalocean.com                  | 2606:4700::6813:ae44                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 251  | icook.hk                              | 104.21.90.210                        | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 255  | 172.67.49.134                         | 172.67.49.134                        | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 271  | cfip.xxxxxxxx.tk                      | 104.16.232.223                       | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 366  | cmcc.877774.xyz                       | 104.16.148.3                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 377  | cmcc.877774.xyz                       | 104.16.149.5                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 387  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b              | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 407  | asia.877774.xyz                       | 104.17.139.62                        | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 421  | bestcf.030101.xyz                     | 104.19.153.222                       | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 424  | cloudflare-ip.mofashi.ltd             | 104.21.72.233                        | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 433  | cf.zhetengsha.eu.org                  | 172.64.145.158                       | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 9    | www.ipchicken.com                     | 104.26.7.112                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 11   | www.ipget.net                         | 104.21.15.212                        | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 55   | palera.in                             | 104.21.58.72                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 70   | ip.gs                                 | 104.21.14.176                        | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 75   | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6            | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 77   | whatismyipaddress.com                 | 104.19.222.79                        | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 85   | singapore.com                         | 2606:4700:20::681a:c8c               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 91   | 456.cloudflare.182682.xyz                          | 104.26.9.160                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 100  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 132  | tasteatlas.com                        | 104.17.37.105                        | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 143  | 104.18.37.40                          | 104.18.37.40                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 150  | 104.18.37.13                          | 104.18.37.13                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 155  | eur.877774.xyz                        | 104.21.47.209                        | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 171  | www.pcmag.com                         | 104.16.20.118                        | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 188  | ipv4.ip.sb                            | 104.26.13.31                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 197  | shopify.com                           | 23.227.38.33                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 205  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47             | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 218  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb            | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 272  | cfip.xxxxxxxx.tk                      | 104.16.241.229                       | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 281  | 172.67.120.0                          | 172.67.120.0                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 288  | www.hugedomains.com                   | 104.26.7.37                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 302  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb              | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 329  | na.877774.xyz                         | 104.19.74.233                        | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 333  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70            | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 425  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac            | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 435  | cf.zhetengsha.eu.org                  | 2a06:98c1:3101::ac40:919e            | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 436  | xn--b6gac.eu.org                      | 172.67.153.253                       | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 36   | time.is                               | 2606:4700:20::ac43:449d              | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 51   | 104.16.61.163                         | 104.16.61.163                        | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 84   | singapore.com                         | 104.26.12.140                        | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 110  | dnschecker.org                        | 2606:4700:20::681a:659               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 112  | dnschecker.org                        | 2606:4700:20::ac43:49d8              | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 124  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242            | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 134  | tasteatlas.com                        | 2606:4700::6811:2569                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 139  | www.udemy.com                         | 2606:4700::6810:8fed                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 153  | www.digitalocean.com                  | 2606:4700::6813:ad44                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 170  | www.pcmag.com                         | 104.16.21.118                        | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 189  | ipv4.ip.sb                            | 104.26.12.31                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 192  | 104.16.45.84                          | 104.16.45.84                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 194  | steamdb.info                          | 104.20.34.212                        | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 209  | ct.877774.xyz                         | 172.64.229.173                       | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 210  | ct.877774.xyz                         | 172.64.229.174                       | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 220  | 104.26.6.112                          | 104.26.6.112                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 246  | japan.com                             | 172.67.70.92                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 257  | www.4chan.org                         | 104.16.228.229                       | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 277  | www.okcupid.com                       | 104.18.160.63                        | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 279  | www.okcupid.com                       | 104.16.223.254                       | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 292  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf              | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 297  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4            | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 310  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1              | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 370  | cmcc.877774.xyz                       | 104.16.149.12                        | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 371  | cmcc.877774.xyz                       | 104.16.149.11                        | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 381  | toy-people.com                        | 2606:4700:20::681a:324               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 422  | bestcf.030101.xyz                     | 2606:4700:0:b684:c5c1:5d02:e603:436c | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 430  | saas.sin.fan                          | 162.159.36.5                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 442  | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae            | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 8    | www.ipchicken.com                     | 104.26.6.112                         | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 38   | time.is                               | 2606:4700:20::681a:d36               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 64   | ip.sb                                 | 104.26.13.31                         | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 83   | singapore.com                         | 172.67.75.194                        | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 98   | 162.159.133.85                        | 162.159.133.85                       | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 111  | dnschecker.org                        | 2606:4700:20::681a:759               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |

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

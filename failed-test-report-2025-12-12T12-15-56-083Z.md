# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:15:56
- **数据来源**: connectivity_results-20251212-121555.json
- **总测试数**: 444
- **失败测试数**: 2
- **成功测试数**: 442
- **失败率**: 0.45%
- **平均延迟**: 54.15ms
- **最小延迟**: 36ms
- **最大延迟**: 679ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:15:56
- **IP地址**: 2a09:bac1:76c0:28::1d2:8c
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
| 45   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 352  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

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
| 423  | cmcc.877774.xyz                       | 104.16.149.244            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 75   | iplocation.io                         | 2606:4700:20::681a:bde    | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 77   | iplocation.io                         | 2606:4700:20::ac43:4664   | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 196  | ip.sb                                 | 2606:4700:20::ac43:4bac   | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 410  | japan.com                             | 2606:4700:20::681a:53c    | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 38   | 172.67.75.172                         | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 62   | ct.877774.xyz                         | 172.64.229.236            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 113  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 150  | toy-people.com                        | 2606:4700:20::ac43:4812   | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 168  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 214  | fbi.gov                               | 2606:4700::6810:95f4      | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 227  | www.ipchicken.com                     | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 292  | 456.cloudflare.182682.xyz             | 104.26.9.160              | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 321  | eur.877774.xyz                        | 104.21.47.209             | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 341  | tasteatlas.com                        | 104.17.36.105             | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 346  | www.udemy.com                         | 104.16.142.237            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 348  | www.udemy.com                         | 2606:4700::6810:8fed      | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 382  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1  | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 51   | www.gov.ua                            | 2606:4700:3033::ac43:d17f | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 90   | 4444.cloudflare.182682.xyz                         | 2606:4700:3033::6815:50b4 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 163  | 104.16.223.179                        | 104.16.223.179            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 185  | www.whatismyip.com                    | 2606:4700:20::ac43:4581   | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 192  | ip.sb                                 | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 208  | cf.zhetengsha.eu.org                  | 104.18.35.15              | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 216  | cf.090227.xyz                         | 104.18.35.15              | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 217  | cf.090227.xyz                         | 172.64.152.241            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 247  | time.is                               | 2606:4700:20::ac43:449d   | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 260  | palera.in                             | 2606:4700:3035::6815:3a48 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 265  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd   | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 279  | www.visa.com.hk                       | 104.18.20.69              | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 288  | whatismyipaddress.com                 | 2606:4700::6813:df4f      | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 289  | whatismyipaddress.com                 | 2606:4700::6813:de4f      | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 290  | 456.cloudflare.182682.xyz             | 172.67.75.208             | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 293  | 456.cloudflare.182682.xyz             | 2606:4700:20::ac43:4bd0   | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 294  | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:9a0    | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 297  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 318  | 162.159.133.85                        | 162.159.133.85            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 366  | www.digitalocean.com                  | 2606:4700::6813:ae44      | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 397  | www.wto.org                           | 104.18.41.190             | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 398  | www.wto.org                           | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 411  | japan.com                             | 2606:4700:20::681a:43c    | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 412  | japan.com                             | 2606:4700:20::ac43:465c   | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 421  | cmcc.877774.xyz                       | 104.16.149.11             | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 10   | comicabc.com                          | 2606:4700:3030::ac43:ae15 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 18   | www.pcmag.com                         | 2606:4700::6810:1576      | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 22   | ipv4.ip.sb                            | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 26   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 36   | 104.16.45.84                          | 104.16.45.84              | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 52   | www.gov.ua                            | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 60   | ct.877774.xyz                         | 172.64.229.195            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 61   | ct.877774.xyz                         | 172.64.229.217            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 68   | icook.hk                              | 172.67.161.104            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 69   | icook.hk                              | 104.21.90.210             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 70   | icook.hk                              | 2606:4700:3031::6815:5ad2 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 71   | icook.hk                              | 2606:4700:3037::ac43:a168 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 99   | www.hugedomains.com                   | 104.26.6.37               | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 100  | www.hugedomains.com                   | 172.67.70.191             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 111  | yx-auto.pages.dev                     | 172.66.44.144             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 126  | cu.877774.xyz                         | 104.26.4.119              | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 134  | na.877774.xyz                         | 104.19.74.233             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 138  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7   | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 153  | zread.ai                              | 172.67.202.78             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 182  | www.whatismyip.com                    | 104.26.13.23              | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 213  | fbi.gov                               | 2606:4700::6810:94f4      | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 235  | www.glassdoor.com                     | 104.17.64.70              | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 237  | 104.16.61.163                         | 104.16.61.163             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 249  | time.is                               | 2606:4700:20::681a:d36    | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 255  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94   | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 261  | 104.17.68.85                          | 104.17.68.85              | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 262  | benedict.ns.cloudflare.com            | 162.159.44.205            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 274  | ip.gs                                 | 172.67.160.28             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 277  | ip.gs                                 | 2606:4700:3035::ac43:a01c | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 281  | singapore.com                         | 104.26.12.140             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 291  | 456.cloudflare.182682.xyz             | 104.26.8.160              | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 302  | trevor.ns.cloudflare.com              | 172.64.35.154             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 303  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a   | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 305  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a   | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 311  | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e   | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 317  | dnschecker.org                        | 2606:4700:20::681a:659    | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 319  | eur.877774.xyz                        | 104.21.29.164             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 322  | cf.877774.xyz                         | 104.18.41.190             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 331  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1   | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 365  | www.digitalocean.com                  | 2606:4700::6813:ad44      | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 370  | 104.19.223.58                         | 104.19.223.58             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 6    | asia.877774.xyz                       | 104.17.142.146            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 21   | ipv4.ip.sb                            | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 29   | steamdb.info                          | 172.66.175.250            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 33   | ipinfo.in                             | 172.67.198.203            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 49   | www.gov.ua                            | 172.67.209.127            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 50   | www.gov.ua                            | 104.21.23.72              | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 53   | 103.160.204.59                        | 103.160.204.59            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 57   | ct.877774.xyz                         | 172.64.229.173            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 64   | cf.877774.xyz                         | cf.877774.xyz             | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 66   | www.4chan.org                         | 104.16.229.229            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 72   | iplocation.io                         | 104.26.10.222             | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 74   | iplocation.io                         | 172.67.70.100             | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 76   | iplocation.io                         | 2606:4700:20::681a:ade    | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 87   | 4444.cloudflare.182682.xyz                         | 172.67.152.183            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 88   | 4444.cloudflare.182682.xyz                         | 104.21.80.180             | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 93   | www.okcupid.com                       | 104.17.48.63              | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |

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

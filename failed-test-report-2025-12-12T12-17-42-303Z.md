# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:17:42
- **数据来源**: connectivity_results-20251212-121741.json
- **总测试数**: 444
- **失败测试数**: 2
- **成功测试数**: 442
- **失败率**: 0.45%
- **平均延迟**: 91.02ms
- **最小延迟**: 55ms
- **最大延迟**: 610ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:17:42
- **IP地址**: 2a09:bac5:7491:2628::3cd:4f
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.6021, -93.6124
- **时区**: America/Chicago
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
| 37   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 288  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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
| 231  | whatismyipaddress.com                 | 2606:4700::6813:de4f      | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 217  | ip.gs                                 | 104.21.14.176             | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 298  | 104.26.13.31                          | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 365  | ipv4.ip.sb                            | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 221  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 228  | whatismyipaddress.com                 | 104.19.222.79             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 327  | ifconfig.co                           | 2606:4700:3037::6815:365b | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 336  | stock.hostmonit.com                   | 104.21.7.193              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 347  | japan.com                             | 104.26.5.60               | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 368  | 172.67.75.172                         | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 386  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 191  | time.is                               | 2606:4700:20::681a:d36    | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 203  | palera.in                             | 2606:4700:3035::6815:3a48 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 215  | ip.sb                                 | 2606:4700:20::681a:d1f    | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 236  | 456.cloudflare.182682.xyz             | 2606:4700:20::ac43:4bd0   | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 270  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 281  | tasteatlas.com                        | 2606:4700::6811:2469      | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 302  | www.digitalocean.com                  | 2606:4700::6813:ad44      | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 325  | ifconfig.co                           | 172.67.168.106            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 372  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 382  | 104.26.6.112                          | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 385  | cf.0sm.com                            | 2606:4700:3032::6815:785  | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 218  | ip.gs                                 | 2606:4700:3036::6815:eb0  | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 224  | singapore.com                         | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 225  | singapore.com                         | 2606:4700:20::681a:d8c    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 227  | singapore.com                         | 2606:4700:20::681a:c8c    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 237  | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:9a0    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 239  | www.visa.com.hk                       | 104.18.21.69              | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 244  | 162.159.133.85                        | 162.159.133.85            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 254  | dnschecker.org                        | 104.26.6.89               | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 259  | cf.877774.xyz                         | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 284  | www.udemy.com                         | 104.16.142.237            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 286  | www.udemy.com                         | 2606:4700::6810:8fed      | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 305  | 104.19.223.58                         | 104.19.223.58             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 330  | www.csgo.com                          | 195.85.59.161             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 332  | www.wto.org                           | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 364  | ipv4.ip.sb                            | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 367  | shopify.com                           | 23.227.38.33              | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 369  | 104.18.37.13                          | 104.18.37.13              | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 370  | ipinfo.in                             | 104.21.21.129             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 371  | ipinfo.in                             | 172.67.198.203            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 373  | ipinfo.in                             | 2606:4700:3031::6815:1581 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 376  | steamdb.info                          | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 377  | steamdb.info                          | 2606:4700:10::ac42:affa   | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 396  | iplocation.io                         | 2606:4700:20::681a:ade    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 411  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a   | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 440  | cmcc.877774.xyz                       | 104.16.149.10             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 202  | palera.in                             | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 214  | ip.sb                                 | 2606:4700:20::681a:c1f    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 255  | dnschecker.org                        | 2606:4700:20::681a:759    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 257  | dnschecker.org                        | 2606:4700:20::681a:659    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 261  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 267  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 293  | icook.tw                              | 172.66.158.115            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 296  | icook.tw                              | 2606:4700:10::ac42:9e73   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 297  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 299  | 104.18.78.214                         | 104.18.78.214             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 300  | www.digitalocean.com                  | 104.19.174.68             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 301  | www.digitalocean.com                  | 104.19.173.68             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 303  | www.digitalocean.com                  | 2606:4700::6813:ae44      | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 304  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47  | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 307  | eur.877774.xyz                        | 104.21.29.164             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 321  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 329  | www.csgo.com                          | 195.85.59.95              | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 348  | japan.com                             | 2606:4700:20::681a:53c    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 360  | www.pcmag.com                         | 104.16.20.118             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 362  | www.pcmag.com                         | 2606:4700::6810:1476      | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 375  | steamdb.info                          | 172.66.175.250            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 388  | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc  | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 391  | iplocation.io                         | 104.26.11.222             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 394  | iplocation.io                         | 2606:4700:20::681a:bde    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 395  | iplocation.io                         | 2606:4700:20::ac43:4664   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 428  | cmcc.877774.xyz                       | 104.16.148.11             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 434  | cmcc.877774.xyz                       | 104.16.149.4              | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 443  | cmcc.877774.xyz                       | 104.16.149.244            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 208  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 226  | singapore.com                         | 2606:4700:20::ac43:4bc2   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 229  | whatismyipaddress.com                 | 104.19.223.79             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 241  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 251  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 260  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 269  | gamer.com.tw                          | 104.18.2.197              | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 277  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 278  | 104.17.142.12                         | 104.17.142.12             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 280  | tasteatlas.com                        | 104.17.36.105             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 289  | cfip.1323123.xyz                      | 104.16.133.220            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 290  | 198.62.62.4                           | 198.62.62.4               | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 291  | 104.18.37.40                          | 104.18.37.40              | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 310  | 172.67.181.209                        | 172.67.181.209            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 326  | ifconfig.co                           | 104.21.54.91              | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 328  | ifconfig.co                           | 2606:4700:3030::ac43:a86a | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 331  | www.wto.org                           | 104.18.41.190             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 334  | www.wto.org                           | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 353  | 104.16.45.84                          | 104.16.45.84              | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 359  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 361  | www.pcmag.com                         | 104.16.21.118             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 366  | ipv4.ip.sb                            | 104.26.12.31              | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 381  | www.gov.ua                            | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 387  | 172.67.110.232                        | 172.67.110.232            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 389  | 103.160.204.59                        | 103.160.204.59            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
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
